#[doc = "Reader of register CONEN"]
pub type R = crate::R<u32, super::CONEN>;
#[doc = "Writer for register CONEN"]
pub type W = crate::W<u32, super::CONEN>;
#[doc = "Register CONEN `reset()`'s with value 0"]
impl crate::ResetValue for super::CONEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NCEN`"]
pub type NCEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NCEN`"]
pub struct NCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NCEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - The SCT requests an interrupt when bit n of this register and the SCT conflict flag register are both one (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub fn ncen(&self) -> NCEN_R {
        NCEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The SCT requests an interrupt when bit n of this register and the SCT conflict flag register are both one (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub fn ncen(&mut self) -> NCEN_W {
        NCEN_W { w: self }
    }
}
