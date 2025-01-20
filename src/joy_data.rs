use vjoy::{
  VJoy,
  ButtonState,
  HatState,
};

use crate::config::JoystickConfig;

/// Joystick information struct.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct JoystickInfo {
  pub throttle: i32,
  pub rudder: i32,
  pub aileron: i32,

  pub axis_x: i32,
  pub axis_y: i32,
  pub axis_z: i32,
  pub axis_xrot: i32,
  pub axis_yrot: i32,
  pub axis_zrot: i32,
  pub slider: i32,
  pub dial: i32,

  pub wheel: i32,
  pub accelerator: i32,
  pub brake: i32,
  pub clutch: i32,
  pub steering: i32,

  pub axis_vx: i32,
  pub axis_vy: i32,
  pub axis_vz: i32,

  pub axis_vbrx: i32,
  pub axis_vbry: i32,
  pub axis_vbrz: i32,

  pub buttons: [u32; 4],
  pub hats: [u32; 4],
}

/// Joystick struct.
pub struct Joystick {
  index: u32,
  vjoy: VJoy,
}

/// Joystick implementation
impl Joystick {

  /// Create a new joystick instance.
  /// return: The joystick instance.
  pub fn new(index: u32) -> anyhow::Result<Self> {
    let joystick = Self {
      index,
      vjoy: VJoy::from_default_dll_location()?,
    };

    Ok(joystick)
  }

  /// Update the joystick data.
  /// param joy_info: The joystick information.
  /// param joy_config: The joystick configuration.
  /// return: The result of the update.
  pub fn update(&mut self, joy_info: &JoystickInfo, joy_config: &JoystickConfig) -> anyhow::Result<()> {
    // Get the joystick device.
    let mut device = self.vjoy.get_device_state(self.index)?;

    // log::debug!("Number of axes: {}", device.num_axes());
    // log::debug!("Number of buttons: {}", device.num_buttons());
    // log::debug!("Number of pov hats: {}", device.num_hats());

    // Update buttons.
    for i in 0..4 {
      for j in 0..32 {
        let button = i * 32 + j + 1;
        if button > device.num_buttons() {
          break;
        }

        // Get the button specific configuration.
        let button_config = joy_config.buttons.iter().find(|b| b.index == button as u32);
        let inverted = button_config.is_some_and(|b| b.inverted);

        let state = if joy_info.buttons[i] & (1 << j) != 0 {
          if inverted { ButtonState::Released } else { ButtonState::Pressed }
        } else if inverted { ButtonState::Pressed } else { ButtonState::Released };

        device.set_button(button as u8, state)?;
      }
    }

    // Update axes.
    let axis_value_modifer = |index: u32, value: i32| -> i32 {
      let axis_config = joy_config.axes.iter().find(|a| a.index == index);
      if let Some(axis_config) = axis_config {
        let inverted = axis_config.inverted;
        let min = axis_config.min;
        let max = axis_config.max;

        let v = if inverted {
          32767 - (value - min) * 32767 / (max - min)
        } else {
          (value - min) * 32768 / (max - min)
        };
        v
      } else {
        joy_info.throttle
      }
    };
    if device.num_axes() > 0 {
      device.set_axis(1, axis_value_modifer(0, joy_info.axis_x))?;
    }
    if device.num_axes() > 1 {
      device.set_axis(2, axis_value_modifer(1, joy_info.axis_y))?;
    }
    if device.num_axes() > 2 {
      device.set_axis(3, axis_value_modifer(2, joy_info.axis_z))?;
    }
    if device.num_axes() > 3 {
      device.set_axis(4, axis_value_modifer(3, joy_info.axis_xrot))?;
    }
    if device.num_axes() > 4 {
      device.set_axis(5, axis_value_modifer(4, joy_info.axis_yrot))?;
    }
    if device.num_axes() > 5 {
      device.set_axis(6, axis_value_modifer(5, joy_info.axis_zrot))?;
    }
    if device.num_axes() > 6 {
      device.set_axis(7, axis_value_modifer(6, joy_info.slider))?;
    }
    if device.num_axes() > 7 {
      device.set_axis(8, axis_value_modifer(7, joy_info.dial))?;
    }

    // Update pov hats.
    for i in 0..4 {
      if i >= device.num_hats() {
        break;
      }

      let value = joy_info.hats[i];
      device.set_hat((i + 1) as u8, HatState::Continuous(value))?;
    }

    // Update device state.
    self.vjoy.update_device_state(&device)?;

    Ok(())
  }

}