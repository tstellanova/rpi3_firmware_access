/*
Copyright (c) 2018 Todd Stellanova

LICENSE: See LICENSE file
*/

use bindings::*;
use core::*;

      
/// Detect whether we have solid +5V power supply:
/// - 1 indicates that power is good 
/// - 0 indicates that main supply voltage has dropped below +5V
pub fn get_power_status() -> u32 {
  read_gpio_value(135)
}

/// Get the temperature of the SOC in Celsius
pub fn get_soc_temperature() -> f32 {
  // we pass a "temperature id" of 0 
  let mut buf: [u32; 2] = [0, 0];
  touch_fw_property(RPiFwPropTag::GetTemperature, &mut buf);
  let result : f32 = (buf[1] as f32) / 1000.0;
  result
}

/// Get one of the single-word firmware values
/// TODO fix this -- doesn't appear to return the proper values
pub fn get_fw_word(tag: RPiFwPropTag) -> u32 {
  let mut buf: [u32; 1] = [0; 1];
  touch_fw_property(tag, &mut buf);
  buf[0]
}

pub fn get_board_model() -> u32 {
  get_fw_word(RPiFwPropTag::GetBoardModel)
}


pub fn read_gpio_value(gpio: u32) -> u32 {
  let mut buf: [u32; 2] = [gpio, 0];
  touch_fw_property(RPiFwPropTag::GetGpioState, &mut buf);
  buf[1]
}

pub fn write_gpio_value(gpio: u32, state: u8) -> u32 {
  let write_val = { if 0 == state { 0 } else { 1 } };
  let mut buf: [u32; 2] = [gpio, write_val];
  touch_fw_property(RPiFwPropTag::GetGpioState, &mut buf);
  buf[1]
}
 


