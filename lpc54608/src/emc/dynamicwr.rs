#[doc = "Reader of register DYNAMICWR"]
pub type R = crate::R<u32, super::DYNAMICWR>;
#[doc = "Writer for register DYNAMICWR"]
pub type W = crate::W<u32, super::DYNAMICWR>;
#[doc = "Register DYNAMICWR `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::DYNAMICWR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Reader of field `TWR`"]
pub type TWR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TWR`"]
pub struct TWR_W<'a> {
    w: &'a mut W,
}
impl<'a> TWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Write recovery time."]
    #[inline(always)]
    pub fn twr(&self) -> TWR_R {
        TWR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Write recovery time."]
    #[inline(always)]
    pub fn twr(&mut self) -> TWR_W {
        TWR_W { w: self }
    }
}
