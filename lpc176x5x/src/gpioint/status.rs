#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Port 0 GPIO interrupt pending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0INT_A {
    #[doc = "0: No pending interrupts on Port 0."]
    NO_PENDING_INTERRUPT = 0,
    #[doc = "1: At least one pending interrupt on Port 0."]
    AT_LEAST_ONE_PENDING = 1,
}
impl From<P0INT_A> for bool {
    #[inline(always)]
    fn from(variant: P0INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0INT`"]
pub type P0INT_R = crate::R<bool, P0INT_A>;
impl P0INT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0INT_A {
        match self.bits {
            false => P0INT_A::NO_PENDING_INTERRUPT,
            true => P0INT_A::AT_LEAST_ONE_PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_pending_interrupt(&self) -> bool {
        *self == P0INT_A::NO_PENDING_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `AT_LEAST_ONE_PENDING`"]
    #[inline(always)]
    pub fn is_at_least_one_pending(&self) -> bool {
        *self == P0INT_A::AT_LEAST_ONE_PENDING
    }
}
#[doc = "Port 2 GPIO interrupt pending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2INT_A {
    #[doc = "0: No pending interrupts on Port 2."]
    NO_PENDING_INTERRUPT = 0,
    #[doc = "1: At least one pending interrupt on Port 2."]
    AT_LEAST_ONE_PENDING = 1,
}
impl From<P2INT_A> for bool {
    #[inline(always)]
    fn from(variant: P2INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P2INT`"]
pub type P2INT_R = crate::R<bool, P2INT_A>;
impl P2INT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2INT_A {
        match self.bits {
            false => P2INT_A::NO_PENDING_INTERRUPT,
            true => P2INT_A::AT_LEAST_ONE_PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_pending_interrupt(&self) -> bool {
        *self == P2INT_A::NO_PENDING_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `AT_LEAST_ONE_PENDING`"]
    #[inline(always)]
    pub fn is_at_least_one_pending(&self) -> bool {
        *self == P2INT_A::AT_LEAST_ONE_PENDING
    }
}
impl R {
    #[doc = "Bit 0 - Port 0 GPIO interrupt pending."]
    #[inline(always)]
    pub fn p0int(&self) -> P0INT_R {
        P0INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port 2 GPIO interrupt pending."]
    #[inline(always)]
    pub fn p2int(&self) -> P2INT_R {
        P2INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
