#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CMD_SPEC>> for R {
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl core::convert::From<crate::W<CMD_SPEC>> for W {
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD_INDEX` reader - Command index."]
pub struct CMD_INDEX_R(crate::FieldReader<u8, u8>);
impl CMD_INDEX_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMD_INDEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMD_INDEX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD_INDEX` writer - Command index."]
pub struct CMD_INDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_INDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `RESPONSE_EXPECT` reader - Response expect."]
pub struct RESPONSE_EXPECT_R(crate::FieldReader<bool, bool>);
impl RESPONSE_EXPECT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESPONSE_EXPECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESPONSE_EXPECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESPONSE_EXPECT` writer - Response expect."]
pub struct RESPONSE_EXPECT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESPONSE_EXPECT_W<'a> {
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
#[doc = "Field `RESPONSE_LENGTH` reader - Response length."]
pub struct RESPONSE_LENGTH_R(crate::FieldReader<bool, bool>);
impl RESPONSE_LENGTH_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESPONSE_LENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESPONSE_LENGTH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESPONSE_LENGTH` writer - Response length."]
pub struct RESPONSE_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> RESPONSE_LENGTH_W<'a> {
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
#[doc = "Field `CHECK_RESPONSE_CRC` reader - Check response CRC."]
pub struct CHECK_RESPONSE_CRC_R(crate::FieldReader<bool, bool>);
impl CHECK_RESPONSE_CRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHECK_RESPONSE_CRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHECK_RESPONSE_CRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHECK_RESPONSE_CRC` writer - Check response CRC."]
pub struct CHECK_RESPONSE_CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CHECK_RESPONSE_CRC_W<'a> {
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
#[doc = "Field `DATA_EXPECTED` reader - Data expected."]
pub struct DATA_EXPECTED_R(crate::FieldReader<bool, bool>);
impl DATA_EXPECTED_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATA_EXPECTED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_EXPECTED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_EXPECTED` writer - Data expected."]
pub struct DATA_EXPECTED_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_EXPECTED_W<'a> {
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
#[doc = "Field `READ_WRITE` reader - read/write."]
pub struct READ_WRITE_R(crate::FieldReader<bool, bool>);
impl READ_WRITE_R {
    pub(crate) fn new(bits: bool) -> Self {
        READ_WRITE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READ_WRITE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READ_WRITE` writer - read/write."]
pub struct READ_WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_WRITE_W<'a> {
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
#[doc = "Field `TRANSFER_MODE` reader - Transfer mode."]
pub struct TRANSFER_MODE_R(crate::FieldReader<bool, bool>);
impl TRANSFER_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRANSFER_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANSFER_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANSFER_MODE` writer - Transfer mode."]
pub struct TRANSFER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSFER_MODE_W<'a> {
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
#[doc = "Field `SEND_AUTO_STOP` reader - Send auto stop."]
pub struct SEND_AUTO_STOP_R(crate::FieldReader<bool, bool>);
impl SEND_AUTO_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEND_AUTO_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_AUTO_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEND_AUTO_STOP` writer - Send auto stop."]
pub struct SEND_AUTO_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_AUTO_STOP_W<'a> {
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
#[doc = "Field `WAIT_PRVDATA_COMPLETE` reader - Wait prvdata complete."]
pub struct WAIT_PRVDATA_COMPLETE_R(crate::FieldReader<bool, bool>);
impl WAIT_PRVDATA_COMPLETE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAIT_PRVDATA_COMPLETE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAIT_PRVDATA_COMPLETE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAIT_PRVDATA_COMPLETE` writer - Wait prvdata complete."]
pub struct WAIT_PRVDATA_COMPLETE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_PRVDATA_COMPLETE_W<'a> {
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
#[doc = "Field `STOP_ABORT_CMD` reader - Stop abort command."]
pub struct STOP_ABORT_CMD_R(crate::FieldReader<bool, bool>);
impl STOP_ABORT_CMD_R {
    pub(crate) fn new(bits: bool) -> Self {
        STOP_ABORT_CMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOP_ABORT_CMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP_ABORT_CMD` writer - Stop abort command."]
pub struct STOP_ABORT_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_ABORT_CMD_W<'a> {
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
#[doc = "Field `SEND_INITIALIZATION` reader - Send initialization."]
pub struct SEND_INITIALIZATION_R(crate::FieldReader<bool, bool>);
impl SEND_INITIALIZATION_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEND_INITIALIZATION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_INITIALIZATION_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEND_INITIALIZATION` writer - Send initialization."]
pub struct SEND_INITIALIZATION_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_INITIALIZATION_W<'a> {
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
#[doc = "Field `UPDATE_CLOCK_REGISTERS_ONLY` reader - Update clock registers only."]
pub struct UPDATE_CLOCK_REGISTERS_ONLY_R(crate::FieldReader<bool, bool>);
impl UPDATE_CLOCK_REGISTERS_ONLY_R {
    pub(crate) fn new(bits: bool) -> Self {
        UPDATE_CLOCK_REGISTERS_ONLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPDATE_CLOCK_REGISTERS_ONLY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPDATE_CLOCK_REGISTERS_ONLY` writer - Update clock registers only."]
pub struct UPDATE_CLOCK_REGISTERS_ONLY_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDATE_CLOCK_REGISTERS_ONLY_W<'a> {
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
#[doc = "Field `READ_CEATA_DEVICE` reader - Read ceata device."]
pub struct READ_CEATA_DEVICE_R(crate::FieldReader<bool, bool>);
impl READ_CEATA_DEVICE_R {
    pub(crate) fn new(bits: bool) -> Self {
        READ_CEATA_DEVICE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READ_CEATA_DEVICE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READ_CEATA_DEVICE` writer - Read ceata device."]
pub struct READ_CEATA_DEVICE_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_CEATA_DEVICE_W<'a> {
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
#[doc = "Field `CCS_EXPECTED` reader - CCS expected."]
pub struct CCS_EXPECTED_R(crate::FieldReader<bool, bool>);
impl CCS_EXPECTED_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCS_EXPECTED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCS_EXPECTED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCS_EXPECTED` writer - CCS expected."]
pub struct CCS_EXPECTED_W<'a> {
    w: &'a mut W,
}
impl<'a> CCS_EXPECTED_W<'a> {
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
#[doc = "Field `ENABLE_BOOT` reader - Enable Boot - this bit should be set only for mandatory boot mode."]
pub struct ENABLE_BOOT_R(crate::FieldReader<bool, bool>);
impl ENABLE_BOOT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_BOOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_BOOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE_BOOT` writer - Enable Boot - this bit should be set only for mandatory boot mode."]
pub struct ENABLE_BOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_BOOT_W<'a> {
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
#[doc = "Field `EXPECT_BOOT_ACK` reader - Expect Boot Acknowledge."]
pub struct EXPECT_BOOT_ACK_R(crate::FieldReader<bool, bool>);
impl EXPECT_BOOT_ACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXPECT_BOOT_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXPECT_BOOT_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXPECT_BOOT_ACK` writer - Expect Boot Acknowledge."]
pub struct EXPECT_BOOT_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> EXPECT_BOOT_ACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `DISABLE_BOOT` reader - Disable Boot."]
pub struct DISABLE_BOOT_R(crate::FieldReader<bool, bool>);
impl DISABLE_BOOT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISABLE_BOOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISABLE_BOOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISABLE_BOOT` writer - Disable Boot."]
pub struct DISABLE_BOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_BOOT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `BOOT_MODE` reader - Boot Mode."]
pub struct BOOT_MODE_R(crate::FieldReader<bool, bool>);
impl BOOT_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOT_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOT_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOT_MODE` writer - Boot Mode."]
pub struct BOOT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_MODE_W<'a> {
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
#[doc = "Field `VOLT_SWITCH` reader - Voltage switch bit."]
pub struct VOLT_SWITCH_R(crate::FieldReader<bool, bool>);
impl VOLT_SWITCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        VOLT_SWITCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VOLT_SWITCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VOLT_SWITCH` writer - Voltage switch bit."]
pub struct VOLT_SWITCH_W<'a> {
    w: &'a mut W,
}
impl<'a> VOLT_SWITCH_W<'a> {
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
#[doc = "Field `USE_HOLD_REG` reader - Use Hold Register."]
pub struct USE_HOLD_REG_R(crate::FieldReader<bool, bool>);
impl USE_HOLD_REG_R {
    pub(crate) fn new(bits: bool) -> Self {
        USE_HOLD_REG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USE_HOLD_REG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USE_HOLD_REG` writer - Use Hold Register."]
pub struct USE_HOLD_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> USE_HOLD_REG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `START_CMD` reader - Start command."]
pub struct START_CMD_R(crate::FieldReader<bool, bool>);
impl START_CMD_R {
    pub(crate) fn new(bits: bool) -> Self {
        START_CMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_CMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START_CMD` writer - Start command."]
pub struct START_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> START_CMD_W<'a> {
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
    #[doc = "Bits 0:5 - Command index."]
    #[inline(always)]
    pub fn cmd_index(&self) -> CMD_INDEX_R {
        CMD_INDEX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Response expect."]
    #[inline(always)]
    pub fn response_expect(&self) -> RESPONSE_EXPECT_R {
        RESPONSE_EXPECT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Response length."]
    #[inline(always)]
    pub fn response_length(&self) -> RESPONSE_LENGTH_R {
        RESPONSE_LENGTH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Check response CRC."]
    #[inline(always)]
    pub fn check_response_crc(&self) -> CHECK_RESPONSE_CRC_R {
        CHECK_RESPONSE_CRC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Data expected."]
    #[inline(always)]
    pub fn data_expected(&self) -> DATA_EXPECTED_R {
        DATA_EXPECTED_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - read/write."]
    #[inline(always)]
    pub fn read_write(&self) -> READ_WRITE_R {
        READ_WRITE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transfer mode."]
    #[inline(always)]
    pub fn transfer_mode(&self) -> TRANSFER_MODE_R {
        TRANSFER_MODE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Send auto stop."]
    #[inline(always)]
    pub fn send_auto_stop(&self) -> SEND_AUTO_STOP_R {
        SEND_AUTO_STOP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Wait prvdata complete."]
    #[inline(always)]
    pub fn wait_prvdata_complete(&self) -> WAIT_PRVDATA_COMPLETE_R {
        WAIT_PRVDATA_COMPLETE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Stop abort command."]
    #[inline(always)]
    pub fn stop_abort_cmd(&self) -> STOP_ABORT_CMD_R {
        STOP_ABORT_CMD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Send initialization."]
    #[inline(always)]
    pub fn send_initialization(&self) -> SEND_INITIALIZATION_R {
        SEND_INITIALIZATION_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Update clock registers only."]
    #[inline(always)]
    pub fn update_clock_registers_only(&self) -> UPDATE_CLOCK_REGISTERS_ONLY_R {
        UPDATE_CLOCK_REGISTERS_ONLY_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Read ceata device."]
    #[inline(always)]
    pub fn read_ceata_device(&self) -> READ_CEATA_DEVICE_R {
        READ_CEATA_DEVICE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - CCS expected."]
    #[inline(always)]
    pub fn ccs_expected(&self) -> CCS_EXPECTED_R {
        CCS_EXPECTED_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable Boot - this bit should be set only for mandatory boot mode."]
    #[inline(always)]
    pub fn enable_boot(&self) -> ENABLE_BOOT_R {
        ENABLE_BOOT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Expect Boot Acknowledge."]
    #[inline(always)]
    pub fn expect_boot_ack(&self) -> EXPECT_BOOT_ACK_R {
        EXPECT_BOOT_ACK_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Disable Boot."]
    #[inline(always)]
    pub fn disable_boot(&self) -> DISABLE_BOOT_R {
        DISABLE_BOOT_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Boot Mode."]
    #[inline(always)]
    pub fn boot_mode(&self) -> BOOT_MODE_R {
        BOOT_MODE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Voltage switch bit."]
    #[inline(always)]
    pub fn volt_switch(&self) -> VOLT_SWITCH_R {
        VOLT_SWITCH_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Use Hold Register."]
    #[inline(always)]
    pub fn use_hold_reg(&self) -> USE_HOLD_REG_R {
        USE_HOLD_REG_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Start command."]
    #[inline(always)]
    pub fn start_cmd(&self) -> START_CMD_R {
        START_CMD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Command index."]
    #[inline(always)]
    pub fn cmd_index(&mut self) -> CMD_INDEX_W {
        CMD_INDEX_W { w: self }
    }
    #[doc = "Bit 6 - Response expect."]
    #[inline(always)]
    pub fn response_expect(&mut self) -> RESPONSE_EXPECT_W {
        RESPONSE_EXPECT_W { w: self }
    }
    #[doc = "Bit 7 - Response length."]
    #[inline(always)]
    pub fn response_length(&mut self) -> RESPONSE_LENGTH_W {
        RESPONSE_LENGTH_W { w: self }
    }
    #[doc = "Bit 8 - Check response CRC."]
    #[inline(always)]
    pub fn check_response_crc(&mut self) -> CHECK_RESPONSE_CRC_W {
        CHECK_RESPONSE_CRC_W { w: self }
    }
    #[doc = "Bit 9 - Data expected."]
    #[inline(always)]
    pub fn data_expected(&mut self) -> DATA_EXPECTED_W {
        DATA_EXPECTED_W { w: self }
    }
    #[doc = "Bit 10 - read/write."]
    #[inline(always)]
    pub fn read_write(&mut self) -> READ_WRITE_W {
        READ_WRITE_W { w: self }
    }
    #[doc = "Bit 11 - Transfer mode."]
    #[inline(always)]
    pub fn transfer_mode(&mut self) -> TRANSFER_MODE_W {
        TRANSFER_MODE_W { w: self }
    }
    #[doc = "Bit 12 - Send auto stop."]
    #[inline(always)]
    pub fn send_auto_stop(&mut self) -> SEND_AUTO_STOP_W {
        SEND_AUTO_STOP_W { w: self }
    }
    #[doc = "Bit 13 - Wait prvdata complete."]
    #[inline(always)]
    pub fn wait_prvdata_complete(&mut self) -> WAIT_PRVDATA_COMPLETE_W {
        WAIT_PRVDATA_COMPLETE_W { w: self }
    }
    #[doc = "Bit 14 - Stop abort command."]
    #[inline(always)]
    pub fn stop_abort_cmd(&mut self) -> STOP_ABORT_CMD_W {
        STOP_ABORT_CMD_W { w: self }
    }
    #[doc = "Bit 15 - Send initialization."]
    #[inline(always)]
    pub fn send_initialization(&mut self) -> SEND_INITIALIZATION_W {
        SEND_INITIALIZATION_W { w: self }
    }
    #[doc = "Bit 21 - Update clock registers only."]
    #[inline(always)]
    pub fn update_clock_registers_only(&mut self) -> UPDATE_CLOCK_REGISTERS_ONLY_W {
        UPDATE_CLOCK_REGISTERS_ONLY_W { w: self }
    }
    #[doc = "Bit 22 - Read ceata device."]
    #[inline(always)]
    pub fn read_ceata_device(&mut self) -> READ_CEATA_DEVICE_W {
        READ_CEATA_DEVICE_W { w: self }
    }
    #[doc = "Bit 23 - CCS expected."]
    #[inline(always)]
    pub fn ccs_expected(&mut self) -> CCS_EXPECTED_W {
        CCS_EXPECTED_W { w: self }
    }
    #[doc = "Bit 24 - Enable Boot - this bit should be set only for mandatory boot mode."]
    #[inline(always)]
    pub fn enable_boot(&mut self) -> ENABLE_BOOT_W {
        ENABLE_BOOT_W { w: self }
    }
    #[doc = "Bit 25 - Expect Boot Acknowledge."]
    #[inline(always)]
    pub fn expect_boot_ack(&mut self) -> EXPECT_BOOT_ACK_W {
        EXPECT_BOOT_ACK_W { w: self }
    }
    #[doc = "Bit 26 - Disable Boot."]
    #[inline(always)]
    pub fn disable_boot(&mut self) -> DISABLE_BOOT_W {
        DISABLE_BOOT_W { w: self }
    }
    #[doc = "Bit 27 - Boot Mode."]
    #[inline(always)]
    pub fn boot_mode(&mut self) -> BOOT_MODE_W {
        BOOT_MODE_W { w: self }
    }
    #[doc = "Bit 28 - Voltage switch bit."]
    #[inline(always)]
    pub fn volt_switch(&mut self) -> VOLT_SWITCH_W {
        VOLT_SWITCH_W { w: self }
    }
    #[doc = "Bit 29 - Use Hold Register."]
    #[inline(always)]
    pub fn use_hold_reg(&mut self) -> USE_HOLD_REG_W {
        USE_HOLD_REG_W { w: self }
    }
    #[doc = "Bit 31 - Start command."]
    #[inline(always)]
    pub fn start_cmd(&mut self) -> START_CMD_W {
        START_CMD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R](R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
