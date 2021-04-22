#[doc = "Register `FIFOTH` reader"]
pub struct R(crate::R<FIFOTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FIFOTH_SPEC>> for R {
    fn from(reader: crate::R<FIFOTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOTH` writer"]
pub struct W(crate::W<FIFOTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOTH_SPEC>;
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
impl core::convert::From<crate::W<FIFOTH_SPEC>> for W {
    fn from(writer: crate::W<FIFOTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_WMARK` reader - FIFO threshold watermark level when transmitting data to card."]
pub struct TX_WMARK_R(crate::FieldReader<u16, u16>);
impl TX_WMARK_R {
    pub(crate) fn new(bits: u16) -> Self {
        TX_WMARK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_WMARK_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_WMARK` writer - FIFO threshold watermark level when transmitting data to card."]
pub struct TX_WMARK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_WMARK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `RX_WMARK` reader - FIFO threshold watermark level when receiving data to card."]
pub struct RX_WMARK_R(crate::FieldReader<u16, u16>);
impl RX_WMARK_R {
    pub(crate) fn new(bits: u16) -> Self {
        RX_WMARK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_WMARK_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_WMARK` writer - FIFO threshold watermark level when receiving data to card."]
pub struct RX_WMARK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_WMARK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Field `DMA_MTS` reader - Burst size of multiple transaction; should be programmed same as DW-DMA controller multiple-transaction-size SRC/DEST_MSIZE."]
pub struct DMA_MTS_R(crate::FieldReader<u8, u8>);
impl DMA_MTS_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA_MTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_MTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_MTS` writer - Burst size of multiple transaction; should be programmed same as DW-DMA controller multiple-transaction-size SRC/DEST_MSIZE."]
pub struct DMA_MTS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_MTS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - FIFO threshold watermark level when transmitting data to card."]
    #[inline(always)]
    pub fn tx_wmark(&self) -> TX_WMARK_R {
        TX_WMARK_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - FIFO threshold watermark level when receiving data to card."]
    #[inline(always)]
    pub fn rx_wmark(&self) -> RX_WMARK_R {
        RX_WMARK_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:30 - Burst size of multiple transaction; should be programmed same as DW-DMA controller multiple-transaction-size SRC/DEST_MSIZE."]
    #[inline(always)]
    pub fn dma_mts(&self) -> DMA_MTS_R {
        DMA_MTS_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - FIFO threshold watermark level when transmitting data to card."]
    #[inline(always)]
    pub fn tx_wmark(&mut self) -> TX_WMARK_W {
        TX_WMARK_W { w: self }
    }
    #[doc = "Bits 16:27 - FIFO threshold watermark level when receiving data to card."]
    #[inline(always)]
    pub fn rx_wmark(&mut self) -> RX_WMARK_W {
        RX_WMARK_W { w: self }
    }
    #[doc = "Bits 28:30 - Burst size of multiple transaction; should be programmed same as DW-DMA controller multiple-transaction-size SRC/DEST_MSIZE."]
    #[inline(always)]
    pub fn dma_mts(&mut self) -> DMA_MTS_W {
        DMA_MTS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Threshold Watermark register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoth](index.html) module"]
pub struct FIFOTH_SPEC;
impl crate::RegisterSpec for FIFOTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifoth::R](R) reader structure"]
impl crate::Readable for FIFOTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifoth::W](W) writer structure"]
impl crate::Writable for FIFOTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOTH to value 0x001f_0000"]
impl crate::Resettable for FIFOTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x001f_0000
    }
}
