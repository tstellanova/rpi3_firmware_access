#![allow(dead_code)]

use bindings::*;
use core::*;

      
pub fn read_gpio_value(gpio: u32) -> u32 {
  let mut buf: [u32; 2] = [0; 2];
  buf[0] = gpio;
  touch_fw_property(RPiFwPropTag::GetGpioState, &mut buf);
  buf[1]
}

pub fn get_power_status() -> u32 {
  read_gpio_value(135)
}

pub fn get_soc_temperature(temp_id: u32) -> u32 {
  let mut buf: [u32; 2] = [0; 2];
  buf[0] = temp_id;
  touch_fw_property(RPiFwPropTag::GetTemperature, &mut buf);
  buf[1]
}

pub fn get_hw_value(tag: RPiFwPropTag) -> u32 {
  let mut buf: [u32; 2] = [0; 2];
  touch_fw_property(tag, &mut buf);
  buf[0]
}

