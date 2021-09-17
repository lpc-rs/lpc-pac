#[doc = "Register `USBPLLCLKUEN` reader"]
pub struct R(crate::R<USBPLLCLKUEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPLLCLKUEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPLLCLKUEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPLLCLKUEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBPLLCLKUEN` writer"]
pub struct W(crate::W<USBPLLCLKUEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPLLCLKUEN_SPEC>;
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
impl From<crate::W<USBPLLCLKUEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPLLCLKUEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable USB PLL clock source update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_A {
    #[doc = "0: No change"]
    NO_CHANGE = 0,
    #[doc = "1: Update clock source"]
    UPDATE_CLOCK_SOURCE = 1,
}
impl From<ENA_A> for bool {
    #[inline(always)]
    fn from(variant: ENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA` reader - Enable USB PLL clock source update"]
pub struct ENA_R(crate::FieldReader<bool, ENA_A>);
impl ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA_A {
        match self.bits {
            false => ENA_A::NO_CHANGE,
            true => ENA_A::UPDATE_CLOCK_SOURCE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        **self == ENA_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `UPDATE_CLOCK_SOURCE`"]
    #[inline(always)]
    pub fn is_update_clock_source(&self) -> bool {
        **self == ENA_A::UPDATE_CLOCK_SOURCE
    }
}
impl core::ops::Deref for ENA_R {
    type Target = crate::FieldReader<bool, ENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA` writer - Enable USB PLL clock source update"]
pub struct ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No change"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(ENA_A::NO_CHANGE)
    }
    #[doc = "Update clock source"]
    #[inline(always)]
    pub fn update_clock_source(self) -> &'a mut W {
        self.variant(ENA_A::UPDATE_CLOCK_SOURCE)
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
    #[doc = "Bit 0 - Enable USB PLL clock source update"]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable USB PLL clock source update"]
    #[inline(always)]
    pub fn ena(&mut self) -> ENA_W {
        ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PLL clock source update enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbpllclkuen](index.html) module"]
pub struct USBPLLCLKUEN_SPEC;
impl crate::RegisterSpec for USBPLLCLKUEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbpllclkuen::R](R) reader structure"]
impl crate::Readable for USBPLLCLKUEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbpllclkuen::W](W) writer structure"]
impl crate::Writable for USBPLLCLKUEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBPLLCLKUEN to value 0"]
impl crate::Resettable for USBPLLCLKUEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
