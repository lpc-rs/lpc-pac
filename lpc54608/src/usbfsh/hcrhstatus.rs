#[doc = "Register `HCRHSTATUS` reader"]
pub struct R(crate::R<HCRHSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCRHSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HCRHSTATUS_SPEC>> for R {
    fn from(reader: crate::R<HCRHSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCRHSTATUS` writer"]
pub struct W(crate::W<HCRHSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCRHSTATUS_SPEC>;
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
impl core::convert::From<crate::W<HCRHSTATUS_SPEC>> for W {
    fn from(writer: crate::W<HCRHSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPS` reader - (read) LocalPowerStatus The Root Hub does not support the local power status feature; thus, this bit is always read as 0."]
pub struct LPS_R(crate::FieldReader<bool, bool>);
impl LPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPS` writer - (read) LocalPowerStatus The Root Hub does not support the local power status feature; thus, this bit is always read as 0."]
pub struct LPS_W<'a> {
    w: &'a mut W,
}
impl<'a> LPS_W<'a> {
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
#[doc = "Field `OCI` reader - OverCurrentIndicator This bit reports overcurrent conditions when the global reporting is implemented."]
pub struct OCI_R(crate::FieldReader<bool, bool>);
impl OCI_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCI` writer - OverCurrentIndicator This bit reports overcurrent conditions when the global reporting is implemented."]
pub struct OCI_W<'a> {
    w: &'a mut W,
}
impl<'a> OCI_W<'a> {
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
#[doc = "Field `DRWE` reader - (read) DeviceRemoteWakeupEnable This bit enables a ConnectStatusChange bit as a resume event, causing a USBSUSPEND to USBRESUME state transition and setting the ResumeDetected interrupt."]
pub struct DRWE_R(crate::FieldReader<bool, bool>);
impl DRWE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DRWE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRWE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRWE` writer - (read) DeviceRemoteWakeupEnable This bit enables a ConnectStatusChange bit as a resume event, causing a USBSUSPEND to USBRESUME state transition and setting the ResumeDetected interrupt."]
pub struct DRWE_W<'a> {
    w: &'a mut W,
}
impl<'a> DRWE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `LPSC` reader - (read) LocalPowerStatusChange The root hub does not support the local power status feature."]
pub struct LPSC_R(crate::FieldReader<bool, bool>);
impl LPSC_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPSC` writer - (read) LocalPowerStatusChange The root hub does not support the local power status feature."]
pub struct LPSC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `OCIC` reader - OverCurrentIndicatorChange This bit is set by hardware when a change has occurred to the OCI field of this register."]
pub struct OCIC_R(crate::FieldReader<bool, bool>);
impl OCIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCIC` writer - OverCurrentIndicatorChange This bit is set by hardware when a change has occurred to the OCI field of this register."]
pub struct OCIC_W<'a> {
    w: &'a mut W,
}
impl<'a> OCIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `CRWE` reader - (write) ClearRemoteWakeupEnable Writing a 1 clears DeviceRemoveWakeupEnable."]
pub struct CRWE_R(crate::FieldReader<bool, bool>);
impl CRWE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRWE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRWE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRWE` writer - (write) ClearRemoteWakeupEnable Writing a 1 clears DeviceRemoveWakeupEnable."]
pub struct CRWE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRWE_W<'a> {
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
    #[doc = "Bit 0 - (read) LocalPowerStatus The Root Hub does not support the local power status feature; thus, this bit is always read as 0."]
    #[inline(always)]
    pub fn lps(&self) -> LPS_R {
        LPS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - OverCurrentIndicator This bit reports overcurrent conditions when the global reporting is implemented."]
    #[inline(always)]
    pub fn oci(&self) -> OCI_R {
        OCI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 15 - (read) DeviceRemoteWakeupEnable This bit enables a ConnectStatusChange bit as a resume event, causing a USBSUSPEND to USBRESUME state transition and setting the ResumeDetected interrupt."]
    #[inline(always)]
    pub fn drwe(&self) -> DRWE_R {
        DRWE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - (read) LocalPowerStatusChange The root hub does not support the local power status feature."]
    #[inline(always)]
    pub fn lpsc(&self) -> LPSC_R {
        LPSC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - OverCurrentIndicatorChange This bit is set by hardware when a change has occurred to the OCI field of this register."]
    #[inline(always)]
    pub fn ocic(&self) -> OCIC_R {
        OCIC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 31 - (write) ClearRemoteWakeupEnable Writing a 1 clears DeviceRemoveWakeupEnable."]
    #[inline(always)]
    pub fn crwe(&self) -> CRWE_R {
        CRWE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - (read) LocalPowerStatus The Root Hub does not support the local power status feature; thus, this bit is always read as 0."]
    #[inline(always)]
    pub fn lps(&mut self) -> LPS_W {
        LPS_W { w: self }
    }
    #[doc = "Bit 1 - OverCurrentIndicator This bit reports overcurrent conditions when the global reporting is implemented."]
    #[inline(always)]
    pub fn oci(&mut self) -> OCI_W {
        OCI_W { w: self }
    }
    #[doc = "Bit 15 - (read) DeviceRemoteWakeupEnable This bit enables a ConnectStatusChange bit as a resume event, causing a USBSUSPEND to USBRESUME state transition and setting the ResumeDetected interrupt."]
    #[inline(always)]
    pub fn drwe(&mut self) -> DRWE_W {
        DRWE_W { w: self }
    }
    #[doc = "Bit 16 - (read) LocalPowerStatusChange The root hub does not support the local power status feature."]
    #[inline(always)]
    pub fn lpsc(&mut self) -> LPSC_W {
        LPSC_W { w: self }
    }
    #[doc = "Bit 17 - OverCurrentIndicatorChange This bit is set by hardware when a change has occurred to the OCI field of this register."]
    #[inline(always)]
    pub fn ocic(&mut self) -> OCIC_W {
        OCIC_W { w: self }
    }
    #[doc = "Bit 31 - (write) ClearRemoteWakeupEnable Writing a 1 clears DeviceRemoveWakeupEnable."]
    #[inline(always)]
    pub fn crwe(&mut self) -> CRWE_W {
        CRWE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is divided into two parts\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcrhstatus](index.html) module"]
pub struct HCRHSTATUS_SPEC;
impl crate::RegisterSpec for HCRHSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcrhstatus::R](R) reader structure"]
impl crate::Readable for HCRHSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcrhstatus::W](W) writer structure"]
impl crate::Writable for HCRHSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCRHSTATUS to value 0"]
impl crate::Resettable for HCRHSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
