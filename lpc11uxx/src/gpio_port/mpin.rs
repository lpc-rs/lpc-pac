#[doc = "Reader of register MPIN%s"]
pub type R = crate::R<u32, super::MPIN>;
#[doc = "Writer for register MPIN%s"]
pub type W = crate::W<u32, super::MPIN>;
#[doc = "Register MPIN%s `reset()`'s with value 0"]
impl crate::ResetValue for super::MPIN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MPORTP0`"]
pub type MPORTP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP0`"]
pub struct MPORTP0_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP1`"]
pub type MPORTP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP1`"]
pub struct MPORTP1_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP2`"]
pub type MPORTP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP2`"]
pub struct MPORTP2_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP3`"]
pub type MPORTP3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP3`"]
pub struct MPORTP3_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP4`"]
pub type MPORTP4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP4`"]
pub struct MPORTP4_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP5`"]
pub type MPORTP5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP5`"]
pub struct MPORTP5_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP6`"]
pub type MPORTP6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP6`"]
pub struct MPORTP6_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP7`"]
pub type MPORTP7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP7`"]
pub struct MPORTP7_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP8`"]
pub type MPORTP8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP8`"]
pub struct MPORTP8_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP9`"]
pub type MPORTP9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP9`"]
pub struct MPORTP9_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP10`"]
pub type MPORTP10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP10`"]
pub struct MPORTP10_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP11`"]
pub type MPORTP11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP11`"]
pub struct MPORTP11_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP12`"]
pub type MPORTP12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP12`"]
pub struct MPORTP12_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP13`"]
pub type MPORTP13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP13`"]
pub struct MPORTP13_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP14`"]
pub type MPORTP14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP14`"]
pub struct MPORTP14_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP15`"]
pub type MPORTP15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP15`"]
pub struct MPORTP15_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP16`"]
pub type MPORTP16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP16`"]
pub struct MPORTP16_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP17`"]
pub type MPORTP17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP17`"]
pub struct MPORTP17_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP18`"]
pub type MPORTP18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP18`"]
pub struct MPORTP18_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP19`"]
pub type MPORTP19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP19`"]
pub struct MPORTP19_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP20`"]
pub type MPORTP20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP20`"]
pub struct MPORTP20_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP21`"]
pub type MPORTP21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP21`"]
pub struct MPORTP21_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP22`"]
pub type MPORTP22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP22`"]
pub struct MPORTP22_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP23`"]
pub type MPORTP23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP23`"]
pub struct MPORTP23_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP24`"]
pub type MPORTP24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP24`"]
pub struct MPORTP24_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP25`"]
pub type MPORTP25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP25`"]
pub struct MPORTP25_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP26`"]
pub type MPORTP26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP26`"]
pub struct MPORTP26_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP27`"]
pub type MPORTP27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP27`"]
pub struct MPORTP27_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP28`"]
pub type MPORTP28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP28`"]
pub struct MPORTP28_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP29`"]
pub type MPORTP29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP29`"]
pub struct MPORTP29_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP30`"]
pub type MPORTP30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP30`"]
pub struct MPORTP30_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MPORTP31`"]
pub type MPORTP31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPORTP31`"]
pub struct MPORTP31_W<'a> {
    w: &'a mut W,
}
impl<'a> MPORTP31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp0(&self) -> MPORTP0_R {
        MPORTP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp1(&self) -> MPORTP1_R {
        MPORTP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp2(&self) -> MPORTP2_R {
        MPORTP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp3(&self) -> MPORTP3_R {
        MPORTP3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp4(&self) -> MPORTP4_R {
        MPORTP4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp5(&self) -> MPORTP5_R {
        MPORTP5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp6(&self) -> MPORTP6_R {
        MPORTP6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp7(&self) -> MPORTP7_R {
        MPORTP7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp8(&self) -> MPORTP8_R {
        MPORTP8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp9(&self) -> MPORTP9_R {
        MPORTP9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp10(&self) -> MPORTP10_R {
        MPORTP10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp11(&self) -> MPORTP11_R {
        MPORTP11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp12(&self) -> MPORTP12_R {
        MPORTP12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp13(&self) -> MPORTP13_R {
        MPORTP13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp14(&self) -> MPORTP14_R {
        MPORTP14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp15(&self) -> MPORTP15_R {
        MPORTP15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp16(&self) -> MPORTP16_R {
        MPORTP16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp17(&self) -> MPORTP17_R {
        MPORTP17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp18(&self) -> MPORTP18_R {
        MPORTP18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp19(&self) -> MPORTP19_R {
        MPORTP19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp20(&self) -> MPORTP20_R {
        MPORTP20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp21(&self) -> MPORTP21_R {
        MPORTP21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp22(&self) -> MPORTP22_R {
        MPORTP22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp23(&self) -> MPORTP23_R {
        MPORTP23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp24(&self) -> MPORTP24_R {
        MPORTP24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp25(&self) -> MPORTP25_R {
        MPORTP25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp26(&self) -> MPORTP26_R {
        MPORTP26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp27(&self) -> MPORTP27_R {
        MPORTP27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp28(&self) -> MPORTP28_R {
        MPORTP28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp29(&self) -> MPORTP29_R {
        MPORTP29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp30(&self) -> MPORTP30_R {
        MPORTP30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp31(&self) -> MPORTP31_R {
        MPORTP31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp0(&mut self) -> MPORTP0_W {
        MPORTP0_W { w: self }
    }
    #[doc = "Bit 1 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp1(&mut self) -> MPORTP1_W {
        MPORTP1_W { w: self }
    }
    #[doc = "Bit 2 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp2(&mut self) -> MPORTP2_W {
        MPORTP2_W { w: self }
    }
    #[doc = "Bit 3 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp3(&mut self) -> MPORTP3_W {
        MPORTP3_W { w: self }
    }
    #[doc = "Bit 4 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp4(&mut self) -> MPORTP4_W {
        MPORTP4_W { w: self }
    }
    #[doc = "Bit 5 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp5(&mut self) -> MPORTP5_W {
        MPORTP5_W { w: self }
    }
    #[doc = "Bit 6 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp6(&mut self) -> MPORTP6_W {
        MPORTP6_W { w: self }
    }
    #[doc = "Bit 7 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp7(&mut self) -> MPORTP7_W {
        MPORTP7_W { w: self }
    }
    #[doc = "Bit 8 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp8(&mut self) -> MPORTP8_W {
        MPORTP8_W { w: self }
    }
    #[doc = "Bit 9 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp9(&mut self) -> MPORTP9_W {
        MPORTP9_W { w: self }
    }
    #[doc = "Bit 10 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp10(&mut self) -> MPORTP10_W {
        MPORTP10_W { w: self }
    }
    #[doc = "Bit 11 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp11(&mut self) -> MPORTP11_W {
        MPORTP11_W { w: self }
    }
    #[doc = "Bit 12 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp12(&mut self) -> MPORTP12_W {
        MPORTP12_W { w: self }
    }
    #[doc = "Bit 13 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp13(&mut self) -> MPORTP13_W {
        MPORTP13_W { w: self }
    }
    #[doc = "Bit 14 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp14(&mut self) -> MPORTP14_W {
        MPORTP14_W { w: self }
    }
    #[doc = "Bit 15 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp15(&mut self) -> MPORTP15_W {
        MPORTP15_W { w: self }
    }
    #[doc = "Bit 16 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp16(&mut self) -> MPORTP16_W {
        MPORTP16_W { w: self }
    }
    #[doc = "Bit 17 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp17(&mut self) -> MPORTP17_W {
        MPORTP17_W { w: self }
    }
    #[doc = "Bit 18 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp18(&mut self) -> MPORTP18_W {
        MPORTP18_W { w: self }
    }
    #[doc = "Bit 19 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp19(&mut self) -> MPORTP19_W {
        MPORTP19_W { w: self }
    }
    #[doc = "Bit 20 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp20(&mut self) -> MPORTP20_W {
        MPORTP20_W { w: self }
    }
    #[doc = "Bit 21 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp21(&mut self) -> MPORTP21_W {
        MPORTP21_W { w: self }
    }
    #[doc = "Bit 22 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp22(&mut self) -> MPORTP22_W {
        MPORTP22_W { w: self }
    }
    #[doc = "Bit 23 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp23(&mut self) -> MPORTP23_W {
        MPORTP23_W { w: self }
    }
    #[doc = "Bit 24 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp24(&mut self) -> MPORTP24_W {
        MPORTP24_W { w: self }
    }
    #[doc = "Bit 25 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp25(&mut self) -> MPORTP25_W {
        MPORTP25_W { w: self }
    }
    #[doc = "Bit 26 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp26(&mut self) -> MPORTP26_W {
        MPORTP26_W { w: self }
    }
    #[doc = "Bit 27 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp27(&mut self) -> MPORTP27_W {
        MPORTP27_W { w: self }
    }
    #[doc = "Bit 28 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp28(&mut self) -> MPORTP28_W {
        MPORTP28_W { w: self }
    }
    #[doc = "Bit 29 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp29(&mut self) -> MPORTP29_W {
        MPORTP29_W { w: self }
    }
    #[doc = "Bit 30 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp30(&mut self) -> MPORTP30_W {
        MPORTP30_W { w: self }
    }
    #[doc = "Bit 31 - Masked port register (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn mportp31(&mut self) -> MPORTP31_W {
        MPORTP31_W { w: self }
    }
}
