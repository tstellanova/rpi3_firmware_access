#![allow(dead_code)]

use std::os::unix::io::{ AsRawFd };
use std::fs::File;
use bindings::*;

const FW_DEVICE_PATH: &'static str = "/dev/vcio";

// want _IOWR(100, 0, char*)
const VC_FW_IOC_MAGIC: u8 = 100 ;
const VC_FW_IOC_IF_NUM: u8 = 0;

ioctl!(readwrite_buf readwrite_vc_fw with VC_FW_IOC_MAGIC, VC_FW_IOC_IF_NUM; u32 );

pub fn touch_fw_property(tag: RPiFwPropTag, prop_data: &mut [u32] ) {
  let f = File::open(FW_DEVICE_PATH)
    .expect("Could not open FW_DEVICE_PATH");
  let buf_size = prop_data.len() * 4;
  let mut req = vec![RPI_FWPROP_STATUS_REQUEST, tag as u32, buf_size as u32, 0 ];
  //copy prop_data into req
  for i in 0..prop_data.len() {
    req.push(prop_data[i]);
  }
  req.push(RPiFwPropTag::PropertyEnd as u32);

  unsafe {
    //TODO handle return value
    let rc = readwrite_vc_fw(f.as_raw_fd(), &mut req)
      .expect("ioctl failed");
    println!("result: {}", rc);
  }
}
