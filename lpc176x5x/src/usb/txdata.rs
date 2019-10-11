#[doc = "Writer for register TXDATA"]
pub type W = crate::W<u32, super::TXDATA>;
#[doc = "Register TXDATA `reset()`'s with value 0"]
impl crate::ResetValue for super::TXDATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TX_DATA`"]
pub struct TX_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit Data."]
    #[inline(always)]
    pub fn tx_data(&mut self) -> TX_DATA_W {
        TX_DATA_W { w: self }
    }
}
