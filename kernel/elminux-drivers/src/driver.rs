//! Driver trait definition

use elminux_ipc::capability::Cap;
use elminux_ipc::message::Msg;

/// Driver interface for kernel-managed drivers
pub trait Driver {
    /// Initialize the driver
    fn init(&self) -> Result<Cap, DriverError>;

    /// Handle IPC message from user space
    fn handle_msg(&self, msg: Msg) -> Msg;
}

/// Driver errors
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DriverError {
    InitFailed,
    NoResources,
    InvalidRequest,
}
