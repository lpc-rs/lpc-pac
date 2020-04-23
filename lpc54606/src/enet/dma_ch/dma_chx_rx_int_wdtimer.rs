#[doc = "Reader of register DMA_CHx_RX_INT_WDTIMER"]
pub type R = crate::R<u32, super::DMA_CHX_RX_INT_WDTIMER>;
#[doc = "Writer for register DMA_CHx_RX_INT_WDTIMER"]
pub type W = crate::W<u32, super::DMA_CHX_RX_INT_WDTIMER>;
#[doc = "Register DMA_CHx_RX_INT_WDTIMER `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_CHX_RX_INT_WDTIMER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RIWT`"]
pub type RIWT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RIWT`"]
pub struct RIWT_W<'a> {
    w: &'a mut W,
}
impl<'a> RIWT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive Interrupt Watchdog Timer Count Indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set."]
    #[inline(always)]
    pub fn riwt(&self) -> RIWT_R {
        RIWT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Interrupt Watchdog Timer Count Indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set."]
    #[inline(always)]
    pub fn riwt(&mut self) -> RIWT_W {
        RIWT_W { w: self }
    }
}
