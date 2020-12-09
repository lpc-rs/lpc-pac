#[doc = "Reader of register DSCADDR"]
pub type R = crate::R<u32, super::DSCADDR>;
#[doc = "Writer for register DSCADDR"]
pub type W = crate::W<u32, super::DSCADDR>;
#[doc = "Register DSCADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSCADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HDA`"]
pub type HDA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HDA`"]
pub struct HDA_W<'a> {
    w: &'a mut W,
}
impl<'a> HDA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Host Descriptor Address Pointer."]
    #[inline(always)]
    pub fn hda(&self) -> HDA_R {
        HDA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Host Descriptor Address Pointer."]
    #[inline(always)]
    pub fn hda(&mut self) -> HDA_W {
        HDA_W { w: self }
    }
}
