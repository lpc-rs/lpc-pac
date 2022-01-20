#[doc = "Register `HCRHPORTSTATUS` reader"]
pub struct R(crate::R<HCRHPORTSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCRHPORTSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCRHPORTSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCRHPORTSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCRHPORTSTATUS` writer"]
pub struct W(crate::W<HCRHPORTSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCRHPORTSTATUS_SPEC>;
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
impl From<crate::W<HCRHPORTSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCRHPORTSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCS` reader - (read) CurrentConnectStatus This bit reflects the current state of the downstream port."]
pub struct CCS_R(crate::FieldReader<bool, bool>);
impl CCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCS` writer - (read) CurrentConnectStatus This bit reflects the current state of the downstream port."]
pub struct CCS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCS_W<'a> {
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
#[doc = "Field `PES` reader - (read) PortEnableStatus This bit indicates whether the port is enabled or disabled."]
pub struct PES_R(crate::FieldReader<bool, bool>);
impl PES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PES` writer - (read) PortEnableStatus This bit indicates whether the port is enabled or disabled."]
pub struct PES_W<'a> {
    w: &'a mut W,
}
impl<'a> PES_W<'a> {
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
#[doc = "Field `PSS` reader - (read) PortSuspendStatus This bit indicates the port is suspended or in the resume sequence."]
pub struct PSS_R(crate::FieldReader<bool, bool>);
impl PSS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSS` writer - (read) PortSuspendStatus This bit indicates the port is suspended or in the resume sequence."]
pub struct PSS_W<'a> {
    w: &'a mut W,
}
impl<'a> PSS_W<'a> {
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
#[doc = "Field `POCI` reader - (read) PortOverCurrentIndicator This bit is only valid when the Root Hub is configured in such a way that overcurrent conditions are reported on a per-port basis."]
pub struct POCI_R(crate::FieldReader<bool, bool>);
impl POCI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POCI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POCI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POCI` writer - (read) PortOverCurrentIndicator This bit is only valid when the Root Hub is configured in such a way that overcurrent conditions are reported on a per-port basis."]
pub struct POCI_W<'a> {
    w: &'a mut W,
}
impl<'a> POCI_W<'a> {
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
#[doc = "Field `PRS` reader - (read) PortResetStatus When this bit is set by a write to SetPortReset, port reset signaling is asserted."]
pub struct PRS_R(crate::FieldReader<bool, bool>);
impl PRS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRS` writer - (read) PortResetStatus When this bit is set by a write to SetPortReset, port reset signaling is asserted."]
pub struct PRS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRS_W<'a> {
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
#[doc = "Field `PPS` reader - (read) PortPowerStatus This bit reflects the porta's power status, regardless of the type of power switching implemented."]
pub struct PPS_R(crate::FieldReader<bool, bool>);
impl PPS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPS` writer - (read) PortPowerStatus This bit reflects the porta's power status, regardless of the type of power switching implemented."]
pub struct PPS_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS_W<'a> {
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
#[doc = "Field `LSDA` reader - (read) LowSpeedDeviceAttached This bit indicates the speed of the device attached to this port."]
pub struct LSDA_R(crate::FieldReader<bool, bool>);
impl LSDA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSDA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSDA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSDA` writer - (read) LowSpeedDeviceAttached This bit indicates the speed of the device attached to this port."]
pub struct LSDA_W<'a> {
    w: &'a mut W,
}
impl<'a> LSDA_W<'a> {
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
#[doc = "Field `CSC` reader - ConnectStatusChange This bit is set whenever a connect or disconnect event occurs."]
pub struct CSC_R(crate::FieldReader<bool, bool>);
impl CSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSC` writer - ConnectStatusChange This bit is set whenever a connect or disconnect event occurs."]
pub struct CSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CSC_W<'a> {
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
#[doc = "Field `PESC` reader - PortEnableStatusChange This bit is set when hardware events cause the PortEnableStatus bit to be cleared."]
pub struct PESC_R(crate::FieldReader<bool, bool>);
impl PESC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PESC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PESC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PESC` writer - PortEnableStatusChange This bit is set when hardware events cause the PortEnableStatus bit to be cleared."]
pub struct PESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PESC_W<'a> {
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
#[doc = "Field `PSSC` reader - PortSuspendStatusChange This bit is set when the full resume sequence is completed."]
pub struct PSSC_R(crate::FieldReader<bool, bool>);
impl PSSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PSSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSSC` writer - PortSuspendStatusChange This bit is set when the full resume sequence is completed."]
pub struct PSSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PSSC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `OCIC` reader - PortOverCurrentIndicatorChange This bit is valid only if overcurrent conditions are reported on a per-port basis."]
pub struct OCIC_R(crate::FieldReader<bool, bool>);
impl OCIC_R {
    #[inline(always)]
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
#[doc = "Field `OCIC` writer - PortOverCurrentIndicatorChange This bit is valid only if overcurrent conditions are reported on a per-port basis."]
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `PRSC` reader - PortResetStatusChange This bit is set at the end of the 10 ms port reset signal."]
pub struct PRSC_R(crate::FieldReader<bool, bool>);
impl PRSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRSC` writer - PortResetStatusChange This bit is set at the end of the 10 ms port reset signal."]
pub struct PRSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSC_W<'a> {
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
    #[doc = "Bit 0 - (read) CurrentConnectStatus This bit reflects the current state of the downstream port."]
    #[inline(always)]
    pub fn ccs(&self) -> CCS_R {
        CCS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - (read) PortEnableStatus This bit indicates whether the port is enabled or disabled."]
    #[inline(always)]
    pub fn pes(&self) -> PES_R {
        PES_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - (read) PortSuspendStatus This bit indicates the port is suspended or in the resume sequence."]
    #[inline(always)]
    pub fn pss(&self) -> PSS_R {
        PSS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - (read) PortOverCurrentIndicator This bit is only valid when the Root Hub is configured in such a way that overcurrent conditions are reported on a per-port basis."]
    #[inline(always)]
    pub fn poci(&self) -> POCI_R {
        POCI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - (read) PortResetStatus When this bit is set by a write to SetPortReset, port reset signaling is asserted."]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - (read) PortPowerStatus This bit reflects the porta's power status, regardless of the type of power switching implemented."]
    #[inline(always)]
    pub fn pps(&self) -> PPS_R {
        PPS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - (read) LowSpeedDeviceAttached This bit indicates the speed of the device attached to this port."]
    #[inline(always)]
    pub fn lsda(&self) -> LSDA_R {
        LSDA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ConnectStatusChange This bit is set whenever a connect or disconnect event occurs."]
    #[inline(always)]
    pub fn csc(&self) -> CSC_R {
        CSC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PortEnableStatusChange This bit is set when hardware events cause the PortEnableStatus bit to be cleared."]
    #[inline(always)]
    pub fn pesc(&self) -> PESC_R {
        PESC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PortSuspendStatusChange This bit is set when the full resume sequence is completed."]
    #[inline(always)]
    pub fn pssc(&self) -> PSSC_R {
        PSSC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PortOverCurrentIndicatorChange This bit is valid only if overcurrent conditions are reported on a per-port basis."]
    #[inline(always)]
    pub fn ocic(&self) -> OCIC_R {
        OCIC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PortResetStatusChange This bit is set at the end of the 10 ms port reset signal."]
    #[inline(always)]
    pub fn prsc(&self) -> PRSC_R {
        PRSC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - (read) CurrentConnectStatus This bit reflects the current state of the downstream port."]
    #[inline(always)]
    pub fn ccs(&mut self) -> CCS_W {
        CCS_W { w: self }
    }
    #[doc = "Bit 1 - (read) PortEnableStatus This bit indicates whether the port is enabled or disabled."]
    #[inline(always)]
    pub fn pes(&mut self) -> PES_W {
        PES_W { w: self }
    }
    #[doc = "Bit 2 - (read) PortSuspendStatus This bit indicates the port is suspended or in the resume sequence."]
    #[inline(always)]
    pub fn pss(&mut self) -> PSS_W {
        PSS_W { w: self }
    }
    #[doc = "Bit 3 - (read) PortOverCurrentIndicator This bit is only valid when the Root Hub is configured in such a way that overcurrent conditions are reported on a per-port basis."]
    #[inline(always)]
    pub fn poci(&mut self) -> POCI_W {
        POCI_W { w: self }
    }
    #[doc = "Bit 4 - (read) PortResetStatus When this bit is set by a write to SetPortReset, port reset signaling is asserted."]
    #[inline(always)]
    pub fn prs(&mut self) -> PRS_W {
        PRS_W { w: self }
    }
    #[doc = "Bit 8 - (read) PortPowerStatus This bit reflects the porta's power status, regardless of the type of power switching implemented."]
    #[inline(always)]
    pub fn pps(&mut self) -> PPS_W {
        PPS_W { w: self }
    }
    #[doc = "Bit 9 - (read) LowSpeedDeviceAttached This bit indicates the speed of the device attached to this port."]
    #[inline(always)]
    pub fn lsda(&mut self) -> LSDA_W {
        LSDA_W { w: self }
    }
    #[doc = "Bit 16 - ConnectStatusChange This bit is set whenever a connect or disconnect event occurs."]
    #[inline(always)]
    pub fn csc(&mut self) -> CSC_W {
        CSC_W { w: self }
    }
    #[doc = "Bit 17 - PortEnableStatusChange This bit is set when hardware events cause the PortEnableStatus bit to be cleared."]
    #[inline(always)]
    pub fn pesc(&mut self) -> PESC_W {
        PESC_W { w: self }
    }
    #[doc = "Bit 18 - PortSuspendStatusChange This bit is set when the full resume sequence is completed."]
    #[inline(always)]
    pub fn pssc(&mut self) -> PSSC_W {
        PSSC_W { w: self }
    }
    #[doc = "Bit 19 - PortOverCurrentIndicatorChange This bit is valid only if overcurrent conditions are reported on a per-port basis."]
    #[inline(always)]
    pub fn ocic(&mut self) -> OCIC_W {
        OCIC_W { w: self }
    }
    #[doc = "Bit 20 - PortResetStatusChange This bit is set at the end of the 10 ms port reset signal."]
    #[inline(always)]
    pub fn prsc(&mut self) -> PRSC_W {
        PRSC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls and reports the port events on a per-port basis\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcrhportstatus](index.html) module"]
pub struct HCRHPORTSTATUS_SPEC;
impl crate::RegisterSpec for HCRHPORTSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcrhportstatus::R](R) reader structure"]
impl crate::Readable for HCRHPORTSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcrhportstatus::W](W) writer structure"]
impl crate::Writable for HCRHPORTSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCRHPORTSTATUS to value 0"]
impl crate::Resettable for HCRHPORTSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
