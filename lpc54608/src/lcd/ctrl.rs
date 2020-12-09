#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LCDEN`"]
pub type LCDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDEN`"]
pub struct LCDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LCDBPP`"]
pub type LCDBPP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LCDBPP`"]
pub struct LCDBPP_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDBPP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Reader of field `LCDBW`"]
pub type LCDBW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDBW`"]
pub struct LCDBW_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDBW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LCDTFT`"]
pub type LCDTFT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDTFT`"]
pub struct LCDTFT_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDTFT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LCDMONO8`"]
pub type LCDMONO8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDMONO8`"]
pub struct LCDMONO8_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDMONO8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LCDDUAL`"]
pub type LCDDUAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDDUAL`"]
pub struct LCDDUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDDUAL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `BGR`"]
pub type BGR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BGR`"]
pub struct BGR_W<'a> {
    w: &'a mut W,
}
impl<'a> BGR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `BEBO`"]
pub type BEBO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEBO`"]
pub struct BEBO_W<'a> {
    w: &'a mut W,
}
impl<'a> BEBO_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `BEPO`"]
pub type BEPO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEPO`"]
pub struct BEPO_W<'a> {
    w: &'a mut W,
}
impl<'a> BEPO_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LCDPWR`"]
pub type LCDPWR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDPWR`"]
pub struct LCDPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDPWR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LCDVCOMP`"]
pub type LCDVCOMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LCDVCOMP`"]
pub struct LCDVCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDVCOMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `WATERMARK`"]
pub type WATERMARK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WATERMARK`"]
pub struct WATERMARK_W<'a> {
    w: &'a mut W,
}
impl<'a> WATERMARK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = "Bit 0 - LCD enable control bit."]
    #[inline(always)]
    pub fn lcden(&self) -> LCDEN_R {
        LCDEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - LCD bits per pixel."]
    #[inline(always)]
    pub fn lcdbpp(&self) -> LCDBPP_R {
        LCDBPP_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 4 - STN LCD monochrome/color selection."]
    #[inline(always)]
    pub fn lcdbw(&self) -> LCDBW_R {
        LCDBW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LCD panel TFT type selection."]
    #[inline(always)]
    pub fn lcdtft(&self) -> LCDTFT_R {
        LCDTFT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Monochrome LCD interface width."]
    #[inline(always)]
    pub fn lcdmono8(&self) -> LCDMONO8_R {
        LCDMONO8_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Single or Dual LCD panel selection."]
    #[inline(always)]
    pub fn lcddual(&self) -> LCDDUAL_R {
        LCDDUAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Color format selection."]
    #[inline(always)]
    pub fn bgr(&self) -> BGR_R {
        BGR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Big-endian Byte Order."]
    #[inline(always)]
    pub fn bebo(&self) -> BEBO_R {
        BEBO_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Big-Endian Pixel Ordering."]
    #[inline(always)]
    pub fn bepo(&self) -> BEPO_R {
        BEPO_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LCD power enable."]
    #[inline(always)]
    pub fn lcdpwr(&self) -> LCDPWR_R {
        LCDPWR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - LCD Vertical Compare Interrupt."]
    #[inline(always)]
    pub fn lcdvcomp(&self) -> LCDVCOMP_R {
        LCDVCOMP_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 16 - LCD DMA FIFO watermark level."]
    #[inline(always)]
    pub fn watermark(&self) -> WATERMARK_R {
        WATERMARK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD enable control bit."]
    #[inline(always)]
    pub fn lcden(&mut self) -> LCDEN_W {
        LCDEN_W { w: self }
    }
    #[doc = "Bits 1:3 - LCD bits per pixel."]
    #[inline(always)]
    pub fn lcdbpp(&mut self) -> LCDBPP_W {
        LCDBPP_W { w: self }
    }
    #[doc = "Bit 4 - STN LCD monochrome/color selection."]
    #[inline(always)]
    pub fn lcdbw(&mut self) -> LCDBW_W {
        LCDBW_W { w: self }
    }
    #[doc = "Bit 5 - LCD panel TFT type selection."]
    #[inline(always)]
    pub fn lcdtft(&mut self) -> LCDTFT_W {
        LCDTFT_W { w: self }
    }
    #[doc = "Bit 6 - Monochrome LCD interface width."]
    #[inline(always)]
    pub fn lcdmono8(&mut self) -> LCDMONO8_W {
        LCDMONO8_W { w: self }
    }
    #[doc = "Bit 7 - Single or Dual LCD panel selection."]
    #[inline(always)]
    pub fn lcddual(&mut self) -> LCDDUAL_W {
        LCDDUAL_W { w: self }
    }
    #[doc = "Bit 8 - Color format selection."]
    #[inline(always)]
    pub fn bgr(&mut self) -> BGR_W {
        BGR_W { w: self }
    }
    #[doc = "Bit 9 - Big-endian Byte Order."]
    #[inline(always)]
    pub fn bebo(&mut self) -> BEBO_W {
        BEBO_W { w: self }
    }
    #[doc = "Bit 10 - Big-Endian Pixel Ordering."]
    #[inline(always)]
    pub fn bepo(&mut self) -> BEPO_W {
        BEPO_W { w: self }
    }
    #[doc = "Bit 11 - LCD power enable."]
    #[inline(always)]
    pub fn lcdpwr(&mut self) -> LCDPWR_W {
        LCDPWR_W { w: self }
    }
    #[doc = "Bits 12:13 - LCD Vertical Compare Interrupt."]
    #[inline(always)]
    pub fn lcdvcomp(&mut self) -> LCDVCOMP_W {
        LCDVCOMP_W { w: self }
    }
    #[doc = "Bit 16 - LCD DMA FIFO watermark level."]
    #[inline(always)]
    pub fn watermark(&mut self) -> WATERMARK_W {
        WATERMARK_W { w: self }
    }
}
