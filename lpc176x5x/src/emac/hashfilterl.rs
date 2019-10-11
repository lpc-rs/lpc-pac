#[doc = "Reader of register HASHFILTERL"]
pub type R = crate::R<u32, super::HASHFILTERL>;
#[doc = "Writer for register HASHFILTERL"]
pub type W = crate::W<u32, super::HASHFILTERL>;
#[doc = "Register HASHFILTERL `reset()`'s with value 0"]
impl crate::ResetValue for super::HASHFILTERL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HFL`"]
pub type HFL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HFL`"]
pub struct HFL_W<'a> {
    w: &'a mut W,
}
impl<'a> HFL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - HashFilterL. Bits 31:0 of the imperfect filter hash table for receive filtering."]
    #[inline(always)]
    pub fn hfl(&self) -> HFL_R {
        HFL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - HashFilterL. Bits 31:0 of the imperfect filter hash table for receive filtering."]
    #[inline(always)]
    pub fn hfl(&mut self) -> HFL_W {
        HFL_W { w: self }
    }
}
