#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRESETCTRL1 {
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
#[doc = "Possible values of the field `CAPT_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_RST_NR {
    #[doc = "Assert the capacitive touch reset."]
    ASSERT,
    #[doc = "Clear the capacitive touch reset."]
    CLEAR,
}
impl CAPT_RST_NR {
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
            CAPT_RST_NR::ASSERT => false,
            CAPT_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPT_RST_NR {
        match value {
            false => CAPT_RST_NR::ASSERT,
            true => CAPT_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == CAPT_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == CAPT_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `DAC1_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC1_RST_NR {
    #[doc = "Assert the DAC1 reset."]
    ASSERT,
    #[doc = "Clear the DAC1 reset."]
    CLEAR,
}
impl DAC1_RST_NR {
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
            DAC1_RST_NR::ASSERT => false,
            DAC1_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DAC1_RST_NR {
        match value {
            false => DAC1_RST_NR::ASSERT,
            true => DAC1_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == DAC1_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == DAC1_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `FRG0_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRG0_RST_NR {
    #[doc = "Assert the FRG0 reset."]
    ASSERT,
    #[doc = "Clear the FRG0 reset."]
    CLEAR,
}
impl FRG0_RST_NR {
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
            FRG0_RST_NR::ASSERT => false,
            FRG0_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRG0_RST_NR {
        match value {
            false => FRG0_RST_NR::ASSERT,
            true => FRG0_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == FRG0_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == FRG0_RST_NR::CLEAR
    }
}
#[doc = "Possible values of the field `FRG1_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRG1_RST_NR {
    #[doc = "Assert the FRG1 reset."]
    ASSERT,
    #[doc = "Clear the FRG1 reset."]
    CLEAR,
}
impl FRG1_RST_NR {
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
            FRG1_RST_NR::ASSERT => false,
            FRG1_RST_NR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRG1_RST_NR {
        match value {
            false => FRG1_RST_NR::ASSERT,
            true => FRG1_RST_NR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline]
    pub fn is_assert(&self) -> bool {
        *self == FRG1_RST_NR::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == FRG1_RST_NR::CLEAR
    }
}
#[doc = "Values that can be written to the field `CAPT_RST_N`"]
pub enum CAPT_RST_NW {
    #[doc = "Assert the capacitive touch reset."]
    ASSERT,
    #[doc = "Clear the capacitive touch reset."]
    CLEAR,
}
impl CAPT_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPT_RST_NW::ASSERT => false,
            CAPT_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPT_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPT_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPT_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the capacitive touch reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(CAPT_RST_NW::ASSERT)
    }
    #[doc = "Clear the capacitive touch reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CAPT_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `DAC1_RST_N`"]
pub enum DAC1_RST_NW {
    #[doc = "Assert the DAC1 reset."]
    ASSERT,
    #[doc = "Clear the DAC1 reset."]
    CLEAR,
}
impl DAC1_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DAC1_RST_NW::ASSERT => false,
            DAC1_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DAC1_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _DAC1_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DAC1_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the DAC1 reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(DAC1_RST_NW::ASSERT)
    }
    #[doc = "Clear the DAC1 reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(DAC1_RST_NW::CLEAR)
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
#[doc = "Values that can be written to the field `FRG0_RST_N`"]
pub enum FRG0_RST_NW {
    #[doc = "Assert the FRG0 reset."]
    ASSERT,
    #[doc = "Clear the FRG0 reset."]
    CLEAR,
}
impl FRG0_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRG0_RST_NW::ASSERT => false,
            FRG0_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRG0_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _FRG0_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRG0_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the FRG0 reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(FRG0_RST_NW::ASSERT)
    }
    #[doc = "Clear the FRG0 reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(FRG0_RST_NW::CLEAR)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FRG1_RST_N`"]
pub enum FRG1_RST_NW {
    #[doc = "Assert the FRG1 reset."]
    ASSERT,
    #[doc = "Clear the FRG1 reset."]
    CLEAR,
}
impl FRG1_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRG1_RST_NW::ASSERT => false,
            FRG1_RST_NW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRG1_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _FRG1_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRG1_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Assert the FRG1 reset."]
    #[inline]
    pub fn assert(self) -> &'a mut W {
        self.variant(FRG1_RST_NW::ASSERT)
    }
    #[doc = "Clear the FRG1 reset."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(FRG1_RST_NW::CLEAR)
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
        const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - Capacitive touch reset control"]
    #[inline]
    pub fn capt_rst_n(&self) -> CAPT_RST_NR {
        CAPT_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - DAC1 reset control"]
    #[inline]
    pub fn dac1_rst_n(&self) -> DAC1_RST_NR {
        DAC1_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Fractional baud rate generator 0 reset control"]
    #[inline]
    pub fn frg0_rst_n(&self) -> FRG0_RST_NR {
        FRG0_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Fractional baud rate generator 1 reset control"]
    #[inline]
    pub fn frg1_rst_n(&self) -> FRG1_RST_NR {
        FRG1_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 31 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Capacitive touch reset control"]
    #[inline]
    pub fn capt_rst_n(&mut self) -> _CAPT_RST_NW {
        _CAPT_RST_NW { w: self }
    }
    #[doc = "Bit 1 - DAC1 reset control"]
    #[inline]
    pub fn dac1_rst_n(&mut self) -> _DAC1_RST_NW {
        _DAC1_RST_NW { w: self }
    }
    #[doc = "Bit 3 - Fractional baud rate generator 0 reset control"]
    #[inline]
    pub fn frg0_rst_n(&mut self) -> _FRG0_RST_NW {
        _FRG0_RST_NW { w: self }
    }
    #[doc = "Bit 4 - Fractional baud rate generator 1 reset control"]
    #[inline]
    pub fn frg1_rst_n(&mut self) -> _FRG1_RST_NW {
        _FRG1_RST_NW { w: self }
    }
}
