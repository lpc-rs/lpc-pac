#[doc = "Reader of register RXDESCRIPTORNUMBER"]
pub type R = crate::R<u32, super::RXDESCRIPTORNUMBER>;
#[doc = "Writer for register RXDESCRIPTORNUMBER"]
pub type W = crate::W<u32, super::RXDESCRIPTORNUMBER>;
#[doc = "Register RXDESCRIPTORNUMBER `reset()`'s with value 0"]
impl crate::ResetValue for super::RXDESCRIPTORNUMBER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXDESCRIPTORN`"]
pub type RXDESCRIPTORN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RXDESCRIPTORN`"]
pub struct RXDESCRIPTORN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDESCRIPTORN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - RxDescriptorNumber. Number of descriptors in the descriptor array for which RxDescriptor is the base address. The number of descriptors is minus one encoded."]
    #[inline(always)]
    pub fn rxdescriptorn(&self) -> RXDESCRIPTORN_R {
        RXDESCRIPTORN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RxDescriptorNumber. Number of descriptors in the descriptor array for which RxDescriptor is the base address. The number of descriptors is minus one encoded."]
    #[inline(always)]
    pub fn rxdescriptorn(&mut self) -> RXDESCRIPTORN_W {
        RXDESCRIPTORN_W { w: self }
    }
}
