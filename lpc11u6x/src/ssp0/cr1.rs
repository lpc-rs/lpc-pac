#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR1 {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `LBM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBMR {
    #[doc = "During normal operation."]
    DURING_NORMAL_OPERAT,
    #[doc = "Serial input is taken from the serial output (MOSI or MISO) rather than the serial input pin (MISO or MOSI respectively)."]
    SERIAL_INPUT_IS_TAKE,
}
impl crate::ToBits<bool> for LBMR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LBMR::DURING_NORMAL_OPERAT => false,
            LBMR::SERIAL_INPUT_IS_TAKE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LBM_R = crate::FR<bool, LBMR>;
impl LBM_R {
    #[doc = "Checks if the value of the field is `DURING_NORMAL_OPERAT`"]
    #[inline(always)]
    pub fn is_during_normal_operat(&self) -> bool {
        *self == LBMR::DURING_NORMAL_OPERAT
    }
    #[doc = "Checks if the value of the field is `SERIAL_INPUT_IS_TAKE`"]
    #[inline(always)]
    pub fn is_serial_input_is_take(&self) -> bool {
        *self == LBMR::SERIAL_INPUT_IS_TAKE
    }
}
#[doc = "Values that can be written to the field `LBM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBMW {
    #[doc = "During normal operation."]
    DURING_NORMAL_OPERAT,
    #[doc = "Serial input is taken from the serial output (MOSI or MISO) rather than the serial input pin (MISO or MOSI respectively)."]
    SERIAL_INPUT_IS_TAKE,
}
impl LBMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LBMW::DURING_NORMAL_OPERAT => false,
            LBMW::SERIAL_INPUT_IS_TAKE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LBMW<'a> {
    w: &'a mut W,
}
impl<'a> _LBMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "During normal operation."]
    #[inline(always)]
    pub fn during_normal_operat(self) -> &'a mut W {
        self.variant(LBMW::DURING_NORMAL_OPERAT)
    }
    #[doc = "Serial input is taken from the serial output (MOSI or MISO) rather than the serial input pin (MISO or MOSI respectively)."]
    #[inline(always)]
    pub fn serial_input_is_take(self) -> &'a mut W {
        self.variant(LBMW::SERIAL_INPUT_IS_TAKE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Possible values of the field `SSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSER {
    #[doc = "The SPI controller is disabled."]
    DISABLED,
    #[doc = "The SPI controller will interact with other devices on the serial bus. Software should write the appropriate control information to the other SSP/SPI registers and interrupt controller registers, before setting this bit."]
    ENABLED,
}
impl crate::ToBits<bool> for SSER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SSER::DISABLED => false,
            SSER::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SSE_R = crate::FR<bool, SSER>;
impl SSE_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSER::ENABLED
    }
}
#[doc = "Values that can be written to the field `SSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSEW {
    #[doc = "The SPI controller is disabled."]
    DISABLED,
    #[doc = "The SPI controller will interact with other devices on the serial bus. Software should write the appropriate control information to the other SSP/SPI registers and interrupt controller registers, before setting this bit."]
    ENABLED,
}
impl SSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SSEW::DISABLED => false,
            SSEW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SSEW<'a> {
    w: &'a mut W,
}
impl<'a> _SSEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SPI controller is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSEW::DISABLED)
    }
    #[doc = "The SPI controller will interact with other devices on the serial bus. Software should write the appropriate control information to the other SSP/SPI registers and interrupt controller registers, before setting this bit."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSEW::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `MS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSR {
    #[doc = "The SPI controller acts as a master on the bus, driving the SCLK, MOSI, and SSEL lines and receiving the MISO line."]
    MASTER,
    #[doc = "The SPI controller acts as a slave on the bus, driving MISO line and receiving SCLK, MOSI, and SSEL lines."]
    SLAVE,
}
impl crate::ToBits<bool> for MSR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MSR::MASTER => false,
            MSR::SLAVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MS_R = crate::FR<bool, MSR>;
impl MS_R {
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MSR::MASTER
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MSR::SLAVE
    }
}
#[doc = "Values that can be written to the field `MS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSW {
    #[doc = "The SPI controller acts as a master on the bus, driving the SCLK, MOSI, and SSEL lines and receiving the MISO line."]
    MASTER,
    #[doc = "The SPI controller acts as a slave on the bus, driving MISO line and receiving SCLK, MOSI, and SSEL lines."]
    SLAVE,
}
impl MSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MSW::MASTER => false,
            MSW::SLAVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MSW<'a> {
    w: &'a mut W,
}
impl<'a> _MSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SPI controller acts as a master on the bus, driving the SCLK, MOSI, and SSEL lines and receiving the MISO line."]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(MSW::MASTER)
    }
    #[doc = "The SPI controller acts as a slave on the bus, driving MISO line and receiving SCLK, MOSI, and SSEL lines."]
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(MSW::SLAVE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SOD_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SODW<'a> {
    w: &'a mut W,
}
impl<'a> _SODW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Loop Back Mode."]
    #[inline(always)]
    pub fn lbm(&self) -> LBM_R {
        LBM_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - SPI Enable."]
    #[inline(always)]
    pub fn sse(&self) -> SSE_R {
        SSE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Master/Slave Mode.This bit can only be written when the SSE bit is 0."]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Slave Output Disable. This bit is relevant only in slave mode (MS = 1). If it is 1, this blocks this SPI controller from driving the transmit data line (MISO)."]
    #[inline(always)]
    pub fn sod(&self) -> SOD_R {
        SOD_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Loop Back Mode."]
    #[inline(always)]
    pub fn lbm(&mut self) -> _LBMW {
        _LBMW { w: self }
    }
    #[doc = "Bit 1 - SPI Enable."]
    #[inline(always)]
    pub fn sse(&mut self) -> _SSEW {
        _SSEW { w: self }
    }
    #[doc = "Bit 2 - Master/Slave Mode.This bit can only be written when the SSE bit is 0."]
    #[inline(always)]
    pub fn ms(&mut self) -> _MSW {
        _MSW { w: self }
    }
    #[doc = "Bit 3 - Slave Output Disable. This bit is relevant only in slave mode (MS = 1). If it is 1, this blocks this SPI controller from driving the transmit data line (MISO)."]
    #[inline(always)]
    pub fn sod(&mut self) -> _SODW {
        _SODW { w: self }
    }
}
