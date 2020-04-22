#[doc = "Reader of register USBPLLSTAT"]
pub type R = crate::R<u32, super::USBPLLSTAT>;
#[doc = "PLL lock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    #[doc = "0: PLL not locked"]
    PLL_NOT_LOCKED = 0,
    #[doc = "1: PLL locked"]
    PLL_LOCKED = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, LOCK_A>;
impl LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::PLL_NOT_LOCKED,
            true => LOCK_A::PLL_LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `PLL_NOT_LOCKED`"]
    #[inline(always)]
    pub fn is_pll_not_locked(&self) -> bool {
        *self == LOCK_A::PLL_NOT_LOCKED
    }
    #[doc = "Checks if the value of the field is `PLL_LOCKED`"]
    #[inline(always)]
    pub fn is_pll_locked(&self) -> bool {
        *self == LOCK_A::PLL_LOCKED
    }
}
impl R {
    #[doc = "Bit 0 - PLL lock status"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 0x01) != 0)
    }
}
