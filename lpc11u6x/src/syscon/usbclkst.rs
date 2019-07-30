#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::USBCLKST {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `NEED_CLKST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEED_CLKSTR {
    #[doc = "LOW"]
    LOW,
    #[doc = "HIGH"]
    HIGH,
}
impl crate::ToBits<bool> for NEED_CLKSTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            NEED_CLKSTR::LOW => false,
            NEED_CLKSTR::HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type NEED_CLKST_R = crate::FR<bool, NEED_CLKSTR>;
impl NEED_CLKST_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == NEED_CLKSTR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == NEED_CLKSTR::HIGH
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - USB need_clock signal status"]
    #[inline(always)]
    pub fn need_clkst(&self) -> NEED_CLKST_R {
        NEED_CLKST_R::new((self.bits() & 0x01) != 0)
    }
}
