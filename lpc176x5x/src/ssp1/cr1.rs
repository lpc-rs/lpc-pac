#[doc = "Reader of register CR1"]
pub type R = crate::R<u32, super::CR1>;
#[doc = "Writer for register CR1"]
pub type W = crate::W<u32, super::CR1>;
#[doc = "Register CR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Loop Back Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBM_A {
    #[doc = "0: During normal operation."]
    NORMAL = 0,
    #[doc = "1: Serial input is taken from the serial output (MOSI or MISO) rather than the serial input pin (MISO or MOSI respectively)."]
    OUPTU = 1,
}
impl From<LBM_A> for bool {
    #[inline(always)]
    fn from(variant: LBM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LBM`"]
pub type LBM_R = crate::R<bool, LBM_A>;
impl LBM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBM_A {
        match self.bits {
            false => LBM_A::NORMAL,
            true => LBM_A::OUPTU,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == LBM_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OUPTU`"]
    #[inline(always)]
    pub fn is_ouptu(&self) -> bool {
        *self == LBM_A::OUPTU
    }
}
#[doc = "Write proxy for field `LBM`"]
pub struct LBM_W<'a> {
    w: &'a mut W,
}
impl<'a> LBM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "During normal operation."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(LBM_A::NORMAL)
    }
    #[doc = "Serial input is taken from the serial output (MOSI or MISO) rather than the serial input pin (MISO or MOSI respectively)."]
    #[inline(always)]
    pub fn ouptu(self) -> &'a mut W {
        self.variant(LBM_A::OUPTU)
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
#[doc = "SSP Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSE_A {
    #[doc = "0: The SSP controller is disabled."]
    DISABLED = 0,
    #[doc = "1: The SSP controller will interact with other devices on the serial bus. Software should write the appropriate control information to the other SSP registers and interrupt controller registers, before setting this bit."]
    ENABLED = 1,
}
impl From<SSE_A> for bool {
    #[inline(always)]
    fn from(variant: SSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSE`"]
pub type SSE_R = crate::R<bool, SSE_A>;
impl SSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSE_A {
        match self.bits {
            false => SSE_A::DISABLED,
            true => SSE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSE_A::ENABLED
    }
}
#[doc = "Write proxy for field `SSE`"]
pub struct SSE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The SSP controller is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSE_A::DISABLED)
    }
    #[doc = "The SSP controller will interact with other devices on the serial bus. Software should write the appropriate control information to the other SSP registers and interrupt controller registers, before setting this bit."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSE_A::ENABLED)
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
#[doc = "Master/Slave Mode.This bit can only be written when the SSE bit is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MS_A {
    #[doc = "0: The SSP controller acts as a master on the bus, driving the SCLK, MOSI, and SSEL lines and receiving the MISO line."]
    MASTER = 0,
    #[doc = "1: The SSP controller acts as a slave on the bus, driving MISO line and receiving SCLK, MOSI, and SSEL lines."]
    SLAVE = 1,
}
impl From<MS_A> for bool {
    #[inline(always)]
    fn from(variant: MS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MS`"]
pub type MS_R = crate::R<bool, MS_A>;
impl MS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MS_A {
        match self.bits {
            false => MS_A::MASTER,
            true => MS_A::SLAVE,
        }
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MS_A::MASTER
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MS_A::SLAVE
    }
}
#[doc = "Write proxy for field `MS`"]
pub struct MS_W<'a> {
    w: &'a mut W,
}
impl<'a> MS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The SSP controller acts as a master on the bus, driving the SCLK, MOSI, and SSEL lines and receiving the MISO line."]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(MS_A::MASTER)
    }
    #[doc = "The SSP controller acts as a slave on the bus, driving MISO line and receiving SCLK, MOSI, and SSEL lines."]
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(MS_A::SLAVE)
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
#[doc = "Reader of field `SOD`"]
pub type SOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOD`"]
pub struct SOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SOD_W<'a> {
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
    #[doc = "Bit 0 - Loop Back Mode."]
    #[inline(always)]
    pub fn lbm(&self) -> LBM_R {
        LBM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SSP Enable."]
    #[inline(always)]
    pub fn sse(&self) -> SSE_R {
        SSE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Master/Slave Mode.This bit can only be written when the SSE bit is 0."]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Slave Output Disable. This bit is relevant only in slave mode (MS = 1). If it is 1, this blocks this SSP controller from driving the transmit data line (MISO)."]
    #[inline(always)]
    pub fn sod(&self) -> SOD_R {
        SOD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Loop Back Mode."]
    #[inline(always)]
    pub fn lbm(&mut self) -> LBM_W {
        LBM_W { w: self }
    }
    #[doc = "Bit 1 - SSP Enable."]
    #[inline(always)]
    pub fn sse(&mut self) -> SSE_W {
        SSE_W { w: self }
    }
    #[doc = "Bit 2 - Master/Slave Mode.This bit can only be written when the SSE bit is 0."]
    #[inline(always)]
    pub fn ms(&mut self) -> MS_W {
        MS_W { w: self }
    }
    #[doc = "Bit 3 - Slave Output Disable. This bit is relevant only in slave mode (MS = 1). If it is 1, this blocks this SSP controller from driving the transmit data line (MISO)."]
    #[inline(always)]
    pub fn sod(&mut self) -> SOD_W {
        SOD_W { w: self }
    }
}
