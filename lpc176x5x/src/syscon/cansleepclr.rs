#[doc = "Reader of register CANSLEEPCLR"]
pub type R = crate::R<u32, super::CANSLEEPCLR>;
#[doc = "Writer for register CANSLEEPCLR"]
pub type W = crate::W<u32, super::CANSLEEPCLR>;
#[doc = "Register CANSLEEPCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::CANSLEEPCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAN1SLEEP`"]
pub type CAN1SLEEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAN1SLEEP`"]
pub struct CAN1SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN1SLEEP_W<'a> {
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
#[doc = "Reader of field `CAN2SLEEP`"]
pub type CAN2SLEEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAN2SLEEP`"]
pub struct CAN2SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN2SLEEP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Sleep status and control for CAN channel 1. Read: when 1, indicates that CAN channel 1 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 1."]
    #[inline(always)]
    pub fn can1sleep(&self) -> CAN1SLEEP_R {
        CAN1SLEEP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Sleep status and control for CAN channel 2. Read: when 1, indicates that CAN channel 2 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 2."]
    #[inline(always)]
    pub fn can2sleep(&self) -> CAN2SLEEP_R {
        CAN2SLEEP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Sleep status and control for CAN channel 1. Read: when 1, indicates that CAN channel 1 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 1."]
    #[inline(always)]
    pub fn can1sleep(&mut self) -> CAN1SLEEP_W {
        CAN1SLEEP_W { w: self }
    }
    #[doc = "Bit 2 - Sleep status and control for CAN channel 2. Read: when 1, indicates that CAN channel 2 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 2."]
    #[inline(always)]
    pub fn can2sleep(&mut self) -> CAN2SLEEP_W {
        CAN2SLEEP_W { w: self }
    }
}
