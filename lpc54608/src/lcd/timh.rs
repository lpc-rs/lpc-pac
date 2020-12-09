#[doc = "Reader of register TIMH"]
pub type R = crate::R<u32, super::TIMH>;
#[doc = "Writer for register TIMH"]
pub type W = crate::W<u32, super::TIMH>;
#[doc = "Register TIMH `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PPL`"]
pub type PPL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PPL`"]
pub struct PPL_W<'a> {
    w: &'a mut W,
}
impl<'a> PPL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u32) & 0x3f) << 2);
        self.w
    }
}
#[doc = "Reader of field `HSW`"]
pub type HSW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSW`"]
pub struct HSW_W<'a> {
    w: &'a mut W,
}
impl<'a> HSW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `HFP`"]
pub type HFP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HFP`"]
pub struct HFP_W<'a> {
    w: &'a mut W,
}
impl<'a> HFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HBP`"]
pub type HBP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HBP`"]
pub struct HBP_W<'a> {
    w: &'a mut W,
}
impl<'a> HBP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:7 - Pixels-per-line."]
    #[inline(always)]
    pub fn ppl(&self) -> PPL_R {
        PPL_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - Horizontal synchronization pulse width."]
    #[inline(always)]
    pub fn hsw(&self) -> HSW_R {
        HSW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Horizontal front porch."]
    #[inline(always)]
    pub fn hfp(&self) -> HFP_R {
        HFP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Horizontal back porch."]
    #[inline(always)]
    pub fn hbp(&self) -> HBP_R {
        HBP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 2:7 - Pixels-per-line."]
    #[inline(always)]
    pub fn ppl(&mut self) -> PPL_W {
        PPL_W { w: self }
    }
    #[doc = "Bits 8:15 - Horizontal synchronization pulse width."]
    #[inline(always)]
    pub fn hsw(&mut self) -> HSW_W {
        HSW_W { w: self }
    }
    #[doc = "Bits 16:23 - Horizontal front porch."]
    #[inline(always)]
    pub fn hfp(&mut self) -> HFP_W {
        HFP_W { w: self }
    }
    #[doc = "Bits 24:31 - Horizontal back porch."]
    #[inline(always)]
    pub fn hbp(&mut self) -> HBP_W {
        HBP_W { w: self }
    }
}
