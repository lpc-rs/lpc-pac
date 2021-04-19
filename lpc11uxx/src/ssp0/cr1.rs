#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CR1_SPEC>> for R {
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<CR1_SPEC>> for W {
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Loop Back Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBM_A {
    #[doc = "0: During normal operation."]
    NORMAL_OPERATION = 0,
    #[doc = "1: Serial input is taken from the serial output (MOSI or MISO) rather than the serial input pin (MISO or MOSI respectively)."]
    SERIAL_OUTPUT = 1,
}
impl From<LBM_A> for bool {
    #[inline(always)]
    fn from(variant: LBM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBM` reader - Loop Back Mode."]
pub struct LBM_R(crate::FieldReader<bool, LBM_A>);
impl LBM_R {
    pub(crate) fn new(bits: bool) -> Self {
        LBM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBM_A {
        match self.bits {
            false => LBM_A::NORMAL_OPERATION,
            true => LBM_A::SERIAL_OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_OPERATION`"]
    #[inline(always)]
    pub fn is_normal_operation(&self) -> bool {
        **self == LBM_A::NORMAL_OPERATION
    }
    #[doc = "Checks if the value of the field is `SERIAL_OUTPUT`"]
    #[inline(always)]
    pub fn is_serial_output(&self) -> bool {
        **self == LBM_A::SERIAL_OUTPUT
    }
}
impl core::ops::Deref for LBM_R {
    type Target = crate::FieldReader<bool, LBM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LBM` writer - Loop Back Mode."]
pub struct LBM_W<'a> {
    w: &'a mut W,
}
impl<'a> LBM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "During normal operation."]
    #[inline(always)]
    pub fn normal_operation(self) -> &'a mut W {
        self.variant(LBM_A::NORMAL_OPERATION)
    }
    #[doc = "Serial input is taken from the serial output (MOSI or MISO) rather than the serial input pin (MISO or MOSI respectively)."]
    #[inline(always)]
    pub fn serial_output(self) -> &'a mut W {
        self.variant(LBM_A::SERIAL_OUTPUT)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "SPI Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSE_A {
    #[doc = "0: The SPI controller is disabled."]
    DISABLED = 0,
    #[doc = "1: The SPI controller will interact with other devices on the serial bus. Software should write the appropriate control information to the other SSP/SPI registers and interrupt controller registers, before setting this bit."]
    ENABLED = 1,
}
impl From<SSE_A> for bool {
    #[inline(always)]
    fn from(variant: SSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSE` reader - SPI Enable."]
pub struct SSE_R(crate::FieldReader<bool, SSE_A>);
impl SSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSE_R(crate::FieldReader::new(bits))
    }
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
        **self == SSE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SSE_A::ENABLED
    }
}
impl core::ops::Deref for SSE_R {
    type Target = crate::FieldReader<bool, SSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSE` writer - SPI Enable."]
pub struct SSE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The SPI controller is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSE_A::DISABLED)
    }
    #[doc = "The SPI controller will interact with other devices on the serial bus. Software should write the appropriate control information to the other SSP/SPI registers and interrupt controller registers, before setting this bit."]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Master/Slave Mode.This bit can only be written when the SSE bit is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MS_A {
    #[doc = "0: The SPI controller acts as a master on the bus, driving the SCLK, MOSI, and SSEL lines and receiving the MISO line."]
    MASTER = 0,
    #[doc = "1: The SPI controller acts as a slave on the bus, driving MISO line and receiving SCLK, MOSI, and SSEL lines."]
    SLAVE = 1,
}
impl From<MS_A> for bool {
    #[inline(always)]
    fn from(variant: MS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MS` reader - Master/Slave Mode.This bit can only be written when the SSE bit is 0."]
pub struct MS_R(crate::FieldReader<bool, MS_A>);
impl MS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MS_R(crate::FieldReader::new(bits))
    }
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
        **self == MS_A::MASTER
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        **self == MS_A::SLAVE
    }
}
impl core::ops::Deref for MS_R {
    type Target = crate::FieldReader<bool, MS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MS` writer - Master/Slave Mode.This bit can only be written when the SSE bit is 0."]
pub struct MS_W<'a> {
    w: &'a mut W,
}
impl<'a> MS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The SPI controller acts as a master on the bus, driving the SCLK, MOSI, and SSEL lines and receiving the MISO line."]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(MS_A::MASTER)
    }
    #[doc = "The SPI controller acts as a slave on the bus, driving MISO line and receiving SCLK, MOSI, and SSEL lines."]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `SOD` reader - Slave Output Disable. This bit is relevant only in slave mode (MS = 1). If it is 1, this blocks this SPI controller from driving the transmit data line (MISO)."]
pub struct SOD_R(crate::FieldReader<bool, bool>);
impl SOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOD` writer - Slave Output Disable. This bit is relevant only in slave mode (MS = 1). If it is 1, this blocks this SPI controller from driving the transmit data line (MISO)."]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Loop Back Mode."]
    #[inline(always)]
    pub fn lbm(&self) -> LBM_R {
        LBM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SPI Enable."]
    #[inline(always)]
    pub fn sse(&self) -> SSE_R {
        SSE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Master/Slave Mode.This bit can only be written when the SSE bit is 0."]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Slave Output Disable. This bit is relevant only in slave mode (MS = 1). If it is 1, this blocks this SPI controller from driving the transmit data line (MISO)."]
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
    #[doc = "Bit 1 - SPI Enable."]
    #[inline(always)]
    pub fn sse(&mut self) -> SSE_W {
        SSE_W { w: self }
    }
    #[doc = "Bit 2 - Master/Slave Mode.This bit can only be written when the SSE bit is 0."]
    #[inline(always)]
    pub fn ms(&mut self) -> MS_W {
        MS_W { w: self }
    }
    #[doc = "Bit 3 - Slave Output Disable. This bit is relevant only in slave mode (MS = 1). If it is 1, this blocks this SPI controller from driving the transmit data line (MISO)."]
    #[inline(always)]
    pub fn sod(&mut self) -> SOD_W {
        SOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register 1. Selects master/slave and other modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
