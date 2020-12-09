#[doc = "Reader of register STATICEXTENDEDWAIT"]
pub type R = crate::R<u32, super::STATICEXTENDEDWAIT>;
#[doc = "Writer for register STATICEXTENDEDWAIT"]
pub type W = crate::W<u32, super::STATICEXTENDEDWAIT>;
#[doc = "Register STATICEXTENDEDWAIT `reset()`'s with value 0"]
impl crate::ResetValue for super::STATICEXTENDEDWAIT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTENDEDWAIT`"]
pub type EXTENDEDWAIT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EXTENDEDWAIT`"]
pub struct EXTENDEDWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTENDEDWAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Extended wait time out."]
    #[inline(always)]
    pub fn extendedwait(&self) -> EXTENDEDWAIT_R {
        EXTENDEDWAIT_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Extended wait time out."]
    #[inline(always)]
    pub fn extendedwait(&mut self) -> EXTENDEDWAIT_W {
        EXTENDEDWAIT_W { w: self }
    }
}
