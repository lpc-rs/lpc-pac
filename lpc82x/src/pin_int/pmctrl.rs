#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PMCTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
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
impl SEL_PMATCHR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SEL_PMATCHR::PIN_INTERRUPT_INTER => false,
            SEL_PMATCHR::PATTERN_MATCH_INTER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEL_PMATCHR {
        match value {
            false => SEL_PMATCHR::PIN_INTERRUPT_INTER,
            true => SEL_PMATCHR::PATTERN_MATCH_INTER,
        }
    }
    #[doc = "Checks if the value of the field is `PIN_INTERRUPT_INTER`"]
    #[inline]
    pub fn is_pin_interrupt_inter(&self) -> bool {
        *self == SEL_PMATCHR::PIN_INTERRUPT_INTER
    }
    #[doc = "Checks if the value of the field is `PATTERN_MATCH_INTER`"]
    #[inline]
    pub fn is_pattern_match_inter(&self) -> bool {
        *self == SEL_PMATCHR::PATTERN_MATCH_INTER
    }
}
#[doc = "Possible values of the field `ENA_RXEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_RXEVR {
    #[doc = "Disabled. RxEv output to the cpu is disabled."]
    DISABLED_RXEV_OUTPU,
    #[doc = "Enabled. RxEv output to the cpu is enabled."]
    ENABLED_RXEV_OUTPUT,
}
impl ENA_RXEVR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ENA_RXEVR::DISABLED_RXEV_OUTPU => false,
            ENA_RXEVR::ENABLED_RXEV_OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENA_RXEVR {
        match value {
            false => ENA_RXEVR::DISABLED_RXEV_OUTPU,
            true => ENA_RXEVR::ENABLED_RXEV_OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_RXEV_OUTPU`"]
    #[inline]
    pub fn is_disabled_rxev_outpu(&self) -> bool {
        *self == ENA_RXEVR::DISABLED_RXEV_OUTPU
    }
    #[doc = "Checks if the value of the field is `ENABLED_RXEV_OUTPUT`"]
    #[inline]
    pub fn is_enabled_rxev_output(&self) -> bool {
        *self == ENA_RXEVR::ENABLED_RXEV_OUTPUT
    }
}
#[doc = r" Value of the field"]
pub struct PMATR {
    bits: u8,
}
impl PMATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `SEL_PMATCH`"]
pub enum SEL_PMATCHW {
    #[doc = "Pin interrupt. Interrupts are driven in response to the standard pin interrupt function"]
    PIN_INTERRUPT_INTER,
    #[doc = "Pattern match. Interrupts are driven in response to pattern matches."]
    PATTERN_MATCH_INTER,
}
impl SEL_PMATCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEL_PMATCHW::PIN_INTERRUPT_INTER => false,
            SEL_PMATCHW::PATTERN_MATCH_INTER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEL_PMATCHW<'a> {
    w: &'a mut W,
}
impl<'a> _SEL_PMATCHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEL_PMATCHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin interrupt. Interrupts are driven in response to the standard pin interrupt function"]
    #[inline]
    pub fn pin_interrupt_inter(self) -> &'a mut W {
        self.variant(SEL_PMATCHW::PIN_INTERRUPT_INTER)
    }
    #[doc = "Pattern match. Interrupts are driven in response to pattern matches."]
    #[inline]
    pub fn pattern_match_inter(self) -> &'a mut W {
        self.variant(SEL_PMATCHW::PATTERN_MATCH_INTER)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENA_RXEV`"]
pub enum ENA_RXEVW {
    #[doc = "Disabled. RxEv output to the cpu is disabled."]
    DISABLED_RXEV_OUTPU,
    #[doc = "Enabled. RxEv output to the cpu is enabled."]
    ENABLED_RXEV_OUTPUT,
}
impl ENA_RXEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENA_RXEVW::DISABLED_RXEV_OUTPU => false,
            ENA_RXEVW::ENABLED_RXEV_OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENA_RXEVW<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_RXEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENA_RXEVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. RxEv output to the cpu is disabled."]
    #[inline]
    pub fn disabled_rxev_outpu(self) -> &'a mut W {
        self.variant(ENA_RXEVW::DISABLED_RXEV_OUTPU)
    }
    #[doc = "Enabled. RxEv output to the cpu is enabled."]
    #[inline]
    pub fn enabled_rxev_output(self) -> &'a mut W {
        self.variant(ENA_RXEVW::ENABLED_RXEV_OUTPUT)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PMATW<'a> {
    w: &'a mut W,
}
impl<'a> _PMATW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Specifies whether the 8 pin interrupts are controlled by the pin interrupt function or by the pattern match function."]
    #[inline]
    pub fn sel_pmatch(&self) -> SEL_PMATCHR {
        SEL_PMATCHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enables the RxEv output to the ARM cpu and/or to a GPIO output when the specified boolean expression evaluates to true."]
    #[inline]
    pub fn ena_rxev(&self) -> ENA_RXEVR {
        ENA_RXEVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:31 - This field displays the current state of pattern matches. A 1 in any bit of this field indicates that the corresponding product term is matched by the current state of the appropriate inputs."]
    #[inline]
    pub fn pmat(&self) -> PMATR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PMATR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Specifies whether the 8 pin interrupts are controlled by the pin interrupt function or by the pattern match function."]
    #[inline]
    pub fn sel_pmatch(&mut self) -> _SEL_PMATCHW {
        _SEL_PMATCHW { w: self }
    }
    #[doc = "Bit 1 - Enables the RxEv output to the ARM cpu and/or to a GPIO output when the specified boolean expression evaluates to true."]
    #[inline]
    pub fn ena_rxev(&mut self) -> _ENA_RXEVW {
        _ENA_RXEVW { w: self }
    }
    #[doc = "Bits 24:31 - This field displays the current state of pattern matches. A 1 in any bit of this field indicates that the corresponding product term is matched by the current state of the appropriate inputs."]
    #[inline]
    pub fn pmat(&mut self) -> _PMATW {
        _PMATW { w: self }
    }
}
