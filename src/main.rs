extern crate rpi3_firmware_access;
use rpi3_firmware_access::bindings::*;


fn main() {
  let val = rpi3_firmware_access::utils::get_hw_value(RPiFwPropTag::GetBoardModel);
  println!("val: {}", val);
}

