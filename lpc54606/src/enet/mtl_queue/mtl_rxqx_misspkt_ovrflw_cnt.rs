#[doc = "Reader of register MTL_RXQx_MISSPKT_OVRFLW_CNT"]
pub type R = crate::R<u32, super::MTL_RXQX_MISSPKT_OVRFLW_CNT>;
#[doc = "Writer for register MTL_RXQx_MISSPKT_OVRFLW_CNT"]
pub type W = crate::W<u32, super::MTL_RXQX_MISSPKT_OVRFLW_CNT>;
#[doc = "Register MTL_RXQx_MISSPKT_OVRFLW_CNT `reset()`'s with value 0"]
impl crate::ResetValue for super::MTL_RXQX_MISSPKT_OVRFLW_CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OVFPKTCNT`"]
pub type OVFPKTCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OVFPKTCNT`"]
pub struct OVFPKTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> OVFPKTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
#[doc = "Reader of field `OVFCNTOVF`"]
pub type OVFCNTOVF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:10 - Overflow Packet Counter This field indicates the number of packets discarded by the Ethernet block because of Receive queue overflow."]
    #[inline(always)]
    pub fn ovfpktcnt(&self) -> OVFPKTCNT_R {
        OVFPKTCNT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Overflow Counter Overflow Bit When set, this bit indicates that the Rx Queue Overflow Packet Counter field crossed the maximum limit."]
    #[inline(always)]
    pub fn ovfcntovf(&self) -> OVFCNTOVF_R {
        OVFCNTOVF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Overflow Packet Counter This field indicates the number of packets discarded by the Ethernet block because of Receive queue overflow."]
    #[inline(always)]
    pub fn ovfpktcnt(&mut self) -> OVFPKTCNT_W {
        OVFPKTCNT_W { w: self }
    }
}
