#[doc = "Register `OSR` reader"]
pub struct R(crate::R<OSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSR` writer"]
pub struct W(crate::W<OSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSR_SPEC>;
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
impl From<crate::W<OSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSFRAC` reader - Fractional part of the oversampling ratio, in units of 1/8th of an input clock period."]
pub struct OSFRAC_R(crate::FieldReader<u8, u8>);
impl OSFRAC_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSFRAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSFRAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSFRAC` writer - Fractional part of the oversampling ratio, in units of 1/8th of an input clock period."]
pub struct OSFRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> OSFRAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
#[doc = "Field `OSINT` reader - Integer part of the oversampling ratio, minus 1."]
pub struct OSINT_R(crate::FieldReader<u8, u8>);
impl OSINT_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSINT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSINT` writer - Integer part of the oversampling ratio, minus 1."]
pub struct OSINT_W<'a> {
    w: &'a mut W,
}
impl<'a> OSINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `FDINT` reader - These bits act as a more-significant extension of the OSint field, allowing an oversampling ratio up to 2048 as required by ISO7816-3."]
pub struct FDINT_R(crate::FieldReader<u8, u8>);
impl FDINT_R {
    pub(crate) fn new(bits: u8) -> Self {
        FDINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDINT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDINT` writer - These bits act as a more-significant extension of the OSint field, allowing an oversampling ratio up to 2048 as required by ISO7816-3."]
pub struct FDINT_W<'a> {
    w: &'a mut W,
}
impl<'a> FDINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:3 - Fractional part of the oversampling ratio, in units of 1/8th of an input clock period."]
    #[inline(always)]
    pub fn osfrac(&self) -> OSFRAC_R {
        OSFRAC_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - Integer part of the oversampling ratio, minus 1."]
    #[inline(always)]
    pub fn osint(&self) -> OSINT_R {
        OSINT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - These bits act as a more-significant extension of the OSint field, allowing an oversampling ratio up to 2048 as required by ISO7816-3."]
    #[inline(always)]
    pub fn fdint(&self) -> FDINT_R {
        FDINT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:3 - Fractional part of the oversampling ratio, in units of 1/8th of an input clock period."]
    #[inline(always)]
    pub fn osfrac(&mut self) -> OSFRAC_W {
        OSFRAC_W { w: self }
    }
    #[doc = "Bits 4:7 - Integer part of the oversampling ratio, minus 1."]
    #[inline(always)]
    pub fn osint(&mut self) -> OSINT_W {
        OSINT_W { w: self }
    }
    #[doc = "Bits 8:14 - These bits act as a more-significant extension of the OSint field, allowing an oversampling ratio up to 2048 as required by ISO7816-3."]
    #[inline(always)]
    pub fn fdint(&mut self) -> FDINT_W {
        FDINT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oversampling register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osr](index.html) module"]
pub struct OSR_SPEC;
impl crate::RegisterSpec for OSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osr::R](R) reader structure"]
impl crate::Readable for OSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osr::W](W) writer structure"]
impl crate::Writable for OSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSR to value 0xf0"]
impl crate::Resettable for OSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf0
    }
}
