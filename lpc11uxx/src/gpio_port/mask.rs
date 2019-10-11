#[doc = "Reader of register MASK%s"]
pub type R = crate::R<u32, super::MASK>;
#[doc = "Writer for register MASK%s"]
pub type W = crate::W<u32, super::MASK>;
#[doc = "Register MASK%s `reset()`'s with value 0"]
impl crate::ResetValue for super::MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MASKP0`"]
pub type MASKP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP0`"]
pub struct MASKP0_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `MASKP1`"]
pub type MASKP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP1`"]
pub struct MASKP1_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `MASKP2`"]
pub type MASKP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP2`"]
pub struct MASKP2_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `MASKP3`"]
pub type MASKP3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP3`"]
pub struct MASKP3_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `MASKP4`"]
pub type MASKP4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP4`"]
pub struct MASKP4_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `MASKP5`"]
pub type MASKP5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP5`"]
pub struct MASKP5_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `MASKP6`"]
pub type MASKP6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP6`"]
pub struct MASKP6_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP6_W<'a> {
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
#[doc = "Reader of field `MASKP7`"]
pub type MASKP7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP7`"]
pub struct MASKP7_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP7_W<'a> {
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
#[doc = "Reader of field `MASKP8`"]
pub type MASKP8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP8`"]
pub struct MASKP8_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP8_W<'a> {
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
#[doc = "Reader of field `MASKP9`"]
pub type MASKP9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP9`"]
pub struct MASKP9_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP9_W<'a> {
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
#[doc = "Reader of field `MASKP10`"]
pub type MASKP10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP10`"]
pub struct MASKP10_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP10_W<'a> {
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
#[doc = "Reader of field `MASKP11`"]
pub type MASKP11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP11`"]
pub struct MASKP11_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP11_W<'a> {
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
#[doc = "Reader of field `MASKP12`"]
pub type MASKP12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP12`"]
pub struct MASKP12_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP12_W<'a> {
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
#[doc = "Reader of field `MASKP13`"]
pub type MASKP13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP13`"]
pub struct MASKP13_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP13_W<'a> {
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
#[doc = "Reader of field `MASKP14`"]
pub type MASKP14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP14`"]
pub struct MASKP14_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP14_W<'a> {
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
#[doc = "Reader of field `MASKP15`"]
pub type MASKP15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP15`"]
pub struct MASKP15_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP15_W<'a> {
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
#[doc = "Reader of field `MASKP16`"]
pub type MASKP16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP16`"]
pub struct MASKP16_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP16_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `MASKP17`"]
pub type MASKP17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP17`"]
pub struct MASKP17_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `MASKP18`"]
pub type MASKP18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP18`"]
pub struct MASKP18_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP18_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `MASKP19`"]
pub type MASKP19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP19`"]
pub struct MASKP19_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP19_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `MASKP20`"]
pub type MASKP20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP20`"]
pub struct MASKP20_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP20_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `MASKP21`"]
pub type MASKP21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP21`"]
pub struct MASKP21_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP21_W<'a> {
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
#[doc = "Reader of field `MASKP22`"]
pub type MASKP22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP22`"]
pub struct MASKP22_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP22_W<'a> {
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
#[doc = "Reader of field `MASKP23`"]
pub type MASKP23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP23`"]
pub struct MASKP23_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP23_W<'a> {
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
#[doc = "Reader of field `MASKP24`"]
pub type MASKP24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP24`"]
pub struct MASKP24_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP24_W<'a> {
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
#[doc = "Reader of field `MASKP25`"]
pub type MASKP25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP25`"]
pub struct MASKP25_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP25_W<'a> {
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
#[doc = "Reader of field `MASKP26`"]
pub type MASKP26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP26`"]
pub struct MASKP26_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP26_W<'a> {
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
#[doc = "Reader of field `MASKP27`"]
pub type MASKP27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP27`"]
pub struct MASKP27_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP27_W<'a> {
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
#[doc = "Reader of field `MASKP28`"]
pub type MASKP28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP28`"]
pub struct MASKP28_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP28_W<'a> {
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
#[doc = "Reader of field `MASKP29`"]
pub type MASKP29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP29`"]
pub struct MASKP29_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP29_W<'a> {
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
#[doc = "Reader of field `MASKP30`"]
pub type MASKP30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP30`"]
pub struct MASKP30_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP30_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `MASKP31`"]
pub type MASKP31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKP31`"]
pub struct MASKP31_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKP31_W<'a> {
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
    #[doc = "Bit 0 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp0(&self) -> MASKP0_R {
        MASKP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp1(&self) -> MASKP1_R {
        MASKP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp2(&self) -> MASKP2_R {
        MASKP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp3(&self) -> MASKP3_R {
        MASKP3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp4(&self) -> MASKP4_R {
        MASKP4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp5(&self) -> MASKP5_R {
        MASKP5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp6(&self) -> MASKP6_R {
        MASKP6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp7(&self) -> MASKP7_R {
        MASKP7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp8(&self) -> MASKP8_R {
        MASKP8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp9(&self) -> MASKP9_R {
        MASKP9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp10(&self) -> MASKP10_R {
        MASKP10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp11(&self) -> MASKP11_R {
        MASKP11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp12(&self) -> MASKP12_R {
        MASKP12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp13(&self) -> MASKP13_R {
        MASKP13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp14(&self) -> MASKP14_R {
        MASKP14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp15(&self) -> MASKP15_R {
        MASKP15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp16(&self) -> MASKP16_R {
        MASKP16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp17(&self) -> MASKP17_R {
        MASKP17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp18(&self) -> MASKP18_R {
        MASKP18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp19(&self) -> MASKP19_R {
        MASKP19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp20(&self) -> MASKP20_R {
        MASKP20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp21(&self) -> MASKP21_R {
        MASKP21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp22(&self) -> MASKP22_R {
        MASKP22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp23(&self) -> MASKP23_R {
        MASKP23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp24(&self) -> MASKP24_R {
        MASKP24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp25(&self) -> MASKP25_R {
        MASKP25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp26(&self) -> MASKP26_R {
        MASKP26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp27(&self) -> MASKP27_R {
        MASKP27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp28(&self) -> MASKP28_R {
        MASKP28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp29(&self) -> MASKP29_R {
        MASKP29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp30(&self) -> MASKP30_R {
        MASKP30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp31(&self) -> MASKP31_R {
        MASKP31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp0(&mut self) -> MASKP0_W {
        MASKP0_W { w: self }
    }
    #[doc = "Bit 1 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp1(&mut self) -> MASKP1_W {
        MASKP1_W { w: self }
    }
    #[doc = "Bit 2 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp2(&mut self) -> MASKP2_W {
        MASKP2_W { w: self }
    }
    #[doc = "Bit 3 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp3(&mut self) -> MASKP3_W {
        MASKP3_W { w: self }
    }
    #[doc = "Bit 4 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp4(&mut self) -> MASKP4_W {
        MASKP4_W { w: self }
    }
    #[doc = "Bit 5 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp5(&mut self) -> MASKP5_W {
        MASKP5_W { w: self }
    }
    #[doc = "Bit 6 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp6(&mut self) -> MASKP6_W {
        MASKP6_W { w: self }
    }
    #[doc = "Bit 7 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp7(&mut self) -> MASKP7_W {
        MASKP7_W { w: self }
    }
    #[doc = "Bit 8 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp8(&mut self) -> MASKP8_W {
        MASKP8_W { w: self }
    }
    #[doc = "Bit 9 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp9(&mut self) -> MASKP9_W {
        MASKP9_W { w: self }
    }
    #[doc = "Bit 10 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp10(&mut self) -> MASKP10_W {
        MASKP10_W { w: self }
    }
    #[doc = "Bit 11 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp11(&mut self) -> MASKP11_W {
        MASKP11_W { w: self }
    }
    #[doc = "Bit 12 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp12(&mut self) -> MASKP12_W {
        MASKP12_W { w: self }
    }
    #[doc = "Bit 13 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp13(&mut self) -> MASKP13_W {
        MASKP13_W { w: self }
    }
    #[doc = "Bit 14 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp14(&mut self) -> MASKP14_W {
        MASKP14_W { w: self }
    }
    #[doc = "Bit 15 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp15(&mut self) -> MASKP15_W {
        MASKP15_W { w: self }
    }
    #[doc = "Bit 16 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp16(&mut self) -> MASKP16_W {
        MASKP16_W { w: self }
    }
    #[doc = "Bit 17 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp17(&mut self) -> MASKP17_W {
        MASKP17_W { w: self }
    }
    #[doc = "Bit 18 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp18(&mut self) -> MASKP18_W {
        MASKP18_W { w: self }
    }
    #[doc = "Bit 19 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp19(&mut self) -> MASKP19_W {
        MASKP19_W { w: self }
    }
    #[doc = "Bit 20 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp20(&mut self) -> MASKP20_W {
        MASKP20_W { w: self }
    }
    #[doc = "Bit 21 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp21(&mut self) -> MASKP21_W {
        MASKP21_W { w: self }
    }
    #[doc = "Bit 22 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp22(&mut self) -> MASKP22_W {
        MASKP22_W { w: self }
    }
    #[doc = "Bit 23 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp23(&mut self) -> MASKP23_W {
        MASKP23_W { w: self }
    }
    #[doc = "Bit 24 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp24(&mut self) -> MASKP24_W {
        MASKP24_W { w: self }
    }
    #[doc = "Bit 25 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp25(&mut self) -> MASKP25_W {
        MASKP25_W { w: self }
    }
    #[doc = "Bit 26 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp26(&mut self) -> MASKP26_W {
        MASKP26_W { w: self }
    }
    #[doc = "Bit 27 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp27(&mut self) -> MASKP27_W {
        MASKP27_W { w: self }
    }
    #[doc = "Bit 28 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp28(&mut self) -> MASKP28_W {
        MASKP28_W { w: self }
    }
    #[doc = "Bit 29 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp29(&mut self) -> MASKP29_W {
        MASKP29_W { w: self }
    }
    #[doc = "Bit 30 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp30(&mut self) -> MASKP30_W {
        MASKP30_W { w: self }
    }
    #[doc = "Bit 31 - Controls which bits corresponding to P0/1_n are active in the P0/1 PIN register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn maskp31(&mut self) -> MASKP31_W {
        MASKP31_W { w: self }
    }
}
