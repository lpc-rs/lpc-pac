#[doc = "Reader of register FMSSTART"]
pub type R = crate::R<u32, super::FMSSTART>;
#[doc = "Writer for register FMSSTART"]
pub type W = crate::W<u32, super::FMSSTART>;
#[doc = "Register FMSSTART `reset()`'s with value 0"]
impl crate::ResetValue for super::FMSSTART {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - Signature generation start address (corresponds to AHB byte address bits\\[20:4\\])."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - Signature generation start address (corresponds to AHB byte address bits\\[20:4\\])."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
}
