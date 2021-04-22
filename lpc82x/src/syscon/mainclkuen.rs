#[doc = "Register `MAINCLKUEN` reader"]
pub struct R(crate::R<MAINCLKUEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAINCLKUEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MAINCLKUEN_SPEC>> for R {
    fn from(reader: crate::R<MAINCLKUEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAINCLKUEN` writer"]
pub struct W(crate::W<MAINCLKUEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAINCLKUEN_SPEC>;
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
impl core::convert::From<crate::W<MAINCLKUEN_SPEC>> for W {
    fn from(writer: crate::W<MAINCLKUEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable main clock source update.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_A {
    #[doc = "0: No change."]
    ENA_0 = 0,
    #[doc = "1: Update clock source."]
    ENA_1 = 1,
}
impl From<ENA_A> for bool {
    #[inline(always)]
    fn from(variant: ENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA` reader - Enable main clock source update."]
pub struct ENA_R(crate::FieldReader<bool, ENA_A>);
impl ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA_A {
        match self.bits {
            false => ENA_A::ENA_0,
            true => ENA_A::ENA_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENA_0`"]
    #[inline(always)]
    pub fn is_ena_0(&self) -> bool {
        **self == ENA_A::ENA_0
    }
    #[doc = "Checks if the value of the field is `ENA_1`"]
    #[inline(always)]
    pub fn is_ena_1(&self) -> bool {
        **self == ENA_A::ENA_1
    }
}
impl core::ops::Deref for ENA_R {
    type Target = crate::FieldReader<bool, ENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA` writer - Enable main clock source update."]
pub struct ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn ena_0(self) -> &'a mut W {
        self.variant(ENA_A::ENA_0)
    }
    #[doc = "Update clock source."]
    #[inline(always)]
    pub fn ena_1(self) -> &'a mut W {
        self.variant(ENA_A::ENA_1)
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
    #[doc = "Bit 0 - Enable main clock source update."]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable main clock source update."]
    #[inline(always)]
    pub fn ena(&mut self) -> ENA_W {
        ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main clock source update enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mainclkuen](index.html) module"]
pub struct MAINCLKUEN_SPEC;
impl crate::RegisterSpec for MAINCLKUEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mainclkuen::R](R) reader structure"]
impl crate::Readable for MAINCLKUEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mainclkuen::W](W) writer structure"]
impl crate::Writable for MAINCLKUEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAINCLKUEN to value 0"]
impl crate::Resettable for MAINCLKUEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
