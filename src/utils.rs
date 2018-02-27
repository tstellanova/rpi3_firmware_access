#![allow(dead_code)]

use bindings::*;
use core::*;

      
pub fn get_power_status() -> u32 {
  read_gpio_value(135)
}

pub fn get_soc_temperature(temp_id: u32) -> u32 {
  let mut buf: [u32; 2] = [temp_id, 0];
  touch_fw_property(RPiFwPropTag::GetTemperature, &mut buf);
  buf[1]
}

pub fn get_hw_value(tag: RPiFwPropTag) -> u32 {
  let mut buf: [u32; 1] = [0; 1];
  touch_fw_property(tag, &mut buf);
  buf[0]
}


pub fn get_board_model() -> u32 {
  get_hw_value(RPiFwPropTag::GetBoardModel)
}
