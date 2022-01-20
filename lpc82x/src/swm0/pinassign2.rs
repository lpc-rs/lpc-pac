#[doc = "Register `PINASSIGN2` reader"]
pub struct R(crate::R<PINASSIGN2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINASSIGN2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINASSIGN2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINASSIGN2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINASSIGN2` writer"]
pub struct W(crate::W<PINASSIGN2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINASSIGN2_SPEC>;
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
impl From<crate::W<PINASSIGN2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINASSIGN2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `U1_CTS_I` reader - U1_CTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
pub struct U1_CTS_I_R(crate::FieldReader<u8, u8>);
impl U1_CTS_I_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        U1_CTS_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U1_CTS_I_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U1_CTS_I` writer - U1_CTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
pub struct U1_CTS_I_W<'a> {
    w: &'a mut W,
}
impl<'a> U1_CTS_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `U1_SCLK_IO` reader - U1_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
pub struct U1_SCLK_IO_R(crate::FieldReader<u8, u8>);
impl U1_SCLK_IO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        U1_SCLK_IO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U1_SCLK_IO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U1_SCLK_IO` writer - U1_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
pub struct U1_SCLK_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> U1_SCLK_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `U2_TXD_O` reader - U2_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
pub struct U2_TXD_O_R(crate::FieldReader<u8, u8>);
impl U2_TXD_O_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        U2_TXD_O_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U2_TXD_O_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U2_TXD_O` writer - U2_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
pub struct U2_TXD_O_W<'a> {
    w: &'a mut W,
}
impl<'a> U2_TXD_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `U2_RXD_I` reader - U2_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
pub struct U2_RXD_I_R(crate::FieldReader<u8, u8>);
impl U2_RXD_I_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        U2_RXD_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U2_RXD_I_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U2_RXD_I` writer - U2_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
pub struct U2_RXD_I_W<'a> {
    w: &'a mut W,
}
impl<'a> U2_RXD_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - U1_CTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u1_cts_i(&self) -> U1_CTS_I_R {
        U1_CTS_I_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - U1_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u1_sclk_io(&self) -> U1_SCLK_IO_R {
        U1_SCLK_IO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - U2_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u2_txd_o(&self) -> U2_TXD_O_R {
        U2_TXD_O_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - U2_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u2_rxd_i(&self) -> U2_RXD_I_R {
        U2_RXD_I_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - U1_CTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u1_cts_i(&mut self) -> U1_CTS_I_W {
        U1_CTS_I_W { w: self }
    }
    #[doc = "Bits 8:15 - U1_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u1_sclk_io(&mut self) -> U1_SCLK_IO_W {
        U1_SCLK_IO_W { w: self }
    }
    #[doc = "Bits 16:23 - U2_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u2_txd_o(&mut self) -> U2_TXD_O_W {
        U2_TXD_O_W { w: self }
    }
    #[doc = "Bits 24:31 - U2_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u2_rxd_i(&mut self) -> U2_RXD_I_W {
        U2_RXD_I_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin assign register 2. Assign movable functions U1_CTS, U1_SCLK, U2_TXD, U2_RXD.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign2](index.html) module"]
pub struct PINASSIGN2_SPEC;
impl crate::RegisterSpec for PINASSIGN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinassign2::R](R) reader structure"]
impl crate::Readable for PINASSIGN2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinassign2::W](W) writer structure"]
impl crate::Writable for PINASSIGN2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINASSIGN2 to value 0xffff_ffff"]
impl crate::Resettable for PINASSIGN2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
