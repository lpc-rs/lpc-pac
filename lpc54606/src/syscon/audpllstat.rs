#[doc = "Register `AUDPLLSTAT` reader"]
pub struct R(crate::R<AUDPLLSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUDPLLSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<AUDPLLSTAT_SPEC>> for R {
    fn from(reader: crate::R<AUDPLLSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUDPLLSTAT` writer"]
pub struct W(crate::W<AUDPLLSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUDPLLSTAT_SPEC>;
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
impl core::convert::From<crate::W<AUDPLLSTAT_SPEC>> for W {
    fn from(writer: crate::W<AUDPLLSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK` reader - PLL lock indicator."]
pub struct LOCK_R(crate::FieldReader<bool, bool>);
impl LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK` writer - PLL lock indicator."]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
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
    #[doc = "Bit 0 - PLL lock indicator."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL lock indicator."]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Audio PLL status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audpllstat](index.html) module"]
pub struct AUDPLLSTAT_SPEC;
impl crate::RegisterSpec for AUDPLLSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [audpllstat::R](R) reader structure"]
impl crate::Readable for AUDPLLSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [audpllstat::W](W) writer structure"]
impl crate::Writable for AUDPLLSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUDPLLSTAT to value 0"]
impl crate::Resettable for AUDPLLSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
