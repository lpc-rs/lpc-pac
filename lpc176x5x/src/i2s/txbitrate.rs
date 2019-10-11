#[doc = "Reader of register TXBITRATE"]
pub type R = crate::R<u32, super::TXBITRATE>;
#[doc = "Writer for register TXBITRATE"]
pub type W = crate::W<u32, super::TXBITRATE>;
#[doc = "Register TXBITRATE `reset()`'s with value 0"]
impl crate::ResetValue for super::TXBITRATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_BITRATE`"]
pub type TX_BITRATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_BITRATE`"]
pub struct TX_BITRATE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BITRATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - I2S transmit bit rate. This value plus one is used to divide TX_MCLK to produce the transmit bit clock."]
    #[inline(always)]
    pub fn tx_bitrate(&self) -> TX_BITRATE_R {
        TX_BITRATE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I2S transmit bit rate. This value plus one is used to divide TX_MCLK to produce the transmit bit clock."]
    #[inline(always)]
    pub fn tx_bitrate(&mut self) -> TX_BITRATE_W {
        TX_BITRATE_W { w: self }
    }
}
