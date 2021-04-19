#[doc = "Register `HCCONTROL` reader"]
pub struct R(crate::R<HCCONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCCONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HCCONTROL_SPEC>> for R {
    fn from(reader: crate::R<HCCONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCCONTROL` writer"]
pub struct W(crate::W<HCCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCCONTROL_SPEC>;
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
impl core::convert::From<crate::W<HCCONTROL_SPEC>> for W {
    fn from(writer: crate::W<HCCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CBSR` reader - ControlBulkServiceRatio."]
pub struct CBSR_R(crate::FieldReader<u8, u8>);
impl CBSR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CBSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBSR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBSR` writer - ControlBulkServiceRatio."]
pub struct CBSR_W<'a> {
    w: &'a mut W,
}
impl<'a> CBSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `PLE` reader - PeriodicListEnable."]
pub struct PLE_R(crate::FieldReader<bool, bool>);
impl PLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLE` writer - PeriodicListEnable."]
pub struct PLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `IE` reader - IsochronousEnable."]
pub struct IE_R(crate::FieldReader<bool, bool>);
impl IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IE` writer - IsochronousEnable."]
pub struct IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_W<'a> {
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
#[doc = "Field `CLE` reader - ControlListEnable."]
pub struct CLE_R(crate::FieldReader<bool, bool>);
impl CLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLE` writer - ControlListEnable."]
pub struct CLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLE_W<'a> {
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
#[doc = "Field `BLE` reader - BulkListEnable This bit is set to enable the processing of the Bulk list in the next Frame."]
pub struct BLE_R(crate::FieldReader<bool, bool>);
impl BLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLE` writer - BulkListEnable This bit is set to enable the processing of the Bulk list in the next Frame."]
pub struct BLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_W<'a> {
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
#[doc = "Field `HCFS` reader - HostControllerFunctionalState for USB 00b: USBRESET 01b: USBRESUME 10b: USBOPERATIONAL 11b: USBSUSPEND A transition to USBOPERATIONAL from another state causes SOFgeneration to begin 1 ms later."]
pub struct HCFS_R(crate::FieldReader<u8, u8>);
impl HCFS_R {
    pub(crate) fn new(bits: u8) -> Self {
        HCFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HCFS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCFS` writer - HostControllerFunctionalState for USB 00b: USBRESET 01b: USBRESUME 10b: USBOPERATIONAL 11b: USBSUSPEND A transition to USBOPERATIONAL from another state causes SOFgeneration to begin 1 ms later."]
pub struct HCFS_W<'a> {
    w: &'a mut W,
}
impl<'a> HCFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `IR` reader - InterruptRouting This bit determines the routing of interrupts generated by events registered in HcInterruptStatus."]
pub struct IR_R(crate::FieldReader<bool, bool>);
impl IR_R {
    pub(crate) fn new(bits: bool) -> Self {
        IR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IR` writer - InterruptRouting This bit determines the routing of interrupts generated by events registered in HcInterruptStatus."]
pub struct IR_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_W<'a> {
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
#[doc = "Field `RWC` reader - RemoteWakeupConnected This bit indicates whether HC supports remote wake-up signaling."]
pub struct RWC_R(crate::FieldReader<bool, bool>);
impl RWC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWC` writer - RemoteWakeupConnected This bit indicates whether HC supports remote wake-up signaling."]
pub struct RWC_W<'a> {
    w: &'a mut W,
}
impl<'a> RWC_W<'a> {
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
#[doc = "Field `RWE` reader - RemoteWakeupEnable This bit is used by HCD to enable or disable the remote wake-up feature upon the detection of upstream resume signaling."]
pub struct RWE_R(crate::FieldReader<bool, bool>);
impl RWE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWE` writer - RemoteWakeupEnable This bit is used by HCD to enable or disable the remote wake-up feature upon the detection of upstream resume signaling."]
pub struct RWE_W<'a> {
    w: &'a mut W,
}
impl<'a> RWE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ControlBulkServiceRatio."]
    #[inline(always)]
    pub fn cbsr(&self) -> CBSR_R {
        CBSR_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - PeriodicListEnable."]
    #[inline(always)]
    pub fn ple(&self) -> PLE_R {
        PLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IsochronousEnable."]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ControlListEnable."]
    #[inline(always)]
    pub fn cle(&self) -> CLE_R {
        CLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - BulkListEnable This bit is set to enable the processing of the Bulk list in the next Frame."]
    #[inline(always)]
    pub fn ble(&self) -> BLE_R {
        BLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - HostControllerFunctionalState for USB 00b: USBRESET 01b: USBRESUME 10b: USBOPERATIONAL 11b: USBSUSPEND A transition to USBOPERATIONAL from another state causes SOFgeneration to begin 1 ms later."]
    #[inline(always)]
    pub fn hcfs(&self) -> HCFS_R {
        HCFS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - InterruptRouting This bit determines the routing of interrupts generated by events registered in HcInterruptStatus."]
    #[inline(always)]
    pub fn ir(&self) -> IR_R {
        IR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RemoteWakeupConnected This bit indicates whether HC supports remote wake-up signaling."]
    #[inline(always)]
    pub fn rwc(&self) -> RWC_R {
        RWC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RemoteWakeupEnable This bit is used by HCD to enable or disable the remote wake-up feature upon the detection of upstream resume signaling."]
    #[inline(always)]
    pub fn rwe(&self) -> RWE_R {
        RWE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ControlBulkServiceRatio."]
    #[inline(always)]
    pub fn cbsr(&mut self) -> CBSR_W {
        CBSR_W { w: self }
    }
    #[doc = "Bit 2 - PeriodicListEnable."]
    #[inline(always)]
    pub fn ple(&mut self) -> PLE_W {
        PLE_W { w: self }
    }
    #[doc = "Bit 3 - IsochronousEnable."]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    #[doc = "Bit 4 - ControlListEnable."]
    #[inline(always)]
    pub fn cle(&mut self) -> CLE_W {
        CLE_W { w: self }
    }
    #[doc = "Bit 5 - BulkListEnable This bit is set to enable the processing of the Bulk list in the next Frame."]
    #[inline(always)]
    pub fn ble(&mut self) -> BLE_W {
        BLE_W { w: self }
    }
    #[doc = "Bits 6:7 - HostControllerFunctionalState for USB 00b: USBRESET 01b: USBRESUME 10b: USBOPERATIONAL 11b: USBSUSPEND A transition to USBOPERATIONAL from another state causes SOFgeneration to begin 1 ms later."]
    #[inline(always)]
    pub fn hcfs(&mut self) -> HCFS_W {
        HCFS_W { w: self }
    }
    #[doc = "Bit 8 - InterruptRouting This bit determines the routing of interrupts generated by events registered in HcInterruptStatus."]
    #[inline(always)]
    pub fn ir(&mut self) -> IR_W {
        IR_W { w: self }
    }
    #[doc = "Bit 9 - RemoteWakeupConnected This bit indicates whether HC supports remote wake-up signaling."]
    #[inline(always)]
    pub fn rwc(&mut self) -> RWC_W {
        RWC_W { w: self }
    }
    #[doc = "Bit 10 - RemoteWakeupEnable This bit is used by HCD to enable or disable the remote wake-up feature upon the detection of upstream resume signaling."]
    #[inline(always)]
    pub fn rwe(&mut self) -> RWE_W {
        RWE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Defines the operating modes of the HC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hccontrol](index.html) module"]
pub struct HCCONTROL_SPEC;
impl crate::RegisterSpec for HCCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hccontrol::R](R) reader structure"]
impl crate::Readable for HCCONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hccontrol::W](W) writer structure"]
impl crate::Writable for HCCONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCCONTROL to value 0"]
impl crate::Resettable for HCCONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
