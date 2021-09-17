#[doc = "Register `IRCCTRL` reader"]
pub struct R(crate::R<IRCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRCCTRL` writer"]
pub struct W(crate::W<IRCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRCCTRL_SPEC>;
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
impl From<crate::W<IRCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIM` reader - Trim value"]
pub struct TRIM_R(crate::FieldReader<u8, u8>);
impl TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIM` writer - Trim value"]
pub struct TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Trim value"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Trim value"]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W {
        TRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IRC control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ircctrl](index.html) module"]
pub struct IRCCTRL_SPEC;
impl crate::RegisterSpec for IRCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ircctrl::R](R) reader structure"]
impl crate::Readable for IRCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ircctrl::W](W) writer structure"]
impl crate::Writable for IRCCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRCCTRL to value 0x80"]
impl crate::Resettable for IRCCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
