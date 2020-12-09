#[doc = "Reader of register UPBASE"]
pub type R = crate::R<u32, super::UPBASE>;
#[doc = "Writer for register UPBASE"]
pub type W = crate::W<u32, super::UPBASE>;
#[doc = "Register UPBASE `reset()`'s with value 0"]
impl crate::ResetValue for super::UPBASE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LCDUPBASE`"]
pub type LCDUPBASE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LCDUPBASE`"]
pub struct LCDUPBASE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDUPBASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - LCD upper panel base address."]
    #[inline(always)]
    pub fn lcdupbase(&self) -> LCDUPBASE_R {
        LCDUPBASE_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 3:31 - LCD upper panel base address."]
    #[inline(always)]
    pub fn lcdupbase(&mut self) -> LCDUPBASE_W {
        LCDUPBASE_W { w: self }
    }
}
