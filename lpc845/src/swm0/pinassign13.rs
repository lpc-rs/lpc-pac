#[doc = "Register `PINASSIGN13` reader"]
pub struct R(crate::R<PINASSIGN13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINASSIGN13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINASSIGN13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINASSIGN13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINASSIGN13` writer"]
pub struct W(crate::W<PINASSIGN13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINASSIGN13_SPEC>;
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
impl From<crate::W<PINASSIGN13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINASSIGN13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART4_SCLK` reader - UART4_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct UART4_SCLK_R(crate::FieldReader<u8, u8>);
impl UART4_SCLK_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART4_SCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART4_SCLK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART4_SCLK` writer - UART4_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct UART4_SCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4_SCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `T0_MAT0` reader - T0_MAT0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct T0_MAT0_R(crate::FieldReader<u8, u8>);
impl T0_MAT0_R {
    pub(crate) fn new(bits: u8) -> Self {
        T0_MAT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_MAT0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T0_MAT0` writer - T0_MAT0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct T0_MAT0_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_MAT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `T0_MAT1` reader - T0_MAT1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct T0_MAT1_R(crate::FieldReader<u8, u8>);
impl T0_MAT1_R {
    pub(crate) fn new(bits: u8) -> Self {
        T0_MAT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_MAT1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T0_MAT1` writer - T0_MAT1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct T0_MAT1_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_MAT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `T0_MAT2` reader - T0_MAT2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct T0_MAT2_R(crate::FieldReader<u8, u8>);
impl T0_MAT2_R {
    pub(crate) fn new(bits: u8) -> Self {
        T0_MAT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_MAT2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T0_MAT2` writer - T0_MAT2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct T0_MAT2_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_MAT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - UART4_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn uart4_sclk(&self) -> UART4_SCLK_R {
        UART4_SCLK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - T0_MAT0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_mat0(&self) -> T0_MAT0_R {
        T0_MAT0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - T0_MAT1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_mat1(&self) -> T0_MAT1_R {
        T0_MAT1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - T0_MAT2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_mat2(&self) -> T0_MAT2_R {
        T0_MAT2_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - UART4_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn uart4_sclk(&mut self) -> UART4_SCLK_W {
        UART4_SCLK_W { w: self }
    }
    #[doc = "Bits 8:15 - T0_MAT0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_mat0(&mut self) -> T0_MAT0_W {
        T0_MAT0_W { w: self }
    }
    #[doc = "Bits 16:23 - T0_MAT1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_mat1(&mut self) -> T0_MAT1_W {
        T0_MAT1_W { w: self }
    }
    #[doc = "Bits 24:31 - T0_MAT2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_mat2(&mut self) -> T0_MAT2_W {
        T0_MAT2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin assign register 13. Assign movable functions UART4_SCLK, T0_MAT0, T0_MAT1, T0_MAT2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign13](index.html) module"]
pub struct PINASSIGN13_SPEC;
impl crate::RegisterSpec for PINASSIGN13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinassign13::R](R) reader structure"]
impl crate::Readable for PINASSIGN13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinassign13::W](W) writer structure"]
impl crate::Writable for PINASSIGN13_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINASSIGN13 to value 0xffff_ffff"]
impl crate::Resettable for PINASSIGN13_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
