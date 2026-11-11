// SPDX-FileCopyrightText: The halogenOS Project
// SPDX-License-Identifier: Apache-2.0

//! The operations of the HAL that can fail.

use std::{io, iter, num::ParseIntError};

/// A sysfs operation of a charger that failed, with the error that caused it.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A sysfs attribute could not be read.
    #[error("cannot read {0}")]
    ReadAttribute(&'static str, #[source] io::Error),
    /// A sysfs attribute contains text that is not a number.
    #[error("{0} contains text that is not a number")]
    ParseAttribute(&'static str, #[source] ParseIntError),
    /// A sysfs attribute contains a number not defined by its driver.
    #[error("{attribute} contains {value}, a number not defined by its driver")]
    UnexpectedValue {
        /// Path of the attribute.
        attribute: &'static str,
        /// The number read from the attribute.
        value: i32,
    },
    /// A sysfs attribute could not be written.
    #[error("cannot write {0}")]
    WriteAttribute(&'static str, #[source] io::Error),
}

/// An operation of the service that failed, with the error that caused it.
#[derive(Debug, thiserror::Error)]
pub(crate) enum ServiceError {
    /// The event loop could not start.
    #[error("cannot start the event loop")]
    StartRuntime(#[source] io::Error),
    /// The uevent socket could not be opened.
    #[error("cannot open the uevent socket")]
    OpenUevents(#[source] io::Error),
    /// The uevent socket stopped delivering uevents.
    #[error("cannot receive uevents")]
    ReceiveUevents(#[source] io::Error),
    /// The service manager returned an error for the service.
    #[error("cannot register the service")]
    RegisterService(#[source] binder::Status),
    /// The system sent a target outside the levels allowed by the interface.
    #[error("the target of {0}% is outside the levels allowed by the interface")]
    TargetOutOfRange(i32),
    /// The signal handler for SIGTERM could not be registered.
    #[error("cannot watch for SIGTERM")]
    WatchTermination(#[source] io::Error),
    /// The socket written by the signal handler for SIGTERM could not be read.
    #[error("cannot receive SIGTERM")]
    ReceiveTermination(#[source] io::Error),
}

/// The error followed by every error that caused it, for the log.
#[must_use]
pub fn describe(error: &dyn std::error::Error) -> String {
    iter::successors(Some(error), |error| error.source())
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(": ")
}
