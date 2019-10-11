#[doc = "Reader of register MASK%s"]
pub type R = crate::R<u32, super::MASK>;
#[doc = "Writer for register MASK%s"]
pub type W = crate::W<u32, super::MASK>;
#[doc = "Register MASK%s `reset()`'s with value 0"]
impl crate::ResetValue for super::MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MASK`"]
pub type MASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MASK`"]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:7 - Mask bits."]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - Mask bits."]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
}
