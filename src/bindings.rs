/*
Copyright (c) 2018 Todd Stellanova

LICENSE: See LICENSE file


Refer to https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface

*/


pub const RPI_FWPROP_STATUS_REQUEST : RPiFwPropStatus = 0;
pub const RPI_FWPROP_STATUS_SUCCESS : RPiFwPropStatus =  0x80000000; 
pub const RPI_FWPROP_STATUS_ERROR: RPiFwPropStatus = 0x80000001; 
pub type RPiFwPropStatus = u32;


pub enum RPiFwPropTag {
  PropertyEnd = 0,
  GetFirmwareRevision = 0x00000001,
  SetCursorInfo = 0x0008010,
  SetCursorState = 0x0008011,
  GetBoardModel = 0x00010001, 
  GetBoardRevision = 0x00010002, 
  GetBoardMacAddress = 0x00010003,
  GetBoardSerial = 0x00010004,
  GetArmMemory = 0x00010005,
  GetVcMemory = 0x00010006,
  GetClocks = 0x00010007,
  GetPowerState = 0x00020001, 
  GetTiming = 0x00020002,  
  SetPowerState = 0x00028001, 
  GetClockState = 0x00030001,
  GetClockRate = 0x00030002,
  GetVoltage = 0x00030003, 
  GetMaxClockRate = 0x00030004,
  GetMaxVoltage = 0x00030005, 
  GetTemperature = 0x00030006,
  GetMinClockRate = 0x00030007,
  GetMinVoltage = 0x00030008,
  GetTurbo = 0x00030009,
  GetMaxTemperature = 0x0003000a,
  GetStc = 0x0003000B,
  AllocateMemory = 0x0003000c,
  LockMemory = 0x0003000d, 
  UnlockMemory = 0x0003000e,
  ReleaseMemory = 0x0003000f,
  ExecuteCode = 0x00030010,
  ExecuteQpu = 0x00030011,
  SetEnableQpu = 0x00030012,
  GetDispmanxResourceMemHandle = 0x00030014,
  GetEdidBlock = 0x00030020,
  GetCustomerOtp = 0x00030021,
  GetDomainState = 0x00030030,
  GetGpioState = 0x00030041, 
  SetGpioState = 0x00038041,
  GetGpioConfig = 0x00030043,
  SetGpioConfig = 0x00038043,
  SetClockState = 0x00038001,
  SetClockRate = 0x00038002,
  SetVoltage = 0x00038003,
  SetTurbo = 0x00038009,
  SetCustomerOtp = 0x00038021,
  SetDomainState = 0x00038030,
  SetSdhostClock = 0x00038042,
  FramebufferAllocate = 262145,
  FramebufferBlank = 262146,
  FramebufferGetPhysicalWidthHeight = 262147,
  FramebufferGetVirtualWidthHeight = 262148,
  FramebufferGetDepth = 262149,
  FramebufferGetPixelOrder = 262150,
  FramebufferGetAlphaMode = 262151,
  FramebufferGetPitch = 262152,
  FramebufferGetVirtualOffset = 262153,
  FramebufferGetOverscan = 262154,
  FramebufferGetPalette = 262155,
  FramebufferGetTouchbuf = 262159,
  FramebufferGetGpiovirtbuf = 262160,
  FramebufferRelease = 294913,
  FramebufferTestPhysicalWidthHeight = 278531,
  FramebufferTestVirtualWidthHeight = 278532,
  FramebufferTestDepth = 278533,
  FramebufferTestPixelOrder = 278534,
  FramebufferTestAlphaMode = 278535,
  FramebufferTestVirtualOffset = 278537,
  FramebufferTestOverscan = 278538,
  FramebufferTestPalette = 278539,
  FramebufferTestVsync = 278542,
  FramebufferSetPhysicalWidthHeight = 294915,
  FramebufferSetVirtualWidthHeight = 294916,
  FramebufferSetDepth = 294917,
  FramebufferSetPixelOrder = 294918,
  FramebufferSetAlphaMode = 294919,
  FramebufferSetVirtualOffset = 294921,
  FramebufferSetOverscan = 294922,
  FramebufferSetPalette = 294923,
  FramebufferSetVsync = 294926,
  FramebufferSetBacklight = 294927,
  VchiqInit = 0x00048010,
  GetCommandLine = 0x00050001,
  GetDmaChannels = 0x00060001,
}

/// Clock IDs for various clocks that can be accessed via the firmware
pub enum RPiFwClockIDs  {
  EMMC = 0x000000001, 
  UART = 0x000000002, 
  ARM = 0x000000003, 
  CORE = 0x000000004, 
  V3D = 0x000000005, 
  H264 = 0x000000006, 
  ISP = 0x000000007, 
  SDRAM = 0x000000008, 
  PIXEL = 0x000000009, 
  PWM = 0x00000000a, 
}

/// Some commonly used extended GPIO values
pub enum RPiFwExtGpioIDs {
  CameraPower= 133, // Power for the RPi camera itself
  CameraLED= 134, // Power for the RPi camera LED
  PiPowerQuality = 135, // 1 if power is solid +5V, 0 if in low voltage (brownout) condition
}
