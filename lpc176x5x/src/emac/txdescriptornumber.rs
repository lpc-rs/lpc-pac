#[doc = "Reader of register TXDESCRIPTORNUMBER"]
pub type R = crate::R<u32, super::TXDESCRIPTORNUMBER>;
#[doc = "Writer for register TXDESCRIPTORNUMBER"]
pub type W = crate::W<u32, super::TXDESCRIPTORNUMBER>;
#[doc = "Register TXDESCRIPTORNUMBER `reset()`'s with value 0"]
impl crate::ResetValue for super::TXDESCRIPTORNUMBER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXDN`"]
pub type TXDN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TXDN`"]
pub struct TXDN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - TxDescriptorNumber. Number of descriptors in the descriptor array for which TxDescriptor is the base address. The register is minus one encoded."]
    #[inline(always)]
    pub fn txdn(&self) -> TXDN_R {
        TXDN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TxDescriptorNumber. Number of descriptors in the descriptor array for which TxDescriptor is the base address. The register is minus one encoded."]
    #[inline(always)]
    pub fn txdn(&mut self) -> TXDN_W {
        TXDN_W { w: self }
    }
}
