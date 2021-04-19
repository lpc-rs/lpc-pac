#[doc = "Register `FROOSCCTRL` reader"]
pub struct R(crate::R<FROOSCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FROOSCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FROOSCCTRL_SPEC>> for R {
    fn from(reader: crate::R<FROOSCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FROOSCCTRL` writer"]
pub struct W(crate::W<FROOSCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FROOSCCTRL_SPEC>;
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
impl core::convert::From<crate::W<FROOSCCTRL_SPEC>> for W {
    fn from(writer: crate::W<FROOSCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "fro direct clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRO_DIRECT_A {
    #[doc = "0: fro clock is divider by 2 or 16,depend on FAIM slow boot value"]
    DISABLED = 0,
    #[doc = "1: fro clock is direct from FRO oscillator"]
    ENABLED = 1,
}
impl From<FRO_DIRECT_A> for bool {
    #[inline(always)]
    fn from(variant: FRO_DIRECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRO_DIRECT` reader - fro direct clock select"]
pub struct FRO_DIRECT_R(crate::FieldReader<bool, FRO_DIRECT_A>);
impl FRO_DIRECT_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRO_DIRECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRO_DIRECT_A {
        match self.bits {
            false => FRO_DIRECT_A::DISABLED,
            true => FRO_DIRECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FRO_DIRECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FRO_DIRECT_A::ENABLED
    }
}
impl core::ops::Deref for FRO_DIRECT_R {
    type Target = crate::FieldReader<bool, FRO_DIRECT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRO_DIRECT` writer - fro direct clock select"]
pub struct FRO_DIRECT_W<'a> {
    w: &'a mut W,
}
impl<'a> FRO_DIRECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRO_DIRECT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "fro clock is divider by 2 or 16,depend on FAIM slow boot value"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FRO_DIRECT_A::DISABLED)
    }
    #[doc = "fro clock is direct from FRO oscillator"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FRO_DIRECT_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 17 - fro direct clock select"]
    #[inline(always)]
    pub fn fro_direct(&self) -> FRO_DIRECT_R {
        FRO_DIRECT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - fro direct clock select"]
    #[inline(always)]
    pub fn fro_direct(&mut self) -> FRO_DIRECT_W {
        FRO_DIRECT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FRO oscillator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frooscctrl](index.html) module"]
pub struct FROOSCCTRL_SPEC;
impl crate::RegisterSpec for FROOSCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frooscctrl::R](R) reader structure"]
impl crate::Readable for FROOSCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frooscctrl::W](W) writer structure"]
impl crate::Writable for FROOSCCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FROOSCCTRL to value 0"]
impl crate::Resettable for FROOSCCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
