#[doc = "Reader of register PMCTRL"]
pub type R = crate::R<u32, super::PMCTRL>;
#[doc = "Writer for register PMCTRL"]
pub type W = crate::W<u32, super::PMCTRL>;
#[doc = "Register PMCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::PMCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `SEL_PMATCH`"]
pub type SEL_PMATCH_R = crate::R<bool, SEL_PMATCH_A>;
impl SEL_PMATCH_R {
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
        *self == SEL_PMATCH_A::PIN_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PATTERN_MATCH`"]
    #[inline(always)]
    pub fn is_pattern_match(&self) -> bool {
        *self == SEL_PMATCH_A::PATTERN_MATCH
    }
}
#[doc = "Write proxy for field `SEL_PMATCH`"]
pub struct SEL_PMATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_PMATCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_PMATCH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
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
#[doc = "Reader of field `ENA_RXEV`"]
pub type ENA_RXEV_R = crate::R<bool, ENA_RXEV_A>;
impl ENA_RXEV_R {
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
        *self == ENA_RXEV_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENA_RXEV_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENA_RXEV`"]
pub struct ENA_RXEV_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_RXEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA_RXEV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `PMAT`"]
pub type PMAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PMAT`"]
pub struct PMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
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
}
