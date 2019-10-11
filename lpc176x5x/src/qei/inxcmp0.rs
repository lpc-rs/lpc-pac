#[doc = "Reader of register INXCMP0"]
pub type R = crate::R<u32, super::INXCMP0>;
#[doc = "Writer for register INXCMP0"]
pub type W = crate::W<u32, super::INXCMP0>;
#[doc = "Register INXCMP0 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::INXCMP0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `ICMP0`"]
pub type ICMP0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ICMP0`"]
pub struct ICMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> ICMP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Index compare value 0."]
    #[inline(always)]
    pub fn icmp0(&self) -> ICMP0_R {
        ICMP0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Index compare value 0."]
    #[inline(always)]
    pub fn icmp0(&mut self) -> ICMP0_W {
        ICMP0_W { w: self }
    }
}
