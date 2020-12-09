#[doc = "Writer for register PRESETCTRLSET[%s]"]
pub type W = crate::W<u32, super::PRESETCTRLSET>;
#[doc = "Register PRESETCTRLSET[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::PRESETCTRLSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RST_SET`"]
pub struct RST_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Writing ones to this register sets the corresponding bit or bits in the PRESETCTRLn register, if they are implemented. Bits that do not correspond to defined bits in PRESETCTRLn are reserved and only zeroes should be written to them."]
    #[inline(always)]
    pub fn rst_set(&mut self) -> RST_SET_W {
        RST_SET_W { w: self }
    }
}
