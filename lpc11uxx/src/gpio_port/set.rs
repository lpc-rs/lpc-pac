#[doc = "Reader of register SET%s"]
pub type R = crate::R<u32, super::SET>;
#[doc = "Writer for register SET%s"]
pub type W = crate::W<u32, super::SET>;
#[doc = "Register SET%s `reset()`'s with value 0"]
impl crate::ResetValue for super::SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SETP0`"]
pub type SETP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP0`"]
pub struct SETP0_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP0_W<'a> {
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
#[doc = "Reader of field `SETP1`"]
pub type SETP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP1`"]
pub struct SETP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP1_W<'a> {
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
#[doc = "Reader of field `SETP2`"]
pub type SETP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP2`"]
pub struct SETP2_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP2_W<'a> {
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
#[doc = "Reader of field `SETP3`"]
pub type SETP3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP3`"]
pub struct SETP3_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP3_W<'a> {
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
#[doc = "Reader of field `SETP4`"]
pub type SETP4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP4`"]
pub struct SETP4_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP4_W<'a> {
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
#[doc = "Reader of field `SETP5`"]
pub type SETP5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP5`"]
pub struct SETP5_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP5_W<'a> {
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
#[doc = "Reader of field `SETP6`"]
pub type SETP6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP6`"]
pub struct SETP6_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP6_W<'a> {
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
#[doc = "Reader of field `SETP7`"]
pub type SETP7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP7`"]
pub struct SETP7_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP7_W<'a> {
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
#[doc = "Reader of field `SETP8`"]
pub type SETP8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP8`"]
pub struct SETP8_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP8_W<'a> {
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
#[doc = "Reader of field `SETP9`"]
pub type SETP9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP9`"]
pub struct SETP9_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP9_W<'a> {
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
#[doc = "Reader of field `SETP10`"]
pub type SETP10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP10`"]
pub struct SETP10_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP10_W<'a> {
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
#[doc = "Reader of field `SETP11`"]
pub type SETP11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP11`"]
pub struct SETP11_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP11_W<'a> {
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
#[doc = "Reader of field `SETP12`"]
pub type SETP12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP12`"]
pub struct SETP12_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP12_W<'a> {
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
#[doc = "Reader of field `SETP13`"]
pub type SETP13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP13`"]
pub struct SETP13_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP13_W<'a> {
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
#[doc = "Reader of field `SETP14`"]
pub type SETP14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP14`"]
pub struct SETP14_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP14_W<'a> {
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
#[doc = "Reader of field `SETP15`"]
pub type SETP15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP15`"]
pub struct SETP15_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP15_W<'a> {
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
#[doc = "Reader of field `SETP16`"]
pub type SETP16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP16`"]
pub struct SETP16_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP16_W<'a> {
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
#[doc = "Reader of field `SETP17`"]
pub type SETP17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP17`"]
pub struct SETP17_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP17_W<'a> {
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
#[doc = "Reader of field `SETP18`"]
pub type SETP18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP18`"]
pub struct SETP18_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP18_W<'a> {
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
#[doc = "Reader of field `SETP19`"]
pub type SETP19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP19`"]
pub struct SETP19_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP19_W<'a> {
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
#[doc = "Reader of field `SETP20`"]
pub type SETP20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP20`"]
pub struct SETP20_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP20_W<'a> {
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
#[doc = "Reader of field `SETP21`"]
pub type SETP21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP21`"]
pub struct SETP21_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP21_W<'a> {
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
#[doc = "Reader of field `SETP22`"]
pub type SETP22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP22`"]
pub struct SETP22_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP22_W<'a> {
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
#[doc = "Reader of field `SETP23`"]
pub type SETP23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP23`"]
pub struct SETP23_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP23_W<'a> {
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
#[doc = "Reader of field `SETP24`"]
pub type SETP24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP24`"]
pub struct SETP24_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP24_W<'a> {
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
#[doc = "Reader of field `SETP25`"]
pub type SETP25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP25`"]
pub struct SETP25_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP25_W<'a> {
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
#[doc = "Reader of field `SETP26`"]
pub type SETP26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP26`"]
pub struct SETP26_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP26_W<'a> {
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
#[doc = "Reader of field `SETP27`"]
pub type SETP27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP27`"]
pub struct SETP27_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP27_W<'a> {
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
#[doc = "Reader of field `SETP28`"]
pub type SETP28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP28`"]
pub struct SETP28_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP28_W<'a> {
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
#[doc = "Reader of field `SETP29`"]
pub type SETP29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP29`"]
pub struct SETP29_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP29_W<'a> {
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
#[doc = "Reader of field `SETP30`"]
pub type SETP30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP30`"]
pub struct SETP30_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP30_W<'a> {
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
#[doc = "Reader of field `SETP31`"]
pub type SETP31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETP31`"]
pub struct SETP31_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP31_W<'a> {
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
    #[doc = "Bit 0 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp0(&self) -> SETP0_R {
        SETP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp1(&self) -> SETP1_R {
        SETP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp2(&self) -> SETP2_R {
        SETP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp3(&self) -> SETP3_R {
        SETP3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp4(&self) -> SETP4_R {
        SETP4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp5(&self) -> SETP5_R {
        SETP5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp6(&self) -> SETP6_R {
        SETP6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp7(&self) -> SETP7_R {
        SETP7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp8(&self) -> SETP8_R {
        SETP8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp9(&self) -> SETP9_R {
        SETP9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp10(&self) -> SETP10_R {
        SETP10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp11(&self) -> SETP11_R {
        SETP11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp12(&self) -> SETP12_R {
        SETP12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp13(&self) -> SETP13_R {
        SETP13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp14(&self) -> SETP14_R {
        SETP14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp15(&self) -> SETP15_R {
        SETP15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp16(&self) -> SETP16_R {
        SETP16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp17(&self) -> SETP17_R {
        SETP17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp18(&self) -> SETP18_R {
        SETP18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp19(&self) -> SETP19_R {
        SETP19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp20(&self) -> SETP20_R {
        SETP20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp21(&self) -> SETP21_R {
        SETP21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp22(&self) -> SETP22_R {
        SETP22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp23(&self) -> SETP23_R {
        SETP23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp24(&self) -> SETP24_R {
        SETP24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp25(&self) -> SETP25_R {
        SETP25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp26(&self) -> SETP26_R {
        SETP26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp27(&self) -> SETP27_R {
        SETP27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp28(&self) -> SETP28_R {
        SETP28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp29(&self) -> SETP29_R {
        SETP29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp30(&self) -> SETP30_R {
        SETP30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp31(&self) -> SETP31_R {
        SETP31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp0(&mut self) -> SETP0_W {
        SETP0_W { w: self }
    }
    #[doc = "Bit 1 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp1(&mut self) -> SETP1_W {
        SETP1_W { w: self }
    }
    #[doc = "Bit 2 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp2(&mut self) -> SETP2_W {
        SETP2_W { w: self }
    }
    #[doc = "Bit 3 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp3(&mut self) -> SETP3_W {
        SETP3_W { w: self }
    }
    #[doc = "Bit 4 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp4(&mut self) -> SETP4_W {
        SETP4_W { w: self }
    }
    #[doc = "Bit 5 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp5(&mut self) -> SETP5_W {
        SETP5_W { w: self }
    }
    #[doc = "Bit 6 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp6(&mut self) -> SETP6_W {
        SETP6_W { w: self }
    }
    #[doc = "Bit 7 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp7(&mut self) -> SETP7_W {
        SETP7_W { w: self }
    }
    #[doc = "Bit 8 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp8(&mut self) -> SETP8_W {
        SETP8_W { w: self }
    }
    #[doc = "Bit 9 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp9(&mut self) -> SETP9_W {
        SETP9_W { w: self }
    }
    #[doc = "Bit 10 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp10(&mut self) -> SETP10_W {
        SETP10_W { w: self }
    }
    #[doc = "Bit 11 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp11(&mut self) -> SETP11_W {
        SETP11_W { w: self }
    }
    #[doc = "Bit 12 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp12(&mut self) -> SETP12_W {
        SETP12_W { w: self }
    }
    #[doc = "Bit 13 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp13(&mut self) -> SETP13_W {
        SETP13_W { w: self }
    }
    #[doc = "Bit 14 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp14(&mut self) -> SETP14_W {
        SETP14_W { w: self }
    }
    #[doc = "Bit 15 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp15(&mut self) -> SETP15_W {
        SETP15_W { w: self }
    }
    #[doc = "Bit 16 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp16(&mut self) -> SETP16_W {
        SETP16_W { w: self }
    }
    #[doc = "Bit 17 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp17(&mut self) -> SETP17_W {
        SETP17_W { w: self }
    }
    #[doc = "Bit 18 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp18(&mut self) -> SETP18_W {
        SETP18_W { w: self }
    }
    #[doc = "Bit 19 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp19(&mut self) -> SETP19_W {
        SETP19_W { w: self }
    }
    #[doc = "Bit 20 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp20(&mut self) -> SETP20_W {
        SETP20_W { w: self }
    }
    #[doc = "Bit 21 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp21(&mut self) -> SETP21_W {
        SETP21_W { w: self }
    }
    #[doc = "Bit 22 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp22(&mut self) -> SETP22_W {
        SETP22_W { w: self }
    }
    #[doc = "Bit 23 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp23(&mut self) -> SETP23_W {
        SETP23_W { w: self }
    }
    #[doc = "Bit 24 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp24(&mut self) -> SETP24_W {
        SETP24_W { w: self }
    }
    #[doc = "Bit 25 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp25(&mut self) -> SETP25_W {
        SETP25_W { w: self }
    }
    #[doc = "Bit 26 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp26(&mut self) -> SETP26_W {
        SETP26_W { w: self }
    }
    #[doc = "Bit 27 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp27(&mut self) -> SETP27_W {
        SETP27_W { w: self }
    }
    #[doc = "Bit 28 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp28(&mut self) -> SETP28_W {
        SETP28_W { w: self }
    }
    #[doc = "Bit 29 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp29(&mut self) -> SETP29_W {
        SETP29_W { w: self }
    }
    #[doc = "Bit 30 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp30(&mut self) -> SETP30_W {
        SETP30_W { w: self }
    }
    #[doc = "Bit 31 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp31(&mut self) -> SETP31_W {
        SETP31_W { w: self }
    }
}
