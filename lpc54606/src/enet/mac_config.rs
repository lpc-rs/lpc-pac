#[doc = "Register `MAC_CONFIG` reader"]
pub struct R(crate::R<MAC_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MAC_CONFIG_SPEC>> for R {
    fn from(reader: crate::R<MAC_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_CONFIG` writer"]
pub struct W(crate::W<MAC_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_CONFIG_SPEC>;
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
impl core::convert::From<crate::W<MAC_CONFIG_SPEC>> for W {
    fn from(writer: crate::W<MAC_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RE` reader - Receiver Enable When this bit is set, the receiver state machine of the MAC is enabled for receiving frames from the MII."]
pub struct RE_R(crate::FieldReader<bool, bool>);
impl RE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RE` writer - Receiver Enable When this bit is set, the receiver state machine of the MAC is enabled for receiving frames from the MII."]
pub struct RE_W<'a> {
    w: &'a mut W,
}
impl<'a> RE_W<'a> {
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
#[doc = "Field `TE` reader - Transmitter Enable When this bit is set, the transmit state machine of the MAC is enabled for transmission on the MII."]
pub struct TE_R(crate::FieldReader<bool, bool>);
impl TE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TE` writer - Transmitter Enable When this bit is set, the transmit state machine of the MAC is enabled for transmission on the MII."]
pub struct TE_W<'a> {
    w: &'a mut W,
}
impl<'a> TE_W<'a> {
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
#[doc = "Field `PRELEN` reader - Preamble Length for Transmit packets These bits control the number of preamble bytes that are added to the beginning of every Tx packet."]
pub struct PRELEN_R(crate::FieldReader<u8, u8>);
impl PRELEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRELEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRELEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRELEN` writer - Preamble Length for Transmit packets These bits control the number of preamble bytes that are added to the beginning of every Tx packet."]
pub struct PRELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `DC` reader - Deferral Check When this bit is set, the deferral check function is enabled in the MAC."]
pub struct DC_R(crate::FieldReader<bool, bool>);
impl DC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DC` writer - Deferral Check When this bit is set, the deferral check function is enabled in the MAC."]
pub struct DC_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_W<'a> {
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
#[doc = "Field `BL` reader - Back-Off Limit The Back-Off limit determines the random integer number (r) of slot time delays (4,096 bit times for 1000 Mbps and 512 bit times for 10/100 Mbps) the MAC waits before rescheduling a transmission attempt during retries after a collision."]
pub struct BL_R(crate::FieldReader<u8, u8>);
impl BL_R {
    pub(crate) fn new(bits: u8) -> Self {
        BL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BL` writer - Back-Off Limit The Back-Off limit determines the random integer number (r) of slot time delays (4,096 bit times for 1000 Mbps and 512 bit times for 10/100 Mbps) the MAC waits before rescheduling a transmission attempt during retries after a collision."]
pub struct BL_W<'a> {
    w: &'a mut W,
}
impl<'a> BL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `DR` reader - Disable Retry When this bit is set, the MAC will attempt only one transmission."]
pub struct DR_R(crate::FieldReader<bool, bool>);
impl DR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DR` writer - Disable Retry When this bit is set, the MAC will attempt only one transmission."]
pub struct DR_W<'a> {
    w: &'a mut W,
}
impl<'a> DR_W<'a> {
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
#[doc = "Field `DCRS` reader - Disable Carrier Sense During Transmission When this bit is set, the MAC transmitter ignores the MII CRS signal during packet transmission in the half-duplex mode."]
pub struct DCRS_R(crate::FieldReader<bool, bool>);
impl DCRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCRS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCRS` writer - Disable Carrier Sense During Transmission When this bit is set, the MAC transmitter ignores the MII CRS signal during packet transmission in the half-duplex mode."]
pub struct DCRS_W<'a> {
    w: &'a mut W,
}
impl<'a> DCRS_W<'a> {
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
#[doc = "Field `DO` reader - Disable Receive Own When this bit is set, the MAC disables the reception of frames when the gmii_txen_o is asserted in Half-Duplex mode."]
pub struct DO_R(crate::FieldReader<bool, bool>);
impl DO_R {
    pub(crate) fn new(bits: bool) -> Self {
        DO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DO` writer - Disable Receive Own When this bit is set, the MAC disables the reception of frames when the gmii_txen_o is asserted in Half-Duplex mode."]
pub struct DO_W<'a> {
    w: &'a mut W,
}
impl<'a> DO_W<'a> {
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
#[doc = "Field `ECRSFD` reader - Enable Carrier Sense Before Transmission in Full-Duplex Mode When this bit is set, the MAC transmitter checks the CRS signal before packet transmission in the full-duplex mode."]
pub struct ECRSFD_R(crate::FieldReader<bool, bool>);
impl ECRSFD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECRSFD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECRSFD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECRSFD` writer - Enable Carrier Sense Before Transmission in Full-Duplex Mode When this bit is set, the MAC transmitter checks the CRS signal before packet transmission in the full-duplex mode."]
pub struct ECRSFD_W<'a> {
    w: &'a mut W,
}
impl<'a> ECRSFD_W<'a> {
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
#[doc = "Field `LM` reader - Loopback Mode When this bit is set, the MAC operates in loopback mode at MII."]
pub struct LM_R(crate::FieldReader<bool, bool>);
impl LM_R {
    pub(crate) fn new(bits: bool) -> Self {
        LM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LM` writer - Loopback Mode When this bit is set, the MAC operates in loopback mode at MII."]
pub struct LM_W<'a> {
    w: &'a mut W,
}
impl<'a> LM_W<'a> {
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
#[doc = "Field `DM` reader - Duplex Mode When this bit is set, the MAC operates in a Full-Duplex mode where it can transmit and receive simultaneously."]
pub struct DM_R(crate::FieldReader<bool, bool>);
impl DM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM` writer - Duplex Mode When this bit is set, the MAC operates in a Full-Duplex mode where it can transmit and receive simultaneously."]
pub struct DM_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `FES` reader - Speed Indicates the speed in Fast Ethernet (MII) mode: This bit is reserved (RO) by default and is enabled only when RMII/SMII is enabled during configuration."]
pub struct FES_R(crate::FieldReader<bool, bool>);
impl FES_R {
    pub(crate) fn new(bits: bool) -> Self {
        FES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FES` writer - Speed Indicates the speed in Fast Ethernet (MII) mode: This bit is reserved (RO) by default and is enabled only when RMII/SMII is enabled during configuration."]
pub struct FES_W<'a> {
    w: &'a mut W,
}
impl<'a> FES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `PS` reader - Portselect."]
pub struct PS_R(crate::FieldReader<bool, bool>);
impl PS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JE` reader - Jumbo Frame Enable When this bit is set, MAC allows Jumbo frames of 9,018 bytes (9,022 bytes for tagged frames) without reporting a giant frame error in the receive frame status."]
pub struct JE_R(crate::FieldReader<bool, bool>);
impl JE_R {
    pub(crate) fn new(bits: bool) -> Self {
        JE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JE` writer - Jumbo Frame Enable When this bit is set, MAC allows Jumbo frames of 9,018 bytes (9,022 bytes for tagged frames) without reporting a giant frame error in the receive frame status."]
pub struct JE_W<'a> {
    w: &'a mut W,
}
impl<'a> JE_W<'a> {
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
#[doc = "Field `JD` reader - Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter, and can transfer frames of up to 16,384 bytes."]
pub struct JD_R(crate::FieldReader<bool, bool>);
impl JD_R {
    pub(crate) fn new(bits: bool) -> Self {
        JD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JD` writer - Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter, and can transfer frames of up to 16,384 bytes."]
pub struct JD_W<'a> {
    w: &'a mut W,
}
impl<'a> JD_W<'a> {
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
#[doc = "Field `BE` reader - Packet Burst Enable When this bit is set, the MAC allows packet bursting during transmission in the MII half-duplex mode."]
pub struct BE_R(crate::FieldReader<bool, bool>);
impl BE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BE` writer - Packet Burst Enable When this bit is set, the MAC allows packet bursting during transmission in the MII half-duplex mode."]
pub struct BE_W<'a> {
    w: &'a mut W,
}
impl<'a> BE_W<'a> {
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
#[doc = "Field `WD` reader - Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver, and can receive frames of up to 16,384 bytes."]
pub struct WD_R(crate::FieldReader<bool, bool>);
impl WD_R {
    pub(crate) fn new(bits: bool) -> Self {
        WD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WD` writer - Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver, and can receive frames of up to 16,384 bytes."]
pub struct WD_W<'a> {
    w: &'a mut W,
}
impl<'a> WD_W<'a> {
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
#[doc = "Field `ACS` reader - Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming packets only if the value of the length field is less than 1,536 bytes."]
pub struct ACS_R(crate::FieldReader<bool, bool>);
impl ACS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACS` writer - Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming packets only if the value of the length field is less than 1,536 bytes."]
pub struct ACS_W<'a> {
    w: &'a mut W,
}
impl<'a> ACS_W<'a> {
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
#[doc = "Field `CST` reader - CRC stripping for Type packets When this bit is set, the last four bytes (FCS) of all packets of Ether type (type field greater than 1,536) are stripped and dropped before forwarding the packet to the application."]
pub struct CST_R(crate::FieldReader<bool, bool>);
impl CST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CST` writer - CRC stripping for Type packets When this bit is set, the last four bytes (FCS) of all packets of Ether type (type field greater than 1,536) are stripped and dropped before forwarding the packet to the application."]
pub struct CST_W<'a> {
    w: &'a mut W,
}
impl<'a> CST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `S2KP` reader - IEEE 802."]
pub struct S2KP_R(crate::FieldReader<bool, bool>);
impl S2KP_R {
    pub(crate) fn new(bits: bool) -> Self {
        S2KP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S2KP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S2KP` writer - IEEE 802."]
pub struct S2KP_W<'a> {
    w: &'a mut W,
}
impl<'a> S2KP_W<'a> {
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
#[doc = "Field `GPSLCE` reader - Giant Packet Size Limit Control Enable When this bit is set, the MAC considers the value in GPSL field in MAC Ext Configuration register to declare a received packet as Giant packet."]
pub struct GPSLCE_R(crate::FieldReader<bool, bool>);
impl GPSLCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPSLCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPSLCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPSLCE` writer - Giant Packet Size Limit Control Enable When this bit is set, the MAC considers the value in GPSL field in MAC Ext Configuration register to declare a received packet as Giant packet."]
pub struct GPSLCE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPSLCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `IPG` reader - Inter-Packet Gap These bits control the minimum IPG between packets during transmission."]
pub struct IPG_R(crate::FieldReader<u8, u8>);
impl IPG_R {
    pub(crate) fn new(bits: u8) -> Self {
        IPG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPG` writer - Inter-Packet Gap These bits control the minimum IPG between packets during transmission."]
pub struct IPG_W<'a> {
    w: &'a mut W,
}
impl<'a> IPG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `IPC` reader - Checksum Offload When set, this bit enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking."]
pub struct IPC_R(crate::FieldReader<bool, bool>);
impl IPC_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPC` writer - Checksum Offload When set, this bit enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking."]
pub struct IPC_W<'a> {
    w: &'a mut W,
}
impl<'a> IPC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Receiver Enable When this bit is set, the receiver state machine of the MAC is enabled for receiving frames from the MII."]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmitter Enable When this bit is set, the transmit state machine of the MAC is enabled for transmission on the MII."]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Preamble Length for Transmit packets These bits control the number of preamble bytes that are added to the beginning of every Tx packet."]
    #[inline(always)]
    pub fn prelen(&self) -> PRELEN_R {
        PRELEN_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Deferral Check When this bit is set, the deferral check function is enabled in the MAC."]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Back-Off Limit The Back-Off limit determines the random integer number (r) of slot time delays (4,096 bit times for 1000 Mbps and 512 bit times for 10/100 Mbps) the MAC waits before rescheduling a transmission attempt during retries after a collision."]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Disable Retry When this bit is set, the MAC will attempt only one transmission."]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Disable Carrier Sense During Transmission When this bit is set, the MAC transmitter ignores the MII CRS signal during packet transmission in the half-duplex mode."]
    #[inline(always)]
    pub fn dcrs(&self) -> DCRS_R {
        DCRS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Disable Receive Own When this bit is set, the MAC disables the reception of frames when the gmii_txen_o is asserted in Half-Duplex mode."]
    #[inline(always)]
    pub fn do_(&self) -> DO_R {
        DO_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable Carrier Sense Before Transmission in Full-Duplex Mode When this bit is set, the MAC transmitter checks the CRS signal before packet transmission in the full-duplex mode."]
    #[inline(always)]
    pub fn ecrsfd(&self) -> ECRSFD_R {
        ECRSFD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Loopback Mode When this bit is set, the MAC operates in loopback mode at MII."]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Duplex Mode When this bit is set, the MAC operates in a Full-Duplex mode where it can transmit and receive simultaneously."]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Speed Indicates the speed in Fast Ethernet (MII) mode: This bit is reserved (RO) by default and is enabled only when RMII/SMII is enabled during configuration."]
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Portselect."]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Jumbo Frame Enable When this bit is set, MAC allows Jumbo frames of 9,018 bytes (9,022 bytes for tagged frames) without reporting a giant frame error in the receive frame status."]
    #[inline(always)]
    pub fn je(&self) -> JE_R {
        JE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter, and can transfer frames of up to 16,384 bytes."]
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Packet Burst Enable When this bit is set, the MAC allows packet bursting during transmission in the MII half-duplex mode."]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver, and can receive frames of up to 16,384 bytes."]
    #[inline(always)]
    pub fn wd(&self) -> WD_R {
        WD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming packets only if the value of the length field is less than 1,536 bytes."]
    #[inline(always)]
    pub fn acs(&self) -> ACS_R {
        ACS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - CRC stripping for Type packets When this bit is set, the last four bytes (FCS) of all packets of Ether type (type field greater than 1,536) are stripped and dropped before forwarding the packet to the application."]
    #[inline(always)]
    pub fn cst(&self) -> CST_R {
        CST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - IEEE 802."]
    #[inline(always)]
    pub fn s2kp(&self) -> S2KP_R {
        S2KP_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Giant Packet Size Limit Control Enable When this bit is set, the MAC considers the value in GPSL field in MAC Ext Configuration register to declare a received packet as Giant packet."]
    #[inline(always)]
    pub fn gpslce(&self) -> GPSLCE_R {
        GPSLCE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Inter-Packet Gap These bits control the minimum IPG between packets during transmission."]
    #[inline(always)]
    pub fn ipg(&self) -> IPG_R {
        IPG_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 27 - Checksum Offload When set, this bit enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking."]
    #[inline(always)]
    pub fn ipc(&self) -> IPC_R {
        IPC_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Enable When this bit is set, the receiver state machine of the MAC is enabled for receiving frames from the MII."]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W {
        RE_W { w: self }
    }
    #[doc = "Bit 1 - Transmitter Enable When this bit is set, the transmit state machine of the MAC is enabled for transmission on the MII."]
    #[inline(always)]
    pub fn te(&mut self) -> TE_W {
        TE_W { w: self }
    }
    #[doc = "Bits 2:3 - Preamble Length for Transmit packets These bits control the number of preamble bytes that are added to the beginning of every Tx packet."]
    #[inline(always)]
    pub fn prelen(&mut self) -> PRELEN_W {
        PRELEN_W { w: self }
    }
    #[doc = "Bit 4 - Deferral Check When this bit is set, the deferral check function is enabled in the MAC."]
    #[inline(always)]
    pub fn dc(&mut self) -> DC_W {
        DC_W { w: self }
    }
    #[doc = "Bits 5:6 - Back-Off Limit The Back-Off limit determines the random integer number (r) of slot time delays (4,096 bit times for 1000 Mbps and 512 bit times for 10/100 Mbps) the MAC waits before rescheduling a transmission attempt during retries after a collision."]
    #[inline(always)]
    pub fn bl(&mut self) -> BL_W {
        BL_W { w: self }
    }
    #[doc = "Bit 8 - Disable Retry When this bit is set, the MAC will attempt only one transmission."]
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W {
        DR_W { w: self }
    }
    #[doc = "Bit 9 - Disable Carrier Sense During Transmission When this bit is set, the MAC transmitter ignores the MII CRS signal during packet transmission in the half-duplex mode."]
    #[inline(always)]
    pub fn dcrs(&mut self) -> DCRS_W {
        DCRS_W { w: self }
    }
    #[doc = "Bit 10 - Disable Receive Own When this bit is set, the MAC disables the reception of frames when the gmii_txen_o is asserted in Half-Duplex mode."]
    #[inline(always)]
    pub fn do_(&mut self) -> DO_W {
        DO_W { w: self }
    }
    #[doc = "Bit 11 - Enable Carrier Sense Before Transmission in Full-Duplex Mode When this bit is set, the MAC transmitter checks the CRS signal before packet transmission in the full-duplex mode."]
    #[inline(always)]
    pub fn ecrsfd(&mut self) -> ECRSFD_W {
        ECRSFD_W { w: self }
    }
    #[doc = "Bit 12 - Loopback Mode When this bit is set, the MAC operates in loopback mode at MII."]
    #[inline(always)]
    pub fn lm(&mut self) -> LM_W {
        LM_W { w: self }
    }
    #[doc = "Bit 13 - Duplex Mode When this bit is set, the MAC operates in a Full-Duplex mode where it can transmit and receive simultaneously."]
    #[inline(always)]
    pub fn dm(&mut self) -> DM_W {
        DM_W { w: self }
    }
    #[doc = "Bit 14 - Speed Indicates the speed in Fast Ethernet (MII) mode: This bit is reserved (RO) by default and is enabled only when RMII/SMII is enabled during configuration."]
    #[inline(always)]
    pub fn fes(&mut self) -> FES_W {
        FES_W { w: self }
    }
    #[doc = "Bit 16 - Jumbo Frame Enable When this bit is set, MAC allows Jumbo frames of 9,018 bytes (9,022 bytes for tagged frames) without reporting a giant frame error in the receive frame status."]
    #[inline(always)]
    pub fn je(&mut self) -> JE_W {
        JE_W { w: self }
    }
    #[doc = "Bit 17 - Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter, and can transfer frames of up to 16,384 bytes."]
    #[inline(always)]
    pub fn jd(&mut self) -> JD_W {
        JD_W { w: self }
    }
    #[doc = "Bit 18 - Packet Burst Enable When this bit is set, the MAC allows packet bursting during transmission in the MII half-duplex mode."]
    #[inline(always)]
    pub fn be(&mut self) -> BE_W {
        BE_W { w: self }
    }
    #[doc = "Bit 19 - Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver, and can receive frames of up to 16,384 bytes."]
    #[inline(always)]
    pub fn wd(&mut self) -> WD_W {
        WD_W { w: self }
    }
    #[doc = "Bit 20 - Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming packets only if the value of the length field is less than 1,536 bytes."]
    #[inline(always)]
    pub fn acs(&mut self) -> ACS_W {
        ACS_W { w: self }
    }
    #[doc = "Bit 21 - CRC stripping for Type packets When this bit is set, the last four bytes (FCS) of all packets of Ether type (type field greater than 1,536) are stripped and dropped before forwarding the packet to the application."]
    #[inline(always)]
    pub fn cst(&mut self) -> CST_W {
        CST_W { w: self }
    }
    #[doc = "Bit 22 - IEEE 802."]
    #[inline(always)]
    pub fn s2kp(&mut self) -> S2KP_W {
        S2KP_W { w: self }
    }
    #[doc = "Bit 23 - Giant Packet Size Limit Control Enable When this bit is set, the MAC considers the value in GPSL field in MAC Ext Configuration register to declare a received packet as Giant packet."]
    #[inline(always)]
    pub fn gpslce(&mut self) -> GPSLCE_W {
        GPSLCE_W { w: self }
    }
    #[doc = "Bits 24:26 - Inter-Packet Gap These bits control the minimum IPG between packets during transmission."]
    #[inline(always)]
    pub fn ipg(&mut self) -> IPG_W {
        IPG_W { w: self }
    }
    #[doc = "Bit 27 - Checksum Offload When set, this bit enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking."]
    #[inline(always)]
    pub fn ipc(&mut self) -> IPC_W {
        IPC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_config](index.html) module"]
pub struct MAC_CONFIG_SPEC;
impl crate::RegisterSpec for MAC_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_config::R](R) reader structure"]
impl crate::Readable for MAC_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_config::W](W) writer structure"]
impl crate::Writable for MAC_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_CONFIG to value 0x8000"]
impl crate::Resettable for MAC_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
