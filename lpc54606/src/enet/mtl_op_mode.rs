#[doc = "Register `MTL_OP_MODE` reader"]
pub struct R(crate::R<MTL_OP_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTL_OP_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MTL_OP_MODE_SPEC>> for R {
    fn from(reader: crate::R<MTL_OP_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTL_OP_MODE` writer"]
pub struct W(crate::W<MTL_OP_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTL_OP_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<MTL_OP_MODE_SPEC>> for W {
    fn from(writer: crate::W<MTL_OP_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTXSTS` reader - Drop Transmit Status When this bit is set, the Tx packet status received from the MAC is dropped in the MTL."]
pub struct DTXSTS_R(crate::FieldReader<bool, bool>);
impl DTXSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTXSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTXSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTXSTS` writer - Drop Transmit Status When this bit is set, the Tx packet status received from the MAC is dropped in the MTL."]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `RAA` reader - Receive Arbitration Algorithm This field is used to select the arbitration algorithm for the Rx side."]
pub struct RAA_R(crate::FieldReader<bool, bool>);
impl RAA_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHALG` reader - Tx Scheduling Algorithm This field indicates the algorithm for Tx scheduling: 0x00: WRR algorithm 0x1: Reserved 0x2: Reserved 0x3: Strict priority algorithm."]
pub struct SCHALG_R(crate::FieldReader<u8, u8>);
impl SCHALG_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCHALG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCHALG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCHALG` writer - Tx Scheduling Algorithm This field indicates the algorithm for Tx scheduling: 0x00: WRR algorithm 0x1: Reserved 0x2: Reserved 0x3: Strict priority algorithm."]
pub struct SCHALG_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHALG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `CNTPRST` reader - Counters Preset When this bit is set, MTL TxQ0 Underflow register (Table 762) and MTL_TxQ1_Underflow (Table 762) registers are initialized/preset to 0x7F0."]
pub struct CNTPRST_R(crate::FieldReader<bool, bool>);
impl CNTPRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTPRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTPRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTPRST` writer - Counters Preset When this bit is set, MTL TxQ0 Underflow register (Table 762) and MTL_TxQ1_Underflow (Table 762) registers are initialized/preset to 0x7F0."]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CNTCLR` reader - Counters Reset When this bit is set, all counters are reset."]
pub struct CNTCLR_R(crate::FieldReader<bool, bool>);
impl CNTCLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTCLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTCLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTCLR` writer - Counters Reset When this bit is set, all counters are reset."]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
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
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MTL Operation Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_op_mode](index.html) module"]
pub struct MTL_OP_MODE_SPEC;
impl crate::RegisterSpec for MTL_OP_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtl_op_mode::R](R) reader structure"]
impl crate::Readable for MTL_OP_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtl_op_mode::W](W) writer structure"]
impl crate::Writable for MTL_OP_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTL_OP_MODE to value 0"]
impl crate::Resettable for MTL_OP_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
