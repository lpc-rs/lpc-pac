#[doc = "Register `PINASSIGN6` reader"]
pub struct R(crate::R<PINASSIGN6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINASSIGN6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PINASSIGN6_SPEC>> for R {
    fn from(reader: crate::R<PINASSIGN6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINASSIGN6` writer"]
pub struct W(crate::W<PINASSIGN6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINASSIGN6_SPEC>;
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
impl core::convert::From<crate::W<PINASSIGN6_SPEC>> for W {
    fn from(writer: crate::W<PINASSIGN6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI1_MISO_IO` reader - SPI1_MISO function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct SPI1_MISO_IO_R(crate::FieldReader<u8, u8>);
impl SPI1_MISO_IO_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPI1_MISO_IO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1_MISO_IO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1_MISO_IO` writer - SPI1_MISO function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct SPI1_MISO_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_MISO_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `SPI1_SSEL0_IO` reader - SPI1_SSEL0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct SPI1_SSEL0_IO_R(crate::FieldReader<u8, u8>);
impl SPI1_SSEL0_IO_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPI1_SSEL0_IO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1_SSEL0_IO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1_SSEL0_IO` writer - SPI1_SSEL0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct SPI1_SSEL0_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_SSEL0_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `SPI1_SSEL1_IO` reader - SPI1_SSEL1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct SPI1_SSEL1_IO_R(crate::FieldReader<u8, u8>);
impl SPI1_SSEL1_IO_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPI1_SSEL1_IO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1_SSEL1_IO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1_SSEL1_IO` writer - SPI1_SSEL1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct SPI1_SSEL1_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_SSEL1_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `SCT_PIN0_I` reader - SCT_PIN0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct SCT_PIN0_I_R(crate::FieldReader<u8, u8>);
impl SCT_PIN0_I_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCT_PIN0_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCT_PIN0_I_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCT_PIN0_I` writer - SCT_PIN0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct SCT_PIN0_I_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_PIN0_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SPI1_MISO function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn spi1_miso_io(&self) -> SPI1_MISO_IO_R {
        SPI1_MISO_IO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SPI1_SSEL0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn spi1_ssel0_io(&self) -> SPI1_SSEL0_IO_R {
        SPI1_SSEL0_IO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SPI1_SSEL1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn spi1_ssel1_io(&self) -> SPI1_SSEL1_IO_R {
        SPI1_SSEL1_IO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SCT_PIN0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn sct_pin0_i(&self) -> SCT_PIN0_I_R {
        SCT_PIN0_I_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI1_MISO function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn spi1_miso_io(&mut self) -> SPI1_MISO_IO_W {
        SPI1_MISO_IO_W { w: self }
    }
    #[doc = "Bits 8:15 - SPI1_SSEL0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn spi1_ssel0_io(&mut self) -> SPI1_SSEL0_IO_W {
        SPI1_SSEL0_IO_W { w: self }
    }
    #[doc = "Bits 16:23 - SPI1_SSEL1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn spi1_ssel1_io(&mut self) -> SPI1_SSEL1_IO_W {
        SPI1_SSEL1_IO_W { w: self }
    }
    #[doc = "Bits 24:31 - SCT_PIN0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn sct_pin0_i(&mut self) -> SCT_PIN0_I_W {
        SCT_PIN0_I_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin assign register 6. Assign movable functions SPI1_MISO, SPI1_SSEL0, SPI1_SSEL1, SCT0_IN0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign6](index.html) module"]
pub struct PINASSIGN6_SPEC;
impl crate::RegisterSpec for PINASSIGN6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinassign6::R](R) reader structure"]
impl crate::Readable for PINASSIGN6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinassign6::W](W) writer structure"]
impl crate::Writable for PINASSIGN6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINASSIGN6 to value 0xffff_ffff"]
impl crate::Resettable for PINASSIGN6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
