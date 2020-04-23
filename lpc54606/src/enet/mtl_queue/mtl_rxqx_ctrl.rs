#[doc = "Reader of register MTL_RXQx_CTRL"]
pub type R = crate::R<u32, super::MTL_RXQX_CTRL>;
#[doc = "Writer for register MTL_RXQx_CTRL"]
pub type W = crate::W<u32, super::MTL_RXQX_CTRL>;
#[doc = "Register MTL_RXQx_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::MTL_RXQX_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXQ_WEGT`"]
pub type RXQ_WEGT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXQ_WEGT`"]
pub struct RXQ_WEGT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXQ_WEGT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `RXQ_FRM_ARBIT`"]
pub type RXQ_FRM_ARBIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXQ_FRM_ARBIT`"]
pub struct RXQ_FRM_ARBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXQ_FRM_ARBIT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Receive Queue Weight This field indicates the weight assigned to the Rx Queue 0."]
    #[inline(always)]
    pub fn rxq_wegt(&self) -> RXQ_WEGT_R {
        RXQ_WEGT_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Receive Queue Packet Arbitration When this bit is set, the The ethernet block drives the packet data to the ARI interface such that the entire packet data of currently-selected queue is transmitted before switching to other queue."]
    #[inline(always)]
    pub fn rxq_frm_arbit(&self) -> RXQ_FRM_ARBIT_R {
        RXQ_FRM_ARBIT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Receive Queue Weight This field indicates the weight assigned to the Rx Queue 0."]
    #[inline(always)]
    pub fn rxq_wegt(&mut self) -> RXQ_WEGT_W {
        RXQ_WEGT_W { w: self }
    }
    #[doc = "Bit 3 - Receive Queue Packet Arbitration When this bit is set, the The ethernet block drives the packet data to the ARI interface such that the entire packet data of currently-selected queue is transmitted before switching to other queue."]
    #[inline(always)]
    pub fn rxq_frm_arbit(&mut self) -> RXQ_FRM_ARBIT_W {
        RXQ_FRM_ARBIT_W { w: self }
    }
}
