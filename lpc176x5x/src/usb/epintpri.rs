#[doc = "Writer for register EPINTPRI"]
pub type W = crate::W<u32, super::EPINTPRI>;
#[doc = "Register EPINTPRI `reset()`'s with value 0"]
impl crate::ResetValue for super::EPINTPRI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `EPPRI0`"]
pub struct EPPRI0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI1`"]
pub struct EPPRI1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI2`"]
pub struct EPPRI2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI3`"]
pub struct EPPRI3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI4`"]
pub struct EPPRI4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI5`"]
pub struct EPPRI5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI6`"]
pub struct EPPRI6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI7`"]
pub struct EPPRI7_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI8`"]
pub struct EPPRI8_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI9`"]
pub struct EPPRI9_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI10`"]
pub struct EPPRI10_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI11`"]
pub struct EPPRI11_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI12`"]
pub struct EPPRI12_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI13`"]
pub struct EPPRI13_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI14`"]
pub struct EPPRI14_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI15`"]
pub struct EPPRI15_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI16`"]
pub struct EPPRI16_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI17`"]
pub struct EPPRI17_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI18`"]
pub struct EPPRI18_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI19`"]
pub struct EPPRI19_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI20`"]
pub struct EPPRI20_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI21`"]
pub struct EPPRI21_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI22`"]
pub struct EPPRI22_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI23`"]
pub struct EPPRI23_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI24`"]
pub struct EPPRI24_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI25`"]
pub struct EPPRI25_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI26`"]
pub struct EPPRI26_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI27`"]
pub struct EPPRI27_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI28`"]
pub struct EPPRI28_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI29`"]
pub struct EPPRI29_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI30`"]
pub struct EPPRI30_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EPPRI31`"]
pub struct EPPRI31_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPRI31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri0(&mut self) -> EPPRI0_W {
        EPPRI0_W { w: self }
    }
    #[doc = "Bit 1 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri1(&mut self) -> EPPRI1_W {
        EPPRI1_W { w: self }
    }
    #[doc = "Bit 2 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri2(&mut self) -> EPPRI2_W {
        EPPRI2_W { w: self }
    }
    #[doc = "Bit 3 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri3(&mut self) -> EPPRI3_W {
        EPPRI3_W { w: self }
    }
    #[doc = "Bit 4 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri4(&mut self) -> EPPRI4_W {
        EPPRI4_W { w: self }
    }
    #[doc = "Bit 5 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri5(&mut self) -> EPPRI5_W {
        EPPRI5_W { w: self }
    }
    #[doc = "Bit 6 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri6(&mut self) -> EPPRI6_W {
        EPPRI6_W { w: self }
    }
    #[doc = "Bit 7 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri7(&mut self) -> EPPRI7_W {
        EPPRI7_W { w: self }
    }
    #[doc = "Bit 8 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri8(&mut self) -> EPPRI8_W {
        EPPRI8_W { w: self }
    }
    #[doc = "Bit 9 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri9(&mut self) -> EPPRI9_W {
        EPPRI9_W { w: self }
    }
    #[doc = "Bit 10 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri10(&mut self) -> EPPRI10_W {
        EPPRI10_W { w: self }
    }
    #[doc = "Bit 11 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri11(&mut self) -> EPPRI11_W {
        EPPRI11_W { w: self }
    }
    #[doc = "Bit 12 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri12(&mut self) -> EPPRI12_W {
        EPPRI12_W { w: self }
    }
    #[doc = "Bit 13 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri13(&mut self) -> EPPRI13_W {
        EPPRI13_W { w: self }
    }
    #[doc = "Bit 14 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri14(&mut self) -> EPPRI14_W {
        EPPRI14_W { w: self }
    }
    #[doc = "Bit 15 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri15(&mut self) -> EPPRI15_W {
        EPPRI15_W { w: self }
    }
    #[doc = "Bit 16 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri16(&mut self) -> EPPRI16_W {
        EPPRI16_W { w: self }
    }
    #[doc = "Bit 17 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri17(&mut self) -> EPPRI17_W {
        EPPRI17_W { w: self }
    }
    #[doc = "Bit 18 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri18(&mut self) -> EPPRI18_W {
        EPPRI18_W { w: self }
    }
    #[doc = "Bit 19 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri19(&mut self) -> EPPRI19_W {
        EPPRI19_W { w: self }
    }
    #[doc = "Bit 20 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri20(&mut self) -> EPPRI20_W {
        EPPRI20_W { w: self }
    }
    #[doc = "Bit 21 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri21(&mut self) -> EPPRI21_W {
        EPPRI21_W { w: self }
    }
    #[doc = "Bit 22 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri22(&mut self) -> EPPRI22_W {
        EPPRI22_W { w: self }
    }
    #[doc = "Bit 23 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri23(&mut self) -> EPPRI23_W {
        EPPRI23_W { w: self }
    }
    #[doc = "Bit 24 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri24(&mut self) -> EPPRI24_W {
        EPPRI24_W { w: self }
    }
    #[doc = "Bit 25 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri25(&mut self) -> EPPRI25_W {
        EPPRI25_W { w: self }
    }
    #[doc = "Bit 26 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri26(&mut self) -> EPPRI26_W {
        EPPRI26_W { w: self }
    }
    #[doc = "Bit 27 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri27(&mut self) -> EPPRI27_W {
        EPPRI27_W { w: self }
    }
    #[doc = "Bit 28 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri28(&mut self) -> EPPRI28_W {
        EPPRI28_W { w: self }
    }
    #[doc = "Bit 29 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri29(&mut self) -> EPPRI29_W {
        EPPRI29_W { w: self }
    }
    #[doc = "Bit 30 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri30(&mut self) -> EPPRI30_W {
        EPPRI30_W { w: self }
    }
    #[doc = "Bit 31 - 0 = The corresponding interrupt is routed to the EP_SLOW bit of USBDevIntSt 1 = The corresponding interrupt is routed to the EP_FAST bit of USBDevIntSt"]
    #[inline(always)]
    pub fn eppri31(&mut self) -> EPPRI31_W {
        EPPRI31_W { w: self }
    }
}
