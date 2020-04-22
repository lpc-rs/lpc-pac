#[doc = "Reader of register MTL_TXQx_LO_CRDT"]
pub type R = crate::R<u32, super::MTL_TXQX_LO_CRDT>;
#[doc = "Writer for register MTL_TXQx_LO_CRDT"]
pub type W = crate::W<u32, super::MTL_TXQX_LO_CRDT>;
#[doc = "Register MTL_TXQx_LO_CRDT `reset()`'s with value 0"]
impl crate::ResetValue for super::MTL_TXQX_LO_CRDT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LC`"]
pub type LC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LC`"]
pub struct LC_W<'a> {
    w: &'a mut W,
}
impl<'a> LC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | ((value as u32) & 0x1fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:28 - loCredit."]
    #[inline(always)]
    pub fn lc(&self) -> LC_R {
        LC_R::new((self.bits & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:28 - loCredit."]
    #[inline(always)]
    pub fn lc(&mut self) -> LC_W {
        LC_W { w: self }
    }
}
