#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::INTSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `ACTIVEINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTIVEINTR {
    #[doc = "Not pending. No enabled interrupts are pending."]
    NOT_PENDING,
    #[doc = "Pending. At least one enabled interrupt is pending."]
    PENDING,
}
impl ACTIVEINTR {
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
            ACTIVEINTR::NOT_PENDING => false,
            ACTIVEINTR::PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACTIVEINTR {
        match value {
            false => ACTIVEINTR::NOT_PENDING,
            true => ACTIVEINTR::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline]
    pub fn is_not_pending(&self) -> bool {
        *self == ACTIVEINTR::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == ACTIVEINTR::PENDING
    }
}
#[doc = "Possible values of the field `ACTIVEERRINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTIVEERRINTR {
    #[doc = "Not pending. No error interrupts are pending."]
    NOT_PENDING,
    #[doc = "Pending. At least one error interrupt is pending."]
    PENDING,
}
impl ACTIVEERRINTR {
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
            ACTIVEERRINTR::NOT_PENDING => false,
            ACTIVEERRINTR::PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACTIVEERRINTR {
        match value {
            false => ACTIVEERRINTR::NOT_PENDING,
            true => ACTIVEERRINTR::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline]
    pub fn is_not_pending(&self) -> bool {
        *self == ACTIVEERRINTR::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == ACTIVEERRINTR::PENDING
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Summarizes whether any enabled interrupts (other than error interrupts) are pending."]
    #[inline]
    pub fn activeint(&self) -> ACTIVEINTR {
        ACTIVEINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Summarizes whether any error interrupts are pending."]
    #[inline]
    pub fn activeerrint(&self) -> ACTIVEERRINTR {
        ACTIVEERRINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
