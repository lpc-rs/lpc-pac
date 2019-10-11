#[doc = "Reader of register EPINTEN"]
pub type R = crate::R<u32, super::EPINTEN>;
#[doc = "Writer for register EPINTEN"]
pub type W = crate::W<u32, super::EPINTEN>;
#[doc = "Register EPINTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::EPINTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EPEN0`"]
pub type EPEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN0`"]
pub struct EPEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN0_W<'a> {
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
#[doc = "Reader of field `EPEN1`"]
pub type EPEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN1`"]
pub struct EPEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN1_W<'a> {
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
#[doc = "Reader of field `EPEN2`"]
pub type EPEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN2`"]
pub struct EPEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN2_W<'a> {
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
#[doc = "Reader of field `EPEN3`"]
pub type EPEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN3`"]
pub struct EPEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN3_W<'a> {
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
#[doc = "Reader of field `EPEN4`"]
pub type EPEN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN4`"]
pub struct EPEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN4_W<'a> {
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
#[doc = "Reader of field `EPEN5`"]
pub type EPEN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN5`"]
pub struct EPEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN5_W<'a> {
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
#[doc = "Reader of field `EPEN6`"]
pub type EPEN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN6`"]
pub struct EPEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN6_W<'a> {
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
#[doc = "Reader of field `EPEN7`"]
pub type EPEN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN7`"]
pub struct EPEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN7_W<'a> {
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
#[doc = "Reader of field `EPEN8`"]
pub type EPEN8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN8`"]
pub struct EPEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN8_W<'a> {
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
#[doc = "Reader of field `EPEN9`"]
pub type EPEN9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN9`"]
pub struct EPEN9_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN9_W<'a> {
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
#[doc = "Reader of field `EPEN10`"]
pub type EPEN10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN10`"]
pub struct EPEN10_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN10_W<'a> {
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
#[doc = "Reader of field `EPEN11`"]
pub type EPEN11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN11`"]
pub struct EPEN11_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN11_W<'a> {
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
#[doc = "Reader of field `EPEN12`"]
pub type EPEN12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN12`"]
pub struct EPEN12_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN12_W<'a> {
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
#[doc = "Reader of field `EPEN13`"]
pub type EPEN13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN13`"]
pub struct EPEN13_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN13_W<'a> {
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
#[doc = "Reader of field `EPEN14`"]
pub type EPEN14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN14`"]
pub struct EPEN14_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN14_W<'a> {
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
#[doc = "Reader of field `EPEN15`"]
pub type EPEN15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN15`"]
pub struct EPEN15_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN15_W<'a> {
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
#[doc = "Reader of field `EPEN16`"]
pub type EPEN16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN16`"]
pub struct EPEN16_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN16_W<'a> {
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
#[doc = "Reader of field `EPEN17`"]
pub type EPEN17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN17`"]
pub struct EPEN17_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN17_W<'a> {
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
#[doc = "Reader of field `EPEN18`"]
pub type EPEN18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN18`"]
pub struct EPEN18_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN18_W<'a> {
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
#[doc = "Reader of field `EPEN19`"]
pub type EPEN19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN19`"]
pub struct EPEN19_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN19_W<'a> {
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
#[doc = "Reader of field `EPEN20`"]
pub type EPEN20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN20`"]
pub struct EPEN20_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN20_W<'a> {
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
#[doc = "Reader of field `EPEN21`"]
pub type EPEN21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN21`"]
pub struct EPEN21_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN21_W<'a> {
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
#[doc = "Reader of field `EPEN22`"]
pub type EPEN22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN22`"]
pub struct EPEN22_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN22_W<'a> {
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
#[doc = "Reader of field `EPEN23`"]
pub type EPEN23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN23`"]
pub struct EPEN23_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN23_W<'a> {
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
#[doc = "Reader of field `EPEN24`"]
pub type EPEN24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN24`"]
pub struct EPEN24_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN24_W<'a> {
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
#[doc = "Reader of field `EPEN25`"]
pub type EPEN25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN25`"]
pub struct EPEN25_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN25_W<'a> {
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
#[doc = "Reader of field `EPEN26`"]
pub type EPEN26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN26`"]
pub struct EPEN26_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN26_W<'a> {
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
#[doc = "Reader of field `EPEN27`"]
pub type EPEN27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN27`"]
pub struct EPEN27_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN27_W<'a> {
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
#[doc = "Reader of field `EPEN28`"]
pub type EPEN28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN28`"]
pub struct EPEN28_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN28_W<'a> {
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
#[doc = "Reader of field `EPEN29`"]
pub type EPEN29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN29`"]
pub struct EPEN29_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN29_W<'a> {
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
#[doc = "Reader of field `EPEN30`"]
pub type EPEN30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN30`"]
pub struct EPEN30_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN30_W<'a> {
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
#[doc = "Reader of field `EPEN31`"]
pub type EPEN31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN31`"]
pub struct EPEN31_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN31_W<'a> {
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
    #[doc = "Bit 0 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen0(&self) -> EPEN0_R {
        EPEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen1(&self) -> EPEN1_R {
        EPEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen2(&self) -> EPEN2_R {
        EPEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen3(&self) -> EPEN3_R {
        EPEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen4(&self) -> EPEN4_R {
        EPEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen5(&self) -> EPEN5_R {
        EPEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen6(&self) -> EPEN6_R {
        EPEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen7(&self) -> EPEN7_R {
        EPEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen8(&self) -> EPEN8_R {
        EPEN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen9(&self) -> EPEN9_R {
        EPEN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen10(&self) -> EPEN10_R {
        EPEN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen11(&self) -> EPEN11_R {
        EPEN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen12(&self) -> EPEN12_R {
        EPEN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen13(&self) -> EPEN13_R {
        EPEN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen14(&self) -> EPEN14_R {
        EPEN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen15(&self) -> EPEN15_R {
        EPEN15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen16(&self) -> EPEN16_R {
        EPEN16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen17(&self) -> EPEN17_R {
        EPEN17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen18(&self) -> EPEN18_R {
        EPEN18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen19(&self) -> EPEN19_R {
        EPEN19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen20(&self) -> EPEN20_R {
        EPEN20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen21(&self) -> EPEN21_R {
        EPEN21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen22(&self) -> EPEN22_R {
        EPEN22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen23(&self) -> EPEN23_R {
        EPEN23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen24(&self) -> EPEN24_R {
        EPEN24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen25(&self) -> EPEN25_R {
        EPEN25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen26(&self) -> EPEN26_R {
        EPEN26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen27(&self) -> EPEN27_R {
        EPEN27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen28(&self) -> EPEN28_R {
        EPEN28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen29(&self) -> EPEN29_R {
        EPEN29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen30(&self) -> EPEN30_R {
        EPEN30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen31(&self) -> EPEN31_R {
        EPEN31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen0(&mut self) -> EPEN0_W {
        EPEN0_W { w: self }
    }
    #[doc = "Bit 1 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen1(&mut self) -> EPEN1_W {
        EPEN1_W { w: self }
    }
    #[doc = "Bit 2 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen2(&mut self) -> EPEN2_W {
        EPEN2_W { w: self }
    }
    #[doc = "Bit 3 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen3(&mut self) -> EPEN3_W {
        EPEN3_W { w: self }
    }
    #[doc = "Bit 4 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen4(&mut self) -> EPEN4_W {
        EPEN4_W { w: self }
    }
    #[doc = "Bit 5 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen5(&mut self) -> EPEN5_W {
        EPEN5_W { w: self }
    }
    #[doc = "Bit 6 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen6(&mut self) -> EPEN6_W {
        EPEN6_W { w: self }
    }
    #[doc = "Bit 7 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen7(&mut self) -> EPEN7_W {
        EPEN7_W { w: self }
    }
    #[doc = "Bit 8 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen8(&mut self) -> EPEN8_W {
        EPEN8_W { w: self }
    }
    #[doc = "Bit 9 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen9(&mut self) -> EPEN9_W {
        EPEN9_W { w: self }
    }
    #[doc = "Bit 10 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen10(&mut self) -> EPEN10_W {
        EPEN10_W { w: self }
    }
    #[doc = "Bit 11 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen11(&mut self) -> EPEN11_W {
        EPEN11_W { w: self }
    }
    #[doc = "Bit 12 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen12(&mut self) -> EPEN12_W {
        EPEN12_W { w: self }
    }
    #[doc = "Bit 13 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen13(&mut self) -> EPEN13_W {
        EPEN13_W { w: self }
    }
    #[doc = "Bit 14 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen14(&mut self) -> EPEN14_W {
        EPEN14_W { w: self }
    }
    #[doc = "Bit 15 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen15(&mut self) -> EPEN15_W {
        EPEN15_W { w: self }
    }
    #[doc = "Bit 16 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen16(&mut self) -> EPEN16_W {
        EPEN16_W { w: self }
    }
    #[doc = "Bit 17 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen17(&mut self) -> EPEN17_W {
        EPEN17_W { w: self }
    }
    #[doc = "Bit 18 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen18(&mut self) -> EPEN18_W {
        EPEN18_W { w: self }
    }
    #[doc = "Bit 19 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen19(&mut self) -> EPEN19_W {
        EPEN19_W { w: self }
    }
    #[doc = "Bit 20 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen20(&mut self) -> EPEN20_W {
        EPEN20_W { w: self }
    }
    #[doc = "Bit 21 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen21(&mut self) -> EPEN21_W {
        EPEN21_W { w: self }
    }
    #[doc = "Bit 22 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen22(&mut self) -> EPEN22_W {
        EPEN22_W { w: self }
    }
    #[doc = "Bit 23 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen23(&mut self) -> EPEN23_W {
        EPEN23_W { w: self }
    }
    #[doc = "Bit 24 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen24(&mut self) -> EPEN24_W {
        EPEN24_W { w: self }
    }
    #[doc = "Bit 25 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen25(&mut self) -> EPEN25_W {
        EPEN25_W { w: self }
    }
    #[doc = "Bit 26 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen26(&mut self) -> EPEN26_W {
        EPEN26_W { w: self }
    }
    #[doc = "Bit 27 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen27(&mut self) -> EPEN27_W {
        EPEN27_W { w: self }
    }
    #[doc = "Bit 28 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen28(&mut self) -> EPEN28_W {
        EPEN28_W { w: self }
    }
    #[doc = "Bit 29 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen29(&mut self) -> EPEN29_W {
        EPEN29_W { w: self }
    }
    #[doc = "Bit 30 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen30(&mut self) -> EPEN30_W {
        EPEN30_W { w: self }
    }
    #[doc = "Bit 31 - 0= The corresponding bit in USBDMARSt is set when an interrupt occurs for this endpoint. 1 = The corresponding bit in USBEpIntSt is set when an interrupt occurs for this endpoint. Implies Slave mode for this endpoint."]
    #[inline(always)]
    pub fn epen31(&mut self) -> EPEN31_W {
        EPEN31_W { w: self }
    }
}
