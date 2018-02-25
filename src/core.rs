#![allow(dead_code)]

use std::os::unix::io::{ AsRawFd };
use std::fs::File;
use bindings::*;

const FW_DEVICE_PATH: &'static str = "/dev/vcio";

// want _IOWR(100, 0, char*)
const VC_FW_IOC_MAGIC: u8 = 100 ;
const VC_FW_IOC_IF_NUM: u8 = 0;

ioctl!(write_ptr readwrite_vc_fw with VC_FW_IOC_MAGIC, VC_FW_IOC_IF_NUM; u32 );


/*
Buffer contents:

u32: buffer size in bytes (including the header values, the end tag and padding)
u32: buffer request/response code
u8...: sequence of concatenated tags
u32: 0x0 (end tag)
u8...: padding


tag format:
u32: tag identifier
u32: value buffer size in bytes
u32:
Request codes:
b31 clear: request
b30-b0: reserved
Response codes:
b31 set: response
b30-b0: value length in bytes
u8...: value buffer
u8...: padding to align the tag to 32 bits.

*/
pub fn touch_fw_property(tag: RPiFwPropTag, prop_data: &mut [u32] ) {
  let f = File::open(FW_DEVICE_PATH)
    .expect("Could not open FW_DEVICE_PATH");
  let buf_size : u32 = prop_data.len() * 4;
  let head_size : u32 = ((6 + prop_data.len()) * 4) as u32;
  let mut req = vec![head_size, RPI_FWPROP_STATUS_REQUEST , tag as u32, buf_size , 0 ];
  //copy prop_data into req
  for i in 0..prop_data.len() {
    req.push(prop_data[i]);
  }
  req.push(RPiFwPropTag::PropertyEnd as u32);
  println!("req size: {}", req.len());

  unsafe {
    //TODO handle return value
    let rc = readwrite_vc_fw(f.as_raw_fd(), req.as_mut_ptr())
      .expect("ioctl failed");
    println!("result: {}", rc);
  }
}
