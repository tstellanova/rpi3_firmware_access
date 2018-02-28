/*
Copyright (c) 2018 Todd Stellanova

LICENSE: See LICENSE file
*/

#![crate_type = "lib"]


pub mod bindings;
mod core;
pub mod utils;


#[cfg(test)]
mod tests {
  use utils::*;

  #[test]
  fn check_soc_get_temperature() {
   let temp = get_soc_temperature(0);
   assert!(temp > 1000);
   //convert to Celsius
   let temp = (temp as f64) / 1000.0;
   // temp should be in a reasonable Celsius range
   assert!(temp > 0.0);
   assert!(temp < 90.0);
  }

  #[test]
  fn check_power_status() {
    let status = get_power_status();
    //we should have adequate power
    assert_eq!(status, 1);
   }
}

