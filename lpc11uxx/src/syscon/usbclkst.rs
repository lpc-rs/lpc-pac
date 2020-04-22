#[doc = "Reader of register USBCLKST"]
pub type R = crate::R<u32, super::USBCLKST>;
#[doc = "USB need_clock signal status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEED_CLKST_A {
    #[doc = "0: LOW"]
    LOW = 0,
    #[doc = "1: HIGH"]
    HIGH = 1,
}
impl From<NEED_CLKST_A> for bool {
    #[inline(always)]
    fn from(variant: NEED_CLKST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NEED_CLKST`"]
pub type NEED_CLKST_R = crate::R<bool, NEED_CLKST_A>;
impl NEED_CLKST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NEED_CLKST_A {
        match self.bits {
            false => NEED_CLKST_A::LOW,
            true => NEED_CLKST_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == NEED_CLKST_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == NEED_CLKST_A::HIGH
    }
}
impl R {
    #[doc = "Bit 0 - USB need_clock signal status"]
    #[inline(always)]
    pub fn need_clkst(&self) -> NEED_CLKST_R {
        NEED_CLKST_R::new((self.bits & 0x01) != 0)
    }
}
