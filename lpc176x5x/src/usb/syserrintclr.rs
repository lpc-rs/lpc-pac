#[doc = "Writer for register SYSERRINTCLR"]
pub type W = crate::W<u32, super::SYSERRINTCLR>;
#[doc = "Register SYSERRINTCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSERRINTCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `EPERRINTCLR0`"]
pub struct EPERRINTCLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR1`"]
pub struct EPERRINTCLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR2`"]
pub struct EPERRINTCLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR3`"]
pub struct EPERRINTCLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR4`"]
pub struct EPERRINTCLR4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR5`"]
pub struct EPERRINTCLR5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR6`"]
pub struct EPERRINTCLR6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR7`"]
pub struct EPERRINTCLR7_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR8`"]
pub struct EPERRINTCLR8_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR9`"]
pub struct EPERRINTCLR9_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR10`"]
pub struct EPERRINTCLR10_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR11`"]
pub struct EPERRINTCLR11_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR12`"]
pub struct EPERRINTCLR12_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR13`"]
pub struct EPERRINTCLR13_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR14`"]
pub struct EPERRINTCLR14_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR15`"]
pub struct EPERRINTCLR15_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR16`"]
pub struct EPERRINTCLR16_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR17`"]
pub struct EPERRINTCLR17_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR18`"]
pub struct EPERRINTCLR18_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR19`"]
pub struct EPERRINTCLR19_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR20`"]
pub struct EPERRINTCLR20_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR21`"]
pub struct EPERRINTCLR21_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR22`"]
pub struct EPERRINTCLR22_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR23`"]
pub struct EPERRINTCLR23_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR24`"]
pub struct EPERRINTCLR24_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR25`"]
pub struct EPERRINTCLR25_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR26`"]
pub struct EPERRINTCLR26_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR27`"]
pub struct EPERRINTCLR27_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR28`"]
pub struct EPERRINTCLR28_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR29`"]
pub struct EPERRINTCLR29_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR30`"]
pub struct EPERRINTCLR30_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPERRINTCLR31`"]
pub struct EPERRINTCLR31_W<'a> {
    w: &'a mut W,
}
impl<'a> EPERRINTCLR31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr0(&mut self) -> EPERRINTCLR0_W {
        EPERRINTCLR0_W { w: self }
    }
    #[doc = "Bit 1 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr1(&mut self) -> EPERRINTCLR1_W {
        EPERRINTCLR1_W { w: self }
    }
    #[doc = "Bit 2 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr2(&mut self) -> EPERRINTCLR2_W {
        EPERRINTCLR2_W { w: self }
    }
    #[doc = "Bit 3 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr3(&mut self) -> EPERRINTCLR3_W {
        EPERRINTCLR3_W { w: self }
    }
    #[doc = "Bit 4 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr4(&mut self) -> EPERRINTCLR4_W {
        EPERRINTCLR4_W { w: self }
    }
    #[doc = "Bit 5 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr5(&mut self) -> EPERRINTCLR5_W {
        EPERRINTCLR5_W { w: self }
    }
    #[doc = "Bit 6 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr6(&mut self) -> EPERRINTCLR6_W {
        EPERRINTCLR6_W { w: self }
    }
    #[doc = "Bit 7 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr7(&mut self) -> EPERRINTCLR7_W {
        EPERRINTCLR7_W { w: self }
    }
    #[doc = "Bit 8 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr8(&mut self) -> EPERRINTCLR8_W {
        EPERRINTCLR8_W { w: self }
    }
    #[doc = "Bit 9 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr9(&mut self) -> EPERRINTCLR9_W {
        EPERRINTCLR9_W { w: self }
    }
    #[doc = "Bit 10 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr10(&mut self) -> EPERRINTCLR10_W {
        EPERRINTCLR10_W { w: self }
    }
    #[doc = "Bit 11 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr11(&mut self) -> EPERRINTCLR11_W {
        EPERRINTCLR11_W { w: self }
    }
    #[doc = "Bit 12 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr12(&mut self) -> EPERRINTCLR12_W {
        EPERRINTCLR12_W { w: self }
    }
    #[doc = "Bit 13 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr13(&mut self) -> EPERRINTCLR13_W {
        EPERRINTCLR13_W { w: self }
    }
    #[doc = "Bit 14 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr14(&mut self) -> EPERRINTCLR14_W {
        EPERRINTCLR14_W { w: self }
    }
    #[doc = "Bit 15 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr15(&mut self) -> EPERRINTCLR15_W {
        EPERRINTCLR15_W { w: self }
    }
    #[doc = "Bit 16 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr16(&mut self) -> EPERRINTCLR16_W {
        EPERRINTCLR16_W { w: self }
    }
    #[doc = "Bit 17 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr17(&mut self) -> EPERRINTCLR17_W {
        EPERRINTCLR17_W { w: self }
    }
    #[doc = "Bit 18 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr18(&mut self) -> EPERRINTCLR18_W {
        EPERRINTCLR18_W { w: self }
    }
    #[doc = "Bit 19 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr19(&mut self) -> EPERRINTCLR19_W {
        EPERRINTCLR19_W { w: self }
    }
    #[doc = "Bit 20 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr20(&mut self) -> EPERRINTCLR20_W {
        EPERRINTCLR20_W { w: self }
    }
    #[doc = "Bit 21 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr21(&mut self) -> EPERRINTCLR21_W {
        EPERRINTCLR21_W { w: self }
    }
    #[doc = "Bit 22 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr22(&mut self) -> EPERRINTCLR22_W {
        EPERRINTCLR22_W { w: self }
    }
    #[doc = "Bit 23 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr23(&mut self) -> EPERRINTCLR23_W {
        EPERRINTCLR23_W { w: self }
    }
    #[doc = "Bit 24 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr24(&mut self) -> EPERRINTCLR24_W {
        EPERRINTCLR24_W { w: self }
    }
    #[doc = "Bit 25 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr25(&mut self) -> EPERRINTCLR25_W {
        EPERRINTCLR25_W { w: self }
    }
    #[doc = "Bit 26 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr26(&mut self) -> EPERRINTCLR26_W {
        EPERRINTCLR26_W { w: self }
    }
    #[doc = "Bit 27 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr27(&mut self) -> EPERRINTCLR27_W {
        EPERRINTCLR27_W { w: self }
    }
    #[doc = "Bit 28 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr28(&mut self) -> EPERRINTCLR28_W {
        EPERRINTCLR28_W { w: self }
    }
    #[doc = "Bit 29 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr29(&mut self) -> EPERRINTCLR29_W {
        EPERRINTCLR29_W { w: self }
    }
    #[doc = "Bit 30 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr30(&mut self) -> EPERRINTCLR30_W {
        EPERRINTCLR30_W { w: self }
    }
    #[doc = "Bit 31 - Clear endpoint xx (2 <= xx <= 31) System Error Interrupt request. 0 = No effect. 1 = Clear the EPxx System Error Interrupt request in the USBSysErrIntSt register."]
    #[inline(always)]
    pub fn eperrintclr31(&mut self) -> EPERRINTCLR31_W {
        EPERRINTCLR31_W { w: self }
    }
}
