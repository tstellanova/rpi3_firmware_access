extern crate rpi3_firmware_access;


fn main() {
  let val = rpi3_firmware_access::utils::get_power_status();
  println!("power status: {}", val);

  let temp = rpi3_firmware_access::utils::get_soc_temperature(0);
  let temp = (temp as f64) / 1000.0;
  println!("temperature: {} C", temp);

}

