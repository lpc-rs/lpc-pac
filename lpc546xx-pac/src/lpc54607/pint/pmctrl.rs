#[doc = "Register `PMCTRL` reader"]
pub struct R(crate::R<PMCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMCTRL` writer"]
pub struct W(crate::W<PMCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMCTRL_SPEC>;
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
impl From<crate::W<PMCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Specifies whether the 8 pin interrupts are controlled by the pin interrupt function or by the pattern match function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_PMATCH_A {
    #[doc = "0: Pin interrupt. Interrupts are driven in response to the standard pin interrupt function."]
    PIN_INTERRUPT = 0,
    #[doc = "1: Pattern match. Interrupts are driven in response to pattern matches."]
    PATTERN_MATCH = 1,
}
impl From<SEL_PMATCH_A> for bool {
    #[inline(always)]
    fn from(variant: SEL_PMATCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEL_PMATCH` reader - Specifies whether the 8 pin interrupts are controlled by the pin interrupt function or by the pattern match function."]
pub struct SEL_PMATCH_R(crate::FieldReader<bool, SEL_PMATCH_A>);
impl SEL_PMATCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEL_PMATCH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_PMATCH_A {
        match self.bits {
            false => SEL_PMATCH_A::PIN_INTERRUPT,
            true => SEL_PMATCH_A::PATTERN_MATCH,
        }
    }
    #[doc = "Checks if the value of the field is `PIN_INTERRUPT`"]
    #[inline(always)]
    pub fn is_pin_interrupt(&self) -> bool {
        **self == SEL_PMATCH_A::PIN_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PATTERN_MATCH`"]
    #[inline(always)]
    pub fn is_pattern_match(&self) -> bool {
        **self == SEL_PMATCH_A::PATTERN_MATCH
    }
}
impl core::ops::Deref for SEL_PMATCH_R {
    type Target = crate::FieldReader<bool, SEL_PMATCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL_PMATCH` writer - Specifies whether the 8 pin interrupts are controlled by the pin interrupt function or by the pattern match function."]
pub struct SEL_PMATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_PMATCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_PMATCH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin interrupt. Interrupts are driven in response to the standard pin interrupt function."]
    #[inline(always)]
    pub fn pin_interrupt(self) -> &'a mut W {
        self.variant(SEL_PMATCH_A::PIN_INTERRUPT)
    }
    #[doc = "Pattern match. Interrupts are driven in response to pattern matches."]
    #[inline(always)]
    pub fn pattern_match(self) -> &'a mut W {
        self.variant(SEL_PMATCH_A::PATTERN_MATCH)
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
#[doc = "Enables the RXEV output to the CPU and/or to a GPIO output when the specified boolean expression evaluates to true.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_RXEV_A {
    #[doc = "0: Disabled. RXEV output to the CPU is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. RXEV output to the CPU is enabled."]
    ENABLED = 1,
}
impl From<ENA_RXEV_A> for bool {
    #[inline(always)]
    fn from(variant: ENA_RXEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA_RXEV` reader - Enables the RXEV output to the CPU and/or to a GPIO output when the specified boolean expression evaluates to true."]
pub struct ENA_RXEV_R(crate::FieldReader<bool, ENA_RXEV_A>);
impl ENA_RXEV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_RXEV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA_RXEV_A {
        match self.bits {
            false => ENA_RXEV_A::DISABLED,
            true => ENA_RXEV_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENA_RXEV_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENA_RXEV_A::ENABLED
    }
}
impl core::ops::Deref for ENA_RXEV_R {
    type Target = crate::FieldReader<bool, ENA_RXEV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_RXEV` writer - Enables the RXEV output to the CPU and/or to a GPIO output when the specified boolean expression evaluates to true."]
pub struct ENA_RXEV_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_RXEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA_RXEV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled. RXEV output to the CPU is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA_RXEV_A::DISABLED)
    }
    #[doc = "Enabled. RXEV output to the CPU is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA_RXEV_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `PMAT` reader - This field displays the current state of pattern matches. A 1 in any bit of this field indicates that the corresponding product term is matched by the current state of the appropriate inputs."]
pub struct PMAT_R(crate::FieldReader<u8, u8>);
impl PMAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PMAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMAT` writer - This field displays the current state of pattern matches. A 1 in any bit of this field indicates that the corresponding product term is matched by the current state of the appropriate inputs."]
pub struct PMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Specifies whether the 8 pin interrupts are controlled by the pin interrupt function or by the pattern match function."]
    #[inline(always)]
    pub fn sel_pmatch(&self) -> SEL_PMATCH_R {
        SEL_PMATCH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables the RXEV output to the CPU and/or to a GPIO output when the specified boolean expression evaluates to true."]
    #[inline(always)]
    pub fn ena_rxev(&self) -> ENA_RXEV_R {
        ENA_RXEV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - This field displays the current state of pattern matches. A 1 in any bit of this field indicates that the corresponding product term is matched by the current state of the appropriate inputs."]
    #[inline(always)]
    pub fn pmat(&self) -> PMAT_R {
        PMAT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies whether the 8 pin interrupts are controlled by the pin interrupt function or by the pattern match function."]
    #[inline(always)]
    pub fn sel_pmatch(&mut self) -> SEL_PMATCH_W {
        SEL_PMATCH_W { w: self }
    }
    #[doc = "Bit 1 - Enables the RXEV output to the CPU and/or to a GPIO output when the specified boolean expression evaluates to true."]
    #[inline(always)]
    pub fn ena_rxev(&mut self) -> ENA_RXEV_W {
        ENA_RXEV_W { w: self }
    }
    #[doc = "Bits 24:31 - This field displays the current state of pattern matches. A 1 in any bit of this field indicates that the corresponding product term is matched by the current state of the appropriate inputs."]
    #[inline(always)]
    pub fn pmat(&mut self) -> PMAT_W {
        PMAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pattern match interrupt control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmctrl](index.html) module"]
pub struct PMCTRL_SPEC;
impl crate::RegisterSpec for PMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmctrl::R](R) reader structure"]
impl crate::Readable for PMCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmctrl::W](W) writer structure"]
impl crate::Writable for PMCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMCTRL to value 0"]
impl crate::Resettable for PMCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
