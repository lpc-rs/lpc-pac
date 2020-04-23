#[doc = "Reader of register DYNAMICRP"]
pub type R = crate::R<u32, super::DYNAMICRP>;
#[doc = "Writer for register DYNAMICRP"]
pub type W = crate::W<u32, super::DYNAMICRP>;
#[doc = "Register DYNAMICRP `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::DYNAMICRP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Reader of field `TRP`"]
pub type TRP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRP`"]
pub struct TRP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Precharge command period."]
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Precharge command period."]
    #[inline(always)]
    pub fn trp(&mut self) -> TRP_W {
        TRP_W { w: self }
    }
}
