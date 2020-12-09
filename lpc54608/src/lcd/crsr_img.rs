#[doc = "Reader of register CRSR_IMG[%s]"]
pub type R = crate::R<u32, super::CRSR_IMG>;
#[doc = "Writer for register CRSR_IMG[%s]"]
pub type W = crate::W<u32, super::CRSR_IMG>;
#[doc = "Register CRSR_IMG[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::CRSR_IMG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRSR_IMG`"]
pub type CRSR_IMG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CRSR_IMG`"]
pub struct CRSR_IMG_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSR_IMG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Cursor Image data."]
    #[inline(always)]
    pub fn crsr_img(&self) -> CRSR_IMG_R {
        CRSR_IMG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cursor Image data."]
    #[inline(always)]
    pub fn crsr_img(&mut self) -> CRSR_IMG_W {
        CRSR_IMG_W { w: self }
    }
}
