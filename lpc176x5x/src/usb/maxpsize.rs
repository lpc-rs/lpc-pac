#[doc = "Reader of register MAXPSIZE"]
pub type R = crate::R<u32, super::MAXPSIZE>;
#[doc = "Writer for register MAXPSIZE"]
pub type W = crate::W<u32, super::MAXPSIZE>;
#[doc = "Register MAXPSIZE `reset()`'s with value 0x08"]
impl crate::ResetValue for super::MAXPSIZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "Reader of field `MPS`"]
pub type MPS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MPS`"]
pub struct MPS_W<'a> {
    w: &'a mut W,
}
impl<'a> MPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - The maximum packet size value."]
    #[inline(always)]
    pub fn mps(&self) -> MPS_R {
        MPS_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - The maximum packet size value."]
    #[inline(always)]
    pub fn mps(&mut self) -> MPS_W {
        MPS_W { w: self }
    }
}
