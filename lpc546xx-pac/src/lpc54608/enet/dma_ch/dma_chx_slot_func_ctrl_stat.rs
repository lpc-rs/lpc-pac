///Register `DMA_CHx_SLOT_FUNC_CTRL_STAT` reader
pub struct R(crate::R<DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMA_CHx_SLOT_FUNC_CTRL_STAT` writer
pub struct W(crate::W<DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC>;
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
impl From<crate::W<DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ESC` reader - Enable Slot Comparison When set, this bit enables the checking of the slot numbers programmed in the Tx descriptor with the current reference given in the RSN field.
pub struct ESC_R(crate::FieldReader<bool, bool>);
impl ESC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ESC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ESC` writer - Enable Slot Comparison When set, this bit enables the checking of the slot numbers programmed in the Tx descriptor with the current reference given in the RSN field.
pub struct ESC_W<'a> {
    w: &'a mut W,
}
impl<'a> ESC_W<'a> {
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
///Field `ASC` reader - Advance Slot Check When set, this bit enables the D MA to fetch the data from the buffer when the slot number (SLOTNUM) programmed in the Tx descriptor is equal to the reference slot number given in the RSN field or, ahead of the reference slot number by up to two slots This bit is applicable only when the ESC bit is set.
pub struct ASC_R(crate::FieldReader<bool, bool>);
impl ASC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ASC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ASC` writer - Advance Slot Check When set, this bit enables the D MA to fetch the data from the buffer when the slot number (SLOTNUM) programmed in the Tx descriptor is equal to the reference slot number given in the RSN field or, ahead of the reference slot number by up to two slots This bit is applicable only when the ESC bit is set.
pub struct ASC_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC_W<'a> {
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
///Field `RSN` reader - Reference Slot Number This field gives the current value of the reference slot number in the DMA.
pub struct RSN_R(crate::FieldReader<u8, u8>);
impl RSN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RSN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 0 - Enable Slot Comparison When set, this bit enables the checking of the slot numbers programmed in the Tx descriptor with the current reference given in the RSN field.
    #[inline(always)]
    pub fn esc(&self) -> ESC_R {
        ESC_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Advance Slot Check When set, this bit enables the D MA to fetch the data from the buffer when the slot number (SLOTNUM) programmed in the Tx descriptor is equal to the reference slot number given in the RSN field or, ahead of the reference slot number by up to two slots This bit is applicable only when the ESC bit is set.
    #[inline(always)]
    pub fn asc(&self) -> ASC_R {
        ASC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bits 16:19 - Reference Slot Number This field gives the current value of the reference slot number in the DMA.
    #[inline(always)]
    pub fn rsn(&self) -> RSN_R {
        RSN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - Enable Slot Comparison When set, this bit enables the checking of the slot numbers programmed in the Tx descriptor with the current reference given in the RSN field.
    #[inline(always)]
    pub fn esc(&mut self) -> ESC_W {
        ESC_W { w: self }
    }
    ///Bit 1 - Advance Slot Check When set, this bit enables the D MA to fetch the data from the buffer when the slot number (SLOTNUM) programmed in the Tx descriptor is equal to the reference slot number given in the RSN field or, ahead of the reference slot number by up to two slots This bit is applicable only when the ESC bit is set.
    #[inline(always)]
    pub fn asc(&mut self) -> ASC_W {
        ASC_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Slot Function Control and Status
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dma_chx_slot_func_ctrl_stat](index.html) module
pub struct DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC;
impl crate::RegisterSpec for DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC {
    type Ux = u32;
}
///`read()` method returns [dma_chx_slot_func_ctrl_stat::R](R) reader structure
impl crate::Readable for DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dma_chx_slot_func_ctrl_stat::W](W) writer structure
impl crate::Writable for DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC {
    type Writer = W;
}
///`reset()` method sets DMA_CHx_SLOT_FUNC_CTRL_STAT to value 0
impl crate::Resettable for DMA_CHX_SLOT_FUNC_CTRL_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
