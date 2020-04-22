#[doc = "Reader of register CALIBRATION"]
pub type R = crate::R<u32, super::CALIBRATION>;
#[doc = "Writer for register CALIBRATION"]
pub type W = crate::W<u32, super::CALIBRATION>;
#[doc = "Register CALIBRATION `reset()`'s with value 0"]
impl crate::ResetValue for super::CALIBRATION {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CALVAL`"]
pub type CALVAL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CALVAL`"]
pub struct CALVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CALVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
#[doc = "Calibration direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALDIR_A {
    #[doc = "1: Backward calibration. When CALVAL is equal to the calibration counter, the RTC timers will stop incrementing for 1 second."]
    BACKWARD_CALIBRATION = 1,
    #[doc = "0: Forward calibration. When CALVAL is equal to the calibration counter, the RTC timers will jump by 2 seconds."]
    FORWARD_CALIBRATION_ = 0,
}
impl From<CALDIR_A> for bool {
    #[inline(always)]
    fn from(variant: CALDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CALDIR`"]
pub type CALDIR_R = crate::R<bool, CALDIR_A>;
impl CALDIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALDIR_A {
        match self.bits {
            true => CALDIR_A::BACKWARD_CALIBRATION,
            false => CALDIR_A::FORWARD_CALIBRATION_,
        }
    }
    #[doc = "Checks if the value of the field is `BACKWARD_CALIBRATION`"]
    #[inline(always)]
    pub fn is_backward_calibration(&self) -> bool {
        *self == CALDIR_A::BACKWARD_CALIBRATION
    }
    #[doc = "Checks if the value of the field is `FORWARD_CALIBRATION_`"]
    #[inline(always)]
    pub fn is_forward_calibration_(&self) -> bool {
        *self == CALDIR_A::FORWARD_CALIBRATION_
    }
}
#[doc = "Write proxy for field `CALDIR`"]
pub struct CALDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> CALDIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALDIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Backward calibration. When CALVAL is equal to the calibration counter, the RTC timers will stop incrementing for 1 second."]
    #[inline(always)]
    pub fn backward_calibration(self) -> &'a mut W {
        self.variant(CALDIR_A::BACKWARD_CALIBRATION)
    }
    #[doc = "Forward calibration. When CALVAL is equal to the calibration counter, the RTC timers will jump by 2 seconds."]
    #[inline(always)]
    pub fn forward_calibration_(self) -> &'a mut W {
        self.variant(CALDIR_A::FORWARD_CALIBRATION_)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - If enabled, the calibration counter counts up to this value. The maximum value is 131, 072 corresponding to about 36.4 hours. Calibration is disabled if CALVAL = 0."]
    #[inline(always)]
    pub fn calval(&self) -> CALVAL_R {
        CALVAL_R::new((self.bits & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 17 - Calibration direction"]
    #[inline(always)]
    pub fn caldir(&self) -> CALDIR_R {
        CALDIR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:16 - If enabled, the calibration counter counts up to this value. The maximum value is 131, 072 corresponding to about 36.4 hours. Calibration is disabled if CALVAL = 0."]
    #[inline(always)]
    pub fn calval(&mut self) -> CALVAL_W {
        CALVAL_W { w: self }
    }
    #[doc = "Bit 17 - Calibration direction"]
    #[inline(always)]
    pub fn caldir(&mut self) -> CALDIR_W {
        CALDIR_W { w: self }
    }
}
