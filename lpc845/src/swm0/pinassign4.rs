#[doc = "Register `PINASSIGN4` reader"]
pub struct R(crate::R<PINASSIGN4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINASSIGN4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINASSIGN4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINASSIGN4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINASSIGN4` writer"]
pub struct W(crate::W<PINASSIGN4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINASSIGN4_SPEC>;
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
impl From<crate::W<PINASSIGN4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINASSIGN4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI0_MOSI_IO` reader - SPI0_MOSI function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct SPI0_MOSI_IO_R(crate::FieldReader<u8, u8>);
impl SPI0_MOSI_IO_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPI0_MOSI_IO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI0_MOSI_IO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI0_MOSI_IO` writer - SPI0_MOSI function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct SPI0_MOSI_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_MOSI_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `SPI0_MISO_IO` reader - SPI0_MISIO function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct SPI0_MISO_IO_R(crate::FieldReader<u8, u8>);
impl SPI0_MISO_IO_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPI0_MISO_IO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI0_MISO_IO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI0_MISO_IO` writer - SPI0_MISIO function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct SPI0_MISO_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_MISO_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `SPI0_SSEL0_IO` reader - SPI0_SSEL0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct SPI0_SSEL0_IO_R(crate::FieldReader<u8, u8>);
impl SPI0_SSEL0_IO_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPI0_SSEL0_IO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI0_SSEL0_IO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI0_SSEL0_IO` writer - SPI0_SSEL0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct SPI0_SSEL0_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_SSEL0_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `SPI0_SSEL1_IO` reader - SPI0_SSEL1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct SPI0_SSEL1_IO_R(crate::FieldReader<u8, u8>);
impl SPI0_SSEL1_IO_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPI0_SSEL1_IO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI0_SSEL1_IO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI0_SSEL1_IO` writer - SPI0_SSEL1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct SPI0_SSEL1_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_SSEL1_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SPI0_MOSI function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn spi0_mosi_io(&self) -> SPI0_MOSI_IO_R {
        SPI0_MOSI_IO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SPI0_MISIO function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn spi0_miso_io(&self) -> SPI0_MISO_IO_R {
        SPI0_MISO_IO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SPI0_SSEL0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn spi0_ssel0_io(&self) -> SPI0_SSEL0_IO_R {
        SPI0_SSEL0_IO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SPI0_SSEL1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn spi0_ssel1_io(&self) -> SPI0_SSEL1_IO_R {
        SPI0_SSEL1_IO_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI0_MOSI function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn spi0_mosi_io(&mut self) -> SPI0_MOSI_IO_W {
        SPI0_MOSI_IO_W { w: self }
    }
    #[doc = "Bits 8:15 - SPI0_MISIO function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn spi0_miso_io(&mut self) -> SPI0_MISO_IO_W {
        SPI0_MISO_IO_W { w: self }
    }
    #[doc = "Bits 16:23 - SPI0_SSEL0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn spi0_ssel0_io(&mut self) -> SPI0_SSEL0_IO_W {
        SPI0_SSEL0_IO_W { w: self }
    }
    #[doc = "Bits 24:31 - SPI0_SSEL1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn spi0_ssel1_io(&mut self) -> SPI0_SSEL1_IO_W {
        SPI0_SSEL1_IO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin assign register 4. Assign movable functions SPI0_MOSI, SPI0_MISO, SPI0_SSEL0, SPI0_SSEL1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign4](index.html) module"]
pub struct PINASSIGN4_SPEC;
impl crate::RegisterSpec for PINASSIGN4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinassign4::R](R) reader structure"]
impl crate::Readable for PINASSIGN4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinassign4::W](W) writer structure"]
impl crate::Writable for PINASSIGN4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINASSIGN4 to value 0xffff_ffff"]
impl crate::Resettable for PINASSIGN4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
