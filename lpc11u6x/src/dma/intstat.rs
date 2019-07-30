#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::INTSTAT {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
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
impl crate::ToBits<bool> for ACTIVEINTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ACTIVEINTR::NOT_PENDING => false,
            ACTIVEINTR::PENDING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ACTIVEINT_R = crate::FR<bool, ACTIVEINTR>;
impl ACTIVEINT_R {
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == ACTIVEINTR::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
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
impl crate::ToBits<bool> for ACTIVEERRINTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ACTIVEERRINTR::NOT_PENDING => false,
            ACTIVEERRINTR::PENDING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ACTIVEERRINT_R = crate::FR<bool, ACTIVEERRINTR>;
impl ACTIVEERRINT_R {
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == ACTIVEERRINTR::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ACTIVEERRINTR::PENDING
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Summarizes whether any enabled interrupts are pending."]
    #[inline(always)]
    pub fn activeint(&self) -> ACTIVEINT_R {
        ACTIVEINT_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Summarizes whether any error interrupts are pending."]
    #[inline(always)]
    pub fn activeerrint(&self) -> ACTIVEERRINT_R {
        ACTIVEERRINT_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
}
