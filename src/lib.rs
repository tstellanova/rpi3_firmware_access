/*
Copyright (c) 2018 Todd Stellanova

LICENSE: See LICENSE file
*/

#![crate_type = "lib"]

#[macro_use]
extern crate nix;

pub mod bindings;
mod core;
pub mod utils;


#[cfg(test)]
mod tests {
  use utils::*;

  #[test]
  fn check_soc_get_temperature() {
   let temp = get_soc_temperature(0);
   assert_eq!(temp, 1); 
  }

  #[test]
  fn check_power_status() {
    let status = get_power_status();
    assert_eq!(status, 1);
   }
}

