#[doc = "Reader of register DIR%s"]
pub type R = crate::R<u32, super::DIR>;
#[doc = "Writer for register DIR%s"]
pub type W = crate::W<u32, super::DIR>;
#[doc = "Register DIR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::DIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIRP0`"]
pub type DIRP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP0`"]
pub struct DIRP0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP0_W<'a> {
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
#[doc = "Reader of field `DIRP1`"]
pub type DIRP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP1`"]
pub struct DIRP1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP1_W<'a> {
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
#[doc = "Reader of field `DIRP2`"]
pub type DIRP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP2`"]
pub struct DIRP2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP2_W<'a> {
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
#[doc = "Reader of field `DIRP3`"]
pub type DIRP3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP3`"]
pub struct DIRP3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP3_W<'a> {
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
#[doc = "Reader of field `DIRP4`"]
pub type DIRP4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP4`"]
pub struct DIRP4_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP4_W<'a> {
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
#[doc = "Reader of field `DIRP5`"]
pub type DIRP5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP5`"]
pub struct DIRP5_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP5_W<'a> {
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
#[doc = "Reader of field `DIRP6`"]
pub type DIRP6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP6`"]
pub struct DIRP6_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP6_W<'a> {
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
#[doc = "Reader of field `DIRP7`"]
pub type DIRP7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP7`"]
pub struct DIRP7_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP7_W<'a> {
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
#[doc = "Reader of field `DIRP8`"]
pub type DIRP8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP8`"]
pub struct DIRP8_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP8_W<'a> {
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
#[doc = "Reader of field `DIRP9`"]
pub type DIRP9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP9`"]
pub struct DIRP9_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP9_W<'a> {
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
#[doc = "Reader of field `DIRP10`"]
pub type DIRP10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP10`"]
pub struct DIRP10_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP10_W<'a> {
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
#[doc = "Reader of field `DIRP11`"]
pub type DIRP11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP11`"]
pub struct DIRP11_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP11_W<'a> {
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
#[doc = "Reader of field `DIRP12`"]
pub type DIRP12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP12`"]
pub struct DIRP12_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP12_W<'a> {
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
#[doc = "Reader of field `DIRP13`"]
pub type DIRP13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP13`"]
pub struct DIRP13_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP13_W<'a> {
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
#[doc = "Reader of field `DIRP14`"]
pub type DIRP14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP14`"]
pub struct DIRP14_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP14_W<'a> {
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
#[doc = "Reader of field `DIRP15`"]
pub type DIRP15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP15`"]
pub struct DIRP15_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP15_W<'a> {
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
#[doc = "Reader of field `DIRP16`"]
pub type DIRP16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP16`"]
pub struct DIRP16_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP16_W<'a> {
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
#[doc = "Reader of field `DIRP17`"]
pub type DIRP17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP17`"]
pub struct DIRP17_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP17_W<'a> {
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
#[doc = "Reader of field `DIRP18`"]
pub type DIRP18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP18`"]
pub struct DIRP18_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP18_W<'a> {
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
#[doc = "Reader of field `DIRP19`"]
pub type DIRP19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP19`"]
pub struct DIRP19_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP19_W<'a> {
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
#[doc = "Reader of field `DIRP20`"]
pub type DIRP20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP20`"]
pub struct DIRP20_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP20_W<'a> {
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
#[doc = "Reader of field `DIRP21`"]
pub type DIRP21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP21`"]
pub struct DIRP21_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP21_W<'a> {
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
#[doc = "Reader of field `DIRP22`"]
pub type DIRP22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP22`"]
pub struct DIRP22_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP22_W<'a> {
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
#[doc = "Reader of field `DIRP23`"]
pub type DIRP23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP23`"]
pub struct DIRP23_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP23_W<'a> {
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
#[doc = "Reader of field `DIRP24`"]
pub type DIRP24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP24`"]
pub struct DIRP24_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP24_W<'a> {
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
#[doc = "Reader of field `DIRP25`"]
pub type DIRP25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP25`"]
pub struct DIRP25_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP25_W<'a> {
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
#[doc = "Reader of field `DIRP26`"]
pub type DIRP26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP26`"]
pub struct DIRP26_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP26_W<'a> {
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
#[doc = "Reader of field `DIRP27`"]
pub type DIRP27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP27`"]
pub struct DIRP27_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP27_W<'a> {
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
#[doc = "Reader of field `DIRP28`"]
pub type DIRP28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP28`"]
pub struct DIRP28_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP28_W<'a> {
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
#[doc = "Reader of field `DIRP29`"]
pub type DIRP29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP29`"]
pub struct DIRP29_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP29_W<'a> {
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
#[doc = "Reader of field `DIRP30`"]
pub type DIRP30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP30`"]
pub struct DIRP30_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP30_W<'a> {
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
#[doc = "Reader of field `DIRP31`"]
pub type DIRP31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRP31`"]
pub struct DIRP31_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP31_W<'a> {
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
    #[doc = "Bit 0 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp0(&self) -> DIRP0_R {
        DIRP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp1(&self) -> DIRP1_R {
        DIRP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp2(&self) -> DIRP2_R {
        DIRP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp3(&self) -> DIRP3_R {
        DIRP3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp4(&self) -> DIRP4_R {
        DIRP4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp5(&self) -> DIRP5_R {
        DIRP5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp6(&self) -> DIRP6_R {
        DIRP6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp7(&self) -> DIRP7_R {
        DIRP7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp8(&self) -> DIRP8_R {
        DIRP8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp9(&self) -> DIRP9_R {
        DIRP9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp10(&self) -> DIRP10_R {
        DIRP10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp11(&self) -> DIRP11_R {
        DIRP11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp12(&self) -> DIRP12_R {
        DIRP12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp13(&self) -> DIRP13_R {
        DIRP13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp14(&self) -> DIRP14_R {
        DIRP14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp15(&self) -> DIRP15_R {
        DIRP15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp16(&self) -> DIRP16_R {
        DIRP16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp17(&self) -> DIRP17_R {
        DIRP17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp18(&self) -> DIRP18_R {
        DIRP18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp19(&self) -> DIRP19_R {
        DIRP19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp20(&self) -> DIRP20_R {
        DIRP20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp21(&self) -> DIRP21_R {
        DIRP21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp22(&self) -> DIRP22_R {
        DIRP22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp23(&self) -> DIRP23_R {
        DIRP23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp24(&self) -> DIRP24_R {
        DIRP24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp25(&self) -> DIRP25_R {
        DIRP25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp26(&self) -> DIRP26_R {
        DIRP26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp27(&self) -> DIRP27_R {
        DIRP27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp28(&self) -> DIRP28_R {
        DIRP28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp29(&self) -> DIRP29_R {
        DIRP29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp30(&self) -> DIRP30_R {
        DIRP30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp31(&self) -> DIRP31_R {
        DIRP31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp0(&mut self) -> DIRP0_W {
        DIRP0_W { w: self }
    }
    #[doc = "Bit 1 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp1(&mut self) -> DIRP1_W {
        DIRP1_W { w: self }
    }
    #[doc = "Bit 2 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp2(&mut self) -> DIRP2_W {
        DIRP2_W { w: self }
    }
    #[doc = "Bit 3 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp3(&mut self) -> DIRP3_W {
        DIRP3_W { w: self }
    }
    #[doc = "Bit 4 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp4(&mut self) -> DIRP4_W {
        DIRP4_W { w: self }
    }
    #[doc = "Bit 5 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp5(&mut self) -> DIRP5_W {
        DIRP5_W { w: self }
    }
    #[doc = "Bit 6 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp6(&mut self) -> DIRP6_W {
        DIRP6_W { w: self }
    }
    #[doc = "Bit 7 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp7(&mut self) -> DIRP7_W {
        DIRP7_W { w: self }
    }
    #[doc = "Bit 8 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp8(&mut self) -> DIRP8_W {
        DIRP8_W { w: self }
    }
    #[doc = "Bit 9 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp9(&mut self) -> DIRP9_W {
        DIRP9_W { w: self }
    }
    #[doc = "Bit 10 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp10(&mut self) -> DIRP10_W {
        DIRP10_W { w: self }
    }
    #[doc = "Bit 11 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp11(&mut self) -> DIRP11_W {
        DIRP11_W { w: self }
    }
    #[doc = "Bit 12 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp12(&mut self) -> DIRP12_W {
        DIRP12_W { w: self }
    }
    #[doc = "Bit 13 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp13(&mut self) -> DIRP13_W {
        DIRP13_W { w: self }
    }
    #[doc = "Bit 14 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp14(&mut self) -> DIRP14_W {
        DIRP14_W { w: self }
    }
    #[doc = "Bit 15 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp15(&mut self) -> DIRP15_W {
        DIRP15_W { w: self }
    }
    #[doc = "Bit 16 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp16(&mut self) -> DIRP16_W {
        DIRP16_W { w: self }
    }
    #[doc = "Bit 17 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp17(&mut self) -> DIRP17_W {
        DIRP17_W { w: self }
    }
    #[doc = "Bit 18 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp18(&mut self) -> DIRP18_W {
        DIRP18_W { w: self }
    }
    #[doc = "Bit 19 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp19(&mut self) -> DIRP19_W {
        DIRP19_W { w: self }
    }
    #[doc = "Bit 20 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp20(&mut self) -> DIRP20_W {
        DIRP20_W { w: self }
    }
    #[doc = "Bit 21 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp21(&mut self) -> DIRP21_W {
        DIRP21_W { w: self }
    }
    #[doc = "Bit 22 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp22(&mut self) -> DIRP22_W {
        DIRP22_W { w: self }
    }
    #[doc = "Bit 23 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp23(&mut self) -> DIRP23_W {
        DIRP23_W { w: self }
    }
    #[doc = "Bit 24 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp24(&mut self) -> DIRP24_W {
        DIRP24_W { w: self }
    }
    #[doc = "Bit 25 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp25(&mut self) -> DIRP25_W {
        DIRP25_W { w: self }
    }
    #[doc = "Bit 26 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp26(&mut self) -> DIRP26_W {
        DIRP26_W { w: self }
    }
    #[doc = "Bit 27 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp27(&mut self) -> DIRP27_W {
        DIRP27_W { w: self }
    }
    #[doc = "Bit 28 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp28(&mut self) -> DIRP28_W {
        DIRP28_W { w: self }
    }
    #[doc = "Bit 29 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp29(&mut self) -> DIRP29_W {
        DIRP29_W { w: self }
    }
    #[doc = "Bit 30 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp30(&mut self) -> DIRP30_W {
        DIRP30_W { w: self }
    }
    #[doc = "Bit 31 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp31(&mut self) -> DIRP31_W {
        DIRP31_W { w: self }
    }
}
