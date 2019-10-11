#[doc = "Reader of register TC"]
pub type R = crate::R<u32, super::TC>;
#[doc = "Writer for register TC"]
pub type W = crate::W<u32, super::TC>;
#[doc = "Register TC `reset()`'s with value 0"]
impl crate::ResetValue for super::TC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TC`"]
pub type TC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TC`"]
pub struct TC_W<'a> {
    w: &'a mut W,
}
impl<'a> TC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timer counter value."]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer counter value."]
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W {
        TC_W { w: self }
    }
}
