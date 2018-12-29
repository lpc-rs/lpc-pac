#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR1 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
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
impl LBMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LBMR::DURING_NORMAL_OPERAT => false,
            LBMR::SERIAL_INPUT_IS_TAKE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LBMR {
        match value {
            false => LBMR::DURING_NORMAL_OPERAT,
            true => LBMR::SERIAL_INPUT_IS_TAKE,
        }
    }
    #[doc = "Checks if the value of the field is `DURING_NORMAL_OPERAT`"]
    #[inline]
    pub fn is_during_normal_operat(&self) -> bool {
        *self == LBMR::DURING_NORMAL_OPERAT
    }
    #[doc = "Checks if the value of the field is `SERIAL_INPUT_IS_TAKE`"]
    #[inline]
    pub fn is_serial_input_is_take(&self) -> bool {
        *self == LBMR::SERIAL_INPUT_IS_TAKE
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
impl SSER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SSER::DISABLED => false,
            SSER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSER {
        match value {
            false => SSER::DISABLED,
            true => SSER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SSER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SSER::ENABLED
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
impl MSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MSR::MASTER => false,
            MSR::SLAVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSR {
        match value {
            false => MSR::MASTER,
            true => MSR::SLAVE,
        }
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline]
    pub fn is_master(&self) -> bool {
        *self == MSR::MASTER
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline]
    pub fn is_slave(&self) -> bool {
        *self == MSR::SLAVE
    }
}
#[doc = r" Value of the field"]
pub struct SODR {
    bits: bool,
}
impl SODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Values that can be written to the field `LBM`"]
pub enum LBMW {
    #[doc = "During normal operation."]
    DURING_NORMAL_OPERAT,
    #[doc = "Serial input is taken from the serial output (MOSI or MISO) rather than the serial input pin (MISO or MOSI respectively)."]
    SERIAL_INPUT_IS_TAKE,
}
impl LBMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LBMW::DURING_NORMAL_OPERAT => false,
            LBMW::SERIAL_INPUT_IS_TAKE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LBMW<'a> {
    w: &'a mut W,
}
impl<'a> _LBMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LBMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "During normal operation."]
    #[inline]
    pub fn during_normal_operat(self) -> &'a mut W {
        self.variant(LBMW::DURING_NORMAL_OPERAT)
    }
    #[doc = "Serial input is taken from the serial output (MOSI or MISO) rather than the serial input pin (MISO or MOSI respectively)."]
    #[inline]
    pub fn serial_input_is_take(self) -> &'a mut W {
        self.variant(LBMW::SERIAL_INPUT_IS_TAKE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SSE`"]
pub enum SSEW {
    #[doc = "The SPI controller is disabled."]
    DISABLED,
    #[doc = "The SPI controller will interact with other devices on the serial bus. Software should write the appropriate control information to the other SSP/SPI registers and interrupt controller registers, before setting this bit."]
    ENABLED,
}
impl SSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSEW::DISABLED => false,
            SSEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSEW<'a> {
    w: &'a mut W,
}
impl<'a> _SSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SPI controller is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSEW::DISABLED)
    }
    #[doc = "The SPI controller will interact with other devices on the serial bus. Software should write the appropriate control information to the other SSP/SPI registers and interrupt controller registers, before setting this bit."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSEW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MS`"]
pub enum MSW {
    #[doc = "The SPI controller acts as a master on the bus, driving the SCLK, MOSI, and SSEL lines and receiving the MISO line."]
    MASTER,
    #[doc = "The SPI controller acts as a slave on the bus, driving MISO line and receiving SCLK, MOSI, and SSEL lines."]
    SLAVE,
}
impl MSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSW::MASTER => false,
            MSW::SLAVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSW<'a> {
    w: &'a mut W,
}
impl<'a> _MSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SPI controller acts as a master on the bus, driving the SCLK, MOSI, and SSEL lines and receiving the MISO line."]
    #[inline]
    pub fn master(self) -> &'a mut W {
        self.variant(MSW::MASTER)
    }
    #[doc = "The SPI controller acts as a slave on the bus, driving MISO line and receiving SCLK, MOSI, and SSEL lines."]
    #[inline]
    pub fn slave(self) -> &'a mut W {
        self.variant(MSW::SLAVE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SODW<'a> {
    w: &'a mut W,
}
impl<'a> _SODW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Loop Back Mode."]
    #[inline]
    pub fn lbm(&self) -> LBMR {
        LBMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - SPI Enable."]
    #[inline]
    pub fn sse(&self) -> SSER {
        SSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Master/Slave Mode.This bit can only be written when the SSE bit is 0."]
    #[inline]
    pub fn ms(&self) -> MSR {
        MSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Slave Output Disable. This bit is relevant only in slave mode (MS = 1). If it is 1, this blocks this SPI controller from driving the transmit data line (MISO)."]
    #[inline]
    pub fn sod(&self) -> SODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SODR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Loop Back Mode."]
    #[inline]
    pub fn lbm(&mut self) -> _LBMW {
        _LBMW { w: self }
    }
    #[doc = "Bit 1 - SPI Enable."]
    #[inline]
    pub fn sse(&mut self) -> _SSEW {
        _SSEW { w: self }
    }
    #[doc = "Bit 2 - Master/Slave Mode.This bit can only be written when the SSE bit is 0."]
    #[inline]
    pub fn ms(&mut self) -> _MSW {
        _MSW { w: self }
    }
    #[doc = "Bit 3 - Slave Output Disable. This bit is relevant only in slave mode (MS = 1). If it is 1, this blocks this SPI controller from driving the transmit data line (MISO)."]
    #[inline]
    pub fn sod(&mut self) -> _SODW {
        _SODW { w: self }
    }
}
