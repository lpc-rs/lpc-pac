#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CTLSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `VALIDPENDING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALIDPENDINGR {
    #[doc = "No effect. No effect on DMA operation."]
    NO_EFFECT,
    #[doc = "Valid pending."]
    VALID_PENDING,
}
impl VALIDPENDINGR {
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
            VALIDPENDINGR::NO_EFFECT => false,
            VALIDPENDINGR::VALID_PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VALIDPENDINGR {
        match value {
            false => VALIDPENDINGR::NO_EFFECT,
            true => VALIDPENDINGR::VALID_PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline]
    pub fn is_no_effect(&self) -> bool {
        *self == VALIDPENDINGR::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `VALID_PENDING`"]
    #[inline]
    pub fn is_valid_pending(&self) -> bool {
        *self == VALIDPENDINGR::VALID_PENDING
    }
}
#[doc = "Possible values of the field `TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGR {
    #[doc = "Not triggered. The trigger for this DMA channel is not set. DMA operations will not be carried out."]
    NOT_TRIGGERED,
    #[doc = "Triggered. The trigger for this DMA channel is set. DMA operations will be carried out."]
    TRIGGERED,
}
impl TRIGR {
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
            TRIGR::NOT_TRIGGERED => false,
            TRIGR::TRIGGERED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGR {
        match value {
            false => TRIGR::NOT_TRIGGERED,
            true => TRIGR::TRIGGERED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_TRIGGERED`"]
    #[inline]
    pub fn is_not_triggered(&self) -> bool {
        *self == TRIGR::NOT_TRIGGERED
    }
    #[doc = "Checks if the value of the field is `TRIGGERED`"]
    #[inline]
    pub fn is_triggered(&self) -> bool {
        *self == TRIGR::TRIGGERED
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Valid pending flag for this channel. This bit is set when a 1 is written to the corresponding bit in the related SETVALID register when CFGVALID = 1 for the same channel."]
    #[inline]
    pub fn validpending(&self) -> VALIDPENDINGR {
        VALIDPENDINGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Trigger flag. Indicates that the trigger for this channel is currently set. This bit is cleared at the end of an entire transfer or upon reload when CLRTRIG = 1."]
    #[inline]
    pub fn trig(&self) -> TRIGR {
        TRIGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
