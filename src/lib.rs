/*
Copyright (c) 2018 Todd Stellanova

LICENSE: See LICENSE file
*/
#![crate_type = "lib"]


#[macro_use]
extern crate nix;

mod bindings;

use std::fs::File;
use std::os::unix::io::{ AsRawFd };

use bindings::*;

const FW_DEVICE_PATH: &'static str = "/dev/vcio";

  // want _IOWR(100, 0, char*)
  const VC_FW_IOC_MAGIC: u8 = 100 ;
  const VC_FW_IOC_IF_NUM: u8 = 0;


  ioctl!(readwrite_buf readwrite_vc_fw with VC_FW_IOC_MAGIC, VC_FW_IOC_IF_NUM; u32 );


fn touch_fw_property(tag: RPiFwPropTag, prop_data: &mut [u32] ) {
  let f = File::open(FW_DEVICE_PATH)
    .expect("Could not open FW_DEVICE_PATH");
  let transaction_buf_len = prop_data.len() + 6 ;
  let prop_data_size = 4 * prop_data.len();
  let mut req= Vec::with_capacity(transaction_buf_len);
  req[0] = RPI_FWPROP_STATUS_REQUEST;
  req[1] = tag as u32;
  req[2] = prop_data_size as u32;
  //req[3] = 0; //response slot
  //TODO copy prop_data into req
 
  unsafe {
    let rc = readwrite_vc_fw(f.as_raw_fd(), &mut req);
  }
}

fn read_gpio_value(gpio: u32) -> u32 {
  let mut buf: [u32; 2] = [0; 2];
  buf[0] = gpio;
  touch_fw_property(RPiFwPropTag::GetGpioState, &mut buf);
  buf[1]
}

pub fn get_power_status() -> u32 {
  read_gpio_value(135)
}

