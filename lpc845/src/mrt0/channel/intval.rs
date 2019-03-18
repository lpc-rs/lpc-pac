#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTVAL {
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
#[doc = r" Value of the field"]
pub struct IVALUER {
    bits: u32,
}
impl IVALUER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `LOAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOADR {
    #[doc = "No force load. The load from the INTVALn register to the TIMERn register is processed at the end of the time interval if the repeat mode is selected."]
    NO_FORCE_LOAD,
    #[doc = "Force load. The INTVALn interval value IVALUE -1 is immediately loaded into the TIMERn register while TIMERn is running."]
    FORCE_LOAD,
}
impl LOADR {
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
            LOADR::NO_FORCE_LOAD => false,
            LOADR::FORCE_LOAD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOADR {
        match value {
            false => LOADR::NO_FORCE_LOAD,
            true => LOADR::FORCE_LOAD,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FORCE_LOAD`"]
    #[inline]
    pub fn is_no_force_load(&self) -> bool {
        *self == LOADR::NO_FORCE_LOAD
    }
    #[doc = "Checks if the value of the field is `FORCE_LOAD`"]
    #[inline]
    pub fn is_force_load(&self) -> bool {
        *self == LOADR::FORCE_LOAD
    }
}
#[doc = r" Proxy"]
pub struct _IVALUEW<'a> {
    w: &'a mut W,
}
impl<'a> _IVALUEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 2147483647;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LOAD`"]
pub enum LOADW {
    #[doc = "No force load. The load from the INTVALn register to the TIMERn register is processed at the end of the time interval if the repeat mode is selected."]
    NO_FORCE_LOAD,
    #[doc = "Force load. The INTVALn interval value IVALUE -1 is immediately loaded into the TIMERn register while TIMERn is running."]
    FORCE_LOAD,
}
impl LOADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOADW::NO_FORCE_LOAD => false,
            LOADW::FORCE_LOAD => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOADW<'a> {
    w: &'a mut W,
}
impl<'a> _LOADW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOADW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No force load. The load from the INTVALn register to the TIMERn register is processed at the end of the time interval if the repeat mode is selected."]
    #[inline]
    pub fn no_force_load(self) -> &'a mut W {
        self.variant(LOADW::NO_FORCE_LOAD)
    }
    #[doc = "Force load. The INTVALn interval value IVALUE -1 is immediately loaded into the TIMERn register while TIMERn is running."]
    #[inline]
    pub fn force_load(self) -> &'a mut W {
        self.variant(LOADW::FORCE_LOAD)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:30 - Time interval load value. This value is loaded into the TIMERn register and the MRT channel n starts counting down from IVALUE -1. If the timer is idle, writing a non-zero value to this bit field starts the timer immediately. If the timer is running, writing a zero to this bit field does the following: If LOAD = 1, the timer stops immediately. If LOAD = 0, the timer stops at the end of the time interval."]
    #[inline]
    pub fn ivalue(&self) -> IVALUER {
        let bits = {
            const MASK: u32 = 2147483647;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        IVALUER { bits }
    }
    #[doc = "Bit 31 - Determines how the timer interval value IVALUE -1 is loaded into the TIMERn register. This bit is write-only. Reading this bit always returns 0."]
    #[inline]
    pub fn load(&self) -> LOADR {
        LOADR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 0:30 - Time interval load value. This value is loaded into the TIMERn register and the MRT channel n starts counting down from IVALUE -1. If the timer is idle, writing a non-zero value to this bit field starts the timer immediately. If the timer is running, writing a zero to this bit field does the following: If LOAD = 1, the timer stops immediately. If LOAD = 0, the timer stops at the end of the time interval."]
    #[inline]
    pub fn ivalue(&mut self) -> _IVALUEW {
        _IVALUEW { w: self }
    }
    #[doc = "Bit 31 - Determines how the timer interval value IVALUE -1 is loaded into the TIMERn register. This bit is write-only. Reading this bit always returns 0."]
    #[inline]
    pub fn load(&mut self) -> _LOADW {
        _LOADW { w: self }
    }
}
