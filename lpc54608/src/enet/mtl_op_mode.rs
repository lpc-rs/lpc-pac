#[doc = "Reader of register MTL_OP_MODE"]
pub type R = crate::R<u32, super::MTL_OP_MODE>;
#[doc = "Writer for register MTL_OP_MODE"]
pub type W = crate::W<u32, super::MTL_OP_MODE>;
#[doc = "Register MTL_OP_MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::MTL_OP_MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DTXSTS`"]
pub type DTXSTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTXSTS`"]
pub struct DTXSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> DTXSTS_W<'a> {
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
#[doc = "Reader of field `RAA`"]
pub type RAA_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCHALG`"]
pub type SCHALG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCHALG`"]
pub struct SCHALG_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHALG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `CNTPRST`"]
pub type CNTPRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNTPRST`"]
pub struct CNTPRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTPRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `CNTCLR`"]
pub type CNTCLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNTCLR`"]
pub struct CNTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Drop Transmit Status When this bit is set, the Tx packet status received from the MAC is dropped in the MTL."]
    #[inline(always)]
    pub fn dtxsts(&self) -> DTXSTS_R {
        DTXSTS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive Arbitration Algorithm This field is used to select the arbitration algorithm for the Rx side."]
    #[inline(always)]
    pub fn raa(&self) -> RAA_R {
        RAA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Tx Scheduling Algorithm This field indicates the algorithm for Tx scheduling: 0x00: WRR algorithm 0x1: Reserved 0x2: Reserved 0x3: Strict priority algorithm."]
    #[inline(always)]
    pub fn schalg(&self) -> SCHALG_R {
        SCHALG_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Counters Preset When this bit is set, MTL TxQ0 Underflow register (Table 762) and MTL_TxQ1_Underflow (Table 762) registers are initialized/preset to 0x7F0."]
    #[inline(always)]
    pub fn cntprst(&self) -> CNTPRST_R {
        CNTPRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Counters Reset When this bit is set, all counters are reset."]
    #[inline(always)]
    pub fn cntclr(&self) -> CNTCLR_R {
        CNTCLR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Drop Transmit Status When this bit is set, the Tx packet status received from the MAC is dropped in the MTL."]
    #[inline(always)]
    pub fn dtxsts(&mut self) -> DTXSTS_W {
        DTXSTS_W { w: self }
    }
    #[doc = "Bits 5:6 - Tx Scheduling Algorithm This field indicates the algorithm for Tx scheduling: 0x00: WRR algorithm 0x1: Reserved 0x2: Reserved 0x3: Strict priority algorithm."]
    #[inline(always)]
    pub fn schalg(&mut self) -> SCHALG_W {
        SCHALG_W { w: self }
    }
    #[doc = "Bit 8 - Counters Preset When this bit is set, MTL TxQ0 Underflow register (Table 762) and MTL_TxQ1_Underflow (Table 762) registers are initialized/preset to 0x7F0."]
    #[inline(always)]
    pub fn cntprst(&mut self) -> CNTPRST_W {
        CNTPRST_W { w: self }
    }
    #[doc = "Bit 9 - Counters Reset When this bit is set, all counters are reset."]
    #[inline(always)]
    pub fn cntclr(&mut self) -> CNTCLR_W {
        CNTCLR_W { w: self }
    }
}
