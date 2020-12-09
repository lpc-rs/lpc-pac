#[doc = "Reader of register CNTRLDVR1"]
pub type R = crate::R<u32, super::CNTRLDVR1>;
#[doc = "Writer for register CNTRLDVR1"]
pub type W = crate::W<u32, super::CNTRLDVR1>;
#[doc = "Register CNTRLDVR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CNTRLDVR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IntitialCount`"]
pub type INTITIALCOUNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IntitialCount`"]
pub struct INTITIALCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> INTITIALCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Initial count."]
    #[inline(always)]
    pub fn intitial_count(&self) -> INTITIALCOUNT_R {
        INTITIALCOUNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Initial count."]
    #[inline(always)]
    pub fn intitial_count(&mut self) -> INTITIALCOUNT_W {
        INTITIALCOUNT_W { w: self }
    }
}
