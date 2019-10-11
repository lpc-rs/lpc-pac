#[doc = "Writer for register EOTINTCLR"]
pub type W = crate::W<u32, super::EOTINTCLR>;
#[doc = "Register EOTINTCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::EOTINTCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `EPTXINTCLR0`"]
pub struct EPTXINTCLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR1`"]
pub struct EPTXINTCLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR2`"]
pub struct EPTXINTCLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR3`"]
pub struct EPTXINTCLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR4`"]
pub struct EPTXINTCLR4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR5`"]
pub struct EPTXINTCLR5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR6`"]
pub struct EPTXINTCLR6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR7`"]
pub struct EPTXINTCLR7_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR8`"]
pub struct EPTXINTCLR8_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR9`"]
pub struct EPTXINTCLR9_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR10`"]
pub struct EPTXINTCLR10_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR11`"]
pub struct EPTXINTCLR11_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR12`"]
pub struct EPTXINTCLR12_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR13`"]
pub struct EPTXINTCLR13_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR14`"]
pub struct EPTXINTCLR14_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR15`"]
pub struct EPTXINTCLR15_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR16`"]
pub struct EPTXINTCLR16_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR17`"]
pub struct EPTXINTCLR17_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR18`"]
pub struct EPTXINTCLR18_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR19`"]
pub struct EPTXINTCLR19_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR20`"]
pub struct EPTXINTCLR20_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR21`"]
pub struct EPTXINTCLR21_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR22`"]
pub struct EPTXINTCLR22_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR23`"]
pub struct EPTXINTCLR23_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR24`"]
pub struct EPTXINTCLR24_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR25`"]
pub struct EPTXINTCLR25_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR26`"]
pub struct EPTXINTCLR26_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR27`"]
pub struct EPTXINTCLR27_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR28`"]
pub struct EPTXINTCLR28_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR29`"]
pub struct EPTXINTCLR29_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR30`"]
pub struct EPTXINTCLR30_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPTXINTCLR31`"]
pub struct EPTXINTCLR31_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXINTCLR31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr0(&mut self) -> EPTXINTCLR0_W {
        EPTXINTCLR0_W { w: self }
    }
    #[doc = "Bit 1 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr1(&mut self) -> EPTXINTCLR1_W {
        EPTXINTCLR1_W { w: self }
    }
    #[doc = "Bit 2 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr2(&mut self) -> EPTXINTCLR2_W {
        EPTXINTCLR2_W { w: self }
    }
    #[doc = "Bit 3 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr3(&mut self) -> EPTXINTCLR3_W {
        EPTXINTCLR3_W { w: self }
    }
    #[doc = "Bit 4 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr4(&mut self) -> EPTXINTCLR4_W {
        EPTXINTCLR4_W { w: self }
    }
    #[doc = "Bit 5 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr5(&mut self) -> EPTXINTCLR5_W {
        EPTXINTCLR5_W { w: self }
    }
    #[doc = "Bit 6 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr6(&mut self) -> EPTXINTCLR6_W {
        EPTXINTCLR6_W { w: self }
    }
    #[doc = "Bit 7 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr7(&mut self) -> EPTXINTCLR7_W {
        EPTXINTCLR7_W { w: self }
    }
    #[doc = "Bit 8 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr8(&mut self) -> EPTXINTCLR8_W {
        EPTXINTCLR8_W { w: self }
    }
    #[doc = "Bit 9 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr9(&mut self) -> EPTXINTCLR9_W {
        EPTXINTCLR9_W { w: self }
    }
    #[doc = "Bit 10 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr10(&mut self) -> EPTXINTCLR10_W {
        EPTXINTCLR10_W { w: self }
    }
    #[doc = "Bit 11 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr11(&mut self) -> EPTXINTCLR11_W {
        EPTXINTCLR11_W { w: self }
    }
    #[doc = "Bit 12 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr12(&mut self) -> EPTXINTCLR12_W {
        EPTXINTCLR12_W { w: self }
    }
    #[doc = "Bit 13 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr13(&mut self) -> EPTXINTCLR13_W {
        EPTXINTCLR13_W { w: self }
    }
    #[doc = "Bit 14 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr14(&mut self) -> EPTXINTCLR14_W {
        EPTXINTCLR14_W { w: self }
    }
    #[doc = "Bit 15 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr15(&mut self) -> EPTXINTCLR15_W {
        EPTXINTCLR15_W { w: self }
    }
    #[doc = "Bit 16 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr16(&mut self) -> EPTXINTCLR16_W {
        EPTXINTCLR16_W { w: self }
    }
    #[doc = "Bit 17 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr17(&mut self) -> EPTXINTCLR17_W {
        EPTXINTCLR17_W { w: self }
    }
    #[doc = "Bit 18 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr18(&mut self) -> EPTXINTCLR18_W {
        EPTXINTCLR18_W { w: self }
    }
    #[doc = "Bit 19 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr19(&mut self) -> EPTXINTCLR19_W {
        EPTXINTCLR19_W { w: self }
    }
    #[doc = "Bit 20 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr20(&mut self) -> EPTXINTCLR20_W {
        EPTXINTCLR20_W { w: self }
    }
    #[doc = "Bit 21 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr21(&mut self) -> EPTXINTCLR21_W {
        EPTXINTCLR21_W { w: self }
    }
    #[doc = "Bit 22 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr22(&mut self) -> EPTXINTCLR22_W {
        EPTXINTCLR22_W { w: self }
    }
    #[doc = "Bit 23 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr23(&mut self) -> EPTXINTCLR23_W {
        EPTXINTCLR23_W { w: self }
    }
    #[doc = "Bit 24 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr24(&mut self) -> EPTXINTCLR24_W {
        EPTXINTCLR24_W { w: self }
    }
    #[doc = "Bit 25 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr25(&mut self) -> EPTXINTCLR25_W {
        EPTXINTCLR25_W { w: self }
    }
    #[doc = "Bit 26 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr26(&mut self) -> EPTXINTCLR26_W {
        EPTXINTCLR26_W { w: self }
    }
    #[doc = "Bit 27 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr27(&mut self) -> EPTXINTCLR27_W {
        EPTXINTCLR27_W { w: self }
    }
    #[doc = "Bit 28 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr28(&mut self) -> EPTXINTCLR28_W {
        EPTXINTCLR28_W { w: self }
    }
    #[doc = "Bit 29 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr29(&mut self) -> EPTXINTCLR29_W {
        EPTXINTCLR29_W { w: self }
    }
    #[doc = "Bit 30 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr30(&mut self) -> EPTXINTCLR30_W {
        EPTXINTCLR30_W { w: self }
    }
    #[doc = "Bit 31 - Clear endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = No effect. 1 = Clear the EPxx End of Transfer Interrupt request in the USBEoTIntSt register."]
    #[inline(always)]
    pub fn eptxintclr31(&mut self) -> EPTXINTCLR31_W {
        EPTXINTCLR31_W { w: self }
    }
}
