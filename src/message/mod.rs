mod common;
mod joystick;

pub use common::*;
pub use joystick::*;

/// Message major id struct.
pub struct MessageMajorId {}

/// Message major id implementation.
impl MessageMajorId {
  pub const COMMON: u16 = 0x0000;
  pub const JOYSTICK: u16 = 0x0001;
}

/// Message header.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct MessageHeader {
  /// Major message id.
  pub major_id: u16,
  /// Minor message id.
  pub minor_id: u16,
  /// Message length.
  pub length: u32,
}

/// AsBytes trait.
pub trait AsBytes<T: Sized + Copy> {

  /// Get bytes of the message.
  /// return: The bytes of the message.
  fn as_bytes(&self) -> &[u8] {
    // SAFETY: const sound because we transmute two types with the same layout
    unsafe {
      std::slice::from_raw_parts(
        self as *const Self as *const u8,
        std::mem::size_of::<T>(),
      )
    }
  }

}

/// FromBytes trait.
pub trait FromBytes<T: Sized + Copy> {

  /// Create a new message from bytes.
  /// return: The message.
  fn from_bytes(bytes: &[u8]) -> Option<T> {
    if bytes.len() != std::mem::size_of::<T>() {
      return None;
    }

    // SAFETY: const sound because we transmute two types with the same layout
    Some(unsafe { *(bytes.as_ptr() as *const T) })
  }

}