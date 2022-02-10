///Register `DMA_CHx_TX_CTRL` reader
pub struct R(crate::R<DMA_CHX_TX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CHX_TX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CHX_TX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CHX_TX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMA_CHx_TX_CTRL` writer
pub struct W(crate::W<DMA_CHX_TX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CHX_TX_CTRL_SPEC>;
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
impl From<crate::W<DMA_CHX_TX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CHX_TX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ST` reader - Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state.
pub struct ST_R(crate::FieldReader<bool, bool>);
impl ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ST` writer - Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state.
pub struct ST_W<'a> {
    w: &'a mut W,
}
impl<'a> ST_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
///Field `TCW` reader - Transmit Channel Weight This field indicates the weight assigned to the corresponding Transmit channel.
pub struct TCW_R(crate::FieldReader<u8, u8>);
impl TCW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TCW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TCW` writer - Transmit Channel Weight This field indicates the weight assigned to the corresponding Transmit channel.
pub struct TCW_W<'a> {
    w: &'a mut W,
}
impl<'a> TCW_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
///Field `OSF` reader - Operate on Second Frame When this bit is set, it instructs the DMA to process the second packet of the Transmit data even before the status for the first packet is obtained.
pub struct OSF_R(crate::FieldReader<bool, bool>);
impl OSF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OSF` writer - Operate on Second Frame When this bit is set, it instructs the DMA to process the second packet of the Transmit data even before the status for the first packet is obtained.
pub struct OSF_W<'a> {
    w: &'a mut W,
}
impl<'a> OSF_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
///Field `TxPBL` reader - Transmit Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer.
pub struct TXPBL_R(crate::FieldReader<u8, u8>);
impl TXPBL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXPBL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXPBL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TxPBL` writer - Transmit Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer.
pub struct TXPBL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPBL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
impl R {
    ///Bit 0 - Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state.
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 0x01) != 0)
    }
    ///Bits 1:3 - Transmit Channel Weight This field indicates the weight assigned to the corresponding Transmit channel.
    #[inline(always)]
    pub fn tcw(&self) -> TCW_R {
        TCW_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    ///Bit 4 - Operate on Second Frame When this bit is set, it instructs the DMA to process the second packet of the Transmit data even before the status for the first packet is obtained.
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bits 16:21 - Transmit Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer.
    #[inline(always)]
    pub fn tx_pbl(&self) -> TXPBL_R {
        TXPBL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    ///Bit 0 - Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state.
    #[inline(always)]
    pub fn st(&mut self) -> ST_W {
        ST_W { w: self }
    }
    ///Bits 1:3 - Transmit Channel Weight This field indicates the weight assigned to the corresponding Transmit channel.
    #[inline(always)]
    pub fn tcw(&mut self) -> TCW_W {
        TCW_W { w: self }
    }
    ///Bit 4 - Operate on Second Frame When this bit is set, it instructs the DMA to process the second packet of the Transmit data even before the status for the first packet is obtained.
    #[inline(always)]
    pub fn osf(&mut self) -> OSF_W {
        OSF_W { w: self }
    }
    ///Bits 16:21 - Transmit Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer.
    #[inline(always)]
    pub fn tx_pbl(&mut self) -> TXPBL_W {
        TXPBL_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DMA Channelx Transmit Control
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dma_chx_tx_ctrl](index.html) module
pub struct DMA_CHX_TX_CTRL_SPEC;
impl crate::RegisterSpec for DMA_CHX_TX_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [dma_chx_tx_ctrl::R](R) reader structure
impl crate::Readable for DMA_CHX_TX_CTRL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dma_chx_tx_ctrl::W](W) writer structure
impl crate::Writable for DMA_CHX_TX_CTRL_SPEC {
    type Writer = W;
}
///`reset()` method sets DMA_CHx_TX_CTRL to value 0
impl crate::Resettable for DMA_CHX_TX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
