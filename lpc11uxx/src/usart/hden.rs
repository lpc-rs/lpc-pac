#[doc = "Register `HDEN` reader"]
pub struct R(crate::R<HDEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HDEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HDEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HDEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HDEN` writer"]
pub struct W(crate::W<HDEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HDEN_SPEC>;
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
impl From<crate::W<HDEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HDEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Half-duplex mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDEN_A {
    #[doc = "0: Disable half-duplex mode."]
    DISABLED = 0,
    #[doc = "1: Enable half-duplex mode."]
    ENABLED = 1,
}
impl From<HDEN_A> for bool {
    #[inline(always)]
    fn from(variant: HDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDEN` reader - Half-duplex mode enable"]
pub struct HDEN_R(crate::FieldReader<bool, HDEN_A>);
impl HDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDEN_A {
        match self.bits {
            false => HDEN_A::DISABLED,
            true => HDEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == HDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == HDEN_A::ENABLED
    }
}
impl core::ops::Deref for HDEN_R {
    type Target = crate::FieldReader<bool, HDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDEN` writer - Half-duplex mode enable"]
pub struct HDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable half-duplex mode."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HDEN_A::DISABLED)
    }
    #[doc = "Enable half-duplex mode."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HDEN_A::ENABLED)
    }
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
    #[doc = "Bit 0 - Half-duplex mode enable"]
    #[inline(always)]
    pub fn hden(&self) -> HDEN_R {
        HDEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Half-duplex mode enable"]
    #[inline(always)]
    pub fn hden(&mut self) -> HDEN_W {
        HDEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Half duplex enable register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hden](index.html) module"]
pub struct HDEN_SPEC;
impl crate::RegisterSpec for HDEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hden::R](R) reader structure"]
impl crate::Readable for HDEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hden::W](W) writer structure"]
impl crate::Writable for HDEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HDEN to value 0"]
impl crate::Resettable for HDEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
