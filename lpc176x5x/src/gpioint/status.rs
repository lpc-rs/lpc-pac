#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `P0INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0INTR {
    #[doc = "No pending interrupts on Port 0."]
    NO_PENDING_INTERRUPT,
    #[doc = "At least one pending interrupt on Port 0."]
    AT_LEAST_ONE_PENDING,
}
impl P0INTR {
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
            P0INTR::NO_PENDING_INTERRUPT => false,
            P0INTR::AT_LEAST_ONE_PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P0INTR {
        match value {
            false => P0INTR::NO_PENDING_INTERRUPT,
            true => P0INTR::AT_LEAST_ONE_PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING_INTERRUPT`"]
    #[inline]
    pub fn is_no_pending_interrupt(&self) -> bool {
        *self == P0INTR::NO_PENDING_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `AT_LEAST_ONE_PENDING`"]
    #[inline]
    pub fn is_at_least_one_pending(&self) -> bool {
        *self == P0INTR::AT_LEAST_ONE_PENDING
    }
}
#[doc = "Possible values of the field `P2INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2INTR {
    #[doc = "No pending interrupts on Port 2."]
    NO_PENDING_INTERRUPT,
    #[doc = "At least one pending interrupt on Port 2."]
    AT_LEAST_ONE_PENDING,
}
impl P2INTR {
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
            P2INTR::NO_PENDING_INTERRUPT => false,
            P2INTR::AT_LEAST_ONE_PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P2INTR {
        match value {
            false => P2INTR::NO_PENDING_INTERRUPT,
            true => P2INTR::AT_LEAST_ONE_PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING_INTERRUPT`"]
    #[inline]
    pub fn is_no_pending_interrupt(&self) -> bool {
        *self == P2INTR::NO_PENDING_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `AT_LEAST_ONE_PENDING`"]
    #[inline]
    pub fn is_at_least_one_pending(&self) -> bool {
        *self == P2INTR::AT_LEAST_ONE_PENDING
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Port 0 GPIO interrupt pending."]
    #[inline]
    pub fn p0int(&self) -> P0INTR {
        P0INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Port 2 GPIO interrupt pending."]
    #[inline]
    pub fn p2int(&self) -> P2INTR {
        P2INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
