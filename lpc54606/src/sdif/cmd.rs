#[doc = "Reader of register CMD"]
pub type R = crate::R<u32, super::CMD>;
#[doc = "Writer for register CMD"]
pub type W = crate::W<u32, super::CMD>;
#[doc = "Register CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMD_INDEX`"]
pub type CMD_INDEX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMD_INDEX`"]
pub struct CMD_INDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_INDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `RESPONSE_EXPECT`"]
pub type RESPONSE_EXPECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESPONSE_EXPECT`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `RESPONSE_LENGTH`"]
pub type RESPONSE_LENGTH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESPONSE_LENGTH`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `CHECK_RESPONSE_CRC`"]
pub type CHECK_RESPONSE_CRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHECK_RESPONSE_CRC`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `DATA_EXPECTED`"]
pub type DATA_EXPECTED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_EXPECTED`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `READ_WRITE`"]
pub type READ_WRITE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `READ_WRITE`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TRANSFER_MODE`"]
pub type TRANSFER_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRANSFER_MODE`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `SEND_AUTO_STOP`"]
pub type SEND_AUTO_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEND_AUTO_STOP`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `WAIT_PRVDATA_COMPLETE`"]
pub type WAIT_PRVDATA_COMPLETE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAIT_PRVDATA_COMPLETE`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `STOP_ABORT_CMD`"]
pub type STOP_ABORT_CMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOP_ABORT_CMD`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `SEND_INITIALIZATION`"]
pub type SEND_INITIALIZATION_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEND_INITIALIZATION`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `UPDATE_CLOCK_REGISTERS_ONLY`"]
pub type UPDATE_CLOCK_REGISTERS_ONLY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPDATE_CLOCK_REGISTERS_ONLY`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `READ_CEATA_DEVICE`"]
pub type READ_CEATA_DEVICE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `READ_CEATA_DEVICE`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `CCS_EXPECTED`"]
pub type CCS_EXPECTED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCS_EXPECTED`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `ENABLE_BOOT`"]
pub type ENABLE_BOOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_BOOT`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `EXPECT_BOOT_ACK`"]
pub type EXPECT_BOOT_ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXPECT_BOOT_ACK`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `DISABLE_BOOT`"]
pub type DISABLE_BOOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLE_BOOT`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `BOOT_MODE`"]
pub type BOOT_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT_MODE`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `VOLT_SWITCH`"]
pub type VOLT_SWITCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VOLT_SWITCH`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `USE_HOLD_REG`"]
pub type USE_HOLD_REG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USE_HOLD_REG`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `START_CMD`"]
pub type START_CMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START_CMD`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
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
}
