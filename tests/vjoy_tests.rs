use vjoy::{
  VJoy,
  ButtonState,
  Error,
  HatState,
  FourWayHat
};

#[test]
fn test_vjoy() -> Result<(), Error> {
  let mut vjoy = VJoy::from_default_dll_location()?;

  let mut counter = 10;
  while counter > 0 {
    {
      let device_1 = vjoy.get_device_state_mut(1)?;

      device_1.set_button(1, ButtonState::Pressed)?;
      device_1.set_axis(1, i32::MAX)?;

      let hat_type = device_1.hat_type();
      let value = match hat_type{
        HatState::Discrete(_) => HatState::Discrete(FourWayHat::East),
        HatState::Continuous(_) => HatState::Continuous(90 * 100),
      };
      device_1.set_hat(1, value)?;
    }

    vjoy.update_all_devices()?;

    println!("Pressed");
    // Sleep 1s.
    std::thread::sleep(std::time::Duration::from_secs(1));

    {
      let device_1 = vjoy.get_device_state_mut(1)?;

      device_1.set_button(1, ButtonState::Released)?;
      device_1.set_axis(1, 0)?;

      let hat_type = device_1.hat_type();
      let value = match hat_type{
        HatState::Discrete(_) => HatState::Discrete(FourWayHat::Centered),
        HatState::Continuous(_) => HatState::Continuous(u32::MAX),
      };
      device_1.set_hat(1, value)?;
    }

    vjoy.update_all_devices()?;

    println!("Released");
    // Sleep 1s.
    std::thread::sleep(std::time::Duration::from_secs(1));

    counter -= 1;
  }

  Ok(())
}