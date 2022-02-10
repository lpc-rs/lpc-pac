///Register `DMA_CHx_RX_INT_WDTIMER` reader
pub struct R(crate::R<DMA_CHX_RX_INT_WDTIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CHX_RX_INT_WDTIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CHX_RX_INT_WDTIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CHX_RX_INT_WDTIMER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMA_CHx_RX_INT_WDTIMER` writer
pub struct W(crate::W<DMA_CHX_RX_INT_WDTIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CHX_RX_INT_WDTIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMA_CHX_RX_INT_WDTIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CHX_RX_INT_WDTIMER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RIWT` reader - Receive Interrupt Watchdog Timer Count Indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set.
pub struct RIWT_R(crate::FieldReader<u8, u8>);
impl RIWT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RIWT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RIWT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RIWT` writer - Receive Interrupt Watchdog Timer Count Indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set.
pub struct RIWT_W<'a> {
    w: &'a mut W,
}
impl<'a> RIWT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    ///Bits 0:7 - Receive Interrupt Watchdog Timer Count Indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set.
    #[inline(always)]
    pub fn riwt(&self) -> RIWT_R {
        RIWT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Receive Interrupt Watchdog Timer Count Indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set.
    #[inline(always)]
    pub fn riwt(&mut self) -> RIWT_W {
        RIWT_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Receive Interrupt Watchdog Timer
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dma_chx_rx_int_wdtimer](index.html) module
pub struct DMA_CHX_RX_INT_WDTIMER_SPEC;
impl crate::RegisterSpec for DMA_CHX_RX_INT_WDTIMER_SPEC {
    type Ux = u32;
}
///`read()` method returns [dma_chx_rx_int_wdtimer::R](R) reader structure
impl crate::Readable for DMA_CHX_RX_INT_WDTIMER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dma_chx_rx_int_wdtimer::W](W) writer structure
impl crate::Writable for DMA_CHX_RX_INT_WDTIMER_SPEC {
    type Writer = W;
}
///`reset()` method sets DMA_CHx_RX_INT_WDTIMER to value 0
impl crate::Resettable for DMA_CHX_RX_INT_WDTIMER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
