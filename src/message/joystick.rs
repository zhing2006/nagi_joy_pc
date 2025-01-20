use super::{
  MessageMajorId,
  MessageHeader,
  AsBytes,
  FromBytes,
};

/// Joystick message minor id struct.
pub struct JoystickMessageMinorId {}

/// Joystick message minor id implementation.
impl JoystickMessageMinorId {
  pub const DATA_SYNC: u16 = 0x0000;
  pub const DATA_ACK: u16 = 0x0001;
}

/// Joystick sync message.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct JoystickSync {
  /// Message header.
  pub header: MessageHeader,
  /// Data.
  pub data: super::super::joy_data::JoystickInfo,
}

/// Joystick sync message FromBytes implementation.
impl FromBytes<JoystickSync> for JoystickSync {}

/// Joystick sync message implementation.
impl JoystickSync {}

/// Joystick ack message.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct JoystickAck {
  /// Message header.
  pub header: MessageHeader,
  /// Payload.
  pub payload: u16,
}

/// Joystick ack message AsBytes implementation.
impl AsBytes<JoystickAck> for JoystickAck {}

/// Joystick ack message implementation.
impl JoystickAck {
  pub const O: u16 = 'O' as u16;
  pub const K: u16 = 'K' as u16;

  /// Create a new joystick ack message.
  /// param payload: The payload.
  /// return: The joystick ack message.
  pub fn new(payload: u16) -> Self {
    Self {
      header: MessageHeader {
        major_id: MessageMajorId::JOYSTICK,
        minor_id: JoystickMessageMinorId::DATA_ACK,
        length: std::mem::size_of::<u16>() as u32,
      },
      payload,
    }
  }

}