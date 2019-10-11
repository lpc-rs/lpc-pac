#[doc = "Reader of register TXDAT"]
pub type R = crate::R<u32, super::TXDAT>;
#[doc = "Writer for register TXDAT"]
pub type W = crate::W<u32, super::TXDAT>;
#[doc = "Register TXDAT `reset()`'s with value 0"]
impl crate::ResetValue for super::TXDAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXDAT`"]
pub type TXDAT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TXDAT`"]
pub struct TXDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Writing to the USART Transmit Data Register causes the data to be transmitted as soon as the transmit shift register is available and any conditions for transmitting data are met: CTS low (if CTSEN bit = 1), TXDIS bit = 0."]
    #[inline(always)]
    pub fn txdat(&self) -> TXDAT_R {
        TXDAT_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Writing to the USART Transmit Data Register causes the data to be transmitted as soon as the transmit shift register is available and any conditions for transmitting data are met: CTS low (if CTSEN bit = 1), TXDIS bit = 0."]
    #[inline(always)]
    pub fn txdat(&mut self) -> TXDAT_W {
        TXDAT_W { w: self }
    }
}
