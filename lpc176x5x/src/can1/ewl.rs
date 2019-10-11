#[doc = "Reader of register EWL"]
pub type R = crate::R<u32, super::EWL>;
#[doc = "Writer for register EWL"]
pub type W = crate::W<u32, super::EWL>;
#[doc = "Register EWL `reset()`'s with value 0x60"]
impl crate::ResetValue for super::EWL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x60
    }
}
#[doc = "Reader of field `EWL`"]
pub type EWL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EWL`"]
pub struct EWL_W<'a> {
    w: &'a mut W,
}
impl<'a> EWL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - During CAN operation, this value is compared to both the Tx and Rx Error Counters. If either of these counter matches this value, the Error Status (ES) bit in CANSR is set."]
    #[inline(always)]
    pub fn ewl(&self) -> EWL_R {
        EWL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - During CAN operation, this value is compared to both the Tx and Rx Error Counters. If either of these counter matches this value, the Error Status (ES) bit in CANSR is set."]
    #[inline(always)]
    pub fn ewl(&mut self) -> EWL_W {
        EWL_W { w: self }
    }
}
