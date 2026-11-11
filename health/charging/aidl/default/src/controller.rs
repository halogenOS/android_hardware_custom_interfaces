// SPDX-FileCopyrightText: The halogenOS Project
// SPDX-License-Identifier: Apache-2.0

//! Applies the latest target to the battery each time the target or a power supply changes.

use std::convert::Infallible;

use log::error;
use tokio::sync::watch;

use crate::{
    band::Band,
    battery::BatteryLevel,
    charger::{ChargeMode, Charger, SetReason},
    error::{describe, ServiceError},
    power_supply::PowerSupplyEvents,
};

/// Sets the charger from the target and the battery level.
pub(crate) struct Controller<C> {
    charger: C,
    /// The latest target. `None` means normal charging.
    targets: watch::Receiver<Option<Band>>,
    /// The mode set last affects whether a battery inside the band charges or rests.
    mode: ChargeMode,
}

impl<C: Charger> Controller<C> {
    pub(crate) fn new(charger: C, targets: watch::Receiver<Option<Band>>) -> Self {
        Self { charger, targets, mode: ChargeMode::NORMAL }
    }

    /// Runs until the uevents stop.
    pub(crate) async fn run(
        mut self,
        power_supply: PowerSupplyEvents,
    ) -> Result<Infallible, ServiceError> {
        let mut reason = SetReason::Target;
        loop {
            self.apply(&reason);
            reason = tokio::select! {
                Ok(()) = self.targets.changed() => SetReason::Target,
                changed = power_supply.changed() => changed?,
            };
        }
    }

    fn apply(&mut self, reason: &SetReason) {
        let target = *self.targets.borrow_and_update();
        let mode = target.map_or(ChargeMode::NORMAL, |target| {
            BatteryLevel::read().map_or_else(
                |error| {
                    // Hold neither drains the battery nor charges it past the target.
                    error!("Cannot read the battery level, using Hold: {}", describe(&error));
                    ChargeMode::Hold
                },
                |level| target.mode_at(level, self.mode),
            )
        });
        self.mode = mode;
        if let Err(error) = self.charger.set(mode, reason) {
            // The next power supply uevent calls `apply` again.
            error!("Cannot set the charger to {mode:?}: {}", describe(&error));
        }
    }
}
