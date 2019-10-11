#[doc = "Reader of register LLI%s"]
pub type R = crate::R<u32, super::LLI>;
#[doc = "Writer for register LLI%s"]
pub type W = crate::W<u32, super::LLI>;
#[doc = "Register LLI%s `reset()`'s with value 0"]
impl crate::ResetValue for super::LLI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LLI`"]
pub type LLI_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LLI`"]
pub struct LLI_W<'a> {
    w: &'a mut W,
}
impl<'a> LLI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Linked list item. Bits \\[31:2\\] of the address for the next LLI. Address bits \\[1:0\\] are 0."]
    #[inline(always)]
    pub fn lli(&self) -> LLI_R {
        LLI_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Linked list item. Bits \\[31:2\\] of the address for the next LLI. Address bits \\[1:0\\] are 0."]
    #[inline(always)]
    pub fn lli(&mut self) -> LLI_W {
        LLI_W { w: self }
    }
}
