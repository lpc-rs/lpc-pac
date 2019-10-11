#[doc = "Writer for register SYSERRINTSET"]
pub type W = crate::W<u32, super::SYSERRINTSET>;
#[doc = "Register SYSERRINTSET `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSERRINTSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `EPERRINTSET0`"]
pub struct EPERRINTSET0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET1`"]
pub struct EPERRINTSET1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET2`"]
pub struct EPERRINTSET2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET3`"]
pub struct EPERRINTSET3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET4`"]
pub struct EPERRINTSET4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET5`"]
pub struct EPERRINTSET5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET6`"]
pub struct EPERRINTSET6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET7`"]
pub struct EPERRINTSET7_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET8`"]
pub struct EPERRINTSET8_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET9`"]
pub struct EPERRINTSET9_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET10`"]
pub struct EPERRINTSET10_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET11`"]
pub struct EPERRINTSET11_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET12`"]
pub struct EPERRINTSET12_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET13`"]
pub struct EPERRINTSET13_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET14`"]
pub struct EPERRINTSET14_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET15`"]
pub struct EPERRINTSET15_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET16`"]
pub struct EPERRINTSET16_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET17`"]
pub struct EPERRINTSET17_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET18`"]
pub struct EPERRINTSET18_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET19`"]
pub struct EPERRINTSET19_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET20`"]
pub struct EPERRINTSET20_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET21`"]
pub struct EPERRINTSET21_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET22`"]
pub struct EPERRINTSET22_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET23`"]
pub struct EPERRINTSET23_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET24`"]
pub struct EPERRINTSET24_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET25`"]
pub struct EPERRINTSET25_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET26`"]
pub struct EPERRINTSET26_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET27`"]
pub struct EPERRINTSET27_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET28`"]
pub struct EPERRINTSET28_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET29`"]
pub struct EPERRINTSET29_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET30`"]
pub struct EPERRINTSET30_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTSET31`"]
pub struct EPERRINTSET31_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTSET31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset0(&mut self) -> EPERRINTSET0_W {
        EPERRINTSET0_W { w: self }
    }
    #[doc = "Bit 1 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset1(&mut self) -> EPERRINTSET1_W {
        EPERRINTSET1_W { w: self }
    }
    #[doc = "Bit 2 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset2(&mut self) -> EPERRINTSET2_W {
        EPERRINTSET2_W { w: self }
    }
    #[doc = "Bit 3 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset3(&mut self) -> EPERRINTSET3_W {
        EPERRINTSET3_W { w: self }
    }
    #[doc = "Bit 4 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset4(&mut self) -> EPERRINTSET4_W {
        EPERRINTSET4_W { w: self }
    }
    #[doc = "Bit 5 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset5(&mut self) -> EPERRINTSET5_W {
        EPERRINTSET5_W { w: self }
    }
    #[doc = "Bit 6 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset6(&mut self) -> EPERRINTSET6_W {
        EPERRINTSET6_W { w: self }
    }
    #[doc = "Bit 7 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset7(&mut self) -> EPERRINTSET7_W {
        EPERRINTSET7_W { w: self }
    }
    #[doc = "Bit 8 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset8(&mut self) -> EPERRINTSET8_W {
        EPERRINTSET8_W { w: self }
    }
    #[doc = "Bit 9 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset9(&mut self) -> EPERRINTSET9_W {
        EPERRINTSET9_W { w: self }
    }
    #[doc = "Bit 10 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset10(&mut self) -> EPERRINTSET10_W {
        EPERRINTSET10_W { w: self }
    }
    #[doc = "Bit 11 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset11(&mut self) -> EPERRINTSET11_W {
        EPERRINTSET11_W { w: self }
    }
    #[doc = "Bit 12 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset12(&mut self) -> EPERRINTSET12_W {
        EPERRINTSET12_W { w: self }
    }
    #[doc = "Bit 13 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset13(&mut self) -> EPERRINTSET13_W {
        EPERRINTSET13_W { w: self }
    }
    #[doc = "Bit 14 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset14(&mut self) -> EPERRINTSET14_W {
        EPERRINTSET14_W { w: self }
    }
    #[doc = "Bit 15 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset15(&mut self) -> EPERRINTSET15_W {
        EPERRINTSET15_W { w: self }
    }
    #[doc = "Bit 16 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset16(&mut self) -> EPERRINTSET16_W {
        EPERRINTSET16_W { w: self }
    }
    #[doc = "Bit 17 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset17(&mut self) -> EPERRINTSET17_W {
        EPERRINTSET17_W { w: self }
    }
    #[doc = "Bit 18 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset18(&mut self) -> EPERRINTSET18_W {
        EPERRINTSET18_W { w: self }
    }
    #[doc = "Bit 19 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset19(&mut self) -> EPERRINTSET19_W {
        EPERRINTSET19_W { w: self }
    }
    #[doc = "Bit 20 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset20(&mut self) -> EPERRINTSET20_W {
        EPERRINTSET20_W { w: self }
    }
    #[doc = "Bit 21 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset21(&mut self) -> EPERRINTSET21_W {
        EPERRINTSET21_W { w: self }
    }
    #[doc = "Bit 22 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset22(&mut self) -> EPERRINTSET22_W {
        EPERRINTSET22_W { w: self }
    }
    #[doc = "Bit 23 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset23(&mut self) -> EPERRINTSET23_W {
        EPERRINTSET23_W { w: self }
    }
    #[doc = "Bit 24 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset24(&mut self) -> EPERRINTSET24_W {
        EPERRINTSET24_W { w: self }
    }
    #[doc = "Bit 25 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset25(&mut self) -> EPERRINTSET25_W {
        EPERRINTSET25_W { w: self }
    }
    #[doc = "Bit 26 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset26(&mut self) -> EPERRINTSET26_W {
        EPERRINTSET26_W { w: self }
    }
    #[doc = "Bit 27 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset27(&mut self) -> EPERRINTSET27_W {
        EPERRINTSET27_W { w: self }
    }
    #[doc = "Bit 28 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset28(&mut self) -> EPERRINTSET28_W {
        EPERRINTSET28_W { w: self }
    }
    #[doc = "Bit 29 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset29(&mut self) -> EPERRINTSET29_W {
        EPERRINTSET29_W { w: self }
    }
    #[doc = "Bit 30 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset30(&mut self) -> EPERRINTSET30_W {
        EPERRINTSET30_W { w: self }
    }
    #[doc = "Bit 31 - Set endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Set the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintset31(&mut self) -> EPERRINTSET31_W {
        EPERRINTSET31_W { w: self }
    }
}
