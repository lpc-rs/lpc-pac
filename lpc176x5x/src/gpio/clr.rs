#[doc = "Writer for register CLR%s"]
pub type W = crate::W<u32, super::CLR>;
#[doc = "Register CLR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `PINCLR0`"]
pub struct PINCLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR1`"]
pub struct PINCLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR2`"]
pub struct PINCLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR3`"]
pub struct PINCLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR4`"]
pub struct PINCLR4_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR5`"]
pub struct PINCLR5_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR6`"]
pub struct PINCLR6_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR7`"]
pub struct PINCLR7_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR8`"]
pub struct PINCLR8_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR9`"]
pub struct PINCLR9_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR10`"]
pub struct PINCLR10_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR11`"]
pub struct PINCLR11_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR12`"]
pub struct PINCLR12_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR13`"]
pub struct PINCLR13_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR14`"]
pub struct PINCLR14_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR15`"]
pub struct PINCLR15_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR16`"]
pub struct PINCLR16_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR17`"]
pub struct PINCLR17_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR18`"]
pub struct PINCLR18_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR19`"]
pub struct PINCLR19_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR20`"]
pub struct PINCLR20_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR21`"]
pub struct PINCLR21_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR22`"]
pub struct PINCLR22_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR23`"]
pub struct PINCLR23_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR24`"]
pub struct PINCLR24_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR25`"]
pub struct PINCLR25_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR26`"]
pub struct PINCLR26_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR27`"]
pub struct PINCLR27_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR28`"]
pub struct PINCLR28_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR29`"]
pub struct PINCLR29_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR30`"]
pub struct PINCLR30_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `PINCLR31`"]
pub struct PINCLR31_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCLR31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr0(&mut self) -> PINCLR0_W {
        PINCLR0_W { w: self }
    }
    #[doc = "Bit 1 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr1(&mut self) -> PINCLR1_W {
        PINCLR1_W { w: self }
    }
    #[doc = "Bit 2 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr2(&mut self) -> PINCLR2_W {
        PINCLR2_W { w: self }
    }
    #[doc = "Bit 3 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr3(&mut self) -> PINCLR3_W {
        PINCLR3_W { w: self }
    }
    #[doc = "Bit 4 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr4(&mut self) -> PINCLR4_W {
        PINCLR4_W { w: self }
    }
    #[doc = "Bit 5 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr5(&mut self) -> PINCLR5_W {
        PINCLR5_W { w: self }
    }
    #[doc = "Bit 6 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr6(&mut self) -> PINCLR6_W {
        PINCLR6_W { w: self }
    }
    #[doc = "Bit 7 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr7(&mut self) -> PINCLR7_W {
        PINCLR7_W { w: self }
    }
    #[doc = "Bit 8 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr8(&mut self) -> PINCLR8_W {
        PINCLR8_W { w: self }
    }
    #[doc = "Bit 9 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr9(&mut self) -> PINCLR9_W {
        PINCLR9_W { w: self }
    }
    #[doc = "Bit 10 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr10(&mut self) -> PINCLR10_W {
        PINCLR10_W { w: self }
    }
    #[doc = "Bit 11 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr11(&mut self) -> PINCLR11_W {
        PINCLR11_W { w: self }
    }
    #[doc = "Bit 12 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr12(&mut self) -> PINCLR12_W {
        PINCLR12_W { w: self }
    }
    #[doc = "Bit 13 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr13(&mut self) -> PINCLR13_W {
        PINCLR13_W { w: self }
    }
    #[doc = "Bit 14 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr14(&mut self) -> PINCLR14_W {
        PINCLR14_W { w: self }
    }
    #[doc = "Bit 15 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr15(&mut self) -> PINCLR15_W {
        PINCLR15_W { w: self }
    }
    #[doc = "Bit 16 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr16(&mut self) -> PINCLR16_W {
        PINCLR16_W { w: self }
    }
    #[doc = "Bit 17 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr17(&mut self) -> PINCLR17_W {
        PINCLR17_W { w: self }
    }
    #[doc = "Bit 18 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr18(&mut self) -> PINCLR18_W {
        PINCLR18_W { w: self }
    }
    #[doc = "Bit 19 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr19(&mut self) -> PINCLR19_W {
        PINCLR19_W { w: self }
    }
    #[doc = "Bit 20 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr20(&mut self) -> PINCLR20_W {
        PINCLR20_W { w: self }
    }
    #[doc = "Bit 21 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr21(&mut self) -> PINCLR21_W {
        PINCLR21_W { w: self }
    }
    #[doc = "Bit 22 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr22(&mut self) -> PINCLR22_W {
        PINCLR22_W { w: self }
    }
    #[doc = "Bit 23 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr23(&mut self) -> PINCLR23_W {
        PINCLR23_W { w: self }
    }
    #[doc = "Bit 24 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr24(&mut self) -> PINCLR24_W {
        PINCLR24_W { w: self }
    }
    #[doc = "Bit 25 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr25(&mut self) -> PINCLR25_W {
        PINCLR25_W { w: self }
    }
    #[doc = "Bit 26 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr26(&mut self) -> PINCLR26_W {
        PINCLR26_W { w: self }
    }
    #[doc = "Bit 27 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr27(&mut self) -> PINCLR27_W {
        PINCLR27_W { w: self }
    }
    #[doc = "Bit 28 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr28(&mut self) -> PINCLR28_W {
        PINCLR28_W { w: self }
    }
    #[doc = "Bit 29 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr29(&mut self) -> PINCLR29_W {
        PINCLR29_W { w: self }
    }
    #[doc = "Bit 30 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr30(&mut self) -> PINCLR30_W {
        PINCLR30_W { w: self }
    }
    #[doc = "Bit 31 - Fast GPIO output value Clear bits. Bit 0 in CLRx controls pin Px\\[0\\], bit 31 in CLRx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to LOW."]
    #[inline(always)]
    pub fn pinclr31(&mut self) -> PINCLR31_W {
        PINCLR31_W { w: self }
    }
}
