#[doc = "Reader of register DESTADDR%s"]
pub type R = crate::R<u32, super::DESTADDR>;
#[doc = "Writer for register DESTADDR%s"]
pub type W = crate::W<u32, super::DESTADDR>;
#[doc = "Register DESTADDR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::DESTADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DESTADDR`"]
pub type DESTADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DESTADDR`"]
pub struct DESTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DESTADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - DMA Destination address. Reading this register will return the current destination address."]
    #[inline(always)]
    pub fn destaddr(&self) -> DESTADDR_R {
        DESTADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Destination address. Reading this register will return the current destination address."]
    #[inline(always)]
    pub fn destaddr(&mut self) -> DESTADDR_W {
        DESTADDR_W { w: self }
    }
}
