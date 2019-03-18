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
    #[doc = "Pin interrupt. Interrupts are driven in response to the standard pin interrupt function."]
    PIN_INTERRUPT,
    #[doc = "Pattern match. Interrupts are driven in response to pattern matches."]
    PATTERN_MATCH,
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
            SEL_PMATCHR::PIN_INTERRUPT => false,
            SEL_PMATCHR::PATTERN_MATCH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEL_PMATCHR {
        match value {
            false => SEL_PMATCHR::PIN_INTERRUPT,
            true => SEL_PMATCHR::PATTERN_MATCH,
        }
    }
    #[doc = "Checks if the value of the field is `PIN_INTERRUPT`"]
    #[inline]
    pub fn is_pin_interrupt(&self) -> bool {
        *self == SEL_PMATCHR::PIN_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PATTERN_MATCH`"]
    #[inline]
    pub fn is_pattern_match(&self) -> bool {
        *self == SEL_PMATCHR::PATTERN_MATCH
    }
}
#[doc = "Possible values of the field `ENA_RXEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_RXEVR {
    #[doc = "Disabled. RXEV output to the CPU is disabled."]
    DISABLED,
    #[doc = "Enabled. RXEV output to the CPU is enabled."]
    ENABLED,
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
            ENA_RXEVR::DISABLED => false,
            ENA_RXEVR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENA_RXEVR {
        match value {
            false => ENA_RXEVR::DISABLED,
            true => ENA_RXEVR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENA_RXEVR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENA_RXEVR::ENABLED
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
    #[doc = "Pin interrupt. Interrupts are driven in response to the standard pin interrupt function."]
    PIN_INTERRUPT,
    #[doc = "Pattern match. Interrupts are driven in response to pattern matches."]
    PATTERN_MATCH,
}
impl SEL_PMATCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEL_PMATCHW::PIN_INTERRUPT => false,
            SEL_PMATCHW::PATTERN_MATCH => true,
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
    #[doc = "Pin interrupt. Interrupts are driven in response to the standard pin interrupt function."]
    #[inline]
    pub fn pin_interrupt(self) -> &'a mut W {
        self.variant(SEL_PMATCHW::PIN_INTERRUPT)
    }
    #[doc = "Pattern match. Interrupts are driven in response to pattern matches."]
    #[inline]
    pub fn pattern_match(self) -> &'a mut W {
        self.variant(SEL_PMATCHW::PATTERN_MATCH)
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
    #[doc = "Disabled. RXEV output to the CPU is disabled."]
    DISABLED,
    #[doc = "Enabled. RXEV output to the CPU is enabled."]
    ENABLED,
}
impl ENA_RXEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENA_RXEVW::DISABLED => false,
            ENA_RXEVW::ENABLED => true,
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
    #[doc = "Disabled. RXEV output to the CPU is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA_RXEVW::DISABLED)
    }
    #[doc = "Enabled. RXEV output to the CPU is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA_RXEVW::ENABLED)
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
    #[doc = "Bit 1 - Enables the RXEV output to the CPU and/or to a GPIO output when the specified boolean expression evaluates to true."]
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
    #[doc = "Bit 1 - Enables the RXEV output to the CPU and/or to a GPIO output when the specified boolean expression evaluates to true."]
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
