#[doc = "Reader of register RXCONSUMEINDEX"]
pub type R = crate::R<u32, super::RXCONSUMEINDEX>;
#[doc = "Writer for register RXCONSUMEINDEX"]
pub type W = crate::W<u32, super::RXCONSUMEINDEX>;
#[doc = "Register RXCONSUMEINDEX `reset()`'s with value 0"]
impl crate::ResetValue for super::RXCONSUMEINDEX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXCONSUMEIX`"]
pub type RXCONSUMEIX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RXCONSUMEIX`"]
pub struct RXCONSUMEIX_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCONSUMEIX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Index of the descriptor that is going to be processed next by the receive"]
    #[inline(always)]
    pub fn rxconsumeix(&self) -> RXCONSUMEIX_R {
        RXCONSUMEIX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Index of the descriptor that is going to be processed next by the receive"]
    #[inline(always)]
    pub fn rxconsumeix(&mut self) -> RXCONSUMEIX_W {
        RXCONSUMEIX_W { w: self }
    }
}
