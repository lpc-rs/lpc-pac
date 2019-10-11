#[doc = "Writer for register EPINTSET"]
pub type W = crate::W<u32, super::EPINTSET>;
#[doc = "Register EPINTSET `reset()`'s with value 0"]
impl crate::ResetValue for super::EPINTSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `EPSET0`"]
pub struct EPSET0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET1`"]
pub struct EPSET1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET2`"]
pub struct EPSET2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET3`"]
pub struct EPSET3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET4`"]
pub struct EPSET4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET5`"]
pub struct EPSET5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET6`"]
pub struct EPSET6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET7`"]
pub struct EPSET7_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET8`"]
pub struct EPSET8_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET9`"]
pub struct EPSET9_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET10`"]
pub struct EPSET10_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET11`"]
pub struct EPSET11_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET12`"]
pub struct EPSET12_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET13`"]
pub struct EPSET13_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET14`"]
pub struct EPSET14_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET15`"]
pub struct EPSET15_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET16`"]
pub struct EPSET16_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET17`"]
pub struct EPSET17_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET18`"]
pub struct EPSET18_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET19`"]
pub struct EPSET19_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET20`"]
pub struct EPSET20_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET21`"]
pub struct EPSET21_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET22`"]
pub struct EPSET22_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET23`"]
pub struct EPSET23_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET24`"]
pub struct EPSET24_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET25`"]
pub struct EPSET25_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET26`"]
pub struct EPSET26_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET27`"]
pub struct EPSET27_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET28`"]
pub struct EPSET28_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET29`"]
pub struct EPSET29_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET30`"]
pub struct EPSET30_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPSET31`"]
pub struct EPSET31_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSET31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset0(&mut self) -> EPSET0_W {
        EPSET0_W { w: self }
    }
    #[doc = "Bit 1 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset1(&mut self) -> EPSET1_W {
        EPSET1_W { w: self }
    }
    #[doc = "Bit 2 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset2(&mut self) -> EPSET2_W {
        EPSET2_W { w: self }
    }
    #[doc = "Bit 3 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset3(&mut self) -> EPSET3_W {
        EPSET3_W { w: self }
    }
    #[doc = "Bit 4 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset4(&mut self) -> EPSET4_W {
        EPSET4_W { w: self }
    }
    #[doc = "Bit 5 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset5(&mut self) -> EPSET5_W {
        EPSET5_W { w: self }
    }
    #[doc = "Bit 6 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset6(&mut self) -> EPSET6_W {
        EPSET6_W { w: self }
    }
    #[doc = "Bit 7 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset7(&mut self) -> EPSET7_W {
        EPSET7_W { w: self }
    }
    #[doc = "Bit 8 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset8(&mut self) -> EPSET8_W {
        EPSET8_W { w: self }
    }
    #[doc = "Bit 9 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset9(&mut self) -> EPSET9_W {
        EPSET9_W { w: self }
    }
    #[doc = "Bit 10 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset10(&mut self) -> EPSET10_W {
        EPSET10_W { w: self }
    }
    #[doc = "Bit 11 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset11(&mut self) -> EPSET11_W {
        EPSET11_W { w: self }
    }
    #[doc = "Bit 12 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset12(&mut self) -> EPSET12_W {
        EPSET12_W { w: self }
    }
    #[doc = "Bit 13 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset13(&mut self) -> EPSET13_W {
        EPSET13_W { w: self }
    }
    #[doc = "Bit 14 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset14(&mut self) -> EPSET14_W {
        EPSET14_W { w: self }
    }
    #[doc = "Bit 15 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset15(&mut self) -> EPSET15_W {
        EPSET15_W { w: self }
    }
    #[doc = "Bit 16 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset16(&mut self) -> EPSET16_W {
        EPSET16_W { w: self }
    }
    #[doc = "Bit 17 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset17(&mut self) -> EPSET17_W {
        EPSET17_W { w: self }
    }
    #[doc = "Bit 18 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset18(&mut self) -> EPSET18_W {
        EPSET18_W { w: self }
    }
    #[doc = "Bit 19 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset19(&mut self) -> EPSET19_W {
        EPSET19_W { w: self }
    }
    #[doc = "Bit 20 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset20(&mut self) -> EPSET20_W {
        EPSET20_W { w: self }
    }
    #[doc = "Bit 21 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset21(&mut self) -> EPSET21_W {
        EPSET21_W { w: self }
    }
    #[doc = "Bit 22 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset22(&mut self) -> EPSET22_W {
        EPSET22_W { w: self }
    }
    #[doc = "Bit 23 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset23(&mut self) -> EPSET23_W {
        EPSET23_W { w: self }
    }
    #[doc = "Bit 24 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset24(&mut self) -> EPSET24_W {
        EPSET24_W { w: self }
    }
    #[doc = "Bit 25 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset25(&mut self) -> EPSET25_W {
        EPSET25_W { w: self }
    }
    #[doc = "Bit 26 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset26(&mut self) -> EPSET26_W {
        EPSET26_W { w: self }
    }
    #[doc = "Bit 27 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset27(&mut self) -> EPSET27_W {
        EPSET27_W { w: self }
    }
    #[doc = "Bit 28 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset28(&mut self) -> EPSET28_W {
        EPSET28_W { w: self }
    }
    #[doc = "Bit 29 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset29(&mut self) -> EPSET29_W {
        EPSET29_W { w: self }
    }
    #[doc = "Bit 30 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset30(&mut self) -> EPSET30_W {
        EPSET30_W { w: self }
    }
    #[doc = "Bit 31 - 0 = No effect. 1 = Sets the corresponding bit in USBEpIntSt."]
    #[inline(always)]
    pub fn epset31(&mut self) -> EPSET31_W {
        EPSET31_W { w: self }
    }
}
