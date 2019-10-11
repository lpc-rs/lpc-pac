#[doc = "Reader of register WINDOW"]
pub type R = crate::R<u32, super::WINDOW>;
#[doc = "Writer for register WINDOW"]
pub type W = crate::W<u32, super::WINDOW>;
#[doc = "Register WINDOW `reset()`'s with value 0x00ff_ffff"]
impl crate::ResetValue for super::WINDOW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00ff_ffff
    }
}
#[doc = "Reader of field `WINDOW`"]
pub type WINDOW_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WINDOW`"]
pub struct WINDOW_W<'a> {
    w: &'a mut W,
}
impl<'a> WINDOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Watchdog window value."]
    #[inline(always)]
    pub fn window(&self) -> WINDOW_R {
        WINDOW_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Watchdog window value."]
    #[inline(always)]
    pub fn window(&mut self) -> WINDOW_W {
        WINDOW_W { w: self }
    }
}
