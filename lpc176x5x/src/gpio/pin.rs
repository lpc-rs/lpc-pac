#[doc = "Reader of register PIN%s"]
pub type R = crate::R<u32, super::PIN>;
#[doc = "Writer for register PIN%s"]
pub type W = crate::W<u32, super::PIN>;
#[doc = "Register PIN%s `reset()`'s with value 0"]
impl crate::ResetValue for super::PIN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PINVAL0`"]
pub type PINVAL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL0`"]
pub struct PINVAL0_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL0_W<'a> {
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
#[doc = "Reader of field `PINVAL1`"]
pub type PINVAL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL1`"]
pub struct PINVAL1_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL1_W<'a> {
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
#[doc = "Reader of field `PINVAL2`"]
pub type PINVAL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL2`"]
pub struct PINVAL2_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL2_W<'a> {
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
#[doc = "Reader of field `PINVAL3`"]
pub type PINVAL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL3`"]
pub struct PINVAL3_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL3_W<'a> {
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
#[doc = "Reader of field `PINVAL4`"]
pub type PINVAL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL4`"]
pub struct PINVAL4_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL4_W<'a> {
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
#[doc = "Reader of field `PINVAL5`"]
pub type PINVAL5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL5`"]
pub struct PINVAL5_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL5_W<'a> {
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
#[doc = "Reader of field `PINVAL6`"]
pub type PINVAL6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL6`"]
pub struct PINVAL6_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL6_W<'a> {
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
#[doc = "Reader of field `PINVAL7`"]
pub type PINVAL7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL7`"]
pub struct PINVAL7_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL7_W<'a> {
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
#[doc = "Reader of field `PINVAL8`"]
pub type PINVAL8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL8`"]
pub struct PINVAL8_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL8_W<'a> {
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
#[doc = "Reader of field `PINVAL9`"]
pub type PINVAL9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL9`"]
pub struct PINVAL9_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL9_W<'a> {
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
#[doc = "Reader of field `PINVAL10`"]
pub type PINVAL10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL10`"]
pub struct PINVAL10_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL10_W<'a> {
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
#[doc = "Reader of field `PINVAL11`"]
pub type PINVAL11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL11`"]
pub struct PINVAL11_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL11_W<'a> {
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
#[doc = "Reader of field `PINVAL12`"]
pub type PINVAL12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL12`"]
pub struct PINVAL12_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL12_W<'a> {
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
#[doc = "Reader of field `PINVAL13`"]
pub type PINVAL13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL13`"]
pub struct PINVAL13_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL13_W<'a> {
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
#[doc = "Reader of field `PINVAL14`"]
pub type PINVAL14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL14`"]
pub struct PINVAL14_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL14_W<'a> {
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
#[doc = "Reader of field `PINVAL15`"]
pub type PINVAL15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL15`"]
pub struct PINVAL15_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL15_W<'a> {
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
#[doc = "Reader of field `PINVAL16`"]
pub type PINVAL16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL16`"]
pub struct PINVAL16_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL16_W<'a> {
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
#[doc = "Reader of field `PINVAL17`"]
pub type PINVAL17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL17`"]
pub struct PINVAL17_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL17_W<'a> {
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
#[doc = "Reader of field `PINVAL18`"]
pub type PINVAL18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL18`"]
pub struct PINVAL18_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL18_W<'a> {
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
#[doc = "Reader of field `PINVAL19`"]
pub type PINVAL19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL19`"]
pub struct PINVAL19_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL19_W<'a> {
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
#[doc = "Reader of field `PINVAL20`"]
pub type PINVAL20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL20`"]
pub struct PINVAL20_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL20_W<'a> {
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
#[doc = "Reader of field `PINVAL21`"]
pub type PINVAL21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL21`"]
pub struct PINVAL21_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL21_W<'a> {
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
#[doc = "Reader of field `PINVAL22`"]
pub type PINVAL22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL22`"]
pub struct PINVAL22_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL22_W<'a> {
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
#[doc = "Reader of field `PINVAL23`"]
pub type PINVAL23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL23`"]
pub struct PINVAL23_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL23_W<'a> {
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
#[doc = "Reader of field `PINVAL24`"]
pub type PINVAL24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL24`"]
pub struct PINVAL24_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL24_W<'a> {
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
#[doc = "Reader of field `PINVAL25`"]
pub type PINVAL25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL25`"]
pub struct PINVAL25_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL25_W<'a> {
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
#[doc = "Reader of field `PINVAL26`"]
pub type PINVAL26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL26`"]
pub struct PINVAL26_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL26_W<'a> {
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
#[doc = "Reader of field `PINVAL27`"]
pub type PINVAL27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL27`"]
pub struct PINVAL27_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL27_W<'a> {
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
#[doc = "Reader of field `PINVAL28`"]
pub type PINVAL28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL28`"]
pub struct PINVAL28_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL28_W<'a> {
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
#[doc = "Reader of field `PINVAL29`"]
pub type PINVAL29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL29`"]
pub struct PINVAL29_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL29_W<'a> {
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
#[doc = "Reader of field `PINVAL30`"]
pub type PINVAL30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL30`"]
pub struct PINVAL30_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL30_W<'a> {
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
#[doc = "Reader of field `PINVAL31`"]
pub type PINVAL31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINVAL31`"]
pub struct PINVAL31_W<'a> {
    w: &'a mut W,
}
impl<'a> PINVAL31_W<'a> {
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
    #[doc = "Bit 0 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval0(&self) -> PINVAL0_R {
        PINVAL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval1(&self) -> PINVAL1_R {
        PINVAL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval2(&self) -> PINVAL2_R {
        PINVAL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval3(&self) -> PINVAL3_R {
        PINVAL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval4(&self) -> PINVAL4_R {
        PINVAL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval5(&self) -> PINVAL5_R {
        PINVAL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval6(&self) -> PINVAL6_R {
        PINVAL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval7(&self) -> PINVAL7_R {
        PINVAL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval8(&self) -> PINVAL8_R {
        PINVAL8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval9(&self) -> PINVAL9_R {
        PINVAL9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval10(&self) -> PINVAL10_R {
        PINVAL10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval11(&self) -> PINVAL11_R {
        PINVAL11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval12(&self) -> PINVAL12_R {
        PINVAL12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval13(&self) -> PINVAL13_R {
        PINVAL13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval14(&self) -> PINVAL14_R {
        PINVAL14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval15(&self) -> PINVAL15_R {
        PINVAL15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval16(&self) -> PINVAL16_R {
        PINVAL16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval17(&self) -> PINVAL17_R {
        PINVAL17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval18(&self) -> PINVAL18_R {
        PINVAL18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval19(&self) -> PINVAL19_R {
        PINVAL19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval20(&self) -> PINVAL20_R {
        PINVAL20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval21(&self) -> PINVAL21_R {
        PINVAL21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval22(&self) -> PINVAL22_R {
        PINVAL22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval23(&self) -> PINVAL23_R {
        PINVAL23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval24(&self) -> PINVAL24_R {
        PINVAL24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval25(&self) -> PINVAL25_R {
        PINVAL25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval26(&self) -> PINVAL26_R {
        PINVAL26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval27(&self) -> PINVAL27_R {
        PINVAL27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval28(&self) -> PINVAL28_R {
        PINVAL28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval29(&self) -> PINVAL29_R {
        PINVAL29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval30(&self) -> PINVAL30_R {
        PINVAL30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval31(&self) -> PINVAL31_R {
        PINVAL31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval0(&mut self) -> PINVAL0_W {
        PINVAL0_W { w: self }
    }
    #[doc = "Bit 1 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval1(&mut self) -> PINVAL1_W {
        PINVAL1_W { w: self }
    }
    #[doc = "Bit 2 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval2(&mut self) -> PINVAL2_W {
        PINVAL2_W { w: self }
    }
    #[doc = "Bit 3 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval3(&mut self) -> PINVAL3_W {
        PINVAL3_W { w: self }
    }
    #[doc = "Bit 4 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval4(&mut self) -> PINVAL4_W {
        PINVAL4_W { w: self }
    }
    #[doc = "Bit 5 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval5(&mut self) -> PINVAL5_W {
        PINVAL5_W { w: self }
    }
    #[doc = "Bit 6 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval6(&mut self) -> PINVAL6_W {
        PINVAL6_W { w: self }
    }
    #[doc = "Bit 7 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval7(&mut self) -> PINVAL7_W {
        PINVAL7_W { w: self }
    }
    #[doc = "Bit 8 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval8(&mut self) -> PINVAL8_W {
        PINVAL8_W { w: self }
    }
    #[doc = "Bit 9 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval9(&mut self) -> PINVAL9_W {
        PINVAL9_W { w: self }
    }
    #[doc = "Bit 10 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval10(&mut self) -> PINVAL10_W {
        PINVAL10_W { w: self }
    }
    #[doc = "Bit 11 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval11(&mut self) -> PINVAL11_W {
        PINVAL11_W { w: self }
    }
    #[doc = "Bit 12 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval12(&mut self) -> PINVAL12_W {
        PINVAL12_W { w: self }
    }
    #[doc = "Bit 13 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval13(&mut self) -> PINVAL13_W {
        PINVAL13_W { w: self }
    }
    #[doc = "Bit 14 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval14(&mut self) -> PINVAL14_W {
        PINVAL14_W { w: self }
    }
    #[doc = "Bit 15 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval15(&mut self) -> PINVAL15_W {
        PINVAL15_W { w: self }
    }
    #[doc = "Bit 16 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval16(&mut self) -> PINVAL16_W {
        PINVAL16_W { w: self }
    }
    #[doc = "Bit 17 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval17(&mut self) -> PINVAL17_W {
        PINVAL17_W { w: self }
    }
    #[doc = "Bit 18 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval18(&mut self) -> PINVAL18_W {
        PINVAL18_W { w: self }
    }
    #[doc = "Bit 19 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval19(&mut self) -> PINVAL19_W {
        PINVAL19_W { w: self }
    }
    #[doc = "Bit 20 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval20(&mut self) -> PINVAL20_W {
        PINVAL20_W { w: self }
    }
    #[doc = "Bit 21 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval21(&mut self) -> PINVAL21_W {
        PINVAL21_W { w: self }
    }
    #[doc = "Bit 22 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval22(&mut self) -> PINVAL22_W {
        PINVAL22_W { w: self }
    }
    #[doc = "Bit 23 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval23(&mut self) -> PINVAL23_W {
        PINVAL23_W { w: self }
    }
    #[doc = "Bit 24 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval24(&mut self) -> PINVAL24_W {
        PINVAL24_W { w: self }
    }
    #[doc = "Bit 25 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval25(&mut self) -> PINVAL25_W {
        PINVAL25_W { w: self }
    }
    #[doc = "Bit 26 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval26(&mut self) -> PINVAL26_W {
        PINVAL26_W { w: self }
    }
    #[doc = "Bit 27 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval27(&mut self) -> PINVAL27_W {
        PINVAL27_W { w: self }
    }
    #[doc = "Bit 28 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval28(&mut self) -> PINVAL28_W {
        PINVAL28_W { w: self }
    }
    #[doc = "Bit 29 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval29(&mut self) -> PINVAL29_W {
        PINVAL29_W { w: self }
    }
    #[doc = "Bit 30 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval30(&mut self) -> PINVAL30_W {
        PINVAL30_W { w: self }
    }
    #[doc = "Bit 31 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinval31(&mut self) -> PINVAL31_W {
        PINVAL31_W { w: self }
    }
}
