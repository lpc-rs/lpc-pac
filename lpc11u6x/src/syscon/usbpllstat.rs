#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::USBPLLSTAT {
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
    #[doc = "No lock. PLL not locked"]
    NO_LOCK,
    #[doc = "Lock. PLL locked"]
    LOCK,
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
            LOCKR::NO_LOCK => false,
            LOCKR::LOCK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCKR {
        match value {
            false => LOCKR::NO_LOCK,
            true => LOCKR::LOCK,
        }
    }
    #[doc = "Checks if the value of the field is `NO_LOCK`"]
    #[inline]
    pub fn is_no_lock(&self) -> bool {
        *self == LOCKR::NO_LOCK
    }
    #[doc = "Checks if the value of the field is `LOCK`"]
    #[inline]
    pub fn is_lock(&self) -> bool {
        *self == LOCKR::LOCK
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
