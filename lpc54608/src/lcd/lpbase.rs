#[doc = "Reader of register LPBASE"]
pub type R = crate::R<u32, super::LPBASE>;
#[doc = "Writer for register LPBASE"]
pub type W = crate::W<u32, super::LPBASE>;
#[doc = "Register LPBASE `reset()`'s with value 0"]
impl crate::ResetValue for super::LPBASE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LCDLPBASE`"]
pub type LCDLPBASE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LCDLPBASE`"]
pub struct LCDLPBASE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDLPBASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - LCD lower panel base address."]
    #[inline(always)]
    pub fn lcdlpbase(&self) -> LCDLPBASE_R {
        LCDLPBASE_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 3:31 - LCD lower panel base address."]
    #[inline(always)]
    pub fn lcdlpbase(&mut self) -> LCDLPBASE_W {
        LCDLPBASE_W { w: self }
    }
}
