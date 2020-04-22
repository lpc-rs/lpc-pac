#[doc = "Reader of register DMA_MODE"]
pub type R = crate::R<u32, super::DMA_MODE>;
#[doc = "Writer for register DMA_MODE"]
pub type W = crate::W<u32, super::DMA_MODE>;
#[doc = "Register DMA_MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SWR`"]
pub type SWR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWR`"]
pub struct SWR_W<'a> {
    w: &'a mut W,
}
impl<'a> SWR_W<'a> {
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
#[doc = "Reader of field `DA`"]
pub type DA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DA`"]
pub struct DA_W<'a> {
    w: &'a mut W,
}
impl<'a> DA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `TAA`"]
pub type TAA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TAA`"]
pub struct TAA_W<'a> {
    w: &'a mut W,
}
impl<'a> TAA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `TXPR`"]
pub type TXPR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXPR`"]
pub struct TXPR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `PR`"]
pub type PR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PR`"]
pub struct PR_W<'a> {
    w: &'a mut W,
}
impl<'a> PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset When this bit is set, the MAC and the OMA controller reset the logic and all internal registers of the OMA, MTL, and MAC."]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA Tx or Rx Arbitration Scheme This bit specifies the arbitration scheme between the Transmit and Receive paths of all channels: The Tx path has priority over the Rx path when the TXPR bit is set."]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - Transmit Arbitration Algorithm This field is used to select the arbitration algorithm for the Transmit side when multiple Tx DMAs are selected."]
    #[inline(always)]
    pub fn taa(&self) -> TAA_R {
        TAA_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Transmit Priority When set, this bit indicates that the Tx DMA has higher priority than the Rx DMA during arbitration for the system-side bus."]
    #[inline(always)]
    pub fn txpr(&self) -> TXPR_R {
        TXPR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - Priority Ratio These bits control the priority ratio in weighted round-robin arbitration between the Rx DMA and Tx DMA."]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset When this bit is set, the MAC and the OMA controller reset the logic and all internal registers of the OMA, MTL, and MAC."]
    #[inline(always)]
    pub fn swr(&mut self) -> SWR_W {
        SWR_W { w: self }
    }
    #[doc = "Bit 1 - DMA Tx or Rx Arbitration Scheme This bit specifies the arbitration scheme between the Transmit and Receive paths of all channels: The Tx path has priority over the Rx path when the TXPR bit is set."]
    #[inline(always)]
    pub fn da(&mut self) -> DA_W {
        DA_W { w: self }
    }
    #[doc = "Bits 2:4 - Transmit Arbitration Algorithm This field is used to select the arbitration algorithm for the Transmit side when multiple Tx DMAs are selected."]
    #[inline(always)]
    pub fn taa(&mut self) -> TAA_W {
        TAA_W { w: self }
    }
    #[doc = "Bit 11 - Transmit Priority When set, this bit indicates that the Tx DMA has higher priority than the Rx DMA during arbitration for the system-side bus."]
    #[inline(always)]
    pub fn txpr(&mut self) -> TXPR_W {
        TXPR_W { w: self }
    }
    #[doc = "Bits 12:14 - Priority Ratio These bits control the priority ratio in weighted round-robin arbitration between the Rx DMA and Tx DMA."]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W {
        PR_W { w: self }
    }
}
