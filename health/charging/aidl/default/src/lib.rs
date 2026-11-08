// SPDX-FileCopyrightText: The halogenOS Project
// SPDX-License-Identifier: Apache-2.0

//! Charge limit HAL without the charger of a device. It limits the battery to the target that the
//! system publishes, through the [`Charger`] passed to [`serve`] by the device.

#![forbid(unsafe_code)]
#![deny(
    clippy::all,
    clippy::pedantic,
    unused_results,
    unreachable_pub,
    clippy::let_underscore_must_use
)]

mod band;
mod battery;
mod charger;
mod controller;
mod error;
mod flow;
mod power_supply;
mod service;
pub mod sysfs;
mod termination;

use std::process::ExitCode;

use android_logger::Config;
use binder::ProcessState;
use log::{error, info, LevelFilter};
use tokio::runtime;

use crate::{
    charger::RestoringCharger,
    controller::Controller,
    error::{describe, ServiceError},
    flow::Flows,
    power_supply::PowerSupplyEvents,
    termination::Termination,
};
pub use crate::{
    charger::{ChargeMode, Charger},
    error::Error,
};

const LOG_TAG: &str = "charge_limit";

/// Registers the service, runs until SIGTERM or a fatal error, returns to normal charging.
pub fn serve(charger: impl Charger) -> ExitCode {
    android_logger::init_once(
        Config::default().with_tag(LOG_TAG).with_max_level(LevelFilter::Info),
    );
    match run(charger) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            error!("{}", describe(&error));
            ExitCode::FAILURE
        }
    }
}

fn run(charger: impl Charger) -> Result<(), ServiceError> {
    // Clean exits and SIGTERM drop the charger, which restores normal charging. After a panic,
    // which aborts the process in device builds, or after SIGKILL, init restarts the HAL, and the
    // first apply sets normal charging.
    let charger = RestoringCharger::new(charger);
    let runtime = runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .map_err(ServiceError::StartRuntime)?;
    runtime.block_on(async {
        let termination = Termination::watch()?;
        let power_supply = PowerSupplyEvents::open()?;
        let (flows, attachments, targets) = Flows::new();
        service::register(attachments)?;
        ProcessState::start_thread_pool();
        tokio::select! {
            result = Controller::new(charger, targets).run(power_supply) => {
                result.map(|never| match never {})
            }
            () = flows.run() => Ok(()),
            received = termination.received() => {
                received.inspect(|()| info!("Received SIGTERM, charging normally"))
            }
        }
    })
}
