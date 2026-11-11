// SPDX-FileCopyrightText: The halogenOS Project
// SPDX-License-Identifier: Apache-2.0

//! Power supply uevents from the kernel: a charger plugged in or out, a new battery level.

use kobject_uevent::UEvent;
use netlink_sys::{
    protocols::NETLINK_KOBJECT_UEVENT, AsyncSocket, AsyncSocketExt, SocketAddr, TokioSocket,
};

use crate::{charger::SetReason, error::ServiceError};

/// The netlink multicast group for uevents sent by the kernel, from `kobject_uevent_net_broadcast`
/// in `lib/kobject_uevent.c` of the kernel.
const KERNEL_UEVENT_GROUP: u32 = 1;

/// The subsystem set by the kernel on power supply uevents.
const POWER_SUPPLY_SUBSYSTEM: &str = "power_supply";

/// The key of the supply name in power supply uevents, from `power_supply_uevent` in
/// `drivers/power/supply/power_supply_sysfs.c` of the kernel.
const POWER_SUPPLY_NAME: &str = "POWER_SUPPLY_NAME";

/// The uevents from the power supplies.
pub(crate) struct PowerSupplyEvents(TokioSocket);

impl PowerSupplyEvents {
    /// Subscribes to uevents from the kernel. Runs inside the Tokio runtime.
    pub(crate) fn open() -> Result<Self, ServiceError> {
        let mut socket =
            TokioSocket::new(NETLINK_KOBJECT_UEVENT).map_err(ServiceError::OpenUevents)?;
        // Port 0 makes the kernel set the port of the socket.
        socket
            .socket_mut()
            .bind(&SocketAddr::new(0, KERNEL_UEVENT_GROUP))
            .map_err(ServiceError::OpenUevents)?;
        Ok(Self(socket))
    }

    /// Waits until a power supply changed, and returns which one.
    pub(crate) async fn changed(&self) -> Result<SetReason, ServiceError> {
        loop {
            match self.0.recv_from_full().await {
                Ok((packet, _kernel)) => {
                    if let Some(reason) = read_power_supply_change(&packet) {
                        return Ok(reason);
                    }
                }
                // The kernel dropped uevents while the socket was full, and some of them may have
                // come from a power supply.
                Err(error) if error.raw_os_error() == Some(libc::ENOBUFS) => {
                    return Ok(SetReason::UnknownPowerSupply)
                }
                Err(error) => return Err(ServiceError::ReceiveUevents(error)),
            }
        }
    }
}

/// The power supply that sent `packet`, or `None` for a packet from another subsystem. A packet
/// that does not parse as a uevent does not come from a power supply.
fn read_power_supply_change(packet: &[u8]) -> Option<SetReason> {
    UEvent::from_netlink_packet(packet)
        .ok()
        .filter(|event| event.subsystem == POWER_SUPPLY_SUBSYSTEM)
        .map(|mut event| {
            event
                .env
                .remove(POWER_SUPPLY_NAME)
                .map_or(SetReason::UnknownPowerSupply, SetReason::PowerSupply)
        })
}
