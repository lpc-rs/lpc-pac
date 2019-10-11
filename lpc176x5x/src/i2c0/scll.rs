#[doc = "Reader of register SCLL"]
pub type R = crate::R<u32, super::SCLL>;
#[doc = "Writer for register SCLL"]
pub type W = crate::W<u32, super::SCLL>;
#[doc = "Register SCLL `reset()`'s with value 0x04"]
impl crate::ResetValue for super::SCLL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reader of field `SCLL`"]
pub type SCLL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SCLL`"]
pub struct SCLL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Count for SCL low time period selection."]
    #[inline(always)]
    pub fn scll(&self) -> SCLL_R {
        SCLL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Count for SCL low time period selection."]
    #[inline(always)]
    pub fn scll(&mut self) -> SCLL_W {
        SCLL_W { w: self }
    }
}
