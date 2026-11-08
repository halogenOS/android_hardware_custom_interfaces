// SPDX-FileCopyrightText: The halogenOS Project
// SPDX-License-Identifier: Apache-2.0

//! SIGTERM, sent by init on `stop` and at shutdown.

use signal_hook::{consts::SIGTERM, low_level::pipe};
use tokio::{io::AsyncReadExt, net::UnixStream};

use crate::error::ServiceError;

/// The read end of a socket pair written to by the signal handler for SIGTERM.
pub(crate) struct Termination(UnixStream);

impl Termination {
    /// Registers the signal handler. Runs inside the Tokio runtime.
    pub(crate) fn watch() -> Result<Self, ServiceError> {
        let (read, write) =
            std::os::unix::net::UnixStream::pair().map_err(ServiceError::WatchTermination)?;
        // The handler is registered for the whole life of the process.
        let _signal_id = pipe::register(SIGTERM, write).map_err(ServiceError::WatchTermination)?;
        read.set_nonblocking(true).map_err(ServiceError::WatchTermination)?;
        UnixStream::from_std(read).map(Self).map_err(ServiceError::WatchTermination)
    }

    /// Waits until SIGTERM arrives.
    pub(crate) async fn received(mut self) -> Result<(), ServiceError> {
        self.0
            .read_exact(&mut [0])
            .await
            .map(|_length| ())
            .map_err(ServiceError::ReceiveTermination)
    }
}
