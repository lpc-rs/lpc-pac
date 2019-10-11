#[doc = "Reader of register TXPRODUCEINDEX"]
pub type R = crate::R<u32, super::TXPRODUCEINDEX>;
#[doc = "Writer for register TXPRODUCEINDEX"]
pub type W = crate::W<u32, super::TXPRODUCEINDEX>;
#[doc = "Register TXPRODUCEINDEX `reset()`'s with value 0"]
impl crate::ResetValue for super::TXPRODUCEINDEX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXPI`"]
pub type TXPI_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TXPI`"]
pub struct TXPI_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - TxProduceIndex. Index of the descriptor that is going to be filled next by the transmit software driver."]
    #[inline(always)]
    pub fn txpi(&self) -> TXPI_R {
        TXPI_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TxProduceIndex. Index of the descriptor that is going to be filled next by the transmit software driver."]
    #[inline(always)]
    pub fn txpi(&mut self) -> TXPI_W {
        TXPI_W { w: self }
    }
}
