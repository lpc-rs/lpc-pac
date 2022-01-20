#[doc = "Register `MTL_TXQx_OP_MODE` reader"]
pub struct R(crate::R<MTL_TXQX_OP_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTL_TXQX_OP_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTL_TXQX_OP_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTL_TXQX_OP_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTL_TXQx_OP_MODE` writer"]
pub struct W(crate::W<MTL_TXQX_OP_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTL_TXQX_OP_MODE_SPEC>;
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
impl From<crate::W<MTL_TXQX_OP_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTL_TXQX_OP_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTQ` reader - Flush Transmit Queue When this bit is set, the Tx queue controller logic is reset to its default values."]
pub struct FTQ_R(crate::FieldReader<bool, bool>);
impl FTQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FTQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTQ` writer - Flush Transmit Queue When this bit is set, the Tx queue controller logic is reset to its default values."]
pub struct FTQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FTQ_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `TSF` reader - Transmit Store and Forward When this bit is set, the transmission starts when a full packet resides in the MTL Tx queue."]
pub struct TSF_R(crate::FieldReader<bool, bool>);
impl TSF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSF` writer - Transmit Store and Forward When this bit is set, the transmission starts when a full packet resides in the MTL Tx queue."]
pub struct TSF_W<'a> {
    w: &'a mut W,
}
impl<'a> TSF_W<'a> {
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
#[doc = "Field `TXQEN` reader - Transmit Queue Enable This field is used to enable/disable the transmit queue 0."]
pub struct TXQEN_R(crate::FieldReader<u8, u8>);
impl TXQEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXQEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXQEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXQEN` writer - Transmit Queue Enable This field is used to enable/disable the transmit queue 0."]
pub struct TXQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXQEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `TTC` reader - Transmit Threshold Control These bits control the threshold level of the MTL Tx Queue."]
pub struct TTC_R(crate::FieldReader<u8, u8>);
impl TTC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TTC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TTC` writer - Transmit Threshold Control These bits control the threshold level of the MTL Tx Queue."]
pub struct TTC_W<'a> {
    w: &'a mut W,
}
impl<'a> TTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `TQS` reader - Transmit Queue Size This field indicates the size of the allocated Transmit queues in blocks of 256 bytes."]
pub struct TQS_R(crate::FieldReader<u8, u8>);
impl TQS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TQS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TQS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TQS` writer - Transmit Queue Size This field indicates the size of the allocated Transmit queues in blocks of 256 bytes."]
pub struct TQS_W<'a> {
    w: &'a mut W,
}
impl<'a> TQS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Flush Transmit Queue When this bit is set, the Tx queue controller logic is reset to its default values."]
    #[inline(always)]
    pub fn ftq(&self) -> FTQ_R {
        FTQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Store and Forward When this bit is set, the transmission starts when a full packet resides in the MTL Tx queue."]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Transmit Queue Enable This field is used to enable/disable the transmit queue 0."]
    #[inline(always)]
    pub fn txqen(&self) -> TXQEN_R {
        TXQEN_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Transmit Threshold Control These bits control the threshold level of the MTL Tx Queue."]
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Transmit Queue Size This field indicates the size of the allocated Transmit queues in blocks of 256 bytes."]
    #[inline(always)]
    pub fn tqs(&self) -> TQS_R {
        TQS_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Flush Transmit Queue When this bit is set, the Tx queue controller logic is reset to its default values."]
    #[inline(always)]
    pub fn ftq(&mut self) -> FTQ_W {
        FTQ_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Store and Forward When this bit is set, the transmission starts when a full packet resides in the MTL Tx queue."]
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W {
        TSF_W { w: self }
    }
    #[doc = "Bits 2:3 - Transmit Queue Enable This field is used to enable/disable the transmit queue 0."]
    #[inline(always)]
    pub fn txqen(&mut self) -> TXQEN_W {
        TXQEN_W { w: self }
    }
    #[doc = "Bits 4:6 - Transmit Threshold Control These bits control the threshold level of the MTL Tx Queue."]
    #[inline(always)]
    pub fn ttc(&mut self) -> TTC_W {
        TTC_W { w: self }
    }
    #[doc = "Bits 16:18 - Transmit Queue Size This field indicates the size of the allocated Transmit queues in blocks of 256 bytes."]
    #[inline(always)]
    pub fn tqs(&mut self) -> TQS_W {
        TQS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MTL TxQx Operation Mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_txqx_op_mode](index.html) module"]
pub struct MTL_TXQX_OP_MODE_SPEC;
impl crate::RegisterSpec for MTL_TXQX_OP_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtl_txqx_op_mode::R](R) reader structure"]
impl crate::Readable for MTL_TXQX_OP_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtl_txqx_op_mode::W](W) writer structure"]
impl crate::Writable for MTL_TXQX_OP_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTL_TXQx_OP_MODE to value 0"]
impl crate::Resettable for MTL_TXQX_OP_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
