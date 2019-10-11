#[doc = "Writer for register NDDRINTCLR"]
pub type W = crate::W<u32, super::NDDRINTCLR>;
#[doc = "Register NDDRINTCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::NDDRINTCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `EPNDDINTCLR0`"]
pub struct EPNDDINTCLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR1`"]
pub struct EPNDDINTCLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR2`"]
pub struct EPNDDINTCLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR3`"]
pub struct EPNDDINTCLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR4`"]
pub struct EPNDDINTCLR4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR5`"]
pub struct EPNDDINTCLR5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR6`"]
pub struct EPNDDINTCLR6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR7`"]
pub struct EPNDDINTCLR7_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR8`"]
pub struct EPNDDINTCLR8_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR9`"]
pub struct EPNDDINTCLR9_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR10`"]
pub struct EPNDDINTCLR10_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR11`"]
pub struct EPNDDINTCLR11_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR12`"]
pub struct EPNDDINTCLR12_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR13`"]
pub struct EPNDDINTCLR13_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR14`"]
pub struct EPNDDINTCLR14_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR15`"]
pub struct EPNDDINTCLR15_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR16`"]
pub struct EPNDDINTCLR16_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR17`"]
pub struct EPNDDINTCLR17_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR18`"]
pub struct EPNDDINTCLR18_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR19`"]
pub struct EPNDDINTCLR19_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR20`"]
pub struct EPNDDINTCLR20_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR21`"]
pub struct EPNDDINTCLR21_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR22`"]
pub struct EPNDDINTCLR22_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR23`"]
pub struct EPNDDINTCLR23_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR24`"]
pub struct EPNDDINTCLR24_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR25`"]
pub struct EPNDDINTCLR25_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR26`"]
pub struct EPNDDINTCLR26_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR27`"]
pub struct EPNDDINTCLR27_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR28`"]
pub struct EPNDDINTCLR28_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR29`"]
pub struct EPNDDINTCLR29_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR30`"]
pub struct EPNDDINTCLR30_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPNDDINTCLR31`"]
pub struct EPNDDINTCLR31_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNDDINTCLR31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr0(&mut self) -> EPNDDINTCLR0_W {
        EPNDDINTCLR0_W { w: self }
    }
    #[doc = "Bit 1 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr1(&mut self) -> EPNDDINTCLR1_W {
        EPNDDINTCLR1_W { w: self }
    }
    #[doc = "Bit 2 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr2(&mut self) -> EPNDDINTCLR2_W {
        EPNDDINTCLR2_W { w: self }
    }
    #[doc = "Bit 3 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr3(&mut self) -> EPNDDINTCLR3_W {
        EPNDDINTCLR3_W { w: self }
    }
    #[doc = "Bit 4 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr4(&mut self) -> EPNDDINTCLR4_W {
        EPNDDINTCLR4_W { w: self }
    }
    #[doc = "Bit 5 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr5(&mut self) -> EPNDDINTCLR5_W {
        EPNDDINTCLR5_W { w: self }
    }
    #[doc = "Bit 6 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr6(&mut self) -> EPNDDINTCLR6_W {
        EPNDDINTCLR6_W { w: self }
    }
    #[doc = "Bit 7 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr7(&mut self) -> EPNDDINTCLR7_W {
        EPNDDINTCLR7_W { w: self }
    }
    #[doc = "Bit 8 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr8(&mut self) -> EPNDDINTCLR8_W {
        EPNDDINTCLR8_W { w: self }
    }
    #[doc = "Bit 9 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr9(&mut self) -> EPNDDINTCLR9_W {
        EPNDDINTCLR9_W { w: self }
    }
    #[doc = "Bit 10 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr10(&mut self) -> EPNDDINTCLR10_W {
        EPNDDINTCLR10_W { w: self }
    }
    #[doc = "Bit 11 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr11(&mut self) -> EPNDDINTCLR11_W {
        EPNDDINTCLR11_W { w: self }
    }
    #[doc = "Bit 12 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr12(&mut self) -> EPNDDINTCLR12_W {
        EPNDDINTCLR12_W { w: self }
    }
    #[doc = "Bit 13 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr13(&mut self) -> EPNDDINTCLR13_W {
        EPNDDINTCLR13_W { w: self }
    }
    #[doc = "Bit 14 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr14(&mut self) -> EPNDDINTCLR14_W {
        EPNDDINTCLR14_W { w: self }
    }
    #[doc = "Bit 15 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr15(&mut self) -> EPNDDINTCLR15_W {
        EPNDDINTCLR15_W { w: self }
    }
    #[doc = "Bit 16 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr16(&mut self) -> EPNDDINTCLR16_W {
        EPNDDINTCLR16_W { w: self }
    }
    #[doc = "Bit 17 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr17(&mut self) -> EPNDDINTCLR17_W {
        EPNDDINTCLR17_W { w: self }
    }
    #[doc = "Bit 18 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr18(&mut self) -> EPNDDINTCLR18_W {
        EPNDDINTCLR18_W { w: self }
    }
    #[doc = "Bit 19 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr19(&mut self) -> EPNDDINTCLR19_W {
        EPNDDINTCLR19_W { w: self }
    }
    #[doc = "Bit 20 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr20(&mut self) -> EPNDDINTCLR20_W {
        EPNDDINTCLR20_W { w: self }
    }
    #[doc = "Bit 21 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr21(&mut self) -> EPNDDINTCLR21_W {
        EPNDDINTCLR21_W { w: self }
    }
    #[doc = "Bit 22 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr22(&mut self) -> EPNDDINTCLR22_W {
        EPNDDINTCLR22_W { w: self }
    }
    #[doc = "Bit 23 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr23(&mut self) -> EPNDDINTCLR23_W {
        EPNDDINTCLR23_W { w: self }
    }
    #[doc = "Bit 24 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr24(&mut self) -> EPNDDINTCLR24_W {
        EPNDDINTCLR24_W { w: self }
    }
    #[doc = "Bit 25 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr25(&mut self) -> EPNDDINTCLR25_W {
        EPNDDINTCLR25_W { w: self }
    }
    #[doc = "Bit 26 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr26(&mut self) -> EPNDDINTCLR26_W {
        EPNDDINTCLR26_W { w: self }
    }
    #[doc = "Bit 27 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr27(&mut self) -> EPNDDINTCLR27_W {
        EPNDDINTCLR27_W { w: self }
    }
    #[doc = "Bit 28 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr28(&mut self) -> EPNDDINTCLR28_W {
        EPNDDINTCLR28_W { w: self }
    }
    #[doc = "Bit 29 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr29(&mut self) -> EPNDDINTCLR29_W {
        EPNDDINTCLR29_W { w: self }
    }
    #[doc = "Bit 30 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr30(&mut self) -> EPNDDINTCLR30_W {
        EPNDDINTCLR30_W { w: self }
    }
    #[doc = "Bit 31 - Clear endpoint xx (2 <= xx <= 31) new DD interrupt request. 0 = No effect. 1 = Clear the EPxx new DD interrupt request in the USBNDDRIntSt register."]
    #[inline(always)]
    pub fn epnddintclr31(&mut self) -> EPNDDINTCLR31_W {
        EPNDDINTCLR31_W { w: self }
    }
}
