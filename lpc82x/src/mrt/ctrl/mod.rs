#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
#[doc = "Possible values of the field `INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTENR {
    #[doc = "Disable."]
    DISABLE_,
    #[doc = "Enable."]
    ENABLE_,
}
impl INTENR {
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
            INTENR::DISABLE_ => false,
            INTENR::ENABLE_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTENR {
        match value {
            false => INTENR::DISABLE_,
            true => INTENR::ENABLE_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_`"]
    #[inline]
    pub fn is_disable_(&self) -> bool {
        *self == INTENR::DISABLE_
    }
    #[doc = "Checks if the value of the field is `ENABLE_`"]
    #[inline]
    pub fn is_enable_(&self) -> bool {
        *self == INTENR::ENABLE_
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Repeat interrupt mode."]
    REPEAT_INTERRUPT_MOD,
    #[doc = "One-shot interrupt mode."]
    ONE_SHOT_INTERRUPT_M,
    #[doc = "One-shot bus stall mode."]
    ONE_SHOT_BUS_STALL_M,
    #[doc = "Reserved."]
    RESERVED_,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::REPEAT_INTERRUPT_MOD => 0,
            MODER::ONE_SHOT_INTERRUPT_M => 1,
            MODER::ONE_SHOT_BUS_STALL_M => 2,
            MODER::RESERVED_ => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::REPEAT_INTERRUPT_MOD,
            1 => MODER::ONE_SHOT_INTERRUPT_M,
            2 => MODER::ONE_SHOT_BUS_STALL_M,
            3 => MODER::RESERVED_,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REPEAT_INTERRUPT_MOD`"]
    #[inline]
    pub fn is_repeat_interrupt_mod(&self) -> bool {
        *self == MODER::REPEAT_INTERRUPT_MOD
    }
    #[doc = "Checks if the value of the field is `ONE_SHOT_INTERRUPT_M`"]
    #[inline]
    pub fn is_one_shot_interrupt_m(&self) -> bool {
        *self == MODER::ONE_SHOT_INTERRUPT_M
    }
    #[doc = "Checks if the value of the field is `ONE_SHOT_BUS_STALL_M`"]
    #[inline]
    pub fn is_one_shot_bus_stall_m(&self) -> bool {
        *self == MODER::ONE_SHOT_BUS_STALL_M
    }
    #[doc = "Checks if the value of the field is `RESERVED_`"]
    #[inline]
    pub fn is_reserved_(&self) -> bool {
        *self == MODER::RESERVED_
    }
}
#[doc = "Values that can be written to the field `INTEN`"]
pub enum INTENW {
    #[doc = "Disable."]
    DISABLE_,
    #[doc = "Enable."]
    ENABLE_,
}
impl INTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INTENW::DISABLE_ => false,
            INTENW::ENABLE_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTENW<'a> {
    w: &'a mut W,
}
impl<'a> _INTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable."]
    #[inline]
    pub fn disable_(self) -> &'a mut W {
        self.variant(INTENW::DISABLE_)
    }
    #[doc = "Enable."]
    #[inline]
    pub fn enable_(self) -> &'a mut W {
        self.variant(INTENW::ENABLE_)
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
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Repeat interrupt mode."]
    REPEAT_INTERRUPT_MOD,
    #[doc = "One-shot interrupt mode."]
    ONE_SHOT_INTERRUPT_M,
    #[doc = "One-shot bus stall mode."]
    ONE_SHOT_BUS_STALL_M,
    #[doc = "Reserved."]
    RESERVED_,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::REPEAT_INTERRUPT_MOD => 0,
            MODEW::ONE_SHOT_INTERRUPT_M => 1,
            MODEW::ONE_SHOT_BUS_STALL_M => 2,
            MODEW::RESERVED_ => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Repeat interrupt mode."]
    #[inline]
    pub fn repeat_interrupt_mod(self) -> &'a mut W {
        self.variant(MODEW::REPEAT_INTERRUPT_MOD)
    }
    #[doc = "One-shot interrupt mode."]
    #[inline]
    pub fn one_shot_interrupt_m(self) -> &'a mut W {
        self.variant(MODEW::ONE_SHOT_INTERRUPT_M)
    }
    #[doc = "One-shot bus stall mode."]
    #[inline]
    pub fn one_shot_bus_stall_m(self) -> &'a mut W {
        self.variant(MODEW::ONE_SHOT_BUS_STALL_M)
    }
    #[doc = "Reserved."]
    #[inline]
    pub fn reserved_(self) -> &'a mut W {
        self.variant(MODEW::RESERVED_)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Enable the TIMERn interrupt."]
    #[inline]
    pub fn inten(&self) -> INTENR {
        INTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:2 - Selects timer mode."]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bit 0 - Enable the TIMERn interrupt."]
    #[inline]
    pub fn inten(&mut self) -> _INTENW {
        _INTENW { w: self }
    }
    #[doc = "Bits 1:2 - Selects timer mode."]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
}
