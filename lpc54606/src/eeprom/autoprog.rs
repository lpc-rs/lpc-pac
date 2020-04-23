#[doc = "Reader of register AUTOPROG"]
pub type R = crate::R<u32, super::AUTOPROG>;
#[doc = "Writer for register AUTOPROG"]
pub type W = crate::W<u32, super::AUTOPROG>;
#[doc = "Register AUTOPROG `reset()`'s with value 0"]
impl crate::ResetValue for super::AUTOPROG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AUTOPROG`"]
pub type AUTOPROG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AUTOPROG`"]
pub struct AUTOPROG_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOPROG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Auto programming mode: 00 = auto programming off 01 = erase/program cycle is triggered after 1 word is written 10 = erase/program cycle is triggered after a write to AHB address ending with ."]
    #[inline(always)]
    pub fn autoprog(&self) -> AUTOPROG_R {
        AUTOPROG_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Auto programming mode: 00 = auto programming off 01 = erase/program cycle is triggered after 1 word is written 10 = erase/program cycle is triggered after a write to AHB address ending with ."]
    #[inline(always)]
    pub fn autoprog(&mut self) -> AUTOPROG_W {
        AUTOPROG_W { w: self }
    }
}
