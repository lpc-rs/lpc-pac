#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Writer for register STAT"]
pub type W = crate::W<u32, super::STAT>;
#[doc = "Register STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Busy status for the primary channel pair. Other BUSY flags may be found in the STAT register for each channel pair.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: The transmitter/receiver for channel pair is currently idle."]
    IDLE = 0,
    #[doc = "1: The transmitter/receiver for channel pair is currently processing data."]
    BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, BUSY_A>;
impl BUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::IDLE,
            true => BUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::BUSY
    }
}
#[doc = "Slave Frame Error flag. This applies when at least one channel pair is operating as a slave. An error indicates that the incoming WS signal did not transition as expected due to a mismatch between FRAMELEN and the actual incoming I2S stream.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVFRMERR_AW {
    #[doc = "0: No error has been recorded."]
    NO_ERROR = 0,
    #[doc = "1: An error has been recorded for some channel pair that is operating in slave mode. ERROR is cleared by writing a 1 to this bit position."]
    ERROR = 1,
}
impl From<SLVFRMERR_AW> for bool {
    #[inline(always)]
    fn from(variant: SLVFRMERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SLVFRMERR`"]
pub struct SLVFRMERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVFRMERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVFRMERR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error has been recorded."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(SLVFRMERR_AW::NO_ERROR)
    }
    #[doc = "An error has been recorded for some channel pair that is operating in slave mode. ERROR is cleared by writing a 1 to this bit position."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(SLVFRMERR_AW::ERROR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Left/Right indication. This flag is considered to be a debugging aid and is not expected to be used by an I2S driver. Valid when one channel pair is busy. Indicates left or right data being processed for the currently busy channel pair.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LR_A {
    #[doc = "0: Left channel."]
    LEFT_CHANNEL = 0,
    #[doc = "1: Right channel."]
    RIGHT_CHANNEL = 1,
}
impl From<LR_A> for bool {
    #[inline(always)]
    fn from(variant: LR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LR`"]
pub type LR_R = crate::R<bool, LR_A>;
impl LR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LR_A {
        match self.bits {
            false => LR_A::LEFT_CHANNEL,
            true => LR_A::RIGHT_CHANNEL,
        }
    }
    #[doc = "Checks if the value of the field is `LEFT_CHANNEL`"]
    #[inline(always)]
    pub fn is_left_channel(&self) -> bool {
        *self == LR_A::LEFT_CHANNEL
    }
    #[doc = "Checks if the value of the field is `RIGHT_CHANNEL`"]
    #[inline(always)]
    pub fn is_right_channel(&self) -> bool {
        *self == LR_A::RIGHT_CHANNEL
    }
}
#[doc = "Data Paused status flag. Applies to all I2S channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATAPAUSED_A {
    #[doc = "0: Data is not currently paused. A data pause may have been requested but is not yet in force, waiting for an allowed pause point. Refer to the description of the DATAPAUSE control bit in the CFG1 register."]
    NOT_PAUSED = 0,
    #[doc = "1: A data pause has been requested and is now in force."]
    PAUSED = 1,
}
impl From<DATAPAUSED_A> for bool {
    #[inline(always)]
    fn from(variant: DATAPAUSED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DATAPAUSED`"]
pub type DATAPAUSED_R = crate::R<bool, DATAPAUSED_A>;
impl DATAPAUSED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATAPAUSED_A {
        match self.bits {
            false => DATAPAUSED_A::NOT_PAUSED,
            true => DATAPAUSED_A::PAUSED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PAUSED`"]
    #[inline(always)]
    pub fn is_not_paused(&self) -> bool {
        *self == DATAPAUSED_A::NOT_PAUSED
    }
    #[doc = "Checks if the value of the field is `PAUSED`"]
    #[inline(always)]
    pub fn is_paused(&self) -> bool {
        *self == DATAPAUSED_A::PAUSED
    }
}
impl R {
    #[doc = "Bit 0 - Busy status for the primary channel pair. Other BUSY flags may be found in the STAT register for each channel pair."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Left/Right indication. This flag is considered to be a debugging aid and is not expected to be used by an I2S driver. Valid when one channel pair is busy. Indicates left or right data being processed for the currently busy channel pair."]
    #[inline(always)]
    pub fn lr(&self) -> LR_R {
        LR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data Paused status flag. Applies to all I2S channels"]
    #[inline(always)]
    pub fn datapaused(&self) -> DATAPAUSED_R {
        DATAPAUSED_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Slave Frame Error flag. This applies when at least one channel pair is operating as a slave. An error indicates that the incoming WS signal did not transition as expected due to a mismatch between FRAMELEN and the actual incoming I2S stream."]
    #[inline(always)]
    pub fn slvfrmerr(&mut self) -> SLVFRMERR_W {
        SLVFRMERR_W { w: self }
    }
}
