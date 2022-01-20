#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATALEN` reader - Except when the POLL bit in this register is 1, this field controls how many data bytes are in the command. 0 indicates that the command does not contain a data field."]
pub struct DATALEN_R(crate::FieldReader<u16, u16>);
impl DATALEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DATALEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATALEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATALEN` writer - Except when the POLL bit in this register is 1, this field controls how many data bytes are in the command. 0 indicates that the command does not contain a data field."]
pub struct DATALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATALEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
#[doc = "Field `POLL` reader - This bit should be written as 1 only with an opcode that a) contains an input data field, and b) causes the serial flash device to return byte status repetitively (e.g., a Read Status command). When this bit is 1, the SPIFI hardware continues to read bytes until the test specified by the DATALEN field is met. The hardware tests the bit in each status byte selected by DATALEN bits 2:0, until a bit is found that is equal to DATALEN bit 3. When the test succeeds, the SPIFI captures the byte that meets this test so that it can be read from the Data Register, and terminates the command by raising CS. The end-of-command interrupt can be enabled to inform software when this occurs"]
pub struct POLL_R(crate::FieldReader<bool, bool>);
impl POLL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POLL` writer - This bit should be written as 1 only with an opcode that a) contains an input data field, and b) causes the serial flash device to return byte status repetitively (e.g., a Read Status command). When this bit is 1, the SPIFI hardware continues to read bytes until the test specified by the DATALEN field is met. The hardware tests the bit in each status byte selected by DATALEN bits 2:0, until a bit is found that is equal to DATALEN bit 3. When the test succeeds, the SPIFI captures the byte that meets this test so that it can be read from the Data Register, and terminates the command by raising CS. The end-of-command interrupt can be enabled to inform software when this occurs"]
pub struct POLL_W<'a> {
    w: &'a mut W,
}
impl<'a> POLL_W<'a> {
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
#[doc = "If the DATALEN field is not zero, this bit controls the direction of the data:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOUT_A {
    #[doc = "0: Input from serial flash."]
    INPUT = 0,
    #[doc = "1: Output to serial flash."]
    OUTPUT = 1,
}
impl From<DOUT_A> for bool {
    #[inline(always)]
    fn from(variant: DOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT` reader - If the DATALEN field is not zero, this bit controls the direction of the data:"]
pub struct DOUT_R(crate::FieldReader<bool, DOUT_A>);
impl DOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUT_A {
        match self.bits {
            false => DOUT_A::INPUT,
            true => DOUT_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == DOUT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == DOUT_A::OUTPUT
    }
}
impl core::ops::Deref for DOUT_R {
    type Target = crate::FieldReader<bool, DOUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT` writer - If the DATALEN field is not zero, this bit controls the direction of the data:"]
pub struct DOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOUT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input from serial flash."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DOUT_A::INPUT)
    }
    #[doc = "Output to serial flash."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DOUT_A::OUTPUT)
    }
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
#[doc = "Field `INTLEN` reader - This field controls how many intermediate bytes precede the data. (Each such byte may require 8 or 2 SCK cycles, depending on whether the intermediate field is in serial, 2-bit, or 4-bit format.) Intermediate bytes are output by the SPIFI, and include post-address control information, dummy and delay bytes. See the description of the Intermediate Data register for the contents of such bytes."]
pub struct INTLEN_R(crate::FieldReader<u8, u8>);
impl INTLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INTLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTLEN` writer - This field controls how many intermediate bytes precede the data. (Each such byte may require 8 or 2 SCK cycles, depending on whether the intermediate field is in serial, 2-bit, or 4-bit format.) Intermediate bytes are output by the SPIFI, and include post-address control information, dummy and delay bytes. See the description of the Intermediate Data register for the contents of such bytes."]
pub struct INTLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "This field controls how the fields of the command are sent.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FIELDFORM_A {
    #[doc = "0: All serial. All fields of the command are serial."]
    ALL_SERIAL = 0,
    #[doc = "1: Quad/dual data. Data field is quad/dual, other fields are serial."]
    QUAD_DUAL_DATA = 1,
    #[doc = "2: Serial opcode. Opcode field is serial. Other fields are quad/dual."]
    SERIAL_OPCODE = 2,
    #[doc = "3: All quad/dual. All fields of the command are in quad/dual format."]
    ALL_QUAD_DUAL = 3,
}
impl From<FIELDFORM_A> for u8 {
    #[inline(always)]
    fn from(variant: FIELDFORM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FIELDFORM` reader - This field controls how the fields of the command are sent."]
