// SPDX-FileCopyrightText: The halogenOS Project
// SPDX-License-Identifier: Apache-2.0

//! The band of levels set by a target, and the charger mode for each battery level.

use std::cmp::Ordering;

use crate::{battery::BatteryLevel, charger::ChargeMode};

/// The levels the battery stays between, from the limit up to the limit plus the variation.
#[derive(Clone, Copy)]
pub(crate) struct Band {
    bottom: BatteryLevel,
    top: BatteryLevel,
}

impl Band {
    /// The band that contains only `level`.
    pub(crate) fn at(level: BatteryLevel) -> Self {
        Self { bottom: level, top: level }
    }

    /// The band from `bottom` up to `variation` percent above it, if a battery can have that level.
    pub(crate) fn above(bottom: BatteryLevel, variation: i32) -> Option<Self> {
        bottom.raised_by(variation).map(|top| Self { bottom, top })
    }

    /// The mode for the battery at `level` after the charger was set to `previous`. The battery
    /// rests at the top. Once it falls to the bottom, it charges back up to the top. The charger
    /// switches less often than it would for a single level. A band without variation has the
    /// limit as both bottom and top. The battery stays at the limit.
    pub(crate) fn mode_at(self, level: BatteryLevel, previous: ChargeMode) -> ChargeMode {
        match level.cmp(&self.top) {
            Ordering::Less if level <= self.bottom || previous == ChargeMode::Charge => {
                ChargeMode::Charge
            }
            Ordering::Less | Ordering::Equal => ChargeMode::Hold,
            Ordering::Greater => ChargeMode::Drain,
        }
    }
}
