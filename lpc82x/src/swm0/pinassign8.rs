#[doc = "Register `PINASSIGN8` reader"]
pub struct R(crate::R<PINASSIGN8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINASSIGN8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PINASSIGN8_SPEC>> for R {
    fn from(reader: crate::R<PINASSIGN8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINASSIGN8` writer"]
pub struct W(crate::W<PINASSIGN8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINASSIGN8_SPEC>;
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
impl core::convert::From<crate::W<PINASSIGN8_SPEC>> for W {
    fn from(writer: crate::W<PINASSIGN8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCT_OUT1_O` reader - SCT_OUT1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct SCT_OUT1_O_R(crate::FieldReader<u8, u8>);
impl SCT_OUT1_O_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCT_OUT1_O_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCT_OUT1_O_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCT_OUT1_O` writer - SCT_OUT1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct SCT_OUT1_O_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_OUT1_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `SCT_OUT2_O` reader - SCT_OUT2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct SCT_OUT2_O_R(crate::FieldReader<u8, u8>);
impl SCT_OUT2_O_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCT_OUT2_O_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCT_OUT2_O_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCT_OUT2_O` writer - SCT_OUT2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct SCT_OUT2_O_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_OUT2_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `SCT_OUT3_O` reader - SCT_OUT3 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct SCT_OUT3_O_R(crate::FieldReader<u8, u8>);
impl SCT_OUT3_O_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCT_OUT3_O_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCT_OUT3_O_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCT_OUT3_O` writer - SCT_OUT3 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct SCT_OUT3_O_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_OUT3_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `SCT_OUT4_O` reader - SCT_OUT4 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct SCT_OUT4_O_R(crate::FieldReader<u8, u8>);
impl SCT_OUT4_O_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCT_OUT4_O_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCT_OUT4_O_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCT_OUT4_O` writer - SCT_OUT4 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
pub struct SCT_OUT4_O_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_OUT4_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SCT_OUT1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn sct_out1_o(&self) -> SCT_OUT1_O_R {
        SCT_OUT1_O_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SCT_OUT2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn sct_out2_o(&self) -> SCT_OUT2_O_R {
        SCT_OUT2_O_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SCT_OUT3 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn sct_out3_o(&self) -> SCT_OUT3_O_R {
        SCT_OUT3_O_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SCT_OUT4 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn sct_out4_o(&self) -> SCT_OUT4_O_R {
        SCT_OUT4_O_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCT_OUT1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn sct_out1_o(&mut self) -> SCT_OUT1_O_W {
        SCT_OUT1_O_W { w: self }
    }
    #[doc = "Bits 8:15 - SCT_OUT2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn sct_out2_o(&mut self) -> SCT_OUT2_O_W {
        SCT_OUT2_O_W { w: self }
    }
    #[doc = "Bits 16:23 - SCT_OUT3 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn sct_out3_o(&mut self) -> SCT_OUT3_O_W {
        SCT_OUT3_O_W { w: self }
    }
    #[doc = "Bits 24:31 - SCT_OUT4 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn sct_out4_o(&mut self) -> SCT_OUT4_O_W {
        SCT_OUT4_O_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin assign register 8. Assign movable functions SCT_OUT1, SCT_OUT2, SCT_OUT3, SCT_OUT4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign8](index.html) module"]
pub struct PINASSIGN8_SPEC;
impl crate::RegisterSpec for PINASSIGN8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinassign8::R](R) reader structure"]
impl crate::Readable for PINASSIGN8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinassign8::W](W) writer structure"]
impl crate::Writable for PINASSIGN8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINASSIGN8 to value 0xffff_ffff"]
impl crate::Resettable for PINASSIGN8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
