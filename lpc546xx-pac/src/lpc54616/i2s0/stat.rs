#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `BUSY` reader - Busy status for the primary channel pair. Other BUSY flags may be found in the STAT register for each channel pair."]
pub struct BUSY_R(crate::FieldReader<bool, BUSY_A>);
impl BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
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
        **self == BUSY_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == BUSY_A::BUSY
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, BUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `SLVFRMERR` writer - Slave Frame Error flag. This applies when at least one channel pair is operating as a slave. An error indicates that the incoming WS signal did not transition as expected due to a mismatch between FRAMELEN and the actual incoming I2S stream."]
pub struct SLVFRMERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVFRMERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVFRMERR_AW) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
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
#[doc = "Field `LR` reader - Left/Right indication. This flag is considered to be a debugging aid and is not expected to be used by an I2S driver. Valid when one channel pair is busy. Indicates left or right data being processed for the currently busy channel pair."]
pub struct LR_R(crate::FieldReader<bool, LR_A>);
impl LR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LR_R(crate::FieldReader::new(bits))
    }
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
        **self == LR_A::LEFT_CHANNEL
    }
    #[doc = "Checks if the value of the field is `RIGHT_CHANNEL`"]
    #[inline(always)]
    pub fn is_right_channel(&self) -> bool {
        **self == LR_A::RIGHT_CHANNEL
    }
}
impl core::ops::Deref for LR_R {
    type Target = crate::FieldReader<bool, LR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `DATAPAUSED` reader - Data Paused status flag. Applies to all I2S channels"]
pub struct DATAPAUSED_R(crate::FieldReader<bool, DATAPAUSED_A>);
impl DATAPAUSED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATAPAUSED_R(crate::FieldReader::new(bits))
    }
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
        **self == DATAPAUSED_A::NOT_PAUSED
    }
    #[doc = "Checks if the value of the field is `PAUSED`"]
    #[inline(always)]
    pub fn is_paused(&self) -> bool {
        **self == DATAPAUSED_A::PAUSED
    }
}
impl core::ops::Deref for DATAPAUSED_R {
    type Target = crate::FieldReader<bool, DATAPAUSED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register for the primary channel pair.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
