//! Channel implementation
//!
//! Synchronous rendezvous with blocking send/receive.

use crate::capability::Cap;
use crate::message::Msg;

/// Channel endpoint
pub struct Channel {
    pub id: u64,
    pub peer: Option<Cap>,
    pub state: ChannelState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChannelState {
    Empty,
    WaitingSend(Msg),
    WaitingRecv,
}

/// Send message on channel (blocks until receiver ready)
pub fn send(_chan: Cap, _msg: Msg) -> Result<(), SendError> {
    // TODO: Check capability rights
    // TODO: If receiver waiting, deliver immediately
    // TODO: Otherwise block sender
    Ok(())
}

/// Receive message on channel (blocks until sender ready)
pub fn recv(_chan: Cap) -> Result<Msg, RecvError> {
    // TODO: Check capability rights
    // TODO: If sender waiting, receive immediately
    // TODO: Otherwise block receiver
    Err(RecvError::WouldBlock)
}

#[derive(Debug)]
pub enum SendError {
    InvalidCap,
    NoRights,
    PeerClosed,
}

#[derive(Debug)]
pub enum RecvError {
    InvalidCap,
    NoRights,
    PeerClosed,
    WouldBlock,
}
