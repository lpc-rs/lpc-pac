#[doc = "Register `PINASSIGN12` reader"]
pub struct R(crate::R<PINASSIGN12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINASSIGN12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINASSIGN12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINASSIGN12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINASSIGN12` writer"]
pub struct W(crate::W<PINASSIGN12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINASSIGN12_SPEC>;
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
impl From<crate::W<PINASSIGN12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINASSIGN12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART3_RXD` reader - UART3_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct UART3_RXD_R(crate::FieldReader<u8, u8>);
impl UART3_RXD_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART3_RXD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART3_RXD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART3_RXD` writer - UART3_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct UART3_RXD_W<'a> {
    w: &'a mut W,
}
impl<'a> UART3_RXD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `UART3_SCLK` reader - UART3_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct UART3_SCLK_R(crate::FieldReader<u8, u8>);
impl UART3_SCLK_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART3_SCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART3_SCLK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART3_SCLK` writer - UART3_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct UART3_SCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> UART3_SCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `UART4_TXD` reader - UART4_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct UART4_TXD_R(crate::FieldReader<u8, u8>);
impl UART4_TXD_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART4_TXD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART4_TXD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART4_TXD` writer - UART4_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct UART4_TXD_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4_TXD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `UART4_RXD` reader - UART4_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct UART4_RXD_R(crate::FieldReader<u8, u8>);
impl UART4_RXD_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART4_RXD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART4_RXD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART4_RXD` writer - UART4_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct UART4_RXD_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4_RXD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - UART3_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn uart3_rxd(&self) -> UART3_RXD_R {
        UART3_RXD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - UART3_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn uart3_sclk(&self) -> UART3_SCLK_R {
        UART3_SCLK_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - UART4_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn uart4_txd(&self) -> UART4_TXD_R {
        UART4_TXD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - UART4_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn uart4_rxd(&self) -> UART4_RXD_R {
        UART4_RXD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - UART3_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn uart3_rxd(&mut self) -> UART3_RXD_W {
        UART3_RXD_W { w: self }
    }
    #[doc = "Bits 8:15 - UART3_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn uart3_sclk(&mut self) -> UART3_SCLK_W {
        UART3_SCLK_W { w: self }
    }
    #[doc = "Bits 16:23 - UART4_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn uart4_txd(&mut self) -> UART4_TXD_W {
        UART4_TXD_W { w: self }
    }
    #[doc = "Bits 24:31 - UART4_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn uart4_rxd(&mut self) -> UART4_RXD_W {
        UART4_RXD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin assign register 12. Assign movable functions UART3_RXD, UART3_SCLK, UART4_TXD, UART4_RXD.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign12](index.html) module"]
pub struct PINASSIGN12_SPEC;
impl crate::RegisterSpec for PINASSIGN12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinassign12::R](R) reader structure"]
impl crate::Readable for PINASSIGN12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinassign12::W](W) writer structure"]
impl crate::Writable for PINASSIGN12_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINASSIGN12 to value 0xffff_ffff"]
impl crate::Resettable for PINASSIGN12_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
