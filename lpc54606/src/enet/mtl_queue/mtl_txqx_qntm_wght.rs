#[doc = "Reader of register MTL_TXQx_QNTM_WGHT"]
pub type R = crate::R<u32, super::MTL_TXQX_QNTM_WGHT>;
#[doc = "Writer for register MTL_TXQx_QNTM_WGHT"]
pub type W = crate::W<u32, super::MTL_TXQX_QNTM_WGHT>;
#[doc = "Register MTL_TXQx_QNTM_WGHT `reset()`'s with value 0"]
impl crate::ResetValue for super::MTL_TXQX_QNTM_WGHT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISCQW`"]
pub type ISCQW_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISCQW`"]
pub struct ISCQW_W<'a> {
    w: &'a mut W,
}
impl<'a> ISCQW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x001f_ffff) | ((value as u32) & 0x001f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:20 - Average Bits per Slot."]
    #[inline(always)]
    pub fn iscqw(&self) -> ISCQW_R {
        ISCQW_R::new((self.bits & 0x001f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:20 - Average Bits per Slot."]
    #[inline(always)]
    pub fn iscqw(&mut self) -> ISCQW_W {
        ISCQW_W { w: self }
    }
}
