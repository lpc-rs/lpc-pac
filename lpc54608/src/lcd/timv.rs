#[doc = "Reader of register TIMV"]
pub type R = crate::R<u32, super::TIMV>;
#[doc = "Writer for register TIMV"]
pub type W = crate::W<u32, super::TIMV>;
#[doc = "Register TIMV `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LPP`"]
pub type LPP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LPP`"]
pub struct LPP_W<'a> {
    w: &'a mut W,
}
impl<'a> LPP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `VSW`"]
pub type VSW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VSW`"]
pub struct VSW_W<'a> {
    w: &'a mut W,
}
impl<'a> VSW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        self.w
    }
}
#[doc = "Reader of field `VFP`"]
pub type VFP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VFP`"]
pub struct VFP_W<'a> {
    w: &'a mut W,
}
impl<'a> VFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `VBP`"]
pub type VBP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VBP`"]
pub struct VBP_W<'a> {
    w: &'a mut W,
}
impl<'a> VBP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Lines per panel."]
    #[inline(always)]
    pub fn lpp(&self) -> LPP_R {
        LPP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - Vertical synchronization pulse width."]
    #[inline(always)]
    pub fn vsw(&self) -> VSW_R {
        VSW_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:23 - Vertical front porch."]
    #[inline(always)]
    pub fn vfp(&self) -> VFP_R {
        VFP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Vertical back porch."]
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Lines per panel."]
    #[inline(always)]
    pub fn lpp(&mut self) -> LPP_W {
        LPP_W { w: self }
    }
    #[doc = "Bits 10:15 - Vertical synchronization pulse width."]
    #[inline(always)]
    pub fn vsw(&mut self) -> VSW_W {
        VSW_W { w: self }
    }
    #[doc = "Bits 16:23 - Vertical front porch."]
    #[inline(always)]
    pub fn vfp(&mut self) -> VFP_W {
        VFP_W { w: self }
    }
    #[doc = "Bits 24:31 - Vertical back porch."]
    #[inline(always)]
    pub fn vbp(&mut self) -> VBP_W {
        VBP_W { w: self }
    }
}
