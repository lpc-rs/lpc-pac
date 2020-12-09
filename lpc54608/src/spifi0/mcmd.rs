#[doc = "Reader of register MCMD"]
pub type R = crate::R<u32, super::MCMD>;
#[doc = "Writer for register MCMD"]
pub type W = crate::W<u32, super::MCMD>;
#[doc = "Register MCMD `reset()`'s with value 0"]
impl crate::ResetValue for super::MCMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `POLL`"]
pub type POLL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POLL`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `DOUT`"]
pub type DOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOUT`"]
pub struct DOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT_W<'a> {
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
#[doc = "Reader of field `INTLEN`"]
pub type INTLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTLEN`"]
pub struct INTLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
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
#[doc = "Reader of field `FIELDFORM`"]
pub type FIELDFORM_R = crate::R<u8, FIELDFORM_A>;
impl FIELDFORM_R {
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
        *self == FIELDFORM_A::ALL_SERIAL
    }
    #[doc = "Checks if the value of the field is `QUAD_DUAL_DATA`"]
    #[inline(always)]
    pub fn is_quad_dual_data(&self) -> bool {
        *self == FIELDFORM_A::QUAD_DUAL_DATA
    }
    #[doc = "Checks if the value of the field is `SERIAL_OPCODE`"]
    #[inline(always)]
    pub fn is_serial_opcode(&self) -> bool {
        *self == FIELDFORM_A::SERIAL_OPCODE
    }
    #[doc = "Checks if the value of the field is `ALL_QUAD_DUAL`"]
    #[inline(always)]
    pub fn is_all_quad_dual(&self) -> bool {
        *self == FIELDFORM_A::ALL_QUAD_DUAL
    }
}
#[doc = "Write proxy for field `FIELDFORM`"]
pub struct FIELDFORM_W<'a> {
    w: &'a mut W,
}
impl<'a> FIELDFORM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIELDFORM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = "This field controls the opcode and address fields.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FRAMEFORM_A {
    #[doc = "1: Opcode. Opcode only, no address."]
    OPCODE = 1,
    #[doc = "2: Opcode one byte. Opcode, least-significant byte of address."]
    OPCODE_1_BYTE = 2,
    #[doc = "3: Opcode two bytes. Opcode, 2 least-significant bytes of address."]
    OPCODE_2_BYTES = 3,
    #[doc = "4: Opcode three bytes. Opcode, 3 least-significant bytes of address."]
    OPCODE_3_BYTES = 4,
    #[doc = "5: Opcode four bytes. Opcode, 4 bytes of address."]
    OPCODE_4_BYTES = 5,
    #[doc = "6: No opcode three bytes. No opcode, 3 least-significant bytes of address."]
    NO_OPCODE_3_BYTES = 6,
    #[doc = "7: No opcode, 4 bytes of address."]
    NO_OPCODE_4_BYTES = 7,
}
impl From<FRAMEFORM_A> for u8 {
    #[inline(always)]
    fn from(variant: FRAMEFORM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FRAMEFORM`"]
pub type FRAMEFORM_R = crate::R<u8, FRAMEFORM_A>;
impl FRAMEFORM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FRAMEFORM_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(FRAMEFORM_A::OPCODE),
            2 => Val(FRAMEFORM_A::OPCODE_1_BYTE),
            3 => Val(FRAMEFORM_A::OPCODE_2_BYTES),
            4 => Val(FRAMEFORM_A::OPCODE_3_BYTES),
            5 => Val(FRAMEFORM_A::OPCODE_4_BYTES),
            6 => Val(FRAMEFORM_A::NO_OPCODE_3_BYTES),
            7 => Val(FRAMEFORM_A::NO_OPCODE_4_BYTES),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OPCODE`"]
    #[inline(always)]
    pub fn is_opcode(&self) -> bool {
        *self == FRAMEFORM_A::OPCODE
    }
    #[doc = "Checks if the value of the field is `OPCODE_1_BYTE`"]
    #[inline(always)]
    pub fn is_opcode_1_byte(&self) -> bool {
        *self == FRAMEFORM_A::OPCODE_1_BYTE
    }
    #[doc = "Checks if the value of the field is `OPCODE_2_BYTES`"]
    #[inline(always)]
    pub fn is_opcode_2_bytes(&self) -> bool {
        *self == FRAMEFORM_A::OPCODE_2_BYTES
    }
    #[doc = "Checks if the value of the field is `OPCODE_3_BYTES`"]
    #[inline(always)]
    pub fn is_opcode_3_bytes(&self) -> bool {
        *self == FRAMEFORM_A::OPCODE_3_BYTES
    }
    #[doc = "Checks if the value of the field is `OPCODE_4_BYTES`"]
    #[inline(always)]
    pub fn is_opcode_4_bytes(&self) -> bool {
        *self == FRAMEFORM_A::OPCODE_4_BYTES
    }
    #[doc = "Checks if the value of the field is `NO_OPCODE_3_BYTES`"]
    #[inline(always)]
    pub fn is_no_opcode_3_bytes(&self) -> bool {
        *self == FRAMEFORM_A::NO_OPCODE_3_BYTES
    }
    #[doc = "Checks if the value of the field is `NO_OPCODE_4_BYTES`"]
    #[inline(always)]
    pub fn is_no_opcode_4_bytes(&self) -> bool {
        *self == FRAMEFORM_A::NO_OPCODE_4_BYTES
    }
}
#[doc = "Write proxy for field `FRAMEFORM`"]
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
    #[doc = "Opcode one byte. Opcode, least-significant byte of address."]
    #[inline(always)]
    pub fn opcode_1_byte(self) -> &'a mut W {
        self.variant(FRAMEFORM_A::OPCODE_1_BYTE)
    }
    #[doc = "Opcode two bytes. Opcode, 2 least-significant bytes of address."]
    #[inline(always)]
    pub fn opcode_2_bytes(self) -> &'a mut W {
        self.variant(FRAMEFORM_A::OPCODE_2_BYTES)
    }
    #[doc = "Opcode three bytes. Opcode, 3 least-significant bytes of address."]
    #[inline(always)]
    pub fn opcode_3_bytes(self) -> &'a mut W {
        self.variant(FRAMEFORM_A::OPCODE_3_BYTES)
    }
    #[doc = "Opcode four bytes. Opcode, 4 bytes of address."]
    #[inline(always)]
    pub fn opcode_4_bytes(self) -> &'a mut W {
        self.variant(FRAMEFORM_A::OPCODE_4_BYTES)
    }
    #[doc = "No opcode three bytes. No opcode, 3 least-significant bytes of address."]
    #[inline(always)]
    pub fn no_opcode_3_bytes(self) -> &'a mut W {
        self.variant(FRAMEFORM_A::NO_OPCODE_3_BYTES)
    }
    #[doc = "No opcode, 4 bytes of address."]
    #[inline(always)]
    pub fn no_opcode_4_bytes(self) -> &'a mut W {
        self.variant(FRAMEFORM_A::NO_OPCODE_4_BYTES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
#[doc = "Reader of field `OPCODE`"]
pub type OPCODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OPCODE`"]
pub struct OPCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 14 - This bit should be written as 0."]
    #[inline(always)]
    pub fn poll(&self) -> POLL_R {
        POLL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - This bit should be written as 0."]
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
    #[doc = "Bit 14 - This bit should be written as 0."]
    #[inline(always)]
    pub fn poll(&mut self) -> POLL_W {
        POLL_W { w: self }
    }
    #[doc = "Bit 15 - This bit should be written as 0."]
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
}
