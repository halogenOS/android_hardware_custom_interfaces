// SPDX-FileCopyrightText: The halogenOS Project
// SPDX-License-Identifier: Apache-2.0

//! Number attributes and switches exposed in sysfs by kernel drivers.

use std::fs;

use log::info;

use crate::error::Error;

/// A sysfs attribute that contains a decimal number.
pub struct Attribute {
    path: &'static str,
}

impl Attribute {
    /// The attribute at `path`.
    #[must_use]
    pub const fn new(path: &'static str) -> Self {
        Self { path }
    }

    /// The path of the attribute.
    pub(crate) const fn path(&self) -> &'static str {
        self.path
    }

    /// Reads the number in the attribute.
    ///
    /// # Errors
    ///
    /// [`Error::ReadAttribute`] or [`Error::ParseAttribute`].
    pub fn read(&self) -> Result<i32, Error> {
        fs::read_to_string(self.path)
            .map_err(|error| Error::ReadAttribute(self.path, error))?
            .trim()
            .parse()
            .map_err(|error| Error::ParseAttribute(self.path, error))
    }

    /// Writes `value` to the attribute.
    pub(crate) fn write(&self, value: i32) -> Result<(), Error> {
        fs::write(self.path, value.to_string())
            .map_err(|error| Error::WriteAttribute(self.path, error))
    }
}

/// A switch of a charger driver in sysfs: 1 is on, 0 is off.
pub struct Switch(Attribute);

impl Switch {
    /// The switch at `path`.
    #[must_use]
    pub const fn new(path: &'static str) -> Self {
        Self(Attribute::new(path))
    }

    /// Writes `on` only if the switch has the other value.
    ///
    /// # Errors
    ///
    /// The errors returned by [`Switch::read`] and [`Switch::write`].
    pub fn set(&self, on: bool) -> Result<(), Error> {
        if self.read()? != on {
            self.write(on)?;
        }
        Ok(())
    }

    /// Writes `on` to the switch.
    ///
    /// # Errors
    ///
    /// [`Error::WriteAttribute`].
    pub fn write(&self, on: bool) -> Result<(), Error> {
        self.0.write(i32::from(on))?;
        info!("Set {} to {}", self.0.path(), i32::from(on));
        Ok(())
    }

    /// Reads whether the switch is on.
    ///
    /// # Errors
    ///
    /// The errors returned by [`Attribute::read`], and [`Error::UnexpectedValue`] for a number other
    /// than 0 and 1.
    pub fn read(&self) -> Result<bool, Error> {
        match self.0.read()? {
            0 => Ok(false),
            1 => Ok(true),
            value => Err(Error::UnexpectedValue { attribute: self.0.path(), value }),
        }
    }
}
