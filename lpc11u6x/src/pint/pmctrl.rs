#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PMCTRL {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `SEL_PMATCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_PMATCHR {
    #[doc = "Pin interrupt. Interrupts are driven in response to the standard pin interrupt function"]
    PIN_INTERRUPT_INTER,
    #[doc = "Pattern match. Interrupts are driven in response to pattern matches."]
    PATTERN_MATCH_INTER,
}
impl crate::ToBits<bool> for SEL_PMATCHR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SEL_PMATCHR::PIN_INTERRUPT_INTER => false,
            SEL_PMATCHR::PATTERN_MATCH_INTER => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SEL_PMATCH_R = crate::FR<bool, SEL_PMATCHR>;
impl SEL_PMATCH_R {
    #[doc = "Checks if the value of the field is `PIN_INTERRUPT_INTER`"]
    #[inline(always)]
    pub fn is_pin_interrupt_inter(&self) -> bool {
        *self == SEL_PMATCHR::PIN_INTERRUPT_INTER
    }
    #[doc = "Checks if the value of the field is `PATTERN_MATCH_INTER`"]
    #[inline(always)]
    pub fn is_pattern_match_inter(&self) -> bool {
        *self == SEL_PMATCHR::PATTERN_MATCH_INTER
    }
}
#[doc = "Values that can be written to the field `SEL_PMATCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_PMATCHW {
    #[doc = "Pin interrupt. Interrupts are driven in response to the standard pin interrupt function"]
    PIN_INTERRUPT_INTER,
    #[doc = "Pattern match. Interrupts are driven in response to pattern matches."]
    PATTERN_MATCH_INTER,
}
impl SEL_PMATCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SEL_PMATCHW::PIN_INTERRUPT_INTER => false,
            SEL_PMATCHW::PATTERN_MATCH_INTER => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SEL_PMATCHW<'a> {
    w: &'a mut W,
}
impl<'a> _SEL_PMATCHW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_PMATCHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt. Interrupts are driven in response to the standard pin interrupt function"]
    #[inline(always)]
    pub fn pin_interrupt_inter(self) -> &'a mut W {
        self.variant(SEL_PMATCHW::PIN_INTERRUPT_INTER)
    }
    #[doc = "Pattern match. Interrupts are driven in response to pattern matches."]
    #[inline(always)]
    pub fn pattern_match_inter(self) -> &'a mut W {
        self.variant(SEL_PMATCHW::PATTERN_MATCH_INTER)
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
#[doc = "Possible values of the field `ENA_RXEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_RXEVR {
    #[doc = "Disabled. RXEV output to the cpu is disabled."]
    DISABLED_RXEV_OUTPU,
    #[doc = "Enabled. RXEV output to the cpu is enabled."]
    ENABLED_RXEV_OUTPUT,
}
impl crate::ToBits<bool> for ENA_RXEVR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ENA_RXEVR::DISABLED_RXEV_OUTPU => false,
            ENA_RXEVR::ENABLED_RXEV_OUTPUT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ENA_RXEV_R = crate::FR<bool, ENA_RXEVR>;
impl ENA_RXEV_R {
    #[doc = "Checks if the value of the field is `DISABLED_RXEV_OUTPU`"]
    #[inline(always)]
    pub fn is_disabled_rxev_outpu(&self) -> bool {
        *self == ENA_RXEVR::DISABLED_RXEV_OUTPU
    }
    #[doc = "Checks if the value of the field is `ENABLED_RXEV_OUTPUT`"]
    #[inline(always)]
    pub fn is_enabled_rxev_output(&self) -> bool {
        *self == ENA_RXEVR::ENABLED_RXEV_OUTPUT
    }
}
#[doc = "Values that can be written to the field `ENA_RXEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_RXEVW {
    #[doc = "Disabled. RXEV output to the cpu is disabled."]
    DISABLED_RXEV_OUTPU,
    #[doc = "Enabled. RXEV output to the cpu is enabled."]
    ENABLED_RXEV_OUTPUT,
}
impl ENA_RXEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ENA_RXEVW::DISABLED_RXEV_OUTPU => false,
            ENA_RXEVW::ENABLED_RXEV_OUTPUT => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ENA_RXEVW<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_RXEVW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA_RXEVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. RXEV output to the cpu is disabled."]
    #[inline(always)]
    pub fn disabled_rxev_outpu(self) -> &'a mut W {
        self.variant(ENA_RXEVW::DISABLED_RXEV_OUTPU)
    }
    #[doc = "Enabled. RXEV output to the cpu is enabled."]
    #[inline(always)]
    pub fn enabled_rxev_output(self) -> &'a mut W {
        self.variant(ENA_RXEVW::ENABLED_RXEV_OUTPUT)
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
#[doc = r"Reader of the field"]
pub type PMAT_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PMATW<'a> {
    w: &'a mut W,
}
impl<'a> _PMATW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Specifies whether the 8 pin interrupts are controlled by the pin interrupt function or by the pattern match function."]
    #[inline(always)]
    pub fn sel_pmatch(&self) -> SEL_PMATCH_R {
        SEL_PMATCH_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables the RXEV output to the ARM cpu and/or to a GPIO output when the specified boolean expression evaluates to true."]
    #[inline(always)]
    pub fn ena_rxev(&self) -> ENA_RXEV_R {
        ENA_RXEV_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - This field displays the current state of pattern matches. A 1 in any bit of this field indicates that the corresponding product term is matched by the current state of the appropriate inputs."]
    #[inline(always)]
    pub fn pmat(&self) -> PMAT_R {
        PMAT_R::new(((self.bits() >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Specifies whether the 8 pin interrupts are controlled by the pin interrupt function or by the pattern match function."]
    #[inline(always)]
    pub fn sel_pmatch(&mut self) -> _SEL_PMATCHW {
        _SEL_PMATCHW { w: self }
    }
    #[doc = "Bit 1 - Enables the RXEV output to the ARM cpu and/or to a GPIO output when the specified boolean expression evaluates to true."]
    #[inline(always)]
    pub fn ena_rxev(&mut self) -> _ENA_RXEVW {
        _ENA_RXEVW { w: self }
    }
    #[doc = "Bits 24:31 - This field displays the current state of pattern matches. A 1 in any bit of this field indicates that the corresponding product term is matched by the current state of the appropriate inputs."]
    #[inline(always)]
    pub fn pmat(&mut self) -> _PMATW {
        _PMATW { w: self }
    }
}
