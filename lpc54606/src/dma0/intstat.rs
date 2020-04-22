#[doc = "Reader of register INTSTAT"]
pub type R = crate::R<u32, super::INTSTAT>;
#[doc = "Summarizes whether any enabled interrupts (other than error interrupts) are pending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTIVEINT_A {
    #[doc = "0: Not pending. No enabled interrupts are pending."]
    NOT_PENDING = 0,
    #[doc = "1: Pending. At least one enabled interrupt is pending."]
    PENDING = 1,
}
impl From<ACTIVEINT_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVEINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACTIVEINT`"]
pub type ACTIVEINT_R = crate::R<bool, ACTIVEINT_A>;
impl ACTIVEINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVEINT_A {
        match self.bits {
            false => ACTIVEINT_A::NOT_PENDING,
            true => ACTIVEINT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == ACTIVEINT_A::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ACTIVEINT_A::PENDING
    }
}
#[doc = "Summarizes whether any error interrupts are pending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTIVEERRINT_A {
    #[doc = "0: Not pending. No error interrupts are pending."]
    NOT_PENDING = 0,
    #[doc = "1: Pending. At least one error interrupt is pending."]
    PENDING = 1,
}
impl From<ACTIVEERRINT_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVEERRINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACTIVEERRINT`"]
pub type ACTIVEERRINT_R = crate::R<bool, ACTIVEERRINT_A>;
impl ACTIVEERRINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVEERRINT_A {
        match self.bits {
            false => ACTIVEERRINT_A::NOT_PENDING,
            true => ACTIVEERRINT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == ACTIVEERRINT_A::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ACTIVEERRINT_A::PENDING
    }
}
impl R {
    #[doc = "Bit 1 - Summarizes whether any enabled interrupts (other than error interrupts) are pending."]
    #[inline(always)]
    pub fn activeint(&self) -> ACTIVEINT_R {
        ACTIVEINT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Summarizes whether any error interrupts are pending."]
    #[inline(always)]
    pub fn activeerrint(&self) -> ACTIVEERRINT_R {
        ACTIVEERRINT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
