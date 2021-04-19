#[doc = "Register `MAC_RXQ_CTRL1` reader"]
pub struct R(crate::R<MAC_RXQ_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_RXQ_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MAC_RXQ_CTRL1_SPEC>> for R {
    fn from(reader: crate::R<MAC_RXQ_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_RXQ_CTRL1` writer"]
pub struct W(crate::W<MAC_RXQ_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_RXQ_CTRL1_SPEC>;
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
impl core::convert::From<crate::W<MAC_RXQ_CTRL1_SPEC>> for W {
    fn from(writer: crate::W<MAC_RXQ_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AVCPQ` reader - AV Untagged Control Packets Queue."]
pub struct AVCPQ_R(crate::FieldReader<u8, u8>);
impl AVCPQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        AVCPQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVCPQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVCPQ` writer - AV Untagged Control Packets Queue."]
pub struct AVCPQ_W<'a> {
    w: &'a mut W,
}
impl<'a> AVCPQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `AVPTPQ` reader - AV PTP Packets Queue."]
pub struct AVPTPQ_R(crate::FieldReader<u8, u8>);
impl AVPTPQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        AVPTPQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVPTPQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVPTPQ` writer - AV PTP Packets Queue."]
pub struct AVPTPQ_W<'a> {
    w: &'a mut W,
}
impl<'a> AVPTPQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `UPQ` reader - Untagged Packet Queue."]
pub struct UPQ_R(crate::FieldReader<u8, u8>);
impl UPQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        UPQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPQ` writer - Untagged Packet Queue."]
pub struct UPQ_W<'a> {
    w: &'a mut W,
}
impl<'a> UPQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `MCBCQ` reader - Multicast and Broadcast Queue."]
pub struct MCBCQ_R(crate::FieldReader<u8, u8>);
impl MCBCQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        MCBCQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCBCQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCBCQ` writer - Multicast and Broadcast Queue."]
pub struct MCBCQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MCBCQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `MCBCQEN` reader - Multicast and Broadcast Queue Enable."]
pub struct MCBCQEN_R(crate::FieldReader<bool, bool>);
impl MCBCQEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCBCQEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCBCQEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCBCQEN` writer - Multicast and Broadcast Queue Enable."]
pub struct MCBCQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCBCQEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - AV Untagged Control Packets Queue."]
    #[inline(always)]
    pub fn avcpq(&self) -> AVCPQ_R {
        AVCPQ_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - AV PTP Packets Queue."]
    #[inline(always)]
    pub fn avptpq(&self) -> AVPTPQ_R {
        AVPTPQ_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Untagged Packet Queue."]
    #[inline(always)]
    pub fn upq(&self) -> UPQ_R {
        UPQ_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Multicast and Broadcast Queue."]
    #[inline(always)]
    pub fn mcbcq(&self) -> MCBCQ_R {
        MCBCQ_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 20 - Multicast and Broadcast Queue Enable."]
    #[inline(always)]
    pub fn mcbcqen(&self) -> MCBCQEN_R {
        MCBCQEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - AV Untagged Control Packets Queue."]
    #[inline(always)]
    pub fn avcpq(&mut self) -> AVCPQ_W {
        AVCPQ_W { w: self }
    }
    #[doc = "Bits 4:6 - AV PTP Packets Queue."]
    #[inline(always)]
    pub fn avptpq(&mut self) -> AVPTPQ_W {
        AVPTPQ_W { w: self }
    }
    #[doc = "Bits 12:14 - Untagged Packet Queue."]
    #[inline(always)]
    pub fn upq(&mut self) -> UPQ_W {
        UPQ_W { w: self }
    }
    #[doc = "Bits 16:18 - Multicast and Broadcast Queue."]
    #[inline(always)]
    pub fn mcbcq(&mut self) -> MCBCQ_W {
        MCBCQ_W { w: self }
    }
    #[doc = "Bit 20 - Multicast and Broadcast Queue Enable."]
    #[inline(always)]
    pub fn mcbcqen(&mut self) -> MCBCQEN_W {
        MCBCQEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Queue Control 0 register 0x0000\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_rxq_ctrl1](index.html) module"]
pub struct MAC_RXQ_CTRL1_SPEC;
impl crate::RegisterSpec for MAC_RXQ_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_rxq_ctrl1::R](R) reader structure"]
impl crate::Readable for MAC_RXQ_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_rxq_ctrl1::W](W) writer structure"]
impl crate::Writable for MAC_RXQ_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_RXQ_CTRL1 to value 0"]
impl crate::Resettable for MAC_RXQ_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
