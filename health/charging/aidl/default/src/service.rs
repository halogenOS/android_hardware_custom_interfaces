// SPDX-FileCopyrightText: The halogenOS Project
// SPDX-License-Identifier: Apache-2.0

//! The charge limit service called by the system.

use binder::{BinderFeatures, Interface, Strong};
use custom_hardware_health_charging::aidl::custom::hardware::health::charging::{
    IChargeLimit::{BnChargeLimit, IChargeLimit},
    IChargeLimitFlow::IChargeLimitFlow,
};

use crate::{error::ServiceError, flow::Attachments};

/// Registers the charge limit service, which sends attached flows to `attachments`.
pub(crate) fn register(attachments: Attachments) -> Result<(), ServiceError> {
    let service = BnChargeLimit::new_binder(ChargeLimit { attachments }, BinderFeatures::default());
    let instance = format!("{}/default", <ChargeLimit as IChargeLimit>::get_descriptor());
    binder::add_service(&instance, service.as_binder())
        .map_err(|code| ServiceError::RegisterService(code.into()))
}

struct ChargeLimit {
    attachments: Attachments,
}

impl Interface for ChargeLimit {}

impl IChargeLimit for ChargeLimit {
    fn attach(&self, flow: &Strong<dyn IChargeLimitFlow>) -> binder::Result<()> {
        self.attachments.attach(flow)
    }
}
