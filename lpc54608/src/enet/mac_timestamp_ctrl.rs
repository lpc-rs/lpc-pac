#[doc = "Register `MAC_TIMESTAMP_CTRL` reader"]
pub struct R(crate::R<MAC_TIMESTAMP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_TIMESTAMP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_TIMESTAMP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_TIMESTAMP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_TIMESTAMP_CTRL` writer"]
pub struct W(crate::W<MAC_TIMESTAMP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_TIMESTAMP_CTRL_SPEC>;
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
impl From<crate::W<MAC_TIMESTAMP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_TIMESTAMP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSENA` reader - Enable Timestamp When this bit is set, the timestamp is added for Transmit and Receive packets."]
pub struct TSENA_R(crate::FieldReader<bool, bool>);
impl TSENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENA` writer - Enable Timestamp When this bit is set, the timestamp is added for Transmit and Receive packets."]
pub struct TSENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENA_W<'a> {
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
#[doc = "Field `TSCFUPDT` reader - Fine or Coarse Timestamp Update When this bit is set, the Fine method is used to update system timestamp."]
pub struct TSCFUPDT_R(crate::FieldReader<bool, bool>);
impl TSCFUPDT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSCFUPDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSCFUPDT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSCFUPDT` writer - Fine or Coarse Timestamp Update When this bit is set, the Fine method is used to update system timestamp."]
pub struct TSCFUPDT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCFUPDT_W<'a> {
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
#[doc = "Field `TSINIT` reader - Initialize Timestamp When this bit is set, the system time is initialized (overwritten) with the value specified in the MAC Register 80 (System Time Seconds Update."]
pub struct TSINIT_R(crate::FieldReader<bool, bool>);
impl TSINIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSINIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSINIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSINIT` writer - Initialize Timestamp When this bit is set, the system time is initialized (overwritten) with the value specified in the MAC Register 80 (System Time Seconds Update."]
pub struct TSINIT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSINIT_W<'a> {
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
#[doc = "Field `TSUPDT` reader - Update Timestamp When this bit is set, the system time is updated (added or subtracted) with the value specified in MAC System Time Seconds Update Table 753 and MAC System Time Nanoseconds Update Table 754."]
pub struct TSUPDT_R(crate::FieldReader<bool, bool>);
impl TSUPDT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSUPDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSUPDT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSUPDT` writer - Update Timestamp When this bit is set, the system time is updated (added or subtracted) with the value specified in MAC System Time Seconds Update Table 753 and MAC System Time Nanoseconds Update Table 754."]
pub struct TSUPDT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSUPDT_W<'a> {
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
#[doc = "Field `TSTRIG` reader - Enable Timestamp Interrupt Trigger When this bit is set, the timestamp interrupt is generated when the System Time becomes greater than the value written in the Target Time register."]
pub struct TSTRIG_R(crate::FieldReader<bool, bool>);
impl TSTRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSTRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTRIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTRIG` writer - Enable Timestamp Interrupt Trigger When this bit is set, the timestamp interrupt is generated when the System Time becomes greater than the value written in the Target Time register."]
pub struct TSTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTRIG_W<'a> {
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
#[doc = "Field `TADDREG` reader - Update Addend Register When this bit is set, the content of the Timestamp Addend register is updated in the PTP block for fine correction."]
pub struct TADDREG_R(crate::FieldReader<bool, bool>);
impl TADDREG_R {
    pub(crate) fn new(bits: bool) -> Self {
        TADDREG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TADDREG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TADDREG` writer - Update Addend Register When this bit is set, the content of the Timestamp Addend register is updated in the PTP block for fine correction."]
pub struct TADDREG_W<'a> {
    w: &'a mut W,
}
impl<'a> TADDREG_W<'a> {
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
#[doc = "Field `TSENALL` reader - Enable Timestamp for All Packets When this bit is set, the timestamp snapshot is enabled for all packets received by the MAC."]
pub struct TSENALL_R(crate::FieldReader<bool, bool>);
impl TSENALL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSENALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENALL` writer - Enable Timestamp for All Packets When this bit is set, the timestamp snapshot is enabled for all packets received by the MAC."]
pub struct TSENALL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENALL_W<'a> {
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
#[doc = "Field `TSCTRLSSR` reader - Timestamp Digital or Binary Rollover Control When this bit is set, the Timestamp Low register rolls over after 0x3B9AC9FF value (that is, 1 nanosecond accuracy) and increments the timestamp (High) seconds."]
pub struct TSCTRLSSR_R(crate::FieldReader<bool, bool>);
impl TSCTRLSSR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSCTRLSSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSCTRLSSR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSCTRLSSR` writer - Timestamp Digital or Binary Rollover Control When this bit is set, the Timestamp Low register rolls over after 0x3B9AC9FF value (that is, 1 nanosecond accuracy) and increments the timestamp (High) seconds."]
pub struct TSCTRLSSR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCTRLSSR_W<'a> {
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
#[doc = "Field `TSVER2ENA` reader - Enable PTP Packet Processing for Version 2 Format When this bit is set, the IEEE 1588 version 2 format is used to process the PTP packets."]
pub struct TSVER2ENA_R(crate::FieldReader<bool, bool>);
impl TSVER2ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSVER2ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSVER2ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSVER2ENA` writer - Enable PTP Packet Processing for Version 2 Format When this bit is set, the IEEE 1588 version 2 format is used to process the PTP packets."]
pub struct TSVER2ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSVER2ENA_W<'a> {
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
#[doc = "Field `TSIPENA` reader - Enable Processing of PTP over Ethernet Packets When this bit is set, the MAC receiver processes the PTP packets encapsulated directly in the Ethernet packets."]
pub struct TSIPENA_R(crate::FieldReader<bool, bool>);
impl TSIPENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSIPENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSIPENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSIPENA` writer - Enable Processing of PTP over Ethernet Packets When this bit is set, the MAC receiver processes the PTP packets encapsulated directly in the Ethernet packets."]
pub struct TSIPENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIPENA_W<'a> {
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
#[doc = "Field `TSIPV6ENA` reader - Enable Processing of PTP Packets Sent over 1Pv6-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv6-UDP packets."]
pub struct TSIPV6ENA_R(crate::FieldReader<bool, bool>);
impl TSIPV6ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSIPV6ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSIPV6ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSIPV6ENA` writer - Enable Processing of PTP Packets Sent over 1Pv6-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv6-UDP packets."]
pub struct TSIPV6ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIPV6ENA_W<'a> {
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
#[doc = "Field `TSIPV4ENA` reader - Enable Processing of PTP Packets Sent over IPv4-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv4-UDP packets."]
pub struct TSIPV4ENA_R(crate::FieldReader<bool, bool>);
impl TSIPV4ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSIPV4ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSIPV4ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSIPV4ENA` writer - Enable Processing of PTP Packets Sent over IPv4-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv4-UDP packets."]
pub struct TSIPV4ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIPV4ENA_W<'a> {
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
#[doc = "Field `TSEVTENA` reader - Enable Timestamp Snapshot for Event Messages When this bit is set, the timestamp snapshot is taken only for event messages (SYNC, Delay_Req, Pdelay_Req, or Pdelay_Resp)."]
pub struct TSEVTENA_R(crate::FieldReader<bool, bool>);
impl TSEVTENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSEVTENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEVTENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEVTENA` writer - Enable Timestamp Snapshot for Event Messages When this bit is set, the timestamp snapshot is taken only for event messages (SYNC, Delay_Req, Pdelay_Req, or Pdelay_Resp)."]
pub struct TSEVTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEVTENA_W<'a> {
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
#[doc = "Field `TSMSTRENA` reader - Enable Snapshot for Messages Relevant to Master When this bit is set, the snapshot is taken only for the messages that are relevant to the master node."]
pub struct TSMSTRENA_R(crate::FieldReader<bool, bool>);
impl TSMSTRENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSMSTRENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSMSTRENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSMSTRENA` writer - Enable Snapshot for Messages Relevant to Master When this bit is set, the snapshot is taken only for the messages that are relevant to the master node."]
pub struct TSMSTRENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSMSTRENA_W<'a> {
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
#[doc = "Field `SNAPTYPSEL` reader - Select PTP packets for Taking Snapshots These bits, along with Bits 15 and 14, decide the set of PTP packet types for which snapshot needs to be taken."]
pub struct SNAPTYPSEL_R(crate::FieldReader<u8, u8>);
impl SNAPTYPSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SNAPTYPSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SNAPTYPSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SNAPTYPSEL` writer - Select PTP packets for Taking Snapshots These bits, along with Bits 15 and 14, decide the set of PTP packet types for which snapshot needs to be taken."]
pub struct SNAPTYPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SNAPTYPSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `TSENMACADDR` reader - Enable MAC Address for PTP Packet Filtering When this bit is set, the DA MAC address (that matches any MAC Address register) is used to filter the PTP packets when PTP is directly sent over Ethernet."]
pub struct TSENMACADDR_R(crate::FieldReader<bool, bool>);
impl TSENMACADDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSENMACADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENMACADDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENMACADDR` writer - Enable MAC Address for PTP Packet Filtering When this bit is set, the DA MAC address (that matches any MAC Address register) is used to filter the PTP packets when PTP is directly sent over Ethernet."]
pub struct TSENMACADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENMACADDR_W<'a> {
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
#[doc = "Field `TXTTSSTSM` reader - Transmit Timestamp Status Mode When this bit is set, the MAC overwrites the earlier transmit timestamp status even if it is not read by the software."]
pub struct TXTTSSTSM_R(crate::FieldReader<bool, bool>);
impl TXTTSSTSM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXTTSSTSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXTTSSTSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXTTSSTSM` writer - Transmit Timestamp Status Mode When this bit is set, the MAC overwrites the earlier transmit timestamp status even if it is not read by the software."]
pub struct TXTTSSTSM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTTSSTSM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `AV8021ASMEN` reader - AV 802."]
pub struct AV8021ASMEN_R(crate::FieldReader<bool, bool>);
impl AV8021ASMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AV8021ASMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AV8021ASMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AV8021ASMEN` writer - AV 802."]
pub struct AV8021ASMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AV8021ASMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable Timestamp When this bit is set, the timestamp is added for Transmit and Receive packets."]
    #[inline(always)]
    pub fn tsena(&self) -> TSENA_R {
        TSENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fine or Coarse Timestamp Update When this bit is set, the Fine method is used to update system timestamp."]
    #[inline(always)]
    pub fn tscfupdt(&self) -> TSCFUPDT_R {
        TSCFUPDT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Initialize Timestamp When this bit is set, the system time is initialized (overwritten) with the value specified in the MAC Register 80 (System Time Seconds Update."]
    #[inline(always)]
    pub fn tsinit(&self) -> TSINIT_R {
        TSINIT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Update Timestamp When this bit is set, the system time is updated (added or subtracted) with the value specified in MAC System Time Seconds Update Table 753 and MAC System Time Nanoseconds Update Table 754."]
    #[inline(always)]
    pub fn tsupdt(&self) -> TSUPDT_R {
        TSUPDT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable Timestamp Interrupt Trigger When this bit is set, the timestamp interrupt is generated when the System Time becomes greater than the value written in the Target Time register."]
    #[inline(always)]
    pub fn tstrig(&self) -> TSTRIG_R {
        TSTRIG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Update Addend Register When this bit is set, the content of the Timestamp Addend register is updated in the PTP block for fine correction."]
    #[inline(always)]
    pub fn taddreg(&self) -> TADDREG_R {
        TADDREG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable Timestamp for All Packets When this bit is set, the timestamp snapshot is enabled for all packets received by the MAC."]
    #[inline(always)]
    pub fn tsenall(&self) -> TSENALL_R {
        TSENALL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Timestamp Digital or Binary Rollover Control When this bit is set, the Timestamp Low register rolls over after 0x3B9AC9FF value (that is, 1 nanosecond accuracy) and increments the timestamp (High) seconds."]
    #[inline(always)]
    pub fn tsctrlssr(&self) -> TSCTRLSSR_R {
        TSCTRLSSR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable PTP Packet Processing for Version 2 Format When this bit is set, the IEEE 1588 version 2 format is used to process the PTP packets."]
    #[inline(always)]
    pub fn tsver2ena(&self) -> TSVER2ENA_R {
        TSVER2ENA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable Processing of PTP over Ethernet Packets When this bit is set, the MAC receiver processes the PTP packets encapsulated directly in the Ethernet packets."]
    #[inline(always)]
    pub fn tsipena(&self) -> TSIPENA_R {
        TSIPENA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable Processing of PTP Packets Sent over 1Pv6-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv6-UDP packets."]
    #[inline(always)]
    pub fn tsipv6ena(&self) -> TSIPV6ENA_R {
        TSIPV6ENA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable Processing of PTP Packets Sent over IPv4-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv4-UDP packets."]
    #[inline(always)]
    pub fn tsipv4ena(&self) -> TSIPV4ENA_R {
        TSIPV4ENA_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable Timestamp Snapshot for Event Messages When this bit is set, the timestamp snapshot is taken only for event messages (SYNC, Delay_Req, Pdelay_Req, or Pdelay_Resp)."]
    #[inline(always)]
    pub fn tsevtena(&self) -> TSEVTENA_R {
        TSEVTENA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master When this bit is set, the snapshot is taken only for the messages that are relevant to the master node."]
    #[inline(always)]
    pub fn tsmstrena(&self) -> TSMSTRENA_R {
        TSMSTRENA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Select PTP packets for Taking Snapshots These bits, along with Bits 15 and 14, decide the set of PTP packet types for which snapshot needs to be taken."]
    #[inline(always)]
    pub fn snaptypsel(&self) -> SNAPTYPSEL_R {
        SNAPTYPSEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Enable MAC Address for PTP Packet Filtering When this bit is set, the DA MAC address (that matches any MAC Address register) is used to filter the PTP packets when PTP is directly sent over Ethernet."]
    #[inline(always)]
    pub fn tsenmacaddr(&self) -> TSENMACADDR_R {
        TSENMACADDR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Transmit Timestamp Status Mode When this bit is set, the MAC overwrites the earlier transmit timestamp status even if it is not read by the software."]
    #[inline(always)]
    pub fn txttsstsm(&self) -> TXTTSSTSM_R {
        TXTTSSTSM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - AV 802."]
    #[inline(always)]
    pub fn av8021asmen(&self) -> AV8021ASMEN_R {
        AV8021ASMEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Timestamp When this bit is set, the timestamp is added for Transmit and Receive packets."]
    #[inline(always)]
    pub fn tsena(&mut self) -> TSENA_W {
        TSENA_W { w: self }
    }
    #[doc = "Bit 1 - Fine or Coarse Timestamp Update When this bit is set, the Fine method is used to update system timestamp."]
    #[inline(always)]
    pub fn tscfupdt(&mut self) -> TSCFUPDT_W {
        TSCFUPDT_W { w: self }
    }
    #[doc = "Bit 2 - Initialize Timestamp When this bit is set, the system time is initialized (overwritten) with the value specified in the MAC Register 80 (System Time Seconds Update."]
    #[inline(always)]
    pub fn tsinit(&mut self) -> TSINIT_W {
        TSINIT_W { w: self }
    }
    #[doc = "Bit 3 - Update Timestamp When this bit is set, the system time is updated (added or subtracted) with the value specified in MAC System Time Seconds Update Table 753 and MAC System Time Nanoseconds Update Table 754."]
    #[inline(always)]
    pub fn tsupdt(&mut self) -> TSUPDT_W {
        TSUPDT_W { w: self }
    }
    #[doc = "Bit 4 - Enable Timestamp Interrupt Trigger When this bit is set, the timestamp interrupt is generated when the System Time becomes greater than the value written in the Target Time register."]
    #[inline(always)]
    pub fn tstrig(&mut self) -> TSTRIG_W {
        TSTRIG_W { w: self }
    }
    #[doc = "Bit 5 - Update Addend Register When this bit is set, the content of the Timestamp Addend register is updated in the PTP block for fine correction."]
    #[inline(always)]
    pub fn taddreg(&mut self) -> TADDREG_W {
        TADDREG_W { w: self }
    }
    #[doc = "Bit 8 - Enable Timestamp for All Packets When this bit is set, the timestamp snapshot is enabled for all packets received by the MAC."]
    #[inline(always)]
    pub fn tsenall(&mut self) -> TSENALL_W {
        TSENALL_W { w: self }
    }
    #[doc = "Bit 9 - Timestamp Digital or Binary Rollover Control When this bit is set, the Timestamp Low register rolls over after 0x3B9AC9FF value (that is, 1 nanosecond accuracy) and increments the timestamp (High) seconds."]
    #[inline(always)]
    pub fn tsctrlssr(&mut self) -> TSCTRLSSR_W {
        TSCTRLSSR_W { w: self }
    }
    #[doc = "Bit 10 - Enable PTP Packet Processing for Version 2 Format When this bit is set, the IEEE 1588 version 2 format is used to process the PTP packets."]
    #[inline(always)]
    pub fn tsver2ena(&mut self) -> TSVER2ENA_W {
        TSVER2ENA_W { w: self }
    }
    #[doc = "Bit 11 - Enable Processing of PTP over Ethernet Packets When this bit is set, the MAC receiver processes the PTP packets encapsulated directly in the Ethernet packets."]
    #[inline(always)]
    pub fn tsipena(&mut self) -> TSIPENA_W {
        TSIPENA_W { w: self }
    }
    #[doc = "Bit 12 - Enable Processing of PTP Packets Sent over 1Pv6-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv6-UDP packets."]
    #[inline(always)]
    pub fn tsipv6ena(&mut self) -> TSIPV6ENA_W {
        TSIPV6ENA_W { w: self }
    }
    #[doc = "Bit 13 - Enable Processing of PTP Packets Sent over IPv4-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv4-UDP packets."]
    #[inline(always)]
    pub fn tsipv4ena(&mut self) -> TSIPV4ENA_W {
        TSIPV4ENA_W { w: self }
    }
    #[doc = "Bit 14 - Enable Timestamp Snapshot for Event Messages When this bit is set, the timestamp snapshot is taken only for event messages (SYNC, Delay_Req, Pdelay_Req, or Pdelay_Resp)."]
    #[inline(always)]
    pub fn tsevtena(&mut self) -> TSEVTENA_W {
        TSEVTENA_W { w: self }
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master When this bit is set, the snapshot is taken only for the messages that are relevant to the master node."]
    #[inline(always)]
    pub fn tsmstrena(&mut self) -> TSMSTRENA_W {
        TSMSTRENA_W { w: self }
    }
    #[doc = "Bits 16:17 - Select PTP packets for Taking Snapshots These bits, along with Bits 15 and 14, decide the set of PTP packet types for which snapshot needs to be taken."]
    #[inline(always)]
    pub fn snaptypsel(&mut self) -> SNAPTYPSEL_W {
        SNAPTYPSEL_W { w: self }
    }
    #[doc = "Bit 18 - Enable MAC Address for PTP Packet Filtering When this bit is set, the DA MAC address (that matches any MAC Address register) is used to filter the PTP packets when PTP is directly sent over Ethernet."]
    #[inline(always)]
    pub fn tsenmacaddr(&mut self) -> TSENMACADDR_W {
        TSENMACADDR_W { w: self }
    }
    #[doc = "Bit 24 - Transmit Timestamp Status Mode When this bit is set, the MAC overwrites the earlier transmit timestamp status even if it is not read by the software."]
    #[inline(always)]
    pub fn txttsstsm(&mut self) -> TXTTSSTSM_W {
        TXTTSSTSM_W { w: self }
    }
    #[doc = "Bit 28 - AV 802."]
    #[inline(always)]
    pub fn av8021asmen(&mut self) -> AV8021ASMEN_W {
        AV8021ASMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time stamp control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_timestamp_ctrl](index.html) module"]
pub struct MAC_TIMESTAMP_CTRL_SPEC;
impl crate::RegisterSpec for MAC_TIMESTAMP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_timestamp_ctrl::R](R) reader structure"]
impl crate::Readable for MAC_TIMESTAMP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_timestamp_ctrl::W](W) writer structure"]
impl crate::Writable for MAC_TIMESTAMP_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_TIMESTAMP_CTRL to value 0x2000"]
impl crate::Resettable for MAC_TIMESTAMP_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000
    }
}
