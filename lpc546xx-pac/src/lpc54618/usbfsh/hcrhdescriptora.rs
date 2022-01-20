#[doc = "Register `HCRHDESCRIPTORA` reader"]
pub struct R(crate::R<HCRHDESCRIPTORA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCRHDESCRIPTORA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCRHDESCRIPTORA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCRHDESCRIPTORA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCRHDESCRIPTORA` writer"]
pub struct W(crate::W<HCRHDESCRIPTORA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCRHDESCRIPTORA_SPEC>;
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
impl From<crate::W<HCRHDESCRIPTORA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCRHDESCRIPTORA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NDP` reader - NumberDownstreamPorts These bits specify the number of downstream ports supported by the root hub."]
pub struct NDP_R(crate::FieldReader<u8, u8>);
impl NDP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NDP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NDP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NDP` writer - NumberDownstreamPorts These bits specify the number of downstream ports supported by the root hub."]
pub struct NDP_W<'a> {
    w: &'a mut W,
}
impl<'a> NDP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `PSM` reader - PowerSwitchingMode This bit is used to specify how the power switching of the root hub ports is controlled."]
pub struct PSM_R(crate::FieldReader<bool, bool>);
impl PSM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSM` writer - PowerSwitchingMode This bit is used to specify how the power switching of the root hub ports is controlled."]
pub struct PSM_W<'a> {
    w: &'a mut W,
}
impl<'a> PSM_W<'a> {
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
#[doc = "Field `NPS` reader - NoPowerSwitching These bits are used to specify whether power switching is supported or port are always powered."]
pub struct NPS_R(crate::FieldReader<bool, bool>);
impl NPS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NPS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NPS` writer - NoPowerSwitching These bits are used to specify whether power switching is supported or port are always powered."]
pub struct NPS_W<'a> {
    w: &'a mut W,
}
impl<'a> NPS_W<'a> {
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
#[doc = "Field `DT` reader - DeviceType This bit specifies that the root hub is not a compound device."]
pub struct DT_R(crate::FieldReader<bool, bool>);
impl DT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT` writer - DeviceType This bit specifies that the root hub is not a compound device."]
pub struct DT_W<'a> {
    w: &'a mut W,
}
impl<'a> DT_W<'a> {
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
#[doc = "Field `OCPM` reader - OverCurrentProtectionMode This bit describes how the overcurrent status for the root hub ports are reported."]
pub struct OCPM_R(crate::FieldReader<bool, bool>);
impl OCPM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OCPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCPM` writer - OverCurrentProtectionMode This bit describes how the overcurrent status for the root hub ports are reported."]
pub struct OCPM_W<'a> {
    w: &'a mut W,
}
impl<'a> OCPM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `NOCP` reader - NoOverCurrentProtection This bit describes how the overcurrent status for the root hub ports are reported."]
pub struct NOCP_R(crate::FieldReader<bool, bool>);
impl NOCP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NOCP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOCP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOCP` writer - NoOverCurrentProtection This bit describes how the overcurrent status for the root hub ports are reported."]
pub struct NOCP_W<'a> {
    w: &'a mut W,
}
impl<'a> NOCP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `POTPGT` reader - PowerOnToPowerGoodTime This byte specifies the duration the HCD has to wait before accessing a powered-on port of the root hub."]
pub struct POTPGT_R(crate::FieldReader<u8, u8>);
impl POTPGT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        POTPGT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POTPGT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POTPGT` writer - PowerOnToPowerGoodTime This byte specifies the duration the HCD has to wait before accessing a powered-on port of the root hub."]
pub struct POTPGT_W<'a> {
    w: &'a mut W,
}
impl<'a> POTPGT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - NumberDownstreamPorts These bits specify the number of downstream ports supported by the root hub."]
    #[inline(always)]
    pub fn ndp(&self) -> NDP_R {
        NDP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PowerSwitchingMode This bit is used to specify how the power switching of the root hub ports is controlled."]
    #[inline(always)]
    pub fn psm(&self) -> PSM_R {
        PSM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - NoPowerSwitching These bits are used to specify whether power switching is supported or port are always powered."]
    #[inline(always)]
    pub fn nps(&self) -> NPS_R {
        NPS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DeviceType This bit specifies that the root hub is not a compound device."]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - OverCurrentProtectionMode This bit describes how the overcurrent status for the root hub ports are reported."]
    #[inline(always)]
    pub fn ocpm(&self) -> OCPM_R {
        OCPM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - NoOverCurrentProtection This bit describes how the overcurrent status for the root hub ports are reported."]
    #[inline(always)]
    pub fn nocp(&self) -> NOCP_R {
        NOCP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - PowerOnToPowerGoodTime This byte specifies the duration the HCD has to wait before accessing a powered-on port of the root hub."]
    #[inline(always)]
    pub fn potpgt(&self) -> POTPGT_R {
        POTPGT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NumberDownstreamPorts These bits specify the number of downstream ports supported by the root hub."]
    #[inline(always)]
    pub fn ndp(&mut self) -> NDP_W {
        NDP_W { w: self }
    }
    #[doc = "Bit 8 - PowerSwitchingMode This bit is used to specify how the power switching of the root hub ports is controlled."]
    #[inline(always)]
    pub fn psm(&mut self) -> PSM_W {
        PSM_W { w: self }
    }
    #[doc = "Bit 9 - NoPowerSwitching These bits are used to specify whether power switching is supported or port are always powered."]
    #[inline(always)]
    pub fn nps(&mut self) -> NPS_W {
        NPS_W { w: self }
    }
    #[doc = "Bit 10 - DeviceType This bit specifies that the root hub is not a compound device."]
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W {
        DT_W { w: self }
    }
    #[doc = "Bit 11 - OverCurrentProtectionMode This bit describes how the overcurrent status for the root hub ports are reported."]
    #[inline(always)]
    pub fn ocpm(&mut self) -> OCPM_W {
        OCPM_W { w: self }
    }
    #[doc = "Bit 12 - NoOverCurrentProtection This bit describes how the overcurrent status for the root hub ports are reported."]
    #[inline(always)]
    pub fn nocp(&mut self) -> NOCP_W {
        NOCP_W { w: self }
    }
    #[doc = "Bits 24:31 - PowerOnToPowerGoodTime This byte specifies the duration the HCD has to wait before accessing a powered-on port of the root hub."]
    #[inline(always)]
    pub fn potpgt(&mut self) -> POTPGT_W {
        POTPGT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "First of the two registers which describes the characteristics of the root hub\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcrhdescriptora](index.html) module"]
pub struct HCRHDESCRIPTORA_SPEC;
impl crate::RegisterSpec for HCRHDESCRIPTORA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcrhdescriptora::R](R) reader structure"]
impl crate::Readable for HCRHDESCRIPTORA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcrhdescriptora::W](W) writer structure"]
impl crate::Writable for HCRHDESCRIPTORA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCRHDESCRIPTORA to value 0xff00_0902"]
impl crate::Resettable for HCRHDESCRIPTORA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff00_0902
    }
}
