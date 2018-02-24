/*
Copyright (c) 2018 Todd Stellanova

LICENSE: See LICENSE file


Refer to https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface

*/


#![allow(dead_code)]


pub const RPI_FWPROP_STATUS_REQUEST : RPiFwPropStatus = 0;
pub const RPI_FWPROP_STATUS_SUCCESS : RPiFwPropStatus =  0x80000000; 
pub const RPI_FWPROP_STATUS_ERROR: RPiFwPropStatus = 0x80000001; 
pub type RPiFwPropStatus = u32;


pub enum RPiFwPropTag {
  PropertyEnd = 0,
  GetFirmwareRevision = 0x00000001,
  SetCursorInfo = 32784,
  SetCursorState = 32785,
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
  GetClockState = 196609,
  GetClockRate = 196610,
  GetVoltage = 0x00030003, 
  GetMaxClockRate = 196612,
  GetMaxVoltage = 0x00030005, 
  GetTemperature = 0x00030006,
  GetMinClockRate = 196615,
  GetMinVoltage = 0x00030008,
  GetTurbo = 196617,
  GetMaxTemperature = 0x0003000a,
  GetStc = 196619,
  AllocateMemory = 0x0003000c,
  LockMemory = 0x0003000d, 
  UnlockMemory = 0x0003000e,
  ReleaseMemory = 0x0003000f,
  ExecuteCode = 0x00030010,
  ExecuteQpu = 196625,
  SetEnableQpu = 196626,
  GetDispmanxResourceMemHandle = 0x00030014,
  GetEdidBlock = 0x00030020,
  GetCustomerOtp = 196641,
  GetDomainState = 196656,
  GetGpioState = 0x00030041, 
  SetGpioState = 229441,
  GetGpioConfig = 196675,
  SetGpioConfig = 229443,
  SetClockState = 229377,
  SetClockRate = 229378,
  SetVoltage = 0x00038003,
  SetTurbo = 229385,
  SetCustomerOtp = 229409,
  SetDomainState = 229424,
  SetSdhostClock = 229442,
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
  VchiqInit = 294928,
  GetCommandLine = 0x00050001,
  GetDmaChannels = 0x00060001,
}
