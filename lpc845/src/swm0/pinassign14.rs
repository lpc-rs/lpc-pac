#[doc = "Register `PINASSIGN14` reader"]
pub struct R(crate::R<PINASSIGN14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINASSIGN14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PINASSIGN14_SPEC>> for R {
    fn from(reader: crate::R<PINASSIGN14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINASSIGN14` writer"]
pub struct W(crate::W<PINASSIGN14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINASSIGN14_SPEC>;
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
impl core::convert::From<crate::W<PINASSIGN14_SPEC>> for W {
    fn from(writer: crate::W<PINASSIGN14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T0_MAT3` reader - T0_MAT3 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct T0_MAT3_R(crate::FieldReader<u8, u8>);
impl T0_MAT3_R {
    pub(crate) fn new(bits: u8) -> Self {
        T0_MAT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_MAT3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T0_MAT3` writer - T0_MAT3 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct T0_MAT3_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_MAT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `T0_CAP0` reader - T0_CAP0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct T0_CAP0_R(crate::FieldReader<u8, u8>);
impl T0_CAP0_R {
    pub(crate) fn new(bits: u8) -> Self {
        T0_CAP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_CAP0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T0_CAP0` writer - T0_CAP0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct T0_CAP0_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_CAP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `T0_CAP1` reader - T0_CAP1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct T0_CAP1_R(crate::FieldReader<u8, u8>);
impl T0_CAP1_R {
    pub(crate) fn new(bits: u8) -> Self {
        T0_CAP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_CAP1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T0_CAP1` writer - T0_CAP1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct T0_CAP1_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_CAP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `T0_CAP2` reader - T0_CAP2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct T0_CAP2_R(crate::FieldReader<u8, u8>);
impl T0_CAP2_R {
    pub(crate) fn new(bits: u8) -> Self {
        T0_CAP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_CAP2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T0_CAP2` writer - T0_CAP2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct T0_CAP2_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_CAP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - T0_MAT3 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_mat3(&self) -> T0_MAT3_R {
        T0_MAT3_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - T0_CAP0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_cap0(&self) -> T0_CAP0_R {
        T0_CAP0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - T0_CAP1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_cap1(&self) -> T0_CAP1_R {
        T0_CAP1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - T0_CAP2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_cap2(&self) -> T0_CAP2_R {
        T0_CAP2_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - T0_MAT3 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_mat3(&mut self) -> T0_MAT3_W {
        T0_MAT3_W { w: self }
    }
    #[doc = "Bits 8:15 - T0_CAP0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_cap0(&mut self) -> T0_CAP0_W {
        T0_CAP0_W { w: self }
    }
    #[doc = "Bits 16:23 - T0_CAP1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_cap1(&mut self) -> T0_CAP1_W {
        T0_CAP1_W { w: self }
    }
    #[doc = "Bits 24:31 - T0_CAP2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_cap2(&mut self) -> T0_CAP2_W {
        T0_CAP2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin assign register 14. Assign movable functions T0_MAT3, T0_CAP0, T0_CAP1, T0_CAP2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign14](index.html) module"]
pub struct PINASSIGN14_SPEC;
impl crate::RegisterSpec for PINASSIGN14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinassign14::R](R) reader structure"]
impl crate::Readable for PINASSIGN14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinassign14::W](W) writer structure"]
impl crate::Writable for PINASSIGN14_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINASSIGN14 to value 0xffff_ffff"]
impl crate::Resettable for PINASSIGN14_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
