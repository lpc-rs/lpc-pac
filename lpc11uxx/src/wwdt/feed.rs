#[doc = "Writer for register FEED"]
pub type W = crate::W<u32, super::FEED>;
#[doc = "Register FEED `reset()`'s with value 0"]
impl crate::ResetValue for super::FEED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `FEED`"]
pub struct FEED_W<'a> {
    w: &'a mut W,
}
impl<'a> FEED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Feed value should be 0xAA followed by 0x55."]
    #[inline(always)]
    pub fn feed(&mut self) -> FEED_W {
        FEED_W { w: self }
    }
}
