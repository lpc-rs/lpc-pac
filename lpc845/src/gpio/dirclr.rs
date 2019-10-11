#[doc = "Writer for register DIRCLR[%s]"]
pub type W = crate::W<u32, super::DIRCLR>;
#[doc = "Register DIRCLR[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::DIRCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DIRCLRP`"]
pub struct DIRCLRP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCLRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | ((value as u32) & 0x1fff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:28 - Clear direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear direction bit."]
    #[inline(always)]
    pub fn dirclrp(&mut self) -> DIRCLRP_W {
        DIRCLRP_W { w: self }
    }
}
