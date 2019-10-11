#[doc = "Writer for register DMARCLR"]
pub type W = crate::W<u32, super::DMARCLR>;
#[doc = "Register DMARCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMARCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `EPRCLR0`"]
pub struct EPRCLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR1`"]
pub struct EPRCLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR2`"]
pub struct EPRCLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR3`"]
pub struct EPRCLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR4`"]
pub struct EPRCLR4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR5`"]
pub struct EPRCLR5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR6`"]
pub struct EPRCLR6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR7`"]
pub struct EPRCLR7_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR8`"]
pub struct EPRCLR8_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR9`"]
pub struct EPRCLR9_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR10`"]
pub struct EPRCLR10_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR11`"]
pub struct EPRCLR11_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR12`"]
pub struct EPRCLR12_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR13`"]
pub struct EPRCLR13_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR14`"]
pub struct EPRCLR14_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR15`"]
pub struct EPRCLR15_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR16`"]
pub struct EPRCLR16_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR17`"]
pub struct EPRCLR17_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR18`"]
pub struct EPRCLR18_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR19`"]
pub struct EPRCLR19_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR20`"]
pub struct EPRCLR20_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR21`"]
pub struct EPRCLR21_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR22`"]
pub struct EPRCLR22_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR23`"]
pub struct EPRCLR23_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR24`"]
pub struct EPRCLR24_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR25`"]
pub struct EPRCLR25_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR26`"]
pub struct EPRCLR26_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR27`"]
pub struct EPRCLR27_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR28`"]
pub struct EPRCLR28_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR29`"]
pub struct EPRCLR29_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR30`"]
pub struct EPRCLR30_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPRCLR31`"]
pub struct EPRCLR31_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRCLR31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    pub fn eprclr0(&mut self) -> EPRCLR0_W {
        EPRCLR0_W { w: self }
    }
    #[doc = "Bit 1 - Control endpoint IN (DMA cannot be enabled for this endpoint and the EP1 bit must be 0)."]
    #[inline(always)]
    pub fn eprclr1(&mut self) -> EPRCLR1_W {
        EPRCLR1_W { w: self }
    }
    #[doc = "Bit 2 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr2(&mut self) -> EPRCLR2_W {
        EPRCLR2_W { w: self }
    }
    #[doc = "Bit 3 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr3(&mut self) -> EPRCLR3_W {
        EPRCLR3_W { w: self }
    }
    #[doc = "Bit 4 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr4(&mut self) -> EPRCLR4_W {
        EPRCLR4_W { w: self }
    }
    #[doc = "Bit 5 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr5(&mut self) -> EPRCLR5_W {
        EPRCLR5_W { w: self }
    }
    #[doc = "Bit 6 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr6(&mut self) -> EPRCLR6_W {
        EPRCLR6_W { w: self }
    }
    #[doc = "Bit 7 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr7(&mut self) -> EPRCLR7_W {
        EPRCLR7_W { w: self }
    }
    #[doc = "Bit 8 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr8(&mut self) -> EPRCLR8_W {
        EPRCLR8_W { w: self }
    }
    #[doc = "Bit 9 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr9(&mut self) -> EPRCLR9_W {
        EPRCLR9_W { w: self }
    }
    #[doc = "Bit 10 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr10(&mut self) -> EPRCLR10_W {
        EPRCLR10_W { w: self }
    }
    #[doc = "Bit 11 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr11(&mut self) -> EPRCLR11_W {
        EPRCLR11_W { w: self }
    }
    #[doc = "Bit 12 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr12(&mut self) -> EPRCLR12_W {
        EPRCLR12_W { w: self }
    }
    #[doc = "Bit 13 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr13(&mut self) -> EPRCLR13_W {
        EPRCLR13_W { w: self }
    }
    #[doc = "Bit 14 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr14(&mut self) -> EPRCLR14_W {
        EPRCLR14_W { w: self }
    }
    #[doc = "Bit 15 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr15(&mut self) -> EPRCLR15_W {
        EPRCLR15_W { w: self }
    }
    #[doc = "Bit 16 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr16(&mut self) -> EPRCLR16_W {
        EPRCLR16_W { w: self }
    }
    #[doc = "Bit 17 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr17(&mut self) -> EPRCLR17_W {
        EPRCLR17_W { w: self }
    }
    #[doc = "Bit 18 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr18(&mut self) -> EPRCLR18_W {
        EPRCLR18_W { w: self }
    }
    #[doc = "Bit 19 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr19(&mut self) -> EPRCLR19_W {
        EPRCLR19_W { w: self }
    }
    #[doc = "Bit 20 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr20(&mut self) -> EPRCLR20_W {
        EPRCLR20_W { w: self }
    }
    #[doc = "Bit 21 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr21(&mut self) -> EPRCLR21_W {
        EPRCLR21_W { w: self }
    }
    #[doc = "Bit 22 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr22(&mut self) -> EPRCLR22_W {
        EPRCLR22_W { w: self }
    }
    #[doc = "Bit 23 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr23(&mut self) -> EPRCLR23_W {
        EPRCLR23_W { w: self }
    }
    #[doc = "Bit 24 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr24(&mut self) -> EPRCLR24_W {
        EPRCLR24_W { w: self }
    }
    #[doc = "Bit 25 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr25(&mut self) -> EPRCLR25_W {
        EPRCLR25_W { w: self }
    }
    #[doc = "Bit 26 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr26(&mut self) -> EPRCLR26_W {
        EPRCLR26_W { w: self }
    }
    #[doc = "Bit 27 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr27(&mut self) -> EPRCLR27_W {
        EPRCLR27_W { w: self }
    }
    #[doc = "Bit 28 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr28(&mut self) -> EPRCLR28_W {
        EPRCLR28_W { w: self }
    }
    #[doc = "Bit 29 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr29(&mut self) -> EPRCLR29_W {
        EPRCLR29_W { w: self }
    }
    #[doc = "Bit 30 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr30(&mut self) -> EPRCLR30_W {
        EPRCLR30_W { w: self }
    }
    #[doc = "Bit 31 - Clear the endpoint xx (2 <= xx <= 31) DMA request. 0 = No effect 1 = Clear the corresponding bit in USBDMARSt."]
    #[inline(always)]
    pub fn eprclr31(&mut self) -> EPRCLR31_W {
        EPRCLR31_W { w: self }
    }
}
