# rpi3_firmware_access
Allow access to Raspberry Pi 3 (RPi3) firmware properties from Rust.
This allows us to read and write firmware properties of the VideoCore IV GPU,
which control a broad range of RPi3 behaviors.

For example, you can read and write GPIOs controlled by the VideoCore 
(which are completely separate from CPU GPIOs that can be accessed by `pigpiod`, for example).

See [RPi Mailbox property interface](https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface) for more info on accessing firmware properties by tag.  

