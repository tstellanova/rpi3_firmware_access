extern crate rpi3_firmware_access;
use rpi3_firmware_access::utils::*;

fn main() {
  let val = get_power_status();
  println!("power status: {}", val);

  let temp = get_soc_temperature();
  println!("temperature: {} C", temp);

}

