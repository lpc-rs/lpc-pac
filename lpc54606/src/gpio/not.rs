#[doc = "Writer for register NOT[%s]"]
pub type W = crate::W<u32, super::NOT>;
#[doc = "Register NOT[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::NOT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `NOTP`"]
pub struct NOTP_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Toggle output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn notp(&mut self) -> NOTP_W {
        NOTP_W { w: self }
    }
}
