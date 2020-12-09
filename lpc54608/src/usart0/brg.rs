#[doc = "Reader of register BRG"]
pub type R = crate::R<u32, super::BRG>;
#[doc = "Writer for register BRG"]
pub type W = crate::W<u32, super::BRG>;
#[doc = "Register BRG `reset()`'s with value 0"]
impl crate::ResetValue for super::BRG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BRGVAL`"]
pub type BRGVAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BRGVAL`"]
pub struct BRGVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BRGVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This value is used to divide the USART input clock to determine the baud rate, based on the input clock from the FRG. 0 = FCLK is used directly by the USART function. 1 = FCLK is divided by 2 before use by the USART function. 2 = FCLK is divided by 3 before use by the USART function. 0xFFFF = FCLK is divided by 65,536 before use by the USART function."]
    #[inline(always)]
    pub fn brgval(&self) -> BRGVAL_R {
        BRGVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This value is used to divide the USART input clock to determine the baud rate, based on the input clock from the FRG. 0 = FCLK is used directly by the USART function. 1 = FCLK is divided by 2 before use by the USART function. 2 = FCLK is divided by 3 before use by the USART function. 0xFFFF = FCLK is divided by 65,536 before use by the USART function."]
    #[inline(always)]
    pub fn brgval(&mut self) -> BRGVAL_W {
        BRGVAL_W { w: self }
    }
}
