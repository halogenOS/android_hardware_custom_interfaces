// SPDX-FileCopyrightText: The halogenOS Project
// SPDX-License-Identifier: Apache-2.0

//! The level of the battery, read from the battery driver.

use std::ops::RangeInclusive;

use crate::{error::Error, sysfs::Attribute};

/// The level shown to the user by the system, from the battery driver.
const CAPACITY: Attribute = Attribute::new("/sys/class/power_supply/battery/capacity");

const PERCENT: RangeInclusive<i32> = 0..=100;

/// A battery level in percent.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct BatteryLevel(u8);

impl BatteryLevel {
    /// Reads the battery level.
    pub(crate) fn read() -> Result<Self, Error> {
        let value = CAPACITY.read()?;
        Self::within(PERCENT, value)
            .ok_or(Error::UnexpectedValue { attribute: CAPACITY.path(), value })
    }

    /// The level of `percent`, if `levels` contains it.
    pub(crate) fn within(levels: RangeInclusive<i32>, percent: i32) -> Option<Self> {
        u8::try_from(percent).ok().filter(|_| levels.contains(&percent)).map(Self)
    }
}
