#[doc = "Writer for register EPINTCLR"]
pub type W = crate::W<u32, super::EPINTCLR>;
#[doc = "Register EPINTCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::EPINTCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `EPCLR0`"]
pub struct EPCLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR1`"]
pub struct EPCLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR2`"]
pub struct EPCLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR3`"]
pub struct EPCLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR4`"]
pub struct EPCLR4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR5`"]
pub struct EPCLR5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR6`"]
pub struct EPCLR6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR7`"]
pub struct EPCLR7_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR8`"]
pub struct EPCLR8_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR9`"]
pub struct EPCLR9_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR10`"]
pub struct EPCLR10_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR11`"]
pub struct EPCLR11_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR12`"]
pub struct EPCLR12_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR13`"]
pub struct EPCLR13_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR14`"]
pub struct EPCLR14_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR15`"]
pub struct EPCLR15_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR16`"]
pub struct EPCLR16_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR17`"]
pub struct EPCLR17_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR18`"]
pub struct EPCLR18_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR19`"]
pub struct EPCLR19_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR20`"]
pub struct EPCLR20_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR21`"]
pub struct EPCLR21_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR22`"]
pub struct EPCLR22_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR23`"]
pub struct EPCLR23_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR24`"]
pub struct EPCLR24_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR25`"]
pub struct EPCLR25_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR26`"]
pub struct EPCLR26_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR27`"]
pub struct EPCLR27_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR28`"]
pub struct EPCLR28_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR29`"]
pub struct EPCLR29_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR30`"]
pub struct EPCLR30_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPCLR31`"]
pub struct EPCLR31_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLR31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr0(&mut self) -> EPCLR0_W {
        EPCLR0_W { w: self }
    }
    #[doc = "Bit 1 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr1(&mut self) -> EPCLR1_W {
        EPCLR1_W { w: self }
    }
    #[doc = "Bit 2 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr2(&mut self) -> EPCLR2_W {
        EPCLR2_W { w: self }
    }
    #[doc = "Bit 3 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr3(&mut self) -> EPCLR3_W {
        EPCLR3_W { w: self }
    }
    #[doc = "Bit 4 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr4(&mut self) -> EPCLR4_W {
        EPCLR4_W { w: self }
    }
    #[doc = "Bit 5 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr5(&mut self) -> EPCLR5_W {
        EPCLR5_W { w: self }
    }
    #[doc = "Bit 6 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr6(&mut self) -> EPCLR6_W {
        EPCLR6_W { w: self }
    }
    #[doc = "Bit 7 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr7(&mut self) -> EPCLR7_W {
        EPCLR7_W { w: self }
    }
    #[doc = "Bit 8 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr8(&mut self) -> EPCLR8_W {
        EPCLR8_W { w: self }
    }
    #[doc = "Bit 9 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr9(&mut self) -> EPCLR9_W {
        EPCLR9_W { w: self }
    }
    #[doc = "Bit 10 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr10(&mut self) -> EPCLR10_W {
        EPCLR10_W { w: self }
    }
    #[doc = "Bit 11 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr11(&mut self) -> EPCLR11_W {
        EPCLR11_W { w: self }
    }
    #[doc = "Bit 12 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr12(&mut self) -> EPCLR12_W {
        EPCLR12_W { w: self }
    }
    #[doc = "Bit 13 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr13(&mut self) -> EPCLR13_W {
        EPCLR13_W { w: self }
    }
    #[doc = "Bit 14 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr14(&mut self) -> EPCLR14_W {
        EPCLR14_W { w: self }
    }
    #[doc = "Bit 15 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr15(&mut self) -> EPCLR15_W {
        EPCLR15_W { w: self }
    }
    #[doc = "Bit 16 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr16(&mut self) -> EPCLR16_W {
        EPCLR16_W { w: self }
    }
    #[doc = "Bit 17 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr17(&mut self) -> EPCLR17_W {
        EPCLR17_W { w: self }
    }
    #[doc = "Bit 18 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr18(&mut self) -> EPCLR18_W {
        EPCLR18_W { w: self }
    }
    #[doc = "Bit 19 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr19(&mut self) -> EPCLR19_W {
        EPCLR19_W { w: self }
    }
    #[doc = "Bit 20 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr20(&mut self) -> EPCLR20_W {
        EPCLR20_W { w: self }
    }
    #[doc = "Bit 21 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr21(&mut self) -> EPCLR21_W {
        EPCLR21_W { w: self }
    }
    #[doc = "Bit 22 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr22(&mut self) -> EPCLR22_W {
        EPCLR22_W { w: self }
    }
    #[doc = "Bit 23 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr23(&mut self) -> EPCLR23_W {
        EPCLR23_W { w: self }
    }
    #[doc = "Bit 24 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr24(&mut self) -> EPCLR24_W {
        EPCLR24_W { w: self }
    }
    #[doc = "Bit 25 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr25(&mut self) -> EPCLR25_W {
        EPCLR25_W { w: self }
    }
    #[doc = "Bit 26 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr26(&mut self) -> EPCLR26_W {
        EPCLR26_W { w: self }
    }
    #[doc = "Bit 27 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr27(&mut self) -> EPCLR27_W {
        EPCLR27_W { w: self }
    }
    #[doc = "Bit 28 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr28(&mut self) -> EPCLR28_W {
        EPCLR28_W { w: self }
    }
    #[doc = "Bit 29 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr29(&mut self) -> EPCLR29_W {
        EPCLR29_W { w: self }
    }
    #[doc = "Bit 30 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr30(&mut self) -> EPCLR30_W {
        EPCLR30_W { w: self }
    }
    #[doc = "Bit 31 - 0 = No effect. 1 = Clears the corresponding bit in USBEpIntSt, by executing the SIE Select Endpoint/Clear Interrupt command for this endpoint."]
    #[inline(always)]
    pub fn epclr31(&mut self) -> EPCLR31_W {
        EPCLR31_W { w: self }
    }
}
