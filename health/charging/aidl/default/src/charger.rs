// SPDX-FileCopyrightText: The halogenOS Project
// SPDX-License-Identifier: Apache-2.0

//! The charger of a device and the modes it is set to.

use log::error;

use crate::error::{describe, Error};

/// What the charger does with the power supplied by a plugged-in charger.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChargeMode {
    /// The charger charges the battery.
    Charge,
    /// The charger powers the phone without charging the battery.
    Hold,
    /// The phone runs on battery while the charger is plugged in.
    Drain,
}

impl ChargeMode {
    /// Normal charging: the mode without a target and after the HAL stops.
    pub(crate) const NORMAL: Self = Self::Charge;
}

/// The charger of one device. The library calls `set` on each new target and each power supply
/// uevent, and with [`ChargeMode::Charge`] when the HAL stops.
pub trait Charger {
    /// Sets the charger to `mode`.
    ///
    /// # Errors
    ///
    /// The error from a driver attribute that could not be read or written. The next power supply
    /// uevent calls `set` again.
    fn set(&mut self, mode: ChargeMode) -> Result<(), Error>;
}

/// The charger of the device. Dropping it restores normal charging.
pub(crate) struct RestoringCharger<C: Charger>(C);

impl<C: Charger> RestoringCharger<C> {
    pub(crate) fn new(charger: C) -> Self {
        Self(charger)
    }
}

impl<C: Charger> Charger for RestoringCharger<C> {
    fn set(&mut self, mode: ChargeMode) -> Result<(), Error> {
        self.0.set(mode)
    }
}

impl<C: Charger> Drop for RestoringCharger<C> {
    fn drop(&mut self) {
        if let Err(error) = self.set(ChargeMode::NORMAL) {
            error!("Cannot return to normal charging: {}", describe(&error));
        }
    }
}
