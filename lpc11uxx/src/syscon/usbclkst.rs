#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::USBCLKST {
    #[doc = r" Reads the contents of the register"]
    #[inline]
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
impl NEED_CLKSTR {
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
            NEED_CLKSTR::LOW => false,
            NEED_CLKSTR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NEED_CLKSTR {
        match value {
            false => NEED_CLKSTR::LOW,
            true => NEED_CLKSTR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == NEED_CLKSTR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == NEED_CLKSTR::HIGH
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - USB need_clock signal status"]
    #[inline]
    pub fn need_clkst(&self) -> NEED_CLKSTR {
        NEED_CLKSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
