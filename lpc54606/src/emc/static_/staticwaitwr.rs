#[doc = "Register `STATICWAITWR` reader"]
pub struct R(crate::R<STATICWAITWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATICWAITWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<STATICWAITWR_SPEC>> for R {
    fn from(reader: crate::R<STATICWAITWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATICWAITWR` writer"]
pub struct W(crate::W<STATICWAITWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATICWAITWR_SPEC>;
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
impl core::convert::From<crate::W<STATICWAITWR_SPEC>> for W {
    fn from(writer: crate::W<STATICWAITWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAITWR` reader - Write wait states."]
pub struct WAITWR_R(crate::FieldReader<u8, u8>);
impl WAITWR_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAITWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAITWR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAITWR` writer - Write wait states."]
pub struct WAITWR_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Write wait states."]
    #[inline(always)]
    pub fn waitwr(&self) -> WAITWR_R {
        WAITWR_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Write wait states."]
    #[inline(always)]
    pub fn waitwr(&mut self) -> WAITWR_W {
        WAITWR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Delay from EMC_CSx to a write access\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticwaitwr](index.html) module"]
pub struct STATICWAITWR_SPEC;
impl crate::RegisterSpec for STATICWAITWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [staticwaitwr::R](R) reader structure"]
impl crate::Readable for STATICWAITWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [staticwaitwr::W](W) writer structure"]
impl crate::Writable for STATICWAITWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATICWAITWR to value 0x1f"]
impl crate::Resettable for STATICWAITWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
