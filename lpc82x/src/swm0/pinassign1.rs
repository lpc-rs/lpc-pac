#[doc = "Register `PINASSIGN1` reader"]
pub struct R(crate::R<PINASSIGN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINASSIGN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINASSIGN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINASSIGN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINASSIGN1` writer"]
pub struct W(crate::W<PINASSIGN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINASSIGN1_SPEC>;
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
impl From<crate::W<PINASSIGN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINASSIGN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `U0_SCLK_IO` reader - U0_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct U0_SCLK_IO_R(crate::FieldReader<u8, u8>);
impl U0_SCLK_IO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        U0_SCLK_IO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U0_SCLK_IO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U0_SCLK_IO` writer - U0_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct U0_SCLK_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> U0_SCLK_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `U1_TXD_O` reader - U1_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct U1_TXD_O_R(crate::FieldReader<u8, u8>);
impl U1_TXD_O_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        U1_TXD_O_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U1_TXD_O_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U1_TXD_O` writer - U1_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct U1_TXD_O_W<'a> {
    w: &'a mut W,
}
impl<'a> U1_TXD_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `U1_RXD_I` reader - U1_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
pub struct U1_RXD_I_R(crate::FieldReader<u8, u8>);
impl U1_RXD_I_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        U1_RXD_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U1_RXD_I_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U1_RXD_I` writer - U1_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
pub struct U1_RXD_I_W<'a> {
    w: &'a mut W,
}
impl<'a> U1_RXD_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `U1_RTS_O` reader - U1_RTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct U1_RTS_O_R(crate::FieldReader<u8, u8>);
impl U1_RTS_O_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        U1_RTS_O_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U1_RTS_O_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U1_RTS_O` writer - U1_RTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct U1_RTS_O_W<'a> {
    w: &'a mut W,
}
impl<'a> U1_RTS_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - U0_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u0_sclk_io(&self) -> U0_SCLK_IO_R {
        U0_SCLK_IO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - U1_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u1_txd_o(&self) -> U1_TXD_O_R {
        U1_TXD_O_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - U1_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u1_rxd_i(&self) -> U1_RXD_I_R {
        U1_RXD_I_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - U1_RTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u1_rts_o(&self) -> U1_RTS_O_R {
        U1_RTS_O_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - U0_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u0_sclk_io(&mut self) -> U0_SCLK_IO_W {
        U0_SCLK_IO_W { w: self }
    }
    #[doc = "Bits 8:15 - U1_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u1_txd_o(&mut self) -> U1_TXD_O_W {
        U1_TXD_O_W { w: self }
    }
    #[doc = "Bits 16:23 - U1_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u1_rxd_i(&mut self) -> U1_RXD_I_W {
        U1_RXD_I_W { w: self }
    }
    #[doc = "Bits 24:31 - U1_RTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u1_rts_o(&mut self) -> U1_RTS_O_W {
        U1_RTS_O_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin assign register 1. Assign movable functions U0_SCLK, U1_TXD, U1_RXD, U1_RTS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign1](index.html) module"]
pub struct PINASSIGN1_SPEC;
impl crate::RegisterSpec for PINASSIGN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinassign1::R](R) reader structure"]
impl crate::Readable for PINASSIGN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinassign1::W](W) writer structure"]
impl crate::Writable for PINASSIGN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINASSIGN1 to value 0xffff_ffff"]
impl crate::Resettable for PINASSIGN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
