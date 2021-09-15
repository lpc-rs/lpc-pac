#[doc = "Register `MTL_RXQx_OP_MODE` reader"]
pub struct R(crate::R<MTL_RXQX_OP_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTL_RXQX_OP_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTL_RXQX_OP_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTL_RXQX_OP_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTL_RXQx_OP_MODE` writer"]
pub struct W(crate::W<MTL_RXQX_OP_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTL_RXQX_OP_MODE_SPEC>;
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
impl From<crate::W<MTL_RXQX_OP_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTL_RXQX_OP_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC` reader - Receive Queue Threshold Control These bits control the threshold level of the MTL Rx queue (in bytes): 00: 64 01: 32 10: 96 11: 128 The packet received is transferred to the application or DMA when the packet size within the MTL Rx queue is larger than the threshold."]
pub struct RTC_R(crate::FieldReader<u8, u8>);
impl RTC_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC` writer - Receive Queue Threshold Control These bits control the threshold level of the MTL Rx queue (in bytes): 00: 64 01: 32 10: 96 11: 128 The packet received is transferred to the application or DMA when the packet size within the MTL Rx queue is larger than the threshold."]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `FUP` reader - Forward Undersized Good Packets When this bit is set, the Rx queue forwards the undersized good packets (packets with no error and length less than 64 bytes), including pad-bytes and CRC."]
pub struct FUP_R(crate::FieldReader<bool, bool>);
impl FUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        FUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUP` writer - Forward Undersized Good Packets When this bit is set, the Rx queue forwards the undersized good packets (packets with no error and length less than 64 bytes), including pad-bytes and CRC."]
pub struct FUP_W<'a> {
    w: &'a mut W,
}
impl<'a> FUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `FEP` reader - Forward Error Packets When this bit is reset, the Rx queue drops packets with error status (CRC error, Mll_ER, watchdog timeout, or overflow)."]
pub struct FEP_R(crate::FieldReader<bool, bool>);
impl FEP_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEP` writer - Forward Error Packets When this bit is reset, the Rx queue drops packets with error status (CRC error, Mll_ER, watchdog timeout, or overflow)."]
pub struct FEP_W<'a> {
    w: &'a mut W,
}
impl<'a> FEP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `RSF` reader - Receive Queue Store and Forward When this bit is set, the ethernet block on this chip reads a packet from the Rx queue only after the complete packet has been written to it, ignoring the RTC field of this register."]
pub struct RSF_R(crate::FieldReader<bool, bool>);
impl RSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSF` writer - Receive Queue Store and Forward When this bit is set, the ethernet block on this chip reads a packet from the Rx queue only after the complete packet has been written to it, ignoring the RTC field of this register."]
pub struct RSF_W<'a> {
    w: &'a mut W,
}
impl<'a> RSF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `DIS_TCP_EF` reader - Disable Dropping of TCP/IP Checksum Error Packets When this bit is set, the MAC does not drop the packets which only have the errors detected by the Receive Checksum Offload engine."]
pub struct DIS_TCP_EF_R(crate::FieldReader<bool, bool>);
impl DIS_TCP_EF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_TCP_EF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_TCP_EF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_TCP_EF` writer - Disable Dropping of TCP/IP Checksum Error Packets When this bit is set, the MAC does not drop the packets which only have the errors detected by the Receive Checksum Offload engine."]
pub struct DIS_TCP_EF_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_TCP_EF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `RQS` reader - This field indicates the size of the allocated Receive queues in blocks of 256 bytes."]
pub struct RQS_R(crate::FieldReader<u8, u8>);
impl RQS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RQS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RQS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RQS` writer - This field indicates the size of the allocated Receive queues in blocks of 256 bytes."]
pub struct RQS_W<'a> {
    w: &'a mut W,
}
impl<'a> RQS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Receive Queue Threshold Control These bits control the threshold level of the MTL Rx queue (in bytes): 00: 64 01: 32 10: 96 11: 128 The packet received is transferred to the application or DMA when the packet size within the MTL Rx queue is larger than the threshold."]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 3 - Forward Undersized Good Packets When this bit is set, the Rx queue forwards the undersized good packets (packets with no error and length less than 64 bytes), including pad-bytes and CRC."]
    #[inline(always)]
    pub fn fup(&self) -> FUP_R {
        FUP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Forward Error Packets When this bit is reset, the Rx queue drops packets with error status (CRC error, Mll_ER, watchdog timeout, or overflow)."]
    #[inline(always)]
    pub fn fep(&self) -> FEP_R {
        FEP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive Queue Store and Forward When this bit is set, the ethernet block on this chip reads a packet from the Rx queue only after the complete packet has been written to it, ignoring the RTC field of this register."]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Disable Dropping of TCP/IP Checksum Error Packets When this bit is set, the MAC does not drop the packets which only have the errors detected by the Receive Checksum Offload engine."]
    #[inline(always)]
    pub fn dis_tcp_ef(&self) -> DIS_TCP_EF_R {
        DIS_TCP_EF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - This field indicates the size of the allocated Receive queues in blocks of 256 bytes."]
    #[inline(always)]
    pub fn rqs(&self) -> RQS_R {
        RQS_R::new(((self.bits >> 20) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Receive Queue Threshold Control These bits control the threshold level of the MTL Rx queue (in bytes): 00: 64 01: 32 10: 96 11: 128 The packet received is transferred to the application or DMA when the packet size within the MTL Rx queue is larger than the threshold."]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
    #[doc = "Bit 3 - Forward Undersized Good Packets When this bit is set, the Rx queue forwards the undersized good packets (packets with no error and length less than 64 bytes), including pad-bytes and CRC."]
    #[inline(always)]
    pub fn fup(&mut self) -> FUP_W {
        FUP_W { w: self }
    }
    #[doc = "Bit 4 - Forward Error Packets When this bit is reset, the Rx queue drops packets with error status (CRC error, Mll_ER, watchdog timeout, or overflow)."]
    #[inline(always)]
    pub fn fep(&mut self) -> FEP_W {
        FEP_W { w: self }
    }
    #[doc = "Bit 5 - Receive Queue Store and Forward When this bit is set, the ethernet block on this chip reads a packet from the Rx queue only after the complete packet has been written to it, ignoring the RTC field of this register."]
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W {
        RSF_W { w: self }
    }
    #[doc = "Bit 6 - Disable Dropping of TCP/IP Checksum Error Packets When this bit is set, the MAC does not drop the packets which only have the errors detected by the Receive Checksum Offload engine."]
    #[inline(always)]
    pub fn dis_tcp_ef(&mut self) -> DIS_TCP_EF_W {
        DIS_TCP_EF_W { w: self }
    }
    #[doc = "Bits 20:22 - This field indicates the size of the allocated Receive queues in blocks of 256 bytes."]
    #[inline(always)]
    pub fn rqs(&mut self) -> RQS_W {
        RQS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MTL RxQx Operation Mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_rxqx_op_mode](index.html) module"]
pub struct MTL_RXQX_OP_MODE_SPEC;
impl crate::RegisterSpec for MTL_RXQX_OP_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtl_rxqx_op_mode::R](R) reader structure"]
impl crate::Readable for MTL_RXQX_OP_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtl_rxqx_op_mode::W](W) writer structure"]
impl crate::Writable for MTL_RXQX_OP_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTL_RXQx_OP_MODE to value 0"]
impl crate::Resettable for MTL_RXQX_OP_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
