#[doc = "Writer for register DMARSET"]
pub type W = crate::W<u32, super::DMARSET>;
#[doc = "Register DMARSET `reset()`'s with value 0"]
impl crate::ResetValue for super::DMARSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `EPRSET0`"]
pub struct EPRSET0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET1`"]
pub struct EPRSET1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET2`"]
pub struct EPRSET2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET3`"]
pub struct EPRSET3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET4`"]
pub struct EPRSET4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET5`"]
pub struct EPRSET5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET6`"]
pub struct EPRSET6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET7`"]
pub struct EPRSET7_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET8`"]
pub struct EPRSET8_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET9`"]
pub struct EPRSET9_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET10`"]
pub struct EPRSET10_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET11`"]
pub struct EPRSET11_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET12`"]
pub struct EPRSET12_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET13`"]
pub struct EPRSET13_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET14`"]
pub struct EPRSET14_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET15`"]
pub struct EPRSET15_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET16`"]
pub struct EPRSET16_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET17`"]
pub struct EPRSET17_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET18`"]
pub struct EPRSET18_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET19`"]
pub struct EPRSET19_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET20`"]
pub struct EPRSET20_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET21`"]
pub struct EPRSET21_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET22`"]
pub struct EPRSET22_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET23`"]
pub struct EPRSET23_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET24`"]
pub struct EPRSET24_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET25`"]
pub struct EPRSET25_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET26`"]
pub struct EPRSET26_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET27`"]
pub struct EPRSET27_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET28`"]
pub struct EPRSET28_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET29`"]
pub struct EPRSET29_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET30`"]
pub struct EPRSET30_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRSET31`"]
pub struct EPRSET31_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRSET31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - Control endpoint OUT (DMA cannot be enabled for this endpoint and the EP0 bit must be 0)."]
    #[inline(always)]
    pub fn eprset0(&mut self) -> EPRSET0_W {
        EPRSET0_W { w: self }
    }
    #[doc = "Bit 1 - Control endpoint IN (DMA cannot be enabled for this endpoint and the EP1 bit must be 0)."]
    #[inline(always)]
    pub fn eprset1(&mut self) -> EPRSET1_W {
        EPRSET1_W { w: self }
    }
    #[doc = "Bit 2 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset2(&mut self) -> EPRSET2_W {
        EPRSET2_W { w: self }
    }
    #[doc = "Bit 3 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset3(&mut self) -> EPRSET3_W {
        EPRSET3_W { w: self }
    }
    #[doc = "Bit 4 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset4(&mut self) -> EPRSET4_W {
        EPRSET4_W { w: self }
    }
    #[doc = "Bit 5 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset5(&mut self) -> EPRSET5_W {
        EPRSET5_W { w: self }
    }
    #[doc = "Bit 6 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset6(&mut self) -> EPRSET6_W {
        EPRSET6_W { w: self }
    }
    #[doc = "Bit 7 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset7(&mut self) -> EPRSET7_W {
        EPRSET7_W { w: self }
    }
    #[doc = "Bit 8 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset8(&mut self) -> EPRSET8_W {
        EPRSET8_W { w: self }
    }
    #[doc = "Bit 9 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset9(&mut self) -> EPRSET9_W {
        EPRSET9_W { w: self }
    }
    #[doc = "Bit 10 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset10(&mut self) -> EPRSET10_W {
        EPRSET10_W { w: self }
    }
    #[doc = "Bit 11 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset11(&mut self) -> EPRSET11_W {
        EPRSET11_W { w: self }
    }
    #[doc = "Bit 12 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset12(&mut self) -> EPRSET12_W {
        EPRSET12_W { w: self }
    }
    #[doc = "Bit 13 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset13(&mut self) -> EPRSET13_W {
        EPRSET13_W { w: self }
    }
    #[doc = "Bit 14 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset14(&mut self) -> EPRSET14_W {
        EPRSET14_W { w: self }
    }
    #[doc = "Bit 15 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset15(&mut self) -> EPRSET15_W {
        EPRSET15_W { w: self }
    }
    #[doc = "Bit 16 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset16(&mut self) -> EPRSET16_W {
        EPRSET16_W { w: self }
    }
    #[doc = "Bit 17 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset17(&mut self) -> EPRSET17_W {
        EPRSET17_W { w: self }
    }
    #[doc = "Bit 18 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset18(&mut self) -> EPRSET18_W {
        EPRSET18_W { w: self }
    }
    #[doc = "Bit 19 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset19(&mut self) -> EPRSET19_W {
        EPRSET19_W { w: self }
    }
    #[doc = "Bit 20 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset20(&mut self) -> EPRSET20_W {
        EPRSET20_W { w: self }
    }
    #[doc = "Bit 21 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset21(&mut self) -> EPRSET21_W {
        EPRSET21_W { w: self }
    }
    #[doc = "Bit 22 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset22(&mut self) -> EPRSET22_W {
        EPRSET22_W { w: self }
    }
    #[doc = "Bit 23 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset23(&mut self) -> EPRSET23_W {
        EPRSET23_W { w: self }
    }
    #[doc = "Bit 24 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset24(&mut self) -> EPRSET24_W {
        EPRSET24_W { w: self }
    }
    #[doc = "Bit 25 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset25(&mut self) -> EPRSET25_W {
        EPRSET25_W { w: self }
    }
    #[doc = "Bit 26 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset26(&mut self) -> EPRSET26_W {
        EPRSET26_W { w: self }
    }
    #[doc = "Bit 27 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset27(&mut self) -> EPRSET27_W {
        EPRSET27_W { w: self }
    }
    #[doc = "Bit 28 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset28(&mut self) -> EPRSET28_W {
        EPRSET28_W { w: self }
    }
    #[doc = "Bit 29 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset29(&mut self) -> EPRSET29_W {
        EPRSET29_W { w: self }
    }
    #[doc = "Bit 30 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset30(&mut self) -> EPRSET30_W {
        EPRSET30_W { w: self }
    }
    #[doc = "Bit 31 - Set the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Set the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprset31(&mut self) -> EPRSET31_W {
        EPRSET31_W { w: self }
    }
}
