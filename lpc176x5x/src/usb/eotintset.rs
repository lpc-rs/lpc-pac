#[doc = "Writer for register EOTINTSET"]
pub type W = crate::W<u32, super::EOTINTSET>;
#[doc = "Register EOTINTSET `reset()`'s with value 0"]
impl crate::ResetValue for super::EOTINTSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `EPTXINTSET0`"]
pub struct EPTXINTSET0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET1`"]
pub struct EPTXINTSET1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET2`"]
pub struct EPTXINTSET2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET3`"]
pub struct EPTXINTSET3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET4`"]
pub struct EPTXINTSET4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET5`"]
pub struct EPTXINTSET5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET6`"]
pub struct EPTXINTSET6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET7`"]
pub struct EPTXINTSET7_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET8`"]
pub struct EPTXINTSET8_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET9`"]
pub struct EPTXINTSET9_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET10`"]
pub struct EPTXINTSET10_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET11`"]
pub struct EPTXINTSET11_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET12`"]
pub struct EPTXINTSET12_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET13`"]
pub struct EPTXINTSET13_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET14`"]
pub struct EPTXINTSET14_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET15`"]
pub struct EPTXINTSET15_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET16`"]
pub struct EPTXINTSET16_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET17`"]
pub struct EPTXINTSET17_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET18`"]
pub struct EPTXINTSET18_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET19`"]
pub struct EPTXINTSET19_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET20`"]
pub struct EPTXINTSET20_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET21`"]
pub struct EPTXINTSET21_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET22`"]
pub struct EPTXINTSET22_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET23`"]
pub struct EPTXINTSET23_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET24`"]
pub struct EPTXINTSET24_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET25`"]
pub struct EPTXINTSET25_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET26`"]
pub struct EPTXINTSET26_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET27`"]
pub struct EPTXINTSET27_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET28`"]
pub struct EPTXINTSET28_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET29`"]
pub struct EPTXINTSET29_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET30`"]
pub struct EPTXINTSET30_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTSET31`"]
pub struct EPTXINTSET31_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTSET31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl W {
    #[doc = "Bit 0 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset0(&mut self) -> EPTXINTSET0_W {
        EPTXINTSET0_W { w: self }
    }
    #[doc = "Bit 1 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset1(&mut self) -> EPTXINTSET1_W {
        EPTXINTSET1_W { w: self }
    }
    #[doc = "Bit 2 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset2(&mut self) -> EPTXINTSET2_W {
        EPTXINTSET2_W { w: self }
    }
    #[doc = "Bit 3 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset3(&mut self) -> EPTXINTSET3_W {
        EPTXINTSET3_W { w: self }
    }
    #[doc = "Bit 4 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset4(&mut self) -> EPTXINTSET4_W {
        EPTXINTSET4_W { w: self }
    }
    #[doc = "Bit 5 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset5(&mut self) -> EPTXINTSET5_W {
        EPTXINTSET5_W { w: self }
    }
    #[doc = "Bit 6 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset6(&mut self) -> EPTXINTSET6_W {
        EPTXINTSET6_W { w: self }
    }
    #[doc = "Bit 7 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset7(&mut self) -> EPTXINTSET7_W {
        EPTXINTSET7_W { w: self }
    }
    #[doc = "Bit 8 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset8(&mut self) -> EPTXINTSET8_W {
        EPTXINTSET8_W { w: self }
    }
    #[doc = "Bit 9 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset9(&mut self) -> EPTXINTSET9_W {
        EPTXINTSET9_W { w: self }
    }
    #[doc = "Bit 10 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset10(&mut self) -> EPTXINTSET10_W {
        EPTXINTSET10_W { w: self }
    }
    #[doc = "Bit 11 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset11(&mut self) -> EPTXINTSET11_W {
        EPTXINTSET11_W { w: self }
    }
    #[doc = "Bit 12 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset12(&mut self) -> EPTXINTSET12_W {
        EPTXINTSET12_W { w: self }
    }
    #[doc = "Bit 13 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset13(&mut self) -> EPTXINTSET13_W {
        EPTXINTSET13_W { w: self }
    }
    #[doc = "Bit 14 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset14(&mut self) -> EPTXINTSET14_W {
        EPTXINTSET14_W { w: self }
    }
    #[doc = "Bit 15 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset15(&mut self) -> EPTXINTSET15_W {
        EPTXINTSET15_W { w: self }
    }
    #[doc = "Bit 16 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset16(&mut self) -> EPTXINTSET16_W {
        EPTXINTSET16_W { w: self }
    }
    #[doc = "Bit 17 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset17(&mut self) -> EPTXINTSET17_W {
        EPTXINTSET17_W { w: self }
    }
    #[doc = "Bit 18 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset18(&mut self) -> EPTXINTSET18_W {
        EPTXINTSET18_W { w: self }
    }
    #[doc = "Bit 19 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset19(&mut self) -> EPTXINTSET19_W {
        EPTXINTSET19_W { w: self }
    }
    #[doc = "Bit 20 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset20(&mut self) -> EPTXINTSET20_W {
        EPTXINTSET20_W { w: self }
    }
    #[doc = "Bit 21 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset21(&mut self) -> EPTXINTSET21_W {
        EPTXINTSET21_W { w: self }
    }
    #[doc = "Bit 22 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset22(&mut self) -> EPTXINTSET22_W {
        EPTXINTSET22_W { w: self }
    }
    #[doc = "Bit 23 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset23(&mut self) -> EPTXINTSET23_W {
        EPTXINTSET23_W { w: self }
    }
    #[doc = "Bit 24 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset24(&mut self) -> EPTXINTSET24_W {
        EPTXINTSET24_W { w: self }
    }
    #[doc = "Bit 25 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset25(&mut self) -> EPTXINTSET25_W {
        EPTXINTSET25_W { w: self }
    }
    #[doc = "Bit 26 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset26(&mut self) -> EPTXINTSET26_W {
        EPTXINTSET26_W { w: self }
    }
    #[doc = "Bit 27 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset27(&mut self) -> EPTXINTSET27_W {
        EPTXINTSET27_W { w: self }
    }
    #[doc = "Bit 28 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset28(&mut self) -> EPTXINTSET28_W {
        EPTXINTSET28_W { w: self }
    }
    #[doc = "Bit 29 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset29(&mut self) -> EPTXINTSET29_W {
        EPTXINTSET29_W { w: self }
    }
    #[doc = "Bit 30 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset30(&mut self) -> EPTXINTSET30_W {
        EPTXINTSET30_W { w: self }
    }
    #[doc = "Bit 31 - Set endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Set the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintset31(&mut self) -> EPTXINTSET31_W {
        EPTXINTSET31_W { w: self }
    }
}
