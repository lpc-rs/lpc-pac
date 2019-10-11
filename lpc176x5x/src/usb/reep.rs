#[doc = "Reader of register REEP"]
pub type R = crate::R<u32, super::REEP>;
#[doc = "Writer for register REEP"]
pub type W = crate::W<u32, super::REEP>;
#[doc = "Register REEP `reset()`'s with value 0x03"]
impl crate::ResetValue for super::REEP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Reader of field `EPR0`"]
pub type EPR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR0`"]
pub struct EPR0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR0_W<'a> {
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
#[doc = "Reader of field `EPR1`"]
pub type EPR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR1`"]
pub struct EPR1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR1_W<'a> {
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
#[doc = "Reader of field `EPR2`"]
pub type EPR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR2`"]
pub struct EPR2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR2_W<'a> {
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
#[doc = "Reader of field `EPR3`"]
pub type EPR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR3`"]
pub struct EPR3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR3_W<'a> {
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
#[doc = "Reader of field `EPR4`"]
pub type EPR4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR4`"]
pub struct EPR4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR4_W<'a> {
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
#[doc = "Reader of field `EPR5`"]
pub type EPR5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR5`"]
pub struct EPR5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR5_W<'a> {
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
#[doc = "Reader of field `EPR6`"]
pub type EPR6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR6`"]
pub struct EPR6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR6_W<'a> {
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
#[doc = "Reader of field `EPR7`"]
pub type EPR7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR7`"]
pub struct EPR7_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR7_W<'a> {
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
#[doc = "Reader of field `EPR8`"]
pub type EPR8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR8`"]
pub struct EPR8_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR8_W<'a> {
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
#[doc = "Reader of field `EPR9`"]
pub type EPR9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR9`"]
pub struct EPR9_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR9_W<'a> {
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
#[doc = "Reader of field `EPR10`"]
pub type EPR10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR10`"]
pub struct EPR10_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR10_W<'a> {
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
#[doc = "Reader of field `EPR11`"]
pub type EPR11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR11`"]
pub struct EPR11_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR11_W<'a> {
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
#[doc = "Reader of field `EPR12`"]
pub type EPR12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR12`"]
pub struct EPR12_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR12_W<'a> {
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
#[doc = "Reader of field `EPR13`"]
pub type EPR13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR13`"]
pub struct EPR13_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR13_W<'a> {
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
#[doc = "Reader of field `EPR14`"]
pub type EPR14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR14`"]
pub struct EPR14_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR14_W<'a> {
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
#[doc = "Reader of field `EPR15`"]
pub type EPR15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR15`"]
pub struct EPR15_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR15_W<'a> {
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
#[doc = "Reader of field `EPR16`"]
pub type EPR16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR16`"]
pub struct EPR16_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR16_W<'a> {
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
#[doc = "Reader of field `EPR17`"]
pub type EPR17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR17`"]
pub struct EPR17_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR17_W<'a> {
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
#[doc = "Reader of field `EPR18`"]
pub type EPR18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR18`"]
pub struct EPR18_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR18_W<'a> {
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
#[doc = "Reader of field `EPR19`"]
pub type EPR19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR19`"]
pub struct EPR19_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR19_W<'a> {
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
#[doc = "Reader of field `EPR20`"]
pub type EPR20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR20`"]
pub struct EPR20_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR20_W<'a> {
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
#[doc = "Reader of field `EPR21`"]
pub type EPR21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR21`"]
pub struct EPR21_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR21_W<'a> {
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
#[doc = "Reader of field `EPR22`"]
pub type EPR22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR22`"]
pub struct EPR22_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR22_W<'a> {
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
#[doc = "Reader of field `EPR23`"]
pub type EPR23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR23`"]
pub struct EPR23_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR23_W<'a> {
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
#[doc = "Reader of field `EPR24`"]
pub type EPR24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR24`"]
pub struct EPR24_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR24_W<'a> {
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
#[doc = "Reader of field `EPR25`"]
pub type EPR25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR25`"]
pub struct EPR25_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR25_W<'a> {
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
#[doc = "Reader of field `EPR26`"]
pub type EPR26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR26`"]
pub struct EPR26_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR26_W<'a> {
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
#[doc = "Reader of field `EPR27`"]
pub type EPR27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR27`"]
pub struct EPR27_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR27_W<'a> {
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
#[doc = "Reader of field `EPR28`"]
pub type EPR28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR28`"]
pub struct EPR28_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR28_W<'a> {
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
#[doc = "Reader of field `EPR29`"]
pub type EPR29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR29`"]
pub struct EPR29_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR29_W<'a> {
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
#[doc = "Reader of field `EPR30`"]
pub type EPR30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR30`"]
pub struct EPR30_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR30_W<'a> {
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
#[doc = "Reader of field `EPR31`"]
pub type EPR31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPR31`"]
pub struct EPR31_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR31_W<'a> {
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
    #[doc = "Bit 0 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr0(&self) -> EPR0_R {
        EPR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr1(&self) -> EPR1_R {
        EPR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr2(&self) -> EPR2_R {
        EPR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr3(&self) -> EPR3_R {
        EPR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr4(&self) -> EPR4_R {
        EPR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr5(&self) -> EPR5_R {
        EPR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr6(&self) -> EPR6_R {
        EPR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr7(&self) -> EPR7_R {
        EPR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr8(&self) -> EPR8_R {
        EPR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr9(&self) -> EPR9_R {
        EPR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr10(&self) -> EPR10_R {
        EPR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr11(&self) -> EPR11_R {
        EPR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr12(&self) -> EPR12_R {
        EPR12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr13(&self) -> EPR13_R {
        EPR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr14(&self) -> EPR14_R {
        EPR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr15(&self) -> EPR15_R {
        EPR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr16(&self) -> EPR16_R {
        EPR16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr17(&self) -> EPR17_R {
        EPR17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr18(&self) -> EPR18_R {
        EPR18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr19(&self) -> EPR19_R {
        EPR19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr20(&self) -> EPR20_R {
        EPR20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr21(&self) -> EPR21_R {
        EPR21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr22(&self) -> EPR22_R {
        EPR22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr23(&self) -> EPR23_R {
        EPR23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr24(&self) -> EPR24_R {
        EPR24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr25(&self) -> EPR25_R {
        EPR25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr26(&self) -> EPR26_R {
        EPR26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr27(&self) -> EPR27_R {
        EPR27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr28(&self) -> EPR28_R {
        EPR28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr29(&self) -> EPR29_R {
        EPR29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr30(&self) -> EPR30_R {
        EPR30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr31(&self) -> EPR31_R {
        EPR31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr0(&mut self) -> EPR0_W {
        EPR0_W { w: self }
    }
    #[doc = "Bit 1 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr1(&mut self) -> EPR1_W {
        EPR1_W { w: self }
    }
    #[doc = "Bit 2 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr2(&mut self) -> EPR2_W {
        EPR2_W { w: self }
    }
    #[doc = "Bit 3 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr3(&mut self) -> EPR3_W {
        EPR3_W { w: self }
    }
    #[doc = "Bit 4 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr4(&mut self) -> EPR4_W {
        EPR4_W { w: self }
    }
    #[doc = "Bit 5 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr5(&mut self) -> EPR5_W {
        EPR5_W { w: self }
    }
    #[doc = "Bit 6 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr6(&mut self) -> EPR6_W {
        EPR6_W { w: self }
    }
    #[doc = "Bit 7 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr7(&mut self) -> EPR7_W {
        EPR7_W { w: self }
    }
    #[doc = "Bit 8 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr8(&mut self) -> EPR8_W {
        EPR8_W { w: self }
    }
    #[doc = "Bit 9 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr9(&mut self) -> EPR9_W {
        EPR9_W { w: self }
    }
    #[doc = "Bit 10 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr10(&mut self) -> EPR10_W {
        EPR10_W { w: self }
    }
    #[doc = "Bit 11 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr11(&mut self) -> EPR11_W {
        EPR11_W { w: self }
    }
    #[doc = "Bit 12 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr12(&mut self) -> EPR12_W {
        EPR12_W { w: self }
    }
    #[doc = "Bit 13 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr13(&mut self) -> EPR13_W {
        EPR13_W { w: self }
    }
    #[doc = "Bit 14 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr14(&mut self) -> EPR14_W {
        EPR14_W { w: self }
    }
    #[doc = "Bit 15 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr15(&mut self) -> EPR15_W {
        EPR15_W { w: self }
    }
    #[doc = "Bit 16 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr16(&mut self) -> EPR16_W {
        EPR16_W { w: self }
    }
    #[doc = "Bit 17 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr17(&mut self) -> EPR17_W {
        EPR17_W { w: self }
    }
    #[doc = "Bit 18 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr18(&mut self) -> EPR18_W {
        EPR18_W { w: self }
    }
    #[doc = "Bit 19 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr19(&mut self) -> EPR19_W {
        EPR19_W { w: self }
    }
    #[doc = "Bit 20 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr20(&mut self) -> EPR20_W {
        EPR20_W { w: self }
    }
    #[doc = "Bit 21 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr21(&mut self) -> EPR21_W {
        EPR21_W { w: self }
    }
    #[doc = "Bit 22 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr22(&mut self) -> EPR22_W {
        EPR22_W { w: self }
    }
    #[doc = "Bit 23 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr23(&mut self) -> EPR23_W {
        EPR23_W { w: self }
    }
    #[doc = "Bit 24 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr24(&mut self) -> EPR24_W {
        EPR24_W { w: self }
    }
    #[doc = "Bit 25 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr25(&mut self) -> EPR25_W {
        EPR25_W { w: self }
    }
    #[doc = "Bit 26 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr26(&mut self) -> EPR26_W {
        EPR26_W { w: self }
    }
    #[doc = "Bit 27 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr27(&mut self) -> EPR27_W {
        EPR27_W { w: self }
    }
    #[doc = "Bit 28 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr28(&mut self) -> EPR28_W {
        EPR28_W { w: self }
    }
    #[doc = "Bit 29 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr29(&mut self) -> EPR29_W {
        EPR29_W { w: self }
    }
    #[doc = "Bit 30 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr30(&mut self) -> EPR30_W {
        EPR30_W { w: self }
    }
    #[doc = "Bit 31 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr31(&mut self) -> EPR31_W {
        EPR31_W { w: self }
    }
}
