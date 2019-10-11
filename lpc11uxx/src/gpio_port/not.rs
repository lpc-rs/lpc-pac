#[doc = "Writer for register NOT%s"]
pub type W = crate::W<u32, super::NOT>;
#[doc = "Register NOT%s `reset()`'s with value 0"]
impl crate::ResetValue for super::NOT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `NOTP0`"]
pub struct NOTP0_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP1`"]
pub struct NOTP1_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP2`"]
pub struct NOTP2_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP3`"]
pub struct NOTP3_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP4`"]
pub struct NOTP4_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP5`"]
pub struct NOTP5_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP6`"]
pub struct NOTP6_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP7`"]
pub struct NOTP7_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP8`"]
pub struct NOTP8_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP9`"]
pub struct NOTP9_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP10`"]
pub struct NOTP10_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP11`"]
pub struct NOTP11_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP12`"]
pub struct NOTP12_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP13`"]
pub struct NOTP13_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP14`"]
pub struct NOTP14_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP15`"]
pub struct NOTP15_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP16`"]
pub struct NOTP16_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP17`"]
pub struct NOTP17_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP18`"]
pub struct NOTP18_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP19`"]
pub struct NOTP19_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP20`"]
pub struct NOTP20_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP21`"]
pub struct NOTP21_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP22`"]
pub struct NOTP22_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP23`"]
pub struct NOTP23_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP24`"]
pub struct NOTP24_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP25`"]
pub struct NOTP25_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP26`"]
pub struct NOTP26_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP27`"]
pub struct NOTP27_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP28`"]
pub struct NOTP28_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP29`"]
pub struct NOTP29_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP30`"]
pub struct NOTP30_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `NOTP31`"]
pub struct NOTP31_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp0(&mut self) -> NOTP0_W {
        NOTP0_W { w: self }
    }
    #[doc = "Bit 1 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp1(&mut self) -> NOTP1_W {
        NOTP1_W { w: self }
    }
    #[doc = "Bit 2 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp2(&mut self) -> NOTP2_W {
        NOTP2_W { w: self }
    }
    #[doc = "Bit 3 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp3(&mut self) -> NOTP3_W {
        NOTP3_W { w: self }
    }
    #[doc = "Bit 4 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp4(&mut self) -> NOTP4_W {
        NOTP4_W { w: self }
    }
    #[doc = "Bit 5 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp5(&mut self) -> NOTP5_W {
        NOTP5_W { w: self }
    }
    #[doc = "Bit 6 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp6(&mut self) -> NOTP6_W {
        NOTP6_W { w: self }
    }
    #[doc = "Bit 7 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp7(&mut self) -> NOTP7_W {
        NOTP7_W { w: self }
    }
    #[doc = "Bit 8 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp8(&mut self) -> NOTP8_W {
        NOTP8_W { w: self }
    }
    #[doc = "Bit 9 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp9(&mut self) -> NOTP9_W {
        NOTP9_W { w: self }
    }
    #[doc = "Bit 10 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp10(&mut self) -> NOTP10_W {
        NOTP10_W { w: self }
    }
    #[doc = "Bit 11 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp11(&mut self) -> NOTP11_W {
        NOTP11_W { w: self }
    }
    #[doc = "Bit 12 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp12(&mut self) -> NOTP12_W {
        NOTP12_W { w: self }
    }
    #[doc = "Bit 13 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp13(&mut self) -> NOTP13_W {
        NOTP13_W { w: self }
    }
    #[doc = "Bit 14 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp14(&mut self) -> NOTP14_W {
        NOTP14_W { w: self }
    }
    #[doc = "Bit 15 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp15(&mut self) -> NOTP15_W {
        NOTP15_W { w: self }
    }
    #[doc = "Bit 16 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp16(&mut self) -> NOTP16_W {
        NOTP16_W { w: self }
    }
    #[doc = "Bit 17 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp17(&mut self) -> NOTP17_W {
        NOTP17_W { w: self }
    }
    #[doc = "Bit 18 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp18(&mut self) -> NOTP18_W {
        NOTP18_W { w: self }
    }
    #[doc = "Bit 19 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp19(&mut self) -> NOTP19_W {
        NOTP19_W { w: self }
    }
    #[doc = "Bit 20 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp20(&mut self) -> NOTP20_W {
        NOTP20_W { w: self }
    }
    #[doc = "Bit 21 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp21(&mut self) -> NOTP21_W {
        NOTP21_W { w: self }
    }
    #[doc = "Bit 22 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp22(&mut self) -> NOTP22_W {
        NOTP22_W { w: self }
    }
    #[doc = "Bit 23 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp23(&mut self) -> NOTP23_W {
        NOTP23_W { w: self }
    }
    #[doc = "Bit 24 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp24(&mut self) -> NOTP24_W {
        NOTP24_W { w: self }
    }
    #[doc = "Bit 25 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp25(&mut self) -> NOTP25_W {
        NOTP25_W { w: self }
    }
    #[doc = "Bit 26 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp26(&mut self) -> NOTP26_W {
        NOTP26_W { w: self }
    }
    #[doc = "Bit 27 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp27(&mut self) -> NOTP27_W {
        NOTP27_W { w: self }
    }
    #[doc = "Bit 28 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp28(&mut self) -> NOTP28_W {
        NOTP28_W { w: self }
    }
    #[doc = "Bit 29 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp29(&mut self) -> NOTP29_W {
        NOTP29_W { w: self }
    }
    #[doc = "Bit 30 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp30(&mut self) -> NOTP30_W {
        NOTP30_W { w: self }
    }
    #[doc = "Bit 31 - Toggle output bits: 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp31(&mut self) -> NOTP31_W {
        NOTP31_W { w: self }
    }
}
