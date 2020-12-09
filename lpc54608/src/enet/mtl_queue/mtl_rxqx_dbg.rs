#[doc = "Reader of register MTL_RXQx_DBG"]
pub type R = crate::R<u32, super::MTL_RXQX_DBG>;
#[doc = "Writer for register MTL_RXQx_DBG"]
pub type W = crate::W<u32, super::MTL_RXQX_DBG>;
#[doc = "Register MTL_RXQx_DBG `reset()`'s with value 0"]
impl crate::ResetValue for super::MTL_RXQX_DBG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RWCSTS`"]
pub type RWCSTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RWCSTS`"]
pub struct RWCSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> RWCSTS_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `RRCSTS`"]
pub type RRCSTS_R = crate::R<u8, u8>;
#[doc = "Reader of field `RXQSTS`"]
pub type RXQSTS_R = crate::R<u8, u8>;
#[doc = "Reader of field `PRXQ`"]
pub type PRXQ_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bit 0 - MTL Rx Queue Write Controller Active Status When high, this bit indicates that the MTL Rx queue Write controller is active, and it is transferring a received packet to the Rx Queue."]
    #[inline(always)]
    pub fn rwcsts(&self) -> RWCSTS_R {
        RWCSTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - MTL Rx Queue Read Controller State This field gives the state of the Rx queue Read controller: 00: Idle state 01: Reading packet data 10: Reading packet status (or timestamp) 11: Flushing the packet data and status."]
    #[inline(always)]
    pub fn rrcsts(&self) -> RRCSTS_R {
        RRCSTS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - MTL Rx Queue Fill-Level Status This field gives the status of the fill-level of the Rx Queue: 0x0: Rx Queue empty 0x1: Rx Queue fill-level below flow-control deactivate threshold 0x2: Rx Queue fill-level above flow-control activate threshold 0x3: Rx Queue full."]
    #[inline(always)]
    pub fn rxqsts(&self) -> RXQSTS_R {
        RXQSTS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 16:29 - Number of Packets in Receive Queue This field indicates the current number of packets in the Rx Queue."]
    #[inline(always)]
    pub fn prxq(&self) -> PRXQ_R {
        PRXQ_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - MTL Rx Queue Write Controller Active Status When high, this bit indicates that the MTL Rx queue Write controller is active, and it is transferring a received packet to the Rx Queue."]
    #[inline(always)]
    pub fn rwcsts(&mut self) -> RWCSTS_W {
        RWCSTS_W { w: self }
    }
}
