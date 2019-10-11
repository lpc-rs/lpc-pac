#[doc = "Writer for register TXPLEN"]
pub type W = crate::W<u32, super::TXPLEN>;
#[doc = "Register TXPLEN `reset()`'s with value 0"]
impl crate::ResetValue for super::TXPLEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `PKT_LNGTH`"]
pub struct PKT_LNGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> PKT_LNGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:9 - The remaining number of bytes to be written to the selected endpoint buffer. This field is decremented by 4 by hardware after each write to USBTxData. When this field decrements to 0, the TxENDPKT bit will be set in USBDevIntSt."]
    #[inline(always)]
    pub fn pkt_lngth(&mut self) -> PKT_LNGTH_W {
        PKT_LNGTH_W { w: self }
    }
}
