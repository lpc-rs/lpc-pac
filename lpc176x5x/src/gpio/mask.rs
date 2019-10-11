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
#[doc = "Reader of field `PINMASK0`"]
pub type PINMASK0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK0`"]
pub struct PINMASK0_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK0_W<'a> {
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
#[doc = "Reader of field `PINMASK1`"]
pub type PINMASK1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK1`"]
pub struct PINMASK1_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK1_W<'a> {
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
#[doc = "Reader of field `PINMASK2`"]
pub type PINMASK2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK2`"]
pub struct PINMASK2_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK2_W<'a> {
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
#[doc = "Reader of field `PINMASK3`"]
pub type PINMASK3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK3`"]
pub struct PINMASK3_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK3_W<'a> {
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
#[doc = "Reader of field `PINMASK4`"]
pub type PINMASK4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK4`"]
pub struct PINMASK4_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK4_W<'a> {
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
#[doc = "Reader of field `PINMASK5`"]
pub type PINMASK5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK5`"]
pub struct PINMASK5_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK5_W<'a> {
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
#[doc = "Reader of field `PINMASK6`"]
pub type PINMASK6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK6`"]
pub struct PINMASK6_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK6_W<'a> {
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
#[doc = "Reader of field `PINMASK7`"]
pub type PINMASK7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK7`"]
pub struct PINMASK7_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK7_W<'a> {
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
#[doc = "Reader of field `PINMASK8`"]
pub type PINMASK8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK8`"]
pub struct PINMASK8_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK8_W<'a> {
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
#[doc = "Reader of field `PINMASK9`"]
pub type PINMASK9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK9`"]
pub struct PINMASK9_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK9_W<'a> {
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
#[doc = "Reader of field `PINMASK10`"]
pub type PINMASK10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK10`"]
pub struct PINMASK10_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK10_W<'a> {
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
#[doc = "Reader of field `PINMASK11`"]
pub type PINMASK11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK11`"]
pub struct PINMASK11_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK11_W<'a> {
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
#[doc = "Reader of field `PINMASK12`"]
pub type PINMASK12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK12`"]
pub struct PINMASK12_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK12_W<'a> {
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
#[doc = "Reader of field `PINMASK13`"]
pub type PINMASK13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK13`"]
pub struct PINMASK13_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK13_W<'a> {
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
#[doc = "Reader of field `PINMASK14`"]
pub type PINMASK14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK14`"]
pub struct PINMASK14_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK14_W<'a> {
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
#[doc = "Reader of field `PINMASK15`"]
pub type PINMASK15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK15`"]
pub struct PINMASK15_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK15_W<'a> {
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
#[doc = "Reader of field `PINMASK16`"]
pub type PINMASK16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK16`"]
pub struct PINMASK16_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK16_W<'a> {
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
#[doc = "Reader of field `PINMASK17`"]
pub type PINMASK17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK17`"]
pub struct PINMASK17_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK17_W<'a> {
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
#[doc = "Reader of field `PINMASK18`"]
pub type PINMASK18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK18`"]
pub struct PINMASK18_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK18_W<'a> {
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
#[doc = "Reader of field `PINMASK19`"]
pub type PINMASK19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK19`"]
pub struct PINMASK19_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK19_W<'a> {
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
#[doc = "Reader of field `PINMASK20`"]
pub type PINMASK20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK20`"]
pub struct PINMASK20_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK20_W<'a> {
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
#[doc = "Reader of field `PINMASK21`"]
pub type PINMASK21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK21`"]
pub struct PINMASK21_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK21_W<'a> {
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
#[doc = "Reader of field `PINMASK22`"]
pub type PINMASK22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK22`"]
pub struct PINMASK22_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK22_W<'a> {
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
#[doc = "Reader of field `PINMASK23`"]
pub type PINMASK23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK23`"]
pub struct PINMASK23_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK23_W<'a> {
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
#[doc = "Reader of field `PINMASK24`"]
pub type PINMASK24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK24`"]
pub struct PINMASK24_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK24_W<'a> {
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
#[doc = "Reader of field `PINMASK25`"]
pub type PINMASK25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK25`"]
pub struct PINMASK25_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK25_W<'a> {
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
#[doc = "Reader of field `PINMASK26`"]
pub type PINMASK26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK26`"]
pub struct PINMASK26_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK26_W<'a> {
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
#[doc = "Reader of field `PINMASK27`"]
pub type PINMASK27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK27`"]
pub struct PINMASK27_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK27_W<'a> {
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
#[doc = "Reader of field `PINMASK28`"]
pub type PINMASK28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK28`"]
pub struct PINMASK28_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK28_W<'a> {
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
#[doc = "Reader of field `PINMASK29`"]
pub type PINMASK29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK29`"]
pub struct PINMASK29_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK29_W<'a> {
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
#[doc = "Reader of field `PINMASK30`"]
pub type PINMASK30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK30`"]
pub struct PINMASK30_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK30_W<'a> {
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
#[doc = "Reader of field `PINMASK31`"]
pub type PINMASK31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINMASK31`"]
pub struct PINMASK31_W<'a> {
    w: &'a mut W,
}
impl<'a> PINMASK31_W<'a> {
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
    #[doc = "Bit 0 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask0(&self) -> PINMASK0_R {
        PINMASK0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask1(&self) -> PINMASK1_R {
        PINMASK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask2(&self) -> PINMASK2_R {
        PINMASK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask3(&self) -> PINMASK3_R {
        PINMASK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask4(&self) -> PINMASK4_R {
        PINMASK4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask5(&self) -> PINMASK5_R {
        PINMASK5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask6(&self) -> PINMASK6_R {
        PINMASK6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask7(&self) -> PINMASK7_R {
        PINMASK7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask8(&self) -> PINMASK8_R {
        PINMASK8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask9(&self) -> PINMASK9_R {
        PINMASK9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask10(&self) -> PINMASK10_R {
        PINMASK10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask11(&self) -> PINMASK11_R {
        PINMASK11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask12(&self) -> PINMASK12_R {
        PINMASK12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask13(&self) -> PINMASK13_R {
        PINMASK13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask14(&self) -> PINMASK14_R {
        PINMASK14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask15(&self) -> PINMASK15_R {
        PINMASK15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask16(&self) -> PINMASK16_R {
        PINMASK16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask17(&self) -> PINMASK17_R {
        PINMASK17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask18(&self) -> PINMASK18_R {
        PINMASK18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask19(&self) -> PINMASK19_R {
        PINMASK19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask20(&self) -> PINMASK20_R {
        PINMASK20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask21(&self) -> PINMASK21_R {
        PINMASK21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask22(&self) -> PINMASK22_R {
        PINMASK22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask23(&self) -> PINMASK23_R {
        PINMASK23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask24(&self) -> PINMASK24_R {
        PINMASK24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask25(&self) -> PINMASK25_R {
        PINMASK25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask26(&self) -> PINMASK26_R {
        PINMASK26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask27(&self) -> PINMASK27_R {
        PINMASK27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask28(&self) -> PINMASK28_R {
        PINMASK28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask29(&self) -> PINMASK29_R {
        PINMASK29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask30(&self) -> PINMASK30_R {
        PINMASK30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask31(&self) -> PINMASK31_R {
        PINMASK31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask0(&mut self) -> PINMASK0_W {
        PINMASK0_W { w: self }
    }
    #[doc = "Bit 1 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask1(&mut self) -> PINMASK1_W {
        PINMASK1_W { w: self }
    }
    #[doc = "Bit 2 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask2(&mut self) -> PINMASK2_W {
        PINMASK2_W { w: self }
    }
    #[doc = "Bit 3 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask3(&mut self) -> PINMASK3_W {
        PINMASK3_W { w: self }
    }
    #[doc = "Bit 4 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask4(&mut self) -> PINMASK4_W {
        PINMASK4_W { w: self }
    }
    #[doc = "Bit 5 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask5(&mut self) -> PINMASK5_W {
        PINMASK5_W { w: self }
    }
    #[doc = "Bit 6 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask6(&mut self) -> PINMASK6_W {
        PINMASK6_W { w: self }
    }
    #[doc = "Bit 7 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask7(&mut self) -> PINMASK7_W {
        PINMASK7_W { w: self }
    }
    #[doc = "Bit 8 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask8(&mut self) -> PINMASK8_W {
        PINMASK8_W { w: self }
    }
    #[doc = "Bit 9 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask9(&mut self) -> PINMASK9_W {
        PINMASK9_W { w: self }
    }
    #[doc = "Bit 10 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask10(&mut self) -> PINMASK10_W {
        PINMASK10_W { w: self }
    }
    #[doc = "Bit 11 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask11(&mut self) -> PINMASK11_W {
        PINMASK11_W { w: self }
    }
    #[doc = "Bit 12 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask12(&mut self) -> PINMASK12_W {
        PINMASK12_W { w: self }
    }
    #[doc = "Bit 13 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask13(&mut self) -> PINMASK13_W {
        PINMASK13_W { w: self }
    }
    #[doc = "Bit 14 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask14(&mut self) -> PINMASK14_W {
        PINMASK14_W { w: self }
    }
    #[doc = "Bit 15 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask15(&mut self) -> PINMASK15_W {
        PINMASK15_W { w: self }
    }
    #[doc = "Bit 16 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask16(&mut self) -> PINMASK16_W {
        PINMASK16_W { w: self }
    }
    #[doc = "Bit 17 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask17(&mut self) -> PINMASK17_W {
        PINMASK17_W { w: self }
    }
    #[doc = "Bit 18 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask18(&mut self) -> PINMASK18_W {
        PINMASK18_W { w: self }
    }
    #[doc = "Bit 19 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask19(&mut self) -> PINMASK19_W {
        PINMASK19_W { w: self }
    }
    #[doc = "Bit 20 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask20(&mut self) -> PINMASK20_W {
        PINMASK20_W { w: self }
    }
    #[doc = "Bit 21 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask21(&mut self) -> PINMASK21_W {
        PINMASK21_W { w: self }
    }
    #[doc = "Bit 22 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask22(&mut self) -> PINMASK22_W {
        PINMASK22_W { w: self }
    }
    #[doc = "Bit 23 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask23(&mut self) -> PINMASK23_W {
        PINMASK23_W { w: self }
    }
    #[doc = "Bit 24 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask24(&mut self) -> PINMASK24_W {
        PINMASK24_W { w: self }
    }
    #[doc = "Bit 25 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask25(&mut self) -> PINMASK25_W {
        PINMASK25_W { w: self }
    }
    #[doc = "Bit 26 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask26(&mut self) -> PINMASK26_W {
        PINMASK26_W { w: self }
    }
    #[doc = "Bit 27 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask27(&mut self) -> PINMASK27_W {
        PINMASK27_W { w: self }
    }
    #[doc = "Bit 28 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask28(&mut self) -> PINMASK28_W {
        PINMASK28_W { w: self }
    }
    #[doc = "Bit 29 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask29(&mut self) -> PINMASK29_W {
        PINMASK29_W { w: self }
    }
    #[doc = "Bit 30 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask30(&mut self) -> PINMASK30_W {
        PINMASK30_W { w: self }
    }
    #[doc = "Bit 31 - Fast GPIO physical pin access control. 0 = Controlled pin is affected by writes to the port's SETx, CLRx, and PINx register(s). Current state of the pin can be read from the PINx register. 1 = Controlled pin is not affected by writes into the port's SETx, CLRx and PINx register(s). When the PINx register is read, this bit will not be updated with the state of the physical pin."]
    #[inline(always)]
    pub fn pinmask31(&mut self) -> PINMASK31_W {
        PINMASK31_W { w: self }
    }
}
