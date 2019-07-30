#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::USBPLLSTAT {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
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
impl crate::ToBits<bool> for LOCKR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LOCKR::NO_LOCK => false,
            LOCKR::LOCK => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LOCK_R = crate::FR<bool, LOCKR>;
impl LOCK_R {
    #[doc = "Checks if the value of the field is `NO_LOCK`"]
    #[inline(always)]
    pub fn is_no_lock(&self) -> bool {
        *self == LOCKR::NO_LOCK
    }
    #[doc = "Checks if the value of the field is `LOCK`"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == LOCKR::LOCK
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - PLL lock status"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits() & 0x01) != 0)
    }
}
