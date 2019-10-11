#[doc = "Reader of register RS485DLY"]
pub type R = crate::R<u32, super::RS485DLY>;
#[doc = "Writer for register RS485DLY"]
pub type W = crate::W<u32, super::RS485DLY>;
#[doc = "Register RS485DLY `reset()`'s with value 0"]
impl crate::ResetValue for super::RS485DLY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DLY`"]
pub type DLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLY`"]
pub struct DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Contains the direction control (RTS or DTR) delay value. This register works in conjunction with an 8-bit counter."]
    #[inline(always)]
    pub fn dly(&self) -> DLY_R {
        DLY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Contains the direction control (RTS or DTR) delay value. This register works in conjunction with an 8-bit counter."]
    #[inline(always)]
    pub fn dly(&mut self) -> DLY_W {
        DLY_W { w: self }
    }
}
