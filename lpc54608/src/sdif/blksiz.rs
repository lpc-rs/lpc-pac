#[doc = "Reader of register BLKSIZ"]
pub type R = crate::R<u32, super::BLKSIZ>;
#[doc = "Writer for register BLKSIZ"]
pub type W = crate::W<u32, super::BLKSIZ>;
#[doc = "Register BLKSIZ `reset()`'s with value 0x0200"]
impl crate::ResetValue for super::BLKSIZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200
    }
}
#[doc = "Reader of field `BLOCK_SIZE`"]
pub type BLOCK_SIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BLOCK_SIZE`"]
pub struct BLOCK_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Block size."]
    #[inline(always)]
    pub fn block_size(&self) -> BLOCK_SIZE_R {
        BLOCK_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Block size."]
    #[inline(always)]
    pub fn block_size(&mut self) -> BLOCK_SIZE_W {
        BLOCK_SIZE_W { w: self }
    }
}
