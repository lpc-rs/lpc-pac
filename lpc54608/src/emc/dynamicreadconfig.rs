#[doc = "Register `DYNAMICREADCONFIG` reader"]
pub struct R(crate::R<DYNAMICREADCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYNAMICREADCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DYNAMICREADCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DYNAMICREADCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DYNAMICREADCONFIG` writer"]
pub struct W(crate::W<DYNAMICREADCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYNAMICREADCONFIG_SPEC>;
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
impl From<crate::W<DYNAMICREADCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DYNAMICREADCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RD` reader - Read data strategy."]
pub struct RD_R(crate::FieldReader<u8, u8>);
impl RD_R {
    pub(crate) fn new(bits: u8) -> Self {
        RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD` writer - Read data strategy."]
pub struct RD_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Read data strategy."]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Read data strategy."]
    #[inline(always)]
    pub fn rd(&mut self) -> RD_W {
        RD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configures dynamic memory read strategy\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicreadconfig](index.html) module"]
pub struct DYNAMICREADCONFIG_SPEC;
impl crate::RegisterSpec for DYNAMICREADCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dynamicreadconfig::R](R) reader structure"]
impl crate::Readable for DYNAMICREADCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dynamicreadconfig::W](W) writer structure"]
impl crate::Writable for DYNAMICREADCONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DYNAMICREADCONFIG to value 0"]
impl crate::Resettable for DYNAMICREADCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