pub struct FIELDFORM_R(crate::FieldReader<u8, FIELDFORM_A>);
impl FIELDFORM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FIELDFORM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIELDFORM_A {
        match self.bits {
            0 => FIELDFORM_A::ALL_SERIAL,
            1 => FIELDFORM_A::QUAD_DUAL_DATA,
            2 => FIELDFORM_A::SERIAL_OPCODE,
            3 => FIELDFORM_A::ALL_QUAD_DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ALL_SERIAL`"]
    #[inline(always)]
    pub fn is_all_serial(&self) -> bool {
        **self == FIELDFORM_A::ALL_SERIAL
    }
    #[doc = "Checks if the value of the field is `QUAD_DUAL_DATA`"]
    #[inline(always)]
    pub fn is_quad_dual_data(&self) -> bool {
        **self == FIELDFORM_A::QUAD_DUAL_DATA
    }
    #[doc = "Checks if the value of the field is `SERIAL_OPCODE`"]
    #[inline(always)]
    pub fn is_serial_opcode(&self) -> bool {
        **self == FIELDFORM_A::SERIAL_OPCODE
    }
    #[doc = "Checks if the value of the field is `ALL_QUAD_DUAL`"]
    #[inline(always)]
    pub fn is_all_quad_dual(&self) -> bool {
        **self == FIELDFORM_A::ALL_QUAD_DUAL
    }
}
impl core::ops::Deref for FIELDFORM_R {
    type Target = crate::FieldReader<u8, FIELDFORM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIELDFORM` writer - This field controls how the fields of the command are sent."]
pub struct FIELDFORM_W<'a> {
    w: &'a mut W,
}
impl<'a> FIELDFORM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIELDFORM_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "All serial. All fields of the command are serial."]
    #[inline(always)]
    pub fn all_serial(self) -> &'a mut W {
        self.variant(FIELDFORM_A::ALL_SERIAL)
    }
    #[doc = "Quad/dual data. Data field is quad/dual, other fields are serial."]
    #[inline(always)]
    pub fn quad_dual_data(self) -> &'a mut W {
        self.variant(FIELDFORM_A::QUAD_DUAL_DATA)
    }
    #[doc = "Serial opcode. Opcode field is serial. Other fields are quad/dual."]
    #[inline(always)]
    pub fn serial_opcode(self) -> &'a mut W {
        self.variant(FIELDFORM_A::SERIAL_OPCODE)
    }
    #[doc = "All quad/dual. All fields of the command are in quad/dual format."]
    #[inline(always)]
    pub fn all_quad_dual(self) -> &'a mut W {
        self.variant(FIELDFORM_A::ALL_QUAD_DUAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | ((value as u32 & 0x03) << 19);
        self.w
    }
}
#[doc = "This field controls the opcode and address fields.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FRAMEFORM_A {
    #[doc = "1: Opcode. Opcode only, no address."]
    OPCODE = 1,
    #[doc = "2: Opcode one byte. Opcode, least significant byte of address."]
    OPCODE_1_BYTE = 2,
    #[doc = "3: Opcode two bytes. Opcode, two least significant bytes of address."]
    OPCODE_2_BYTES = 3,
    #[doc = "4: Opcode three bytes. Opcode, three least significant bytes of address."]
    OPCODE_3_BYTES = 4,
    #[doc = "5: Opcode four bytes. Opcode, 4 bytes of address."]
    OPCODE_4_BYTES = 5,
    #[doc = "6: No opcode three bytes. No opcode, 3 least significant bytes of address."]
    NO_OPCODE_3_BYTES = 6,
    #[doc = "7: No opcode four bytes. No opcode, 4 bytes of address."]
    NO_OPCODE_4_BYTES = 7,
}
impl From<FRAMEFORM_A> for u8 {
    #[inline(always)]
    fn from(variant: FRAMEFORM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FRAMEFORM` reader - This field controls the opcode and address fields."]
pub struct FRAMEFORM_R(crate::FieldReader<u8, FRAMEFORM_A>);
impl FRAMEFORM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FRAMEFORM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FRAMEFORM_A> {
        match self.bits {
            1 => Some(FRAMEFORM_A::OPCODE),
            2 => Some(FRAMEFORM_A::OPCODE_1_BYTE),
            3 => Some(FRAMEFORM_A::OPCODE_2_BYTES),
            4 => Some(FRAMEFORM_A::OPCODE_3_BYTES),
            5 => Some(FRAMEFORM_A::OPCODE_4_BYTES),
            6 => Some(FRAMEFORM_A::NO_OPCODE_3_BYTES),
            7 => Some(FRAMEFORM_A::NO_OPCODE_4_BYTES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OPCODE`"]
    #[inline(always)]
    pub fn is_opcode(&self) -> bool {
        **self == FRAMEFORM_A::OPCODE
    }
    #[doc = "Checks if the value of the field is `OPCODE_1_BYTE`"]
    #[inline(always)]
    pub fn is_opcode_1_byte(&self) -> bool {
        **self == FRAMEFORM_A::OPCODE_1_BYTE
    }
    #[doc = "Checks if the value of the field is `OPCODE_2_BYTES`"]
    #[inline(always)]
    pub fn is_opcode_2_bytes(&self) -> bool {
        **self == FRAMEFORM_A::OPCODE_2_BYTES
    }
    #[doc = "Checks if the value of the field is `OPCODE_3_BYTES`"]
    #[inline(always)]
    pub fn is_opcode_3_bytes(&self) -> bool {
        **self == FRAMEFORM_A::OPCODE_3_BYTES
    }
    #[doc = "Checks if the value of the field is `OPCODE_4_BYTES`"]
    #[inline(always)]
    pub fn is_opcode_4_bytes(&self) -> bool {
        **self == FRAMEFORM_A::OPCODE_4_BYTES
    }
    #[doc = "Checks if the value of the field is `NO_OPCODE_3_BYTES`"]
    #[inline(always)]
    pub fn is_no_opcode_3_bytes(&self) -> bool {
        **self == FRAMEFORM_A::NO_OPCODE_3_BYTES
    }
    #[doc = "Checks if the value of the field is `NO_OPCODE_4_BYTES`"]
    #[inline(always)]
    pub fn is_no_opcode_4_bytes(&self) -> bool {
        **self == FRAMEFORM_A::NO_OPCODE_4_BYTES
    }
}
impl core::ops::Deref for FRAMEFORM_R {
    type Target = crate::FieldReader<u8, FRAMEFORM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAMEFORM` writer - This field controls the opcode and address fields."]
pub struct FRAMEFORM_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMEFORM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRAMEFORM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Opcode. Opcode only, no address."]
    #[inline(always)]
    pub fn opcode(self) -> &'a mut W {
        self.variant(FRAMEFORM_A::OPCODE)
    }
    #[doc = "Opcode one byte. Opcode, least significant byte of address."]
    #[inline(always)]
    pub fn opcode_1_byte(self) -> &'a mut W {
        self.variant(FRAMEFORM_A::OPCODE_1_BYTE)
    }
    #[doc = "Opcode two bytes. Opcode, two least significant bytes of address."]
    #[inline(always)]
    pub fn opcode_2_bytes(self) -> &'a mut W {
        self.variant(FRAMEFORM_A::OPCODE_2_BYTES)
    }
    #[doc = "Opcode three bytes. Opcode, three least significant bytes of address."]
    #[inline(always)]
    pub fn opcode_3_bytes(self) -> &'a mut W {
        self.variant(FRAMEFORM_A::OPCODE_3_BYTES)
    }
    #[doc = "Opcode four bytes. Opcode, 4 bytes of address."]
    #[inline(always)]
    pub fn opcode_4_bytes(self) -> &'a mut W {
        self.variant(FRAMEFORM_A::OPCODE_4_BYTES)
    }
    #[doc = "No opcode three bytes. No opcode, 3 least significant bytes of address."]
    #[inline(always)]
    pub fn no_opcode_3_bytes(self) -> &'a mut W {
        self.variant(FRAMEFORM_A::NO_OPCODE_3_BYTES)
    }
    #[doc = "No opcode four bytes. No opcode, 4 bytes of address."]
    #[inline(always)]
    pub fn no_opcode_4_bytes(self) -> &'a mut W {
        self.variant(FRAMEFORM_A::NO_OPCODE_4_BYTES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | ((value as u32 & 0x07) << 21);
        self.w
    }
}
#[doc = "Field `OPCODE` reader - The opcode of the command (not used for some FRAMEFORM values)."]
pub struct OPCODE_R(crate::FieldReader<u8, u8>);
impl OPCODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OPCODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPCODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPCODE` writer - The opcode of the command (not used for some FRAMEFORM values)."]
pub struct OPCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Except when the POLL bit in this register is 1, this field controls how many data bytes are in the command. 0 indicates that the command does not contain a data field."]
    #[inline(always)]
    pub fn datalen(&self) -> DATALEN_R {
        DATALEN_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 14 - This bit should be written as 1 only with an opcode that a) contains an input data field, and b) causes the serial flash device to return byte status repetitively (e.g., a Read Status command). When this bit is 1, the SPIFI hardware continues to read bytes until the test specified by the DATALEN field is met. The hardware tests the bit in each status byte selected by DATALEN bits 2:0, until a bit is found that is equal to DATALEN bit 3. When the test succeeds, the SPIFI captures the byte that meets this test so that it can be read from the Data Register, and terminates the command by raising CS. The end-of-command interrupt can be enabled to inform software when this occurs"]
    #[inline(always)]
    pub fn poll(&self) -> POLL_R {
        POLL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - If the DATALEN field is not zero, this bit controls the direction of the data:"]
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - This field controls how many intermediate bytes precede the data. (Each such byte may require 8 or 2 SCK cycles, depending on whether the intermediate field is in serial, 2-bit, or 4-bit format.) Intermediate bytes are output by the SPIFI, and include post-address control information, dummy and delay bytes. See the description of the Intermediate Data register for the contents of such bytes."]
    #[inline(always)]
    pub fn intlen(&self) -> INTLEN_R {
        INTLEN_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 19:20 - This field controls how the fields of the command are sent."]
    #[inline(always)]
    pub fn fieldform(&self) -> FIELDFORM_R {
        FIELDFORM_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bits 21:23 - This field controls the opcode and address fields."]
    #[inline(always)]
    pub fn frameform(&self) -> FRAMEFORM_R {
        FRAMEFORM_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 24:31 - The opcode of the command (not used for some FRAMEFORM values)."]
    #[inline(always)]
    pub fn opcode(&self) -> OPCODE_R {
        OPCODE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - Except when the POLL bit in this register is 1, this field controls how many data bytes are in the command. 0 indicates that the command does not contain a data field."]
    #[inline(always)]
    pub fn datalen(&mut self) -> DATALEN_W {
        DATALEN_W { w: self }
    }
    #[doc = "Bit 14 - This bit should be written as 1 only with an opcode that a) contains an input data field, and b) causes the serial flash device to return byte status repetitively (e.g., a Read Status command). When this bit is 1, the SPIFI hardware continues to read bytes until the test specified by the DATALEN field is met. The hardware tests the bit in each status byte selected by DATALEN bits 2:0, until a bit is found that is equal to DATALEN bit 3. When the test succeeds, the SPIFI captures the byte that meets this test so that it can be read from the Data Register, and terminates the command by raising CS. The end-of-command interrupt can be enabled to inform software when this occurs"]
    #[inline(always)]
    pub fn poll(&mut self) -> POLL_W {
        POLL_W { w: self }
    }
    #[doc = "Bit 15 - If the DATALEN field is not zero, this bit controls the direction of the data:"]
    #[inline(always)]
    pub fn dout(&mut self) -> DOUT_W {
        DOUT_W { w: self }
    }
    #[doc = "Bits 16:18 - This field controls how many intermediate bytes precede the data. (Each such byte may require 8 or 2 SCK cycles, depending on whether the intermediate field is in serial, 2-bit, or 4-bit format.) Intermediate bytes are output by the SPIFI, and include post-address control information, dummy and delay bytes. See the description of the Intermediate Data register for the contents of such bytes."]
    #[inline(always)]
    pub fn intlen(&mut self) -> INTLEN_W {
        INTLEN_W { w: self }
    }
    #[doc = "Bits 19:20 - This field controls how the fields of the command are sent."]
    #[inline(always)]
    pub fn fieldform(&mut self) -> FIELDFORM_W {
        FIELDFORM_W { w: self }
    }
    #[doc = "Bits 21:23 - This field controls the opcode and address fields."]
    #[inline(always)]
    pub fn frameform(&mut self) -> FRAMEFORM_W {
        FRAMEFORM_W { w: self }
    }
    #[doc = "Bits 24:31 - The opcode of the command (not used for some FRAMEFORM values)."]
    #[inline(always)]
    pub fn opcode(&mut self) -> OPCODE_W {
        OPCODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPIFI command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
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
