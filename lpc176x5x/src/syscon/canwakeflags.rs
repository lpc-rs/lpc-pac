#[doc = "Reader of register CANWAKEFLAGS"]
pub type R = crate::R<u32, super::CANWAKEFLAGS>;
#[doc = "Writer for register CANWAKEFLAGS"]
pub type W = crate::W<u32, super::CANWAKEFLAGS>;
#[doc = "Register CANWAKEFLAGS `reset()`'s with value 0"]
impl crate::ResetValue for super::CANWAKEFLAGS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAN1WAKE`"]
pub type CAN1WAKE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAN1WAKE`"]
pub struct CAN1WAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN1WAKE_W<'a> {
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
#[doc = "Reader of field `CAN2WAKE`"]
pub type CAN2WAKE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAN2WAKE`"]
pub struct CAN2WAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN2WAKE_W<'a> {
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
    #[doc = "Bit 1 - Wake-up status for CAN channel 1. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 1. Write: writing a 1 clears this bit."]
    #[inline(always)]
    pub fn can1wake(&self) -> CAN1WAKE_R {
        CAN1WAKE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wake-up status for CAN channel 2. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 2. Write: writing a 1 clears this bit."]
    #[inline(always)]
    pub fn can2wake(&self) -> CAN2WAKE_R {
        CAN2WAKE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Wake-up status for CAN channel 1. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 1. Write: writing a 1 clears this bit."]
    #[inline(always)]
    pub fn can1wake(&mut self) -> CAN1WAKE_W {
        CAN1WAKE_W { w: self }
    }
    #[doc = "Bit 2 - Wake-up status for CAN channel 2. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 2. Write: writing a 1 clears this bit."]
    #[inline(always)]
    pub fn can2wake(&mut self) -> CAN2WAKE_W {
        CAN2WAKE_W { w: self }
    }
}
