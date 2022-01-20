#[doc = "Register `WRTPRT` reader"]
pub struct R(crate::R<WRTPRT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRTPRT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRTPRT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRTPRT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRTPRT` writer"]
pub struct W(crate::W<WRTPRT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRTPRT_SPEC>;
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
impl From<crate::W<WRTPRT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRTPRT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRITE_PROTECT` reader - Write protect."]
pub struct WRITE_PROTECT_R(crate::FieldReader<bool, bool>);
impl WRITE_PROTECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WRITE_PROTECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRITE_PROTECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRITE_PROTECT` writer - Write protect."]
pub struct WRITE_PROTECT_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_PROTECT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write protect."]
    #[inline(always)]
    pub fn write_protect(&self) -> WRITE_PROTECT_R {
        WRITE_PROTECT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write protect."]
    #[inline(always)]
    pub fn write_protect(&mut self) -> WRITE_PROTECT_W {
        WRITE_PROTECT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Protect register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrtprt](index.html) module"]
pub struct WRTPRT_SPEC;
impl crate::RegisterSpec for WRTPRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrtprt::R](R) reader structure"]
impl crate::Readable for WRTPRT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrtprt::W](W) writer structure"]
impl crate::Writable for WRTPRT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WRTPRT to value 0"]
impl crate::Resettable for WRTPRT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
