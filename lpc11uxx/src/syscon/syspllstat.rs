#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SYSPLLSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKR {
    #[doc = "PLL not locked"]
    PLL_NOT_LOCKED,
    #[doc = "PLL locked"]
    PLL_LOCKED,
}
impl LOCKR {
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
            LOCKR::PLL_NOT_LOCKED => false,
            LOCKR::PLL_LOCKED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCKR {
        match value {
            false => LOCKR::PLL_NOT_LOCKED,
            true => LOCKR::PLL_LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `PLL_NOT_LOCKED`"]
    #[inline]
    pub fn is_pll_not_locked(&self) -> bool {
        *self == LOCKR::PLL_NOT_LOCKED
    }
    #[doc = "Checks if the value of the field is `PLL_LOCKED`"]
    #[inline]
    pub fn is_pll_locked(&self) -> bool {
        *self == LOCKR::PLL_LOCKED
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - PLL lock status"]
    #[inline]
    pub fn lock(&self) -> LOCKR {
        LOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
