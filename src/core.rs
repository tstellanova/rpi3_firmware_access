#![allow(dead_code)]

use std::os::unix::io::{ AsRawFd };
use std::fs::File;
use bindings::*;

/// The path to the VideoControl firmware device driver
const VC_FW_DEVICE_PATH: &'static str = "/dev/vcio";


// want _IOWR(100, 0, char*)
const VC_FW_IOC_MAGIC_IOWR: u32 = 0xc0046400;
//TODO use libc crate's ioctl instead?
extern {
  //int ioctl(int fd, unsigned long request, ...);
  fn ioctl(fd: i32, req: u32, ...) -> i32;
}

/// The length of a request sent to the firmware
const REQ_HEADER_LEN: usize = 5;

/*
Message contents:

u32: buffer size in bytes (including the header values, the end tag and padding)
u32: buffer request/response code
u8...: sequence of concatenated tags (see below)
u32: 0x0 (end tag)
u8...: padding

Tag format:
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


/// Read/Write firmware properties by tag ID
pub fn touch_fw_property(tag: RPiFwPropTag, prop_data: &mut [u32] ) {
  let f = File::open(VC_FW_DEVICE_PATH)
    .expect("Could not open VC_FW_DEVICE_PATH");
  // the size of the buffer into which the fw can r/w transaction data
  let prop_data_len  = prop_data.len() ;
  let buf_size : u32 = (prop_data_len  * 4) as u32;
  // total length of message, including header, data, and terminating tag
  let msg_size: u32 = ((REQ_HEADER_LEN + prop_data_len + 1) * 4) as u32;
  let mut msg= vec![msg_size, RPI_FWPROP_STATUS_REQUEST, tag as u32, buf_size, 0 ];
  //copy prop_data into msg 
  for i in 0..prop_data_len {
    msg.push(prop_data[i]);
  }
  msg.push(RPiFwPropTag::PropertyEnd as u32);

  unsafe {
    let rc = ioctl(f.as_raw_fd(), VC_FW_IOC_MAGIC_IOWR, msg.as_mut_slice());
    if 0 != rc {
      println!("error result: {}", rc);
      //TODO handle this error case separately from below?
    }
  }

  let req_result : u32 = msg[1];
  if RPI_FWPROP_STATUS_SUCCESS == req_result {
    //copy out any data that was written into the writable value buffer
    for i in 0..prop_data_len {
      prop_data[i] = msg[REQ_HEADER_LEN + i];
    }
  }
  else {
    println!("req_result: {} ", req_result);
  }

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

