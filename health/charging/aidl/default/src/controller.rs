// SPDX-FileCopyrightText: The halogenOS Project
// SPDX-License-Identifier: Apache-2.0

//! Applies the latest target to the battery each time the target or a power supply changes.

use std::{cmp::Ordering, convert::Infallible};

use log::error;
use tokio::sync::watch;

use crate::{
    battery::BatteryLevel,
    charger::{ChargeMode, Charger},
    error::{describe, ServiceError},
    power_supply::PowerSupplyEvents,
};

/// Sets the charger from the target and the battery level.
pub(crate) struct Controller<C> {
    charger: C,
    /// The latest target. `None` means normal charging.
    targets: watch::Receiver<Option<BatteryLevel>>,
}

impl<C: Charger> Controller<C> {
    pub(crate) fn new(charger: C, targets: watch::Receiver<Option<BatteryLevel>>) -> Self {
        Self { charger, targets }
    }

    /// Runs until the uevents stop.
    pub(crate) async fn run(
        mut self,
        power_supply: PowerSupplyEvents,
    ) -> Result<Infallible, ServiceError> {
        loop {
            self.apply();
            tokio::select! {
                Ok(()) = self.targets.changed() => {}
                changed = power_supply.changed() => changed?,
            }
        }
    }

    fn apply(&mut self) {
        let target = *self.targets.borrow_and_update();
        let mode = target.map_or(ChargeMode::Charge, |target| {
            BatteryLevel::read().map_or_else(
                |error| {
                    // Hold neither drains the battery nor charges it past the target.
                    error!("Cannot read the battery level, using Hold: {}", describe(&error));
                    ChargeMode::Hold
                },
                |level| mode_toward(target, level),
            )
        });
        if let Err(error) = self.charger.set(mode) {
            // The next power supply uevent calls `apply` again.
            error!("Cannot set the charger to {mode:?}: {}", describe(&error));
        }
    }
}

/// The mode that moves the battery from `level` to `target`.
fn mode_toward(target: BatteryLevel, level: BatteryLevel) -> ChargeMode {
    match level.cmp(&target) {
        Ordering::Less => ChargeMode::Charge,
        Ordering::Equal => ChargeMode::Hold,
        Ordering::Greater => ChargeMode::Drain,
    }
}
