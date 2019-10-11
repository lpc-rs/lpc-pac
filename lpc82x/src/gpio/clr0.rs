#[doc = "Reader of register CLR0"]
pub type R = crate::R<u32, super::CLR0>;
#[doc = "Writer for register CLR0"]
pub type W = crate::W<u32, super::CLR0>;
#[doc = "Register CLR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CLR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CLRP`"]
pub struct CLRP_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | ((value as u32) & 0x1fff_ffff);
        self.w
    }
}
impl R {}
impl W {
    #[doc = "Bits 0:28 - Clear output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp(&mut self) -> CLRP_W {
        CLRP_W { w: self }
    }
}
