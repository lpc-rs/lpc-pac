#[doc = "Reader of register DYNAMICREADCONFIG"]
pub type R = crate::R<u32, super::DYNAMICREADCONFIG>;
#[doc = "Writer for register DYNAMICREADCONFIG"]
pub type W = crate::W<u32, super::DYNAMICREADCONFIG>;
#[doc = "Register DYNAMICREADCONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::DYNAMICREADCONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RD`"]
pub type RD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RD`"]
pub struct RD_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Read data strategy."]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Read data strategy."]
    #[inline(always)]
    pub fn rd(&mut self) -> RD_W {
        RD_W { w: self }
    }
}
