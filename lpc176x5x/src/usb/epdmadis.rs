#[doc = "Writer for register EPDMADIS"]
pub type W = crate::W<u32, super::EPDMADIS>;
#[doc = "Register EPDMADIS `reset()`'s with value 0"]
impl crate::ResetValue for super::EPDMADIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `EP_DMA_DIS0`"]
pub struct EP_DMA_DIS0_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS1`"]
pub struct EP_DMA_DIS1_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS2`"]
pub struct EP_DMA_DIS2_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS3`"]
pub struct EP_DMA_DIS3_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS4`"]
pub struct EP_DMA_DIS4_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS5`"]
pub struct EP_DMA_DIS5_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS6`"]
pub struct EP_DMA_DIS6_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS7`"]
pub struct EP_DMA_DIS7_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS8`"]
pub struct EP_DMA_DIS8_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS9`"]
pub struct EP_DMA_DIS9_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS10`"]
pub struct EP_DMA_DIS10_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS11`"]
pub struct EP_DMA_DIS11_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS12`"]
pub struct EP_DMA_DIS12_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS13`"]
pub struct EP_DMA_DIS13_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS14`"]
pub struct EP_DMA_DIS14_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS15`"]
pub struct EP_DMA_DIS15_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS16`"]
pub struct EP_DMA_DIS16_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS17`"]
pub struct EP_DMA_DIS17_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS18`"]
pub struct EP_DMA_DIS18_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS19`"]
pub struct EP_DMA_DIS19_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS20`"]
pub struct EP_DMA_DIS20_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS21`"]
pub struct EP_DMA_DIS21_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS22`"]
pub struct EP_DMA_DIS22_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS23`"]
pub struct EP_DMA_DIS23_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS24`"]
pub struct EP_DMA_DIS24_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS25`"]
pub struct EP_DMA_DIS25_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS26`"]
pub struct EP_DMA_DIS26_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS27`"]
pub struct EP_DMA_DIS27_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS28`"]
pub struct EP_DMA_DIS28_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS29`"]
pub struct EP_DMA_DIS29_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS30`"]
pub struct EP_DMA_DIS30_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `EP_DMA_DIS31`"]
pub struct EP_DMA_DIS31_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_DIS31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - Control endpoint OUT (DMA cannot be enabled for this endpoint and the EP0_DMA_DISABLE bit value must be 0)."]
    #[inline(always)]
    pub fn ep_dma_dis0(&mut self) -> EP_DMA_DIS0_W {
        EP_DMA_DIS0_W { w: self }
    }
    #[doc = "Bit 1 - Control endpoint IN (DMA cannot be enabled for this endpoint and the EP1_DMA_DISABLE bit value must be 0)."]
    #[inline(always)]
    pub fn ep_dma_dis1(&mut self) -> EP_DMA_DIS1_W {
        EP_DMA_DIS1_W { w: self }
    }
    #[doc = "Bit 2 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis2(&mut self) -> EP_DMA_DIS2_W {
        EP_DMA_DIS2_W { w: self }
    }
    #[doc = "Bit 3 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis3(&mut self) -> EP_DMA_DIS3_W {
        EP_DMA_DIS3_W { w: self }
    }
    #[doc = "Bit 4 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis4(&mut self) -> EP_DMA_DIS4_W {
        EP_DMA_DIS4_W { w: self }
    }
    #[doc = "Bit 5 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis5(&mut self) -> EP_DMA_DIS5_W {
        EP_DMA_DIS5_W { w: self }
    }
    #[doc = "Bit 6 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis6(&mut self) -> EP_DMA_DIS6_W {
        EP_DMA_DIS6_W { w: self }
    }
    #[doc = "Bit 7 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis7(&mut self) -> EP_DMA_DIS7_W {
        EP_DMA_DIS7_W { w: self }
    }
    #[doc = "Bit 8 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis8(&mut self) -> EP_DMA_DIS8_W {
        EP_DMA_DIS8_W { w: self }
    }
    #[doc = "Bit 9 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis9(&mut self) -> EP_DMA_DIS9_W {
        EP_DMA_DIS9_W { w: self }
    }
    #[doc = "Bit 10 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis10(&mut self) -> EP_DMA_DIS10_W {
        EP_DMA_DIS10_W { w: self }
    }
    #[doc = "Bit 11 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis11(&mut self) -> EP_DMA_DIS11_W {
        EP_DMA_DIS11_W { w: self }
    }
    #[doc = "Bit 12 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis12(&mut self) -> EP_DMA_DIS12_W {
        EP_DMA_DIS12_W { w: self }
    }
    #[doc = "Bit 13 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis13(&mut self) -> EP_DMA_DIS13_W {
        EP_DMA_DIS13_W { w: self }
    }
    #[doc = "Bit 14 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis14(&mut self) -> EP_DMA_DIS14_W {
        EP_DMA_DIS14_W { w: self }
    }
    #[doc = "Bit 15 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis15(&mut self) -> EP_DMA_DIS15_W {
        EP_DMA_DIS15_W { w: self }
    }
    #[doc = "Bit 16 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis16(&mut self) -> EP_DMA_DIS16_W {
        EP_DMA_DIS16_W { w: self }
    }
    #[doc = "Bit 17 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis17(&mut self) -> EP_DMA_DIS17_W {
        EP_DMA_DIS17_W { w: self }
    }
    #[doc = "Bit 18 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis18(&mut self) -> EP_DMA_DIS18_W {
        EP_DMA_DIS18_W { w: self }
    }
    #[doc = "Bit 19 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis19(&mut self) -> EP_DMA_DIS19_W {
        EP_DMA_DIS19_W { w: self }
    }
    #[doc = "Bit 20 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis20(&mut self) -> EP_DMA_DIS20_W {
        EP_DMA_DIS20_W { w: self }
    }
    #[doc = "Bit 21 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis21(&mut self) -> EP_DMA_DIS21_W {
        EP_DMA_DIS21_W { w: self }
    }
    #[doc = "Bit 22 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis22(&mut self) -> EP_DMA_DIS22_W {
        EP_DMA_DIS22_W { w: self }
    }
    #[doc = "Bit 23 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis23(&mut self) -> EP_DMA_DIS23_W {
        EP_DMA_DIS23_W { w: self }
    }
    #[doc = "Bit 24 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis24(&mut self) -> EP_DMA_DIS24_W {
        EP_DMA_DIS24_W { w: self }
    }
    #[doc = "Bit 25 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis25(&mut self) -> EP_DMA_DIS25_W {
        EP_DMA_DIS25_W { w: self }
    }
    #[doc = "Bit 26 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis26(&mut self) -> EP_DMA_DIS26_W {
        EP_DMA_DIS26_W { w: self }
    }
    #[doc = "Bit 27 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis27(&mut self) -> EP_DMA_DIS27_W {
        EP_DMA_DIS27_W { w: self }
    }
    #[doc = "Bit 28 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis28(&mut self) -> EP_DMA_DIS28_W {
        EP_DMA_DIS28_W { w: self }
    }
    #[doc = "Bit 29 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis29(&mut self) -> EP_DMA_DIS29_W {
        EP_DMA_DIS29_W { w: self }
    }
    #[doc = "Bit 30 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis30(&mut self) -> EP_DMA_DIS30_W {
        EP_DMA_DIS30_W { w: self }
    }
    #[doc = "Bit 31 - Endpoint xx (2 <= xx <= 31) DMA disable control bit. 0 = No effect. 1 = Disable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_dis31(&mut self) -> EP_DMA_DIS31_W {
        EP_DMA_DIS31_W { w: self }
    }
}
