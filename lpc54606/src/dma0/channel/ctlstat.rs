#[doc = "Reader of register CTLSTAT"]
pub type R = crate::R<u32, super::CTLSTAT>;
#[doc = "Valid pending flag for this channel. This bit is set when a 1 is written to the corresponding bit in the related SETVALID register when CFGVALID = 1 for the same channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALIDPENDING_A {
    #[doc = "0: No effect. No effect on DMA operation."]
    NO_EFFECT = 0,
    #[doc = "1: Valid pending."]
    VALID_PENDING = 1,
}
impl From<VALIDPENDING_A> for bool {
    #[inline(always)]
    fn from(variant: VALIDPENDING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VALIDPENDING`"]
pub type VALIDPENDING_R = crate::R<bool, VALIDPENDING_A>;
impl VALIDPENDING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VALIDPENDING_A {
        match self.bits {
            false => VALIDPENDING_A::NO_EFFECT,
            true => VALIDPENDING_A::VALID_PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == VALIDPENDING_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `VALID_PENDING`"]
    #[inline(always)]
    pub fn is_valid_pending(&self) -> bool {
        *self == VALIDPENDING_A::VALID_PENDING
    }
}
#[doc = "Trigger flag. Indicates that the trigger for this channel is currently set. This bit is cleared at the end of an entire transfer or upon reload when CLRTRIG = 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIG_A {
    #[doc = "0: Not triggered. The trigger for this DMA channel is not set. DMA operations will not be carried out."]
    NOT_TRIGGERED = 0,
    #[doc = "1: Triggered. The trigger for this DMA channel is set. DMA operations will be carried out."]
    TRIGGERED = 1,
}
impl From<TRIG_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIG`"]
pub type TRIG_R = crate::R<bool, TRIG_A>;
impl TRIG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG_A {
        match self.bits {
            false => TRIG_A::NOT_TRIGGERED,
            true => TRIG_A::TRIGGERED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_TRIGGERED`"]
    #[inline(always)]
    pub fn is_not_triggered(&self) -> bool {
        *self == TRIG_A::NOT_TRIGGERED
    }
    #[doc = "Checks if the value of the field is `TRIGGERED`"]
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        *self == TRIG_A::TRIGGERED
    }
}
impl R {
    #[doc = "Bit 0 - Valid pending flag for this channel. This bit is set when a 1 is written to the corresponding bit in the related SETVALID register when CFGVALID = 1 for the same channel."]
    #[inline(always)]
    pub fn validpending(&self) -> VALIDPENDING_R {
        VALIDPENDING_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Trigger flag. Indicates that the trigger for this channel is currently set. This bit is cleared at the end of an entire transfer or upon reload when CLRTRIG = 1."]
    #[inline(always)]
    pub fn trig(&self) -> TRIG_R {
        TRIG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
