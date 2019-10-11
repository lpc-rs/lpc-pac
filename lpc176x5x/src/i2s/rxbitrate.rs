#[doc = "Reader of register RXBITRATE"]
pub type R = crate::R<u32, super::RXBITRATE>;
#[doc = "Writer for register RXBITRATE"]
pub type W = crate::W<u32, super::RXBITRATE>;
#[doc = "Register RXBITRATE `reset()`'s with value 0"]
impl crate::ResetValue for super::RXBITRATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX_BITRATE`"]
pub type RX_BITRATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_BITRATE`"]
pub struct RX_BITRATE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BITRATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - I2S receive bit rate. This value plus one is used to divide RX_MCLK to produce the receive bit clock."]
    #[inline(always)]
    pub fn rx_bitrate(&self) -> RX_BITRATE_R {
        RX_BITRATE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I2S receive bit rate. This value plus one is used to divide RX_MCLK to produce the receive bit clock."]
    #[inline(always)]
    pub fn rx_bitrate(&mut self) -> RX_BITRATE_W {
        RX_BITRATE_W { w: self }
    }
}
