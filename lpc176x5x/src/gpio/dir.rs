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
#[doc = "Reader of field `PINDIR0`"]
pub type PINDIR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR0`"]
pub struct PINDIR0_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR0_W<'a> {
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
#[doc = "Reader of field `PINDIR1`"]
pub type PINDIR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR1`"]
pub struct PINDIR1_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR1_W<'a> {
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
#[doc = "Reader of field `PINDIR2`"]
pub type PINDIR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR2`"]
pub struct PINDIR2_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR2_W<'a> {
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
#[doc = "Reader of field `PINDIR3`"]
pub type PINDIR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR3`"]
pub struct PINDIR3_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR3_W<'a> {
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
#[doc = "Reader of field `PINDIR4`"]
pub type PINDIR4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR4`"]
pub struct PINDIR4_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR4_W<'a> {
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
#[doc = "Reader of field `PINDIR5`"]
pub type PINDIR5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR5`"]
pub struct PINDIR5_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR5_W<'a> {
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
#[doc = "Reader of field `PINDIR6`"]
pub type PINDIR6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR6`"]
pub struct PINDIR6_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR6_W<'a> {
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
#[doc = "Reader of field `PINDIR7`"]
pub type PINDIR7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR7`"]
pub struct PINDIR7_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR7_W<'a> {
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
#[doc = "Reader of field `PINDIR8`"]
pub type PINDIR8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR8`"]
pub struct PINDIR8_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR8_W<'a> {
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
#[doc = "Reader of field `PINDIR9`"]
pub type PINDIR9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR9`"]
pub struct PINDIR9_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR9_W<'a> {
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
#[doc = "Reader of field `PINDIR10`"]
pub type PINDIR10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR10`"]
pub struct PINDIR10_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR10_W<'a> {
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
#[doc = "Reader of field `PINDIR11`"]
pub type PINDIR11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR11`"]
pub struct PINDIR11_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR11_W<'a> {
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
#[doc = "Reader of field `PINDIR12`"]
pub type PINDIR12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR12`"]
pub struct PINDIR12_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR12_W<'a> {
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
#[doc = "Reader of field `PINDIR13`"]
pub type PINDIR13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR13`"]
pub struct PINDIR13_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR13_W<'a> {
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
#[doc = "Reader of field `PINDIR14`"]
pub type PINDIR14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR14`"]
pub struct PINDIR14_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR14_W<'a> {
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
#[doc = "Reader of field `PINDIR15`"]
pub type PINDIR15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR15`"]
pub struct PINDIR15_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR15_W<'a> {
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
#[doc = "Reader of field `PINDIR16`"]
pub type PINDIR16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR16`"]
pub struct PINDIR16_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR16_W<'a> {
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
#[doc = "Reader of field `PINDIR17`"]
pub type PINDIR17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR17`"]
pub struct PINDIR17_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR17_W<'a> {
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
#[doc = "Reader of field `PINDIR18`"]
pub type PINDIR18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR18`"]
pub struct PINDIR18_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR18_W<'a> {
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
#[doc = "Reader of field `PINDIR19`"]
pub type PINDIR19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR19`"]
pub struct PINDIR19_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR19_W<'a> {
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
#[doc = "Reader of field `PINDIR20`"]
pub type PINDIR20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR20`"]
pub struct PINDIR20_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR20_W<'a> {
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
#[doc = "Reader of field `PINDIR21`"]
pub type PINDIR21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR21`"]
pub struct PINDIR21_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR21_W<'a> {
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
#[doc = "Reader of field `PINDIR22`"]
pub type PINDIR22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR22`"]
pub struct PINDIR22_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR22_W<'a> {
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
#[doc = "Reader of field `PINDIR23`"]
pub type PINDIR23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR23`"]
pub struct PINDIR23_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR23_W<'a> {
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
#[doc = "Reader of field `PINDIR24`"]
pub type PINDIR24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR24`"]
pub struct PINDIR24_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR24_W<'a> {
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
#[doc = "Reader of field `PINDIR25`"]
pub type PINDIR25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR25`"]
pub struct PINDIR25_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR25_W<'a> {
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
#[doc = "Reader of field `PINDIR26`"]
pub type PINDIR26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR26`"]
pub struct PINDIR26_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR26_W<'a> {
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
#[doc = "Reader of field `PINDIR27`"]
pub type PINDIR27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR27`"]
pub struct PINDIR27_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR27_W<'a> {
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
#[doc = "Reader of field `PINDIR28`"]
pub type PINDIR28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR28`"]
pub struct PINDIR28_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR28_W<'a> {
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
#[doc = "Reader of field `PINDIR29`"]
pub type PINDIR29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR29`"]
pub struct PINDIR29_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR29_W<'a> {
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
#[doc = "Reader of field `PINDIR30`"]
pub type PINDIR30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR30`"]
pub struct PINDIR30_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR30_W<'a> {
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
#[doc = "Reader of field `PINDIR31`"]
pub type PINDIR31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINDIR31`"]
pub struct PINDIR31_W<'a> {
    w: &'a mut W,
}
impl<'a> PINDIR31_W<'a> {
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
    #[doc = "Bit 0 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir0(&self) -> PINDIR0_R {
        PINDIR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir1(&self) -> PINDIR1_R {
        PINDIR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir2(&self) -> PINDIR2_R {
        PINDIR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir3(&self) -> PINDIR3_R {
        PINDIR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir4(&self) -> PINDIR4_R {
        PINDIR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir5(&self) -> PINDIR5_R {
        PINDIR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir6(&self) -> PINDIR6_R {
        PINDIR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir7(&self) -> PINDIR7_R {
        PINDIR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir8(&self) -> PINDIR8_R {
        PINDIR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir9(&self) -> PINDIR9_R {
        PINDIR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir10(&self) -> PINDIR10_R {
        PINDIR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir11(&self) -> PINDIR11_R {
        PINDIR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir12(&self) -> PINDIR12_R {
        PINDIR12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir13(&self) -> PINDIR13_R {
        PINDIR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir14(&self) -> PINDIR14_R {
        PINDIR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir15(&self) -> PINDIR15_R {
        PINDIR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir16(&self) -> PINDIR16_R {
        PINDIR16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir17(&self) -> PINDIR17_R {
        PINDIR17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir18(&self) -> PINDIR18_R {
        PINDIR18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir19(&self) -> PINDIR19_R {
        PINDIR19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir20(&self) -> PINDIR20_R {
        PINDIR20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir21(&self) -> PINDIR21_R {
        PINDIR21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir22(&self) -> PINDIR22_R {
        PINDIR22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir23(&self) -> PINDIR23_R {
        PINDIR23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir24(&self) -> PINDIR24_R {
        PINDIR24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir25(&self) -> PINDIR25_R {
        PINDIR25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir26(&self) -> PINDIR26_R {
        PINDIR26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir27(&self) -> PINDIR27_R {
        PINDIR27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir28(&self) -> PINDIR28_R {
        PINDIR28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir29(&self) -> PINDIR29_R {
        PINDIR29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir30(&self) -> PINDIR30_R {
        PINDIR30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir31(&self) -> PINDIR31_R {
        PINDIR31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir0(&mut self) -> PINDIR0_W {
        PINDIR0_W { w: self }
    }
    #[doc = "Bit 1 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir1(&mut self) -> PINDIR1_W {
        PINDIR1_W { w: self }
    }
    #[doc = "Bit 2 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir2(&mut self) -> PINDIR2_W {
        PINDIR2_W { w: self }
    }
    #[doc = "Bit 3 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir3(&mut self) -> PINDIR3_W {
        PINDIR3_W { w: self }
    }
    #[doc = "Bit 4 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir4(&mut self) -> PINDIR4_W {
        PINDIR4_W { w: self }
    }
    #[doc = "Bit 5 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir5(&mut self) -> PINDIR5_W {
        PINDIR5_W { w: self }
    }
    #[doc = "Bit 6 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir6(&mut self) -> PINDIR6_W {
        PINDIR6_W { w: self }
    }
    #[doc = "Bit 7 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir7(&mut self) -> PINDIR7_W {
        PINDIR7_W { w: self }
    }
    #[doc = "Bit 8 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir8(&mut self) -> PINDIR8_W {
        PINDIR8_W { w: self }
    }
    #[doc = "Bit 9 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir9(&mut self) -> PINDIR9_W {
        PINDIR9_W { w: self }
    }
    #[doc = "Bit 10 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir10(&mut self) -> PINDIR10_W {
        PINDIR10_W { w: self }
    }
    #[doc = "Bit 11 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir11(&mut self) -> PINDIR11_W {
        PINDIR11_W { w: self }
    }
    #[doc = "Bit 12 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir12(&mut self) -> PINDIR12_W {
        PINDIR12_W { w: self }
    }
    #[doc = "Bit 13 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir13(&mut self) -> PINDIR13_W {
        PINDIR13_W { w: self }
    }
    #[doc = "Bit 14 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir14(&mut self) -> PINDIR14_W {
        PINDIR14_W { w: self }
    }
    #[doc = "Bit 15 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir15(&mut self) -> PINDIR15_W {
        PINDIR15_W { w: self }
    }
    #[doc = "Bit 16 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir16(&mut self) -> PINDIR16_W {
        PINDIR16_W { w: self }
    }
    #[doc = "Bit 17 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir17(&mut self) -> PINDIR17_W {
        PINDIR17_W { w: self }
    }
    #[doc = "Bit 18 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir18(&mut self) -> PINDIR18_W {
        PINDIR18_W { w: self }
    }
    #[doc = "Bit 19 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir19(&mut self) -> PINDIR19_W {
        PINDIR19_W { w: self }
    }
    #[doc = "Bit 20 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir20(&mut self) -> PINDIR20_W {
        PINDIR20_W { w: self }
    }
    #[doc = "Bit 21 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir21(&mut self) -> PINDIR21_W {
        PINDIR21_W { w: self }
    }
    #[doc = "Bit 22 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir22(&mut self) -> PINDIR22_W {
        PINDIR22_W { w: self }
    }
    #[doc = "Bit 23 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir23(&mut self) -> PINDIR23_W {
        PINDIR23_W { w: self }
    }
    #[doc = "Bit 24 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir24(&mut self) -> PINDIR24_W {
        PINDIR24_W { w: self }
    }
    #[doc = "Bit 25 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir25(&mut self) -> PINDIR25_W {
        PINDIR25_W { w: self }
    }
    #[doc = "Bit 26 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir26(&mut self) -> PINDIR26_W {
        PINDIR26_W { w: self }
    }
    #[doc = "Bit 27 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir27(&mut self) -> PINDIR27_W {
        PINDIR27_W { w: self }
    }
    #[doc = "Bit 28 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir28(&mut self) -> PINDIR28_W {
        PINDIR28_W { w: self }
    }
    #[doc = "Bit 29 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir29(&mut self) -> PINDIR29_W {
        PINDIR29_W { w: self }
    }
    #[doc = "Bit 30 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir30(&mut self) -> PINDIR30_W {
        PINDIR30_W { w: self }
    }
    #[doc = "Bit 31 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline(always)]
    pub fn pindir31(&mut self) -> PINDIR31_W {
        PINDIR31_W { w: self }
    }
}
