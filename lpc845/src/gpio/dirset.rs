#[doc = "Writer for register DIRSET[%s]"]
pub type W = crate::W<u32, super::DIRSET>;
#[doc = "Register DIRSET[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::DIRSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DIRSETP`"]
pub struct DIRSETP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSETP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | ((value as u32) & 0x1fff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:28 - Set direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Set direction bit."]
    #[inline(always)]
    pub fn dirsetp(&mut self) -> DIRSETP_W {
        DIRSETP_W { w: self }
    }
}
