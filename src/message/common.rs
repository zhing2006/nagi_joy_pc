use super::{
  MessageMajorId,
  MessageHeader,
  AsBytes,
  FromBytes,
};

/// Common message minor id struct.
pub struct CommonMessageMinorId {}

/// Common message minor id implementation.
impl CommonMessageMinorId {
  pub const PING: u16 = 0x0000;
  pub const PONG: u16 = 0x0002;
}

/// Ping request message.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct PingRequest {
  /// Message header.
  pub header: MessageHeader,
  /// Magic number.
  pub magic: u32,
}

/// Ping request message FromBytes implementation.
impl FromBytes<PingRequest> for PingRequest {}

/// Ping request message implementation.
impl PingRequest {

  /// The magic number of the message.
  /// return: The magic number.
  pub fn magic_number() -> u32 {
    "NAGI".chars().fold(0, |acc, c| (acc << 8) | c as u32)
  }

}

/// Pong response message.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct PongResponse {
  /// Message header.
  pub header: MessageHeader,
  /// Magic number.
  pub magic: u32,
}

/// Pong response message AsBytes implementation.
impl AsBytes<PongResponse> for PongResponse {}

/// Pong response message implementation.
impl PongResponse {

  /// The magic number of the message.
  /// return: The magic number.
  pub fn magic_number() -> u32 {
    "GIAN".chars().fold(0, |acc, c| (acc << 8) | c as u32)
  }

  /// Create a new common response message.
  /// return: The new common response message.
  pub fn new() -> Self {
    Self {
      header: MessageHeader {
        major_id: MessageMajorId::COMMON,
        minor_id: CommonMessageMinorId::PONG,
        length: std::mem::size_of::<Self>() as u32,
      },
      magic: Self::magic_number(),
    }
  }

}