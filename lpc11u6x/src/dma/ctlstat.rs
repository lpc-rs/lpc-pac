#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CTLSTAT {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `VALIDPENDING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALIDPENDINGR {
    #[doc = "No effect on DMA operation."]
    NO_EFFECT_ON_DMA_OPE,
    #[doc = "Valid pending."]
    VALID_PENDING,
}
impl crate::ToBits<bool> for VALIDPENDINGR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            VALIDPENDINGR::NO_EFFECT_ON_DMA_OPE => false,
            VALIDPENDINGR::VALID_PENDING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type VALIDPENDING_R = crate::FR<bool, VALIDPENDINGR>;
impl VALIDPENDING_R {
    #[doc = "Checks if the value of the field is `NO_EFFECT_ON_DMA_OPE`"]
    #[inline(always)]
    pub fn is_no_effect_on_dma_ope(&self) -> bool {
        *self == VALIDPENDINGR::NO_EFFECT_ON_DMA_OPE
    }
    #[doc = "Checks if the value of the field is `VALID_PENDING`"]
    #[inline(always)]
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
impl crate::ToBits<bool> for TRIGR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TRIGR::NOT_TRIGGERED => false,
            TRIGR::TRIGGERED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TRIG_R = crate::FR<bool, TRIGR>;
impl TRIG_R {
    #[doc = "Checks if the value of the field is `NOT_TRIGGERED`"]
    #[inline(always)]
    pub fn is_not_triggered(&self) -> bool {
        *self == TRIGR::NOT_TRIGGERED
    }
    #[doc = "Checks if the value of the field is `TRIGGERED`"]
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        *self == TRIGR::TRIGGERED
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Valid pending flag for this channel. This bit is set when a 1 is written to the corresponding bit in the related SETVALID register when CFGVALID = 1 for the same channel."]
    #[inline(always)]
    pub fn validpending(&self) -> VALIDPENDING_R {
        VALIDPENDING_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 2 - Trigger flag. Indicates that the trigger for this channel is currently set. This bit is cleared at the end of an entire transfer or upon reload when CLRTRIG = 1."]
    #[inline(always)]
    pub fn trig(&self) -> TRIG_R {
        TRIG_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
}
