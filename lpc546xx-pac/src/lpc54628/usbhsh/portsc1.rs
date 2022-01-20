#[doc = "Register `PORTSC1` reader"]
pub struct R(crate::R<PORTSC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORTSC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORTSC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORTSC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORTSC1` writer"]
pub struct W(crate::W<PORTSC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORTSC1_SPEC>;
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
impl From<crate::W<PORTSC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORTSC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCS` reader - Current Connect Status: Logic 1 indicates a device is present on the port."]
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
#[doc = "Field `CCS` writer - Current Connect Status: Logic 1 indicates a device is present on the port."]
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
#[doc = "Field `CSC` reader - Connect Status Change: Logic 1 means that the value of CCS has changed."]
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
#[doc = "Field `CSC` writer - Connect Status Change: Logic 1 means that the value of CCS has changed."]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `PED` reader - Port Enabled/Disabled."]
pub struct PED_R(crate::FieldReader<bool, bool>);
impl PED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PED` writer - Port Enabled/Disabled."]
pub struct PED_W<'a> {
    w: &'a mut W,
}
impl<'a> PED_W<'a> {
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
#[doc = "Field `PEDC` reader - Port Enabled/Disabled Change: Logic 1 means that the value of PED has changed."]
pub struct PEDC_R(crate::FieldReader<bool, bool>);
impl PEDC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEDC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEDC` writer - Port Enabled/Disabled Change: Logic 1 means that the value of PED has changed."]
pub struct PEDC_W<'a> {
    w: &'a mut W,
}
impl<'a> PEDC_W<'a> {
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
#[doc = "Field `OCA` reader - Over-current active: Logic 1 means that this port has an over-current condition."]
pub struct OCA_R(crate::FieldReader<bool, bool>);
impl OCA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OCA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCA` writer - Over-current active: Logic 1 means that this port has an over-current condition."]
pub struct OCA_W<'a> {
    w: &'a mut W,
}
impl<'a> OCA_W<'a> {
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
#[doc = "Field `OCC` reader - Over-current change: Logic 1 means that the value of OCA has changed."]
pub struct OCC_R(crate::FieldReader<bool, bool>);
impl OCC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OCC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCC` writer - Over-current change: Logic 1 means that the value of OCA has changed."]
pub struct OCC_W<'a> {
    w: &'a mut W,
}
impl<'a> OCC_W<'a> {
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
#[doc = "Field `FPR` reader - Force Port Resume: Logic 1 means resume (K-state) detected or driven on the port."]
pub struct FPR_R(crate::FieldReader<bool, bool>);
impl FPR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPR` writer - Force Port Resume: Logic 1 means resume (K-state) detected or driven on the port."]
pub struct FPR_W<'a> {
    w: &'a mut W,
}
impl<'a> FPR_W<'a> {
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
#[doc = "Field `SUSP` reader - Suspend: Logic 1 means port is in the suspend state."]
pub struct SUSP_R(crate::FieldReader<bool, bool>);
impl SUSP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSP` writer - Suspend: Logic 1 means port is in the suspend state."]
pub struct SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `PR` reader - Port Reset: Logic 1 means the port is in the reset state."]
pub struct PR_R(crate::FieldReader<bool, bool>);
impl PR_R {
    #[inline(always)]
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
#[doc = "Field `PR` writer - Port Reset: Logic 1 means the port is in the reset state."]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `SUS_L1` reader - Suspend using L1 0b = Suspend using L2 1b = Suspend using L1 When this bit is set to a 1 and a non-zero value is specified in the Device Address field, the host controller will generate an LPM Token to enter the L1 state whenever software writes a one to the Suspend bit, as well as L1 exit timing during any device or host-initiated resume."]
pub struct SUS_L1_R(crate::FieldReader<bool, bool>);
impl SUS_L1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SUS_L1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUS_L1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUS_L1` writer - Suspend using L1 0b = Suspend using L2 1b = Suspend using L1 When this bit is set to a 1 and a non-zero value is specified in the Device Address field, the host controller will generate an LPM Token to enter the L1 state whenever software writes a one to the Suspend bit, as well as L1 exit timing during any device or host-initiated resume."]
pub struct SUS_L1_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS_L1_W<'a> {
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
#[doc = "Field `LS` reader - Line Status: This field reflects the current logical levels of the DP (bit 11) and DM (bit 10) signal lines."]
pub struct LS_R(crate::FieldReader<u8, u8>);
impl LS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PP` reader - Port Power: The function of this bit depends on the value of the Port Power Control (PPC) bit in the HCSPARAMS register."]
pub struct PP_R(crate::FieldReader<bool, bool>);
impl PP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PP` writer - Port Power: The function of this bit depends on the value of the Port Power Control (PPC) bit in the HCSPARAMS register."]
pub struct PP_W<'a> {
    w: &'a mut W,
}
impl<'a> PP_W<'a> {
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
#[doc = "Field `PIC` reader - Port Indicator Control : Writing to this field has no effect if the P_INDICATOR bit in the HCSPARAMS register is logic 0."]
pub struct PIC_R(crate::FieldReader<u8, u8>);
impl PIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIC` writer - Port Indicator Control : Writing to this field has no effect if the P_INDICATOR bit in the HCSPARAMS register is logic 0."]
pub struct PIC_W<'a> {
    w: &'a mut W,
}
impl<'a> PIC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `PTC` reader - Port Test Control: A non-zero value indicates that the port is operating in the test mode as indicated by the value."]
pub struct PTC_R(crate::FieldReader<u8, u8>);
impl PTC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTC` writer - Port Test Control: A non-zero value indicates that the port is operating in the test mode as indicated by the value."]
pub struct PTC_W<'a> {
    w: &'a mut W,
}
impl<'a> PTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `PSPD` reader - Port Speed: 00b: Low-speed 01b: Full-speed 10b: High-speed 11b: Reserved."]
pub struct PSPD_R(crate::FieldReader<u8, u8>);
impl PSPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PSPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSPD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSPD` writer - Port Speed: 00b: Low-speed 01b: Full-speed 10b: High-speed 11b: Reserved."]
pub struct PSPD_W<'a> {
    w: &'a mut W,
}
impl<'a> PSPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `WOO` reader - Wake on overcurrent enable: Writing this bit to a one enables the port to be sensitive to overcurrent conditions as wake-up events."]
pub struct WOO_R(crate::FieldReader<bool, bool>);
impl WOO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WOO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WOO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WOO` writer - Wake on overcurrent enable: Writing this bit to a one enables the port to be sensitive to overcurrent conditions as wake-up events."]
pub struct WOO_W<'a> {
    w: &'a mut W,
}
impl<'a> WOO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `SUS_STAT` reader - These two bits are used by software to determine whether the most recent L1 suspend request was successful: 00b: Success-state transition was successful (ACK) 01b: Not Yet - Device was unable to enter the L1 state at this time (NYET) 10b: Not supported - Device does not support the L1 state (STALL) 11b: Timeout/Error - Device failed to respond or an error occurred."]
pub struct SUS_STAT_R(crate::FieldReader<u8, u8>);
impl SUS_STAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SUS_STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUS_STAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUS_STAT` writer - These two bits are used by software to determine whether the most recent L1 suspend request was successful: 00b: Success-state transition was successful (ACK) 01b: Not Yet - Device was unable to enter the L1 state at this time (NYET) 10b: Not supported - Device does not support the L1 state (STALL) 11b: Timeout/Error - Device failed to respond or an error occurred."]
pub struct SUS_STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SUS_STAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | ((value as u32 & 0x03) << 23);
        self.w
    }
}
#[doc = "Field `DEV_ADD` reader - Device Address for LPM tokens."]
pub struct DEV_ADD_R(crate::FieldReader<u8, u8>);
impl DEV_ADD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEV_ADD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEV_ADD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEV_ADD` writer - Device Address for LPM tokens."]
pub struct DEV_ADD_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_ADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | ((value as u32 & 0x7f) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Current Connect Status: Logic 1 indicates a device is present on the port."]
    #[inline(always)]
    pub fn ccs(&self) -> CCS_R {
        CCS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Connect Status Change: Logic 1 means that the value of CCS has changed."]
    #[inline(always)]
    pub fn csc(&self) -> CSC_R {
        CSC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Enabled/Disabled."]
    #[inline(always)]
    pub fn ped(&self) -> PED_R {
        PED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port Enabled/Disabled Change: Logic 1 means that the value of PED has changed."]
    #[inline(always)]
    pub fn pedc(&self) -> PEDC_R {
        PEDC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Over-current active: Logic 1 means that this port has an over-current condition."]
    #[inline(always)]
    pub fn oca(&self) -> OCA_R {
        OCA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Over-current change: Logic 1 means that the value of OCA has changed."]
    #[inline(always)]
    pub fn occ(&self) -> OCC_R {
        OCC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Force Port Resume: Logic 1 means resume (K-state) detected or driven on the port."]
    #[inline(always)]
    pub fn fpr(&self) -> FPR_R {
        FPR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Suspend: Logic 1 means port is in the suspend state."]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port Reset: Logic 1 means the port is in the reset state."]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Suspend using L1 0b = Suspend using L2 1b = Suspend using L1 When this bit is set to a 1 and a non-zero value is specified in the Device Address field, the host controller will generate an LPM Token to enter the L1 state whenever software writes a one to the Suspend bit, as well as L1 exit timing during any device or host-initiated resume."]
    #[inline(always)]
    pub fn sus_l1(&self) -> SUS_L1_R {
        SUS_L1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Line Status: This field reflects the current logical levels of the DP (bit 11) and DM (bit 10) signal lines."]
    #[inline(always)]
    pub fn ls(&self) -> LS_R {
        LS_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Port Power: The function of this bit depends on the value of the Port Power Control (PPC) bit in the HCSPARAMS register."]
    #[inline(always)]
    pub fn pp(&self) -> PP_R {
        PP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Port Indicator Control : Writing to this field has no effect if the P_INDICATOR bit in the HCSPARAMS register is logic 0."]
    #[inline(always)]
    pub fn pic(&self) -> PIC_R {
        PIC_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - Port Test Control: A non-zero value indicates that the port is operating in the test mode as indicated by the value."]
    #[inline(always)]
    pub fn ptc(&self) -> PTC_R {
        PTC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Port Speed: 00b: Low-speed 01b: Full-speed 10b: High-speed 11b: Reserved."]
    #[inline(always)]
    pub fn pspd(&self) -> PSPD_R {
        PSPD_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 22 - Wake on overcurrent enable: Writing this bit to a one enables the port to be sensitive to overcurrent conditions as wake-up events."]
    #[inline(always)]
    pub fn woo(&self) -> WOO_R {
        WOO_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 23:24 - These two bits are used by software to determine whether the most recent L1 suspend request was successful: 00b: Success-state transition was successful (ACK) 01b: Not Yet - Device was unable to enter the L1 state at this time (NYET) 10b: Not supported - Device does not support the L1 state (STALL) 11b: Timeout/Error - Device failed to respond or an error occurred."]
    #[inline(always)]
    pub fn sus_stat(&self) -> SUS_STAT_R {
        SUS_STAT_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    #[doc = "Bits 25:31 - Device Address for LPM tokens."]
    #[inline(always)]
    pub fn dev_add(&self) -> DEV_ADD_R {
        DEV_ADD_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Current Connect Status: Logic 1 indicates a device is present on the port."]
    #[inline(always)]
    pub fn ccs(&mut self) -> CCS_W {
        CCS_W { w: self }
    }
    #[doc = "Bit 1 - Connect Status Change: Logic 1 means that the value of CCS has changed."]
    #[inline(always)]
    pub fn csc(&mut self) -> CSC_W {
        CSC_W { w: self }
    }
    #[doc = "Bit 2 - Port Enabled/Disabled."]
    #[inline(always)]
    pub fn ped(&mut self) -> PED_W {
        PED_W { w: self }
    }
    #[doc = "Bit 3 - Port Enabled/Disabled Change: Logic 1 means that the value of PED has changed."]
    #[inline(always)]
    pub fn pedc(&mut self) -> PEDC_W {
        PEDC_W { w: self }
    }
    #[doc = "Bit 4 - Over-current active: Logic 1 means that this port has an over-current condition."]
    #[inline(always)]
    pub fn oca(&mut self) -> OCA_W {
        OCA_W { w: self }
    }
    #[doc = "Bit 5 - Over-current change: Logic 1 means that the value of OCA has changed."]
    #[inline(always)]
    pub fn occ(&mut self) -> OCC_W {
        OCC_W { w: self }
    }
    #[doc = "Bit 6 - Force Port Resume: Logic 1 means resume (K-state) detected or driven on the port."]
    #[inline(always)]
    pub fn fpr(&mut self) -> FPR_W {
        FPR_W { w: self }
    }
    #[doc = "Bit 7 - Suspend: Logic 1 means port is in the suspend state."]
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W {
        SUSP_W { w: self }
    }
    #[doc = "Bit 8 - Port Reset: Logic 1 means the port is in the reset state."]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W {
        PR_W { w: self }
    }
    #[doc = "Bit 9 - Suspend using L1 0b = Suspend using L2 1b = Suspend using L1 When this bit is set to a 1 and a non-zero value is specified in the Device Address field, the host controller will generate an LPM Token to enter the L1 state whenever software writes a one to the Suspend bit, as well as L1 exit timing during any device or host-initiated resume."]
    #[inline(always)]
    pub fn sus_l1(&mut self) -> SUS_L1_W {
        SUS_L1_W { w: self }
    }
    #[doc = "Bit 12 - Port Power: The function of this bit depends on the value of the Port Power Control (PPC) bit in the HCSPARAMS register."]
    #[inline(always)]
    pub fn pp(&mut self) -> PP_W {
        PP_W { w: self }
    }
    #[doc = "Bits 14:15 - Port Indicator Control : Writing to this field has no effect if the P_INDICATOR bit in the HCSPARAMS register is logic 0."]
    #[inline(always)]
    pub fn pic(&mut self) -> PIC_W {
        PIC_W { w: self }
    }
    #[doc = "Bits 16:19 - Port Test Control: A non-zero value indicates that the port is operating in the test mode as indicated by the value."]
    #[inline(always)]
    pub fn ptc(&mut self) -> PTC_W {
        PTC_W { w: self }
    }
    #[doc = "Bits 20:21 - Port Speed: 00b: Low-speed 01b: Full-speed 10b: High-speed 11b: Reserved."]
    #[inline(always)]
    pub fn pspd(&mut self) -> PSPD_W {
        PSPD_W { w: self }
    }
    #[doc = "Bit 22 - Wake on overcurrent enable: Writing this bit to a one enables the port to be sensitive to overcurrent conditions as wake-up events."]
    #[inline(always)]
    pub fn woo(&mut self) -> WOO_W {
        WOO_W { w: self }
    }
    #[doc = "Bits 23:24 - These two bits are used by software to determine whether the most recent L1 suspend request was successful: 00b: Success-state transition was successful (ACK) 01b: Not Yet - Device was unable to enter the L1 state at this time (NYET) 10b: Not supported - Device does not support the L1 state (STALL) 11b: Timeout/Error - Device failed to respond or an error occurred."]
    #[inline(always)]
    pub fn sus_stat(&mut self) -> SUS_STAT_W {
        SUS_STAT_W { w: self }
    }
    #[doc = "Bits 25:31 - Device Address for LPM tokens."]
    #[inline(always)]
    pub fn dev_add(&mut self) -> DEV_ADD_W {
        DEV_ADD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Status and Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portsc1](index.html) module"]
pub struct PORTSC1_SPEC;
impl crate::RegisterSpec for PORTSC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [portsc1::R](R) reader structure"]
impl crate::Readable for PORTSC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [portsc1::W](W) writer structure"]
impl crate::Writable for PORTSC1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PORTSC1 to value 0"]
impl crate::Resettable for PORTSC1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
