#[doc = "Reader of register DLM"]
pub type R = crate::R<u32, super::DLM>;
#[doc = "Writer for register DLM"]
pub type W = crate::W<u32, super::DLM>;
#[doc = "Register DLM `reset()`'s with value 0"]
impl crate::ResetValue for super::DLM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DLMSB`"]
pub type DLMSB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLMSB`"]
pub struct DLMSB_W<'a> {
    w: &'a mut W,
}
impl<'a> DLMSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - The USART Divisor Latch MSB Register, along with the DLL register, determines the baud rate of the USART."]
    #[inline(always)]
    pub fn dlmsb(&self) -> DLMSB_R {
        DLMSB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The USART Divisor Latch MSB Register, along with the DLL register, determines the baud rate of the USART."]
    #[inline(always)]
    pub fn dlmsb(&mut self) -> DLMSB_W {
        DLMSB_W { w: self }
    }
}
