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
#[doc = "Reader of field `PINSET0`"]
pub type PINSET0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET0`"]
pub struct PINSET0_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET1`"]
pub type PINSET1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET1`"]
pub struct PINSET1_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET2`"]
pub type PINSET2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET2`"]
pub struct PINSET2_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET3`"]
pub type PINSET3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET3`"]
pub struct PINSET3_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET4`"]
pub type PINSET4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET4`"]
pub struct PINSET4_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET5`"]
pub type PINSET5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET5`"]
pub struct PINSET5_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET6`"]
pub type PINSET6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET6`"]
pub struct PINSET6_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET7`"]
pub type PINSET7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET7`"]
pub struct PINSET7_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET8`"]
pub type PINSET8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET8`"]
pub struct PINSET8_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET9`"]
pub type PINSET9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET9`"]
pub struct PINSET9_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET10`"]
pub type PINSET10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET10`"]
pub struct PINSET10_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET11`"]
pub type PINSET11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET11`"]
pub struct PINSET11_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET12`"]
pub type PINSET12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET12`"]
pub struct PINSET12_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET13`"]
pub type PINSET13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET13`"]
pub struct PINSET13_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET14`"]
pub type PINSET14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET14`"]
pub struct PINSET14_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET15`"]
pub type PINSET15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET15`"]
pub struct PINSET15_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET16`"]
pub type PINSET16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET16`"]
pub struct PINSET16_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET17`"]
pub type PINSET17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET17`"]
pub struct PINSET17_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET18`"]
pub type PINSET18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET18`"]
pub struct PINSET18_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET19`"]
pub type PINSET19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET19`"]
pub struct PINSET19_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET20`"]
pub type PINSET20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET20`"]
pub struct PINSET20_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET21`"]
pub type PINSET21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET21`"]
pub struct PINSET21_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET22`"]
pub type PINSET22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET22`"]
pub struct PINSET22_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET23`"]
pub type PINSET23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET23`"]
pub struct PINSET23_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET24`"]
pub type PINSET24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET24`"]
pub struct PINSET24_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET25`"]
pub type PINSET25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET25`"]
pub struct PINSET25_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET26`"]
pub type PINSET26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET26`"]
pub struct PINSET26_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET27`"]
pub type PINSET27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET27`"]
pub struct PINSET27_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET28`"]
pub type PINSET28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET28`"]
pub struct PINSET28_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET29`"]
pub type PINSET29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET29`"]
pub struct PINSET29_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET30`"]
pub type PINSET30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET30`"]
pub struct PINSET30_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PINSET31`"]
pub type PINSET31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINSET31`"]
pub struct PINSET31_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSET31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset0(&self) -> PINSET0_R {
        PINSET0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset1(&self) -> PINSET1_R {
        PINSET1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset2(&self) -> PINSET2_R {
        PINSET2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset3(&self) -> PINSET3_R {
        PINSET3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset4(&self) -> PINSET4_R {
        PINSET4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset5(&self) -> PINSET5_R {
        PINSET5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset6(&self) -> PINSET6_R {
        PINSET6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset7(&self) -> PINSET7_R {
        PINSET7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset8(&self) -> PINSET8_R {
        PINSET8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset9(&self) -> PINSET9_R {
        PINSET9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset10(&self) -> PINSET10_R {
        PINSET10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset11(&self) -> PINSET11_R {
        PINSET11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset12(&self) -> PINSET12_R {
        PINSET12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset13(&self) -> PINSET13_R {
        PINSET13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset14(&self) -> PINSET14_R {
        PINSET14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset15(&self) -> PINSET15_R {
        PINSET15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset16(&self) -> PINSET16_R {
        PINSET16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset17(&self) -> PINSET17_R {
        PINSET17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset18(&self) -> PINSET18_R {
        PINSET18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset19(&self) -> PINSET19_R {
        PINSET19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset20(&self) -> PINSET20_R {
        PINSET20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset21(&self) -> PINSET21_R {
        PINSET21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset22(&self) -> PINSET22_R {
        PINSET22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset23(&self) -> PINSET23_R {
        PINSET23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset24(&self) -> PINSET24_R {
        PINSET24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset25(&self) -> PINSET25_R {
        PINSET25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset26(&self) -> PINSET26_R {
        PINSET26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset27(&self) -> PINSET27_R {
        PINSET27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset28(&self) -> PINSET28_R {
        PINSET28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset29(&self) -> PINSET29_R {
        PINSET29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset30(&self) -> PINSET30_R {
        PINSET30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset31(&self) -> PINSET31_R {
        PINSET31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset0(&mut self) -> PINSET0_W {
        PINSET0_W { w: self }
    }
    #[doc = "Bit 1 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset1(&mut self) -> PINSET1_W {
        PINSET1_W { w: self }
    }
    #[doc = "Bit 2 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset2(&mut self) -> PINSET2_W {
        PINSET2_W { w: self }
    }
    #[doc = "Bit 3 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset3(&mut self) -> PINSET3_W {
        PINSET3_W { w: self }
    }
    #[doc = "Bit 4 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset4(&mut self) -> PINSET4_W {
        PINSET4_W { w: self }
    }
    #[doc = "Bit 5 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset5(&mut self) -> PINSET5_W {
        PINSET5_W { w: self }
    }
    #[doc = "Bit 6 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset6(&mut self) -> PINSET6_W {
        PINSET6_W { w: self }
    }
    #[doc = "Bit 7 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset7(&mut self) -> PINSET7_W {
        PINSET7_W { w: self }
    }
    #[doc = "Bit 8 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset8(&mut self) -> PINSET8_W {
        PINSET8_W { w: self }
    }
    #[doc = "Bit 9 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset9(&mut self) -> PINSET9_W {
        PINSET9_W { w: self }
    }
    #[doc = "Bit 10 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset10(&mut self) -> PINSET10_W {
        PINSET10_W { w: self }
    }
    #[doc = "Bit 11 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset11(&mut self) -> PINSET11_W {
        PINSET11_W { w: self }
    }
    #[doc = "Bit 12 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset12(&mut self) -> PINSET12_W {
        PINSET12_W { w: self }
    }
    #[doc = "Bit 13 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset13(&mut self) -> PINSET13_W {
        PINSET13_W { w: self }
    }
    #[doc = "Bit 14 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset14(&mut self) -> PINSET14_W {
        PINSET14_W { w: self }
    }
    #[doc = "Bit 15 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset15(&mut self) -> PINSET15_W {
        PINSET15_W { w: self }
    }
    #[doc = "Bit 16 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset16(&mut self) -> PINSET16_W {
        PINSET16_W { w: self }
    }
    #[doc = "Bit 17 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset17(&mut self) -> PINSET17_W {
        PINSET17_W { w: self }
    }
    #[doc = "Bit 18 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset18(&mut self) -> PINSET18_W {
        PINSET18_W { w: self }
    }
    #[doc = "Bit 19 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset19(&mut self) -> PINSET19_W {
        PINSET19_W { w: self }
    }
    #[doc = "Bit 20 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset20(&mut self) -> PINSET20_W {
        PINSET20_W { w: self }
    }
    #[doc = "Bit 21 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset21(&mut self) -> PINSET21_W {
        PINSET21_W { w: self }
    }
    #[doc = "Bit 22 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset22(&mut self) -> PINSET22_W {
        PINSET22_W { w: self }
    }
    #[doc = "Bit 23 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset23(&mut self) -> PINSET23_W {
        PINSET23_W { w: self }
    }
    #[doc = "Bit 24 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset24(&mut self) -> PINSET24_W {
        PINSET24_W { w: self }
    }
    #[doc = "Bit 25 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset25(&mut self) -> PINSET25_W {
        PINSET25_W { w: self }
    }
    #[doc = "Bit 26 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset26(&mut self) -> PINSET26_W {
        PINSET26_W { w: self }
    }
    #[doc = "Bit 27 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset27(&mut self) -> PINSET27_W {
        PINSET27_W { w: self }
    }
    #[doc = "Bit 28 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset28(&mut self) -> PINSET28_W {
        PINSET28_W { w: self }
    }
    #[doc = "Bit 29 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset29(&mut self) -> PINSET29_W {
        PINSET29_W { w: self }
    }
    #[doc = "Bit 30 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset30(&mut self) -> PINSET30_W {
        PINSET30_W { w: self }
    }
    #[doc = "Bit 31 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline(always)]
    pub fn pinset31(&mut self) -> PINSET31_W {
        PINSET31_W { w: self }
    }
}
