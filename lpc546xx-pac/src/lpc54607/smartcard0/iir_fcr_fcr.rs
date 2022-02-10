///Register `FCR` writer
pub struct W(crate::W<IIR_FCR_FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IIR_FCR_FCR_SPEC>;
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
impl From<crate::W<IIR_FCR_FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IIR_FCR_FCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FIFOEN` writer - FIFO Enable.
pub struct FIFOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOEN_W<'a> {
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
///Field `RXFIFORES` writer - RX FIFO Reset.
pub struct RXFIFORES_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFORES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
///Field `TXFIFORES` writer - TX FIFO Reset.
pub struct TXFIFORES_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFORES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
///Field `DMAMODE` writer - DMA Mode Select.
pub struct DMAMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
///Field `RXTRIGLVL` writer - RX Trigger Level.
pub struct RXTRIGLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTRIGLVL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
impl W {
    ///Bit 0 - FIFO Enable.
    #[inline(always)]
    pub fn fifoen(&mut self) -> FIFOEN_W {
        FIFOEN_W { w: self }
    }
    ///Bit 1 - RX FIFO Reset.
    #[inline(always)]
    pub fn rxfifores(&mut self) -> RXFIFORES_W {
        RXFIFORES_W { w: self }
    }
    ///Bit 2 - TX FIFO Reset.
    #[inline(always)]
    pub fn txfifores(&mut self) -> TXFIFORES_W {
        TXFIFORES_W { w: self }
    }
    ///Bit 3 - DMA Mode Select.
    #[inline(always)]
    pub fn dmamode(&mut self) -> DMAMODE_W {
        DMAMODE_W { w: self }
    }
    ///Bits 6:7 - RX Trigger Level.
    #[inline(always)]
    pub fn rxtriglvl(&mut self) -> RXTRIGLVL_W {
        RXTRIGLVL_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FIFO Control Register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [iir_fcr_fcr](index.html) module
pub struct IIR_FCR_FCR_SPEC;
impl crate::RegisterSpec for IIR_FCR_FCR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [iir_fcr_fcr::W](W) writer structure
impl crate::Writable for IIR_FCR_FCR_SPEC {
    type Writer = W;
}
///`reset()` method sets FCR to value 0
impl crate::Resettable for IIR_FCR_FCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
