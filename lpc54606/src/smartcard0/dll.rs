#[doc = "Reader of register DLL"]
pub type R = crate::R<u32, super::DLL>;
#[doc = "Writer for register DLL"]
pub type W = crate::W<u32, super::DLL>;
#[doc = "Register DLL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::DLL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `DLLSB`"]
pub type DLLSB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLLSB`"]
pub struct DLLSB_W<'a> {
    w: &'a mut W,
}
impl<'a> DLLSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - The SCIn Divisor Latch LSB Register, along with the SCInDLM register, determines the baud rate of the SCIn."]
    #[inline(always)]
    pub fn dllsb(&self) -> DLLSB_R {
        DLLSB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The SCIn Divisor Latch LSB Register, along with the SCInDLM register, determines the baud rate of the SCIn."]
    #[inline(always)]
    pub fn dllsb(&mut self) -> DLLSB_W {
        DLLSB_W { w: self }
    }
}
