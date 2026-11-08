// SPDX-FileCopyrightText: The halogenOS Project
// SPDX-License-Identifier: Apache-2.0

//! The binder side of the flows attached by the system. The latest target emitted by the attached
//! flow is published to the controller, and `None` once that flow dies.

use binder::{BinderFeatures, DeathRecipient, IBinder, Interface, Status, StatusCode, Strong};
use custom_hardware_health_charging::aidl::custom::hardware::health::charging::{
    ChargeLimitTarget::{ChargeLimitTarget, MAX_LEVEL_PERCENT, MIN_LEVEL_PERCENT},
    IChargeLimitCollector::{BnChargeLimitCollector, IChargeLimitCollector},
    IChargeLimitFlow::IChargeLimitFlow,
};
use log::{error, info, warn};
use tokio::sync::{mpsc, watch};

use crate::{
    band::Band,
    battery::BatteryLevel,
    error::{describe, ServiceError},
};

/// What the system did. Each event contains the flow it came from.
enum Event {
    /// The system attached a flow that emits targets.
    Attached(Strong<dyn IChargeLimitFlow>),
    /// A flow emitted a target. `None` means normal charging.
    Emitted { flow: Strong<dyn IChargeLimitFlow>, target: Option<Band> },
    /// A flow died with the process that published it.
    FlowDied(Strong<dyn IChargeLimitFlow>),
}

/// Sends the flows attached by the system to [`Flows`] from binder threads.
pub(crate) struct Attachments(mpsc::Sender<Event>);

impl Attachments {
    pub(crate) fn attach(&self, flow: &Strong<dyn IChargeLimitFlow>) -> binder::Result<()> {
        send(&self.0, Event::Attached(flow.clone()))
    }
}

/// Collects the latest attached flow and publishes the targets emitted by that flow.
pub(crate) struct Flows {
    /// Handed to the collectors and death recipients of the subscriptions.
    events: mpsc::Sender<Event>,
    inbox: mpsc::Receiver<Event>,
    subscription: Option<Subscription>,
    targets: watch::Sender<Option<Band>>,
}

impl Flows {
    /// Returns the flows, the sender for attached flows, and the receiver for targets.
    pub(crate) fn new() -> (Self, Attachments, watch::Receiver<Option<Band>>) {
        let (events, inbox) = mpsc::channel(1);
        let (targets, receiver) = watch::channel(None);
        let flows = Self { events: events.clone(), inbox, subscription: None, targets };
        (flows, Attachments(events), receiver)
    }

    /// Runs as long as the process. `Flows` contains a sender for the channel, so the channel never
    /// closes.
    pub(crate) async fn run(mut self) {
        while let Some(event) = self.inbox.recv().await {
            self.handle(event);
        }
    }

    fn handle(&mut self, event: Event) {
        match event {
            Event::Attached(flow) => {
                self.subscription = Subscription::start(flow, &self.events)
                    .inspect_err(|status| warn!("The attached flow died at once: {status}"))
                    .ok();
                self.publish(None);
            }
            Event::Emitted { flow, target } => {
                if self.subscription.as_ref().is_some_and(|s| s.flow == flow) {
                    self.publish(target);
                }
            }
            Event::FlowDied(flow) => {
                if self.subscription.take_if(|s| s.flow == flow).is_some() {
                    info!("The flow that emits targets died, charging normally");
                    self.publish(None);
                }
            }
        }
    }

    fn publish(&self, target: Option<Band>) {
        self.targets.send_modify(|latest| *latest = target);
    }
}

/// The flow collected by the HAL.
struct Subscription {
    flow: Strong<dyn IChargeLimitFlow>,
    /// Reports when the flow dies, until the subscription is dropped.
    _death: DeathRecipient,
}

impl Subscription {
    fn start(
        flow: Strong<dyn IChargeLimitFlow>,
        events: &mpsc::Sender<Event>,
    ) -> Result<Self, Status> {
        let mut death = DeathRecipient::new({
            let (flow, events) = (flow.clone(), events.clone());
            move || {
                if let Err(status) = send(&events, Event::FlowDied(flow.clone())) {
                    error!("Cannot report that a flow died: {status}");
                }
            }
        });
        flow.as_binder().link_to_death(&mut death)?;
        flow.collect(&Collector::new_binder(flow.clone(), events.clone()))?;
        Ok(Self { flow, _death: death })
    }
}

/// Collects the targets emitted by one flow.
struct Collector {
    flow: Strong<dyn IChargeLimitFlow>,
    events: mpsc::Sender<Event>,
}

impl Collector {
    fn new_binder(
        flow: Strong<dyn IChargeLimitFlow>,
        events: mpsc::Sender<Event>,
    ) -> Strong<dyn IChargeLimitCollector> {
        BnChargeLimitCollector::new_binder(Self { flow, events }, BinderFeatures::default())
    }
}

impl Interface for Collector {}

impl IChargeLimitCollector for Collector {
    fn emit(&self, target: Option<&ChargeLimitTarget>) -> binder::Result<()> {
        let target = target.map(Band::try_from).transpose().unwrap_or_else(|error| {
            error!("{}, charging normally", describe(&error));
            None
        });
        send(&self.events, Event::Emitted { flow: self.flow.clone(), target })
    }
}

impl TryFrom<&ChargeLimitTarget> for Band {
    type Error = ServiceError;

    fn try_from(target: &ChargeLimitTarget) -> Result<Self, ServiceError> {
        let ChargeLimitTarget { levelPercent: limit, variationPercent: variation } = *target;
        let bottom = BatteryLevel::within(MIN_LEVEL_PERCENT..=MAX_LEVEL_PERCENT, limit)
            .ok_or(ServiceError::TargetOutOfRange(limit))?;
        Ok(Self::above(bottom, variation).unwrap_or_else(|| {
            // The interface reads a variation outside the allowed range as 0.
            warn!("The variation of {variation}% does not fit above {limit}%, using 0");
            Self::at(bottom)
        }))
    }
}

/// Sends an event to [`Flows`] from a binder thread. It fails only once the flows have stopped,
/// and the process ends then.
fn send(events: &mpsc::Sender<Event>, event: Event) -> binder::Result<()> {
    events.blocking_send(event).map_err(|_stopped| StatusCode::DEAD_OBJECT.into())
}
