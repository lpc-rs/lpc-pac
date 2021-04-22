#[doc = "Register `DYNAMICDAL` reader"]
pub struct R(crate::R<DYNAMICDAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYNAMICDAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DYNAMICDAL_SPEC>> for R {
    fn from(reader: crate::R<DYNAMICDAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DYNAMICDAL` writer"]
pub struct W(crate::W<DYNAMICDAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYNAMICDAL_SPEC>;
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
impl core::convert::From<crate::W<DYNAMICDAL_SPEC>> for W {
    fn from(writer: crate::W<DYNAMICDAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDAL` reader - Data-in to active command."]
pub struct TDAL_R(crate::FieldReader<u8, u8>);
impl TDAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TDAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDAL` writer - Data-in to active command."]
pub struct TDAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TDAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Data-in to active command."]
    #[inline(always)]
    pub fn tdal(&self) -> TDAL_R {
        TDAL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data-in to active command."]
    #[inline(always)]
    pub fn tdal(&mut self) -> TDAL_W {
        TDAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data-in to active command time\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicdal](index.html) module"]
pub struct DYNAMICDAL_SPEC;
impl crate::RegisterSpec for DYNAMICDAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dynamicdal::R](R) reader structure"]
impl crate::Readable for DYNAMICDAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dynamicdal::W](W) writer structure"]
impl crate::Writable for DYNAMICDAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DYNAMICDAL to value 0x0f"]
impl crate::Resettable for DYNAMICDAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
