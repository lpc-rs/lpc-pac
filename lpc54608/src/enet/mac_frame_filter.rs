#[doc = "Register `MAC_FRAME_FILTER` reader"]
pub struct R(crate::R<MAC_FRAME_FILTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_FRAME_FILTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_FRAME_FILTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_FRAME_FILTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_FRAME_FILTER` writer"]
pub struct W(crate::W<MAC_FRAME_FILTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_FRAME_FILTER_SPEC>;
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
impl From<crate::W<MAC_FRAME_FILTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_FRAME_FILTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PR` reader - Promiscuous Mode When this bit is set, the Address Filter module passes all incoming frames regardless of its destination or source address."]
pub struct PR_R(crate::FieldReader<bool, bool>);
impl PR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR` writer - Promiscuous Mode When this bit is set, the Address Filter module passes all incoming frames regardless of its destination or source address."]
pub struct PR_W<'a> {
    w: &'a mut W,
}
impl<'a> PR_W<'a> {
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
#[doc = "Field `DAIF` reader - DA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast frames."]
pub struct DAIF_R(crate::FieldReader<bool, bool>);
impl DAIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAIF` writer - DA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast frames."]
pub struct DAIF_W<'a> {
    w: &'a mut W,
}
impl<'a> DAIF_W<'a> {
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
#[doc = "Field `PM` reader - Pass All Multicast When set, this bit indicates that all received frames with a multicast destination address (first bit in the destination address field is '1') are passed."]
pub struct PM_R(crate::FieldReader<bool, bool>);
impl PM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PM` writer - Pass All Multicast When set, this bit indicates that all received frames with a multicast destination address (first bit in the destination address field is '1') are passed."]
pub struct PM_W<'a> {
    w: &'a mut W,
}
impl<'a> PM_W<'a> {
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
#[doc = "Field `DBF` reader - Disable Broadcast Frames When this bit is set, the AFM module filters all incoming broadcast frames."]
pub struct DBF_R(crate::FieldReader<bool, bool>);
impl DBF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBF` writer - Disable Broadcast Frames When this bit is set, the AFM module filters all incoming broadcast frames."]
pub struct DBF_W<'a> {
    w: &'a mut W,
}
impl<'a> DBF_W<'a> {
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
#[doc = "Field `PCF` reader - Pass Control Frames These bits control the forwarding of all control frames (including unicast and multicast PAUSE frames)."]
pub struct PCF_R(crate::FieldReader<u8, u8>);
impl PCF_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCF` writer - Pass Control Frames These bits control the forwarding of all control frames (including unicast and multicast PAUSE frames)."]
pub struct PCF_W<'a> {
    w: &'a mut W,
}
impl<'a> PCF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `SAIF` reader - SA Inverse Filtering When this bit is set, the Address Check block operates in the inverse filtering mode for SA address comparison."]
pub struct SAIF_R(crate::FieldReader<bool, bool>);
impl SAIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAF` reader - Source Address Filter Enable When this bit is set, the MAC compares the SA field of the received packets with the values programmed in the enabled SA registers."]
pub struct SAF_R(crate::FieldReader<bool, bool>);
impl SAF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RA` reader - Receive all When this bit is set, the MAC Receiver module passes to the Application all frames received irrespective of whether they pass the address filter."]
pub struct RA_R(crate::FieldReader<bool, bool>);
impl RA_R {
    pub(crate) fn new(bits: bool) -> Self {
        RA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RA` writer - Receive all When this bit is set, the MAC Receiver module passes to the Application all frames received irrespective of whether they pass the address filter."]
pub struct RA_W<'a> {
    w: &'a mut W,
}
impl<'a> RA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Promiscuous Mode When this bit is set, the Address Filter module passes all incoming frames regardless of its destination or source address."]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - DA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast frames."]
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pass All Multicast When set, this bit indicates that all received frames with a multicast destination address (first bit in the destination address field is '1') are passed."]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Disable Broadcast Frames When this bit is set, the AFM module filters all incoming broadcast frames."]
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Pass Control Frames These bits control the forwarding of all control frames (including unicast and multicast PAUSE frames)."]
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - SA Inverse Filtering When this bit is set, the Address Check block operates in the inverse filtering mode for SA address comparison."]
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Source Address Filter Enable When this bit is set, the MAC compares the SA field of the received packets with the values programmed in the enabled SA registers."]
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Receive all When this bit is set, the MAC Receiver module passes to the Application all frames received irrespective of whether they pass the address filter."]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Promiscuous Mode When this bit is set, the Address Filter module passes all incoming frames regardless of its destination or source address."]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W {
        PR_W { w: self }
    }
    #[doc = "Bit 3 - DA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast frames."]
    #[inline(always)]
    pub fn daif(&mut self) -> DAIF_W {
        DAIF_W { w: self }
    }
    #[doc = "Bit 4 - Pass All Multicast When set, this bit indicates that all received frames with a multicast destination address (first bit in the destination address field is '1') are passed."]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W {
        PM_W { w: self }
    }
    #[doc = "Bit 5 - Disable Broadcast Frames When this bit is set, the AFM module filters all incoming broadcast frames."]
    #[inline(always)]
    pub fn dbf(&mut self) -> DBF_W {
        DBF_W { w: self }
    }
    #[doc = "Bits 6:7 - Pass Control Frames These bits control the forwarding of all control frames (including unicast and multicast PAUSE frames)."]
    #[inline(always)]
    pub fn pcf(&mut self) -> PCF_W {
        PCF_W { w: self }
    }
    #[doc = "Bit 31 - Receive all When this bit is set, the MAC Receiver module passes to the Application all frames received irrespective of whether they pass the address filter."]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W {
        RA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC frame filter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_frame_filter](index.html) module"]
pub struct MAC_FRAME_FILTER_SPEC;
impl crate::RegisterSpec for MAC_FRAME_FILTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_frame_filter::R](R) reader structure"]
impl crate::Readable for MAC_FRAME_FILTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_frame_filter::W](W) writer structure"]
impl crate::Writable for MAC_FRAME_FILTER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_FRAME_FILTER to value 0"]
impl crate::Resettable for MAC_FRAME_FILTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
