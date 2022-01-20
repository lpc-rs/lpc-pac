#[doc = "Register `DMA_INTR_STAT` reader"]
pub struct R(crate::R<DMA_INTR_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_INTR_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_INTR_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_INTR_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_INTR_STAT` writer"]
pub struct W(crate::W<DMA_INTR_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_INTR_STAT_SPEC>;
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
impl From<crate::W<DMA_INTR_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_INTR_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DC0IS` reader - DMA Channel 0 Interrupt Status This bit indicates an interrupt event in DMA Channel 0."]
pub struct DC0IS_R(crate::FieldReader<bool, bool>);
impl DC0IS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DC0IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC0IS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DC0IS` writer - DMA Channel 0 Interrupt Status This bit indicates an interrupt event in DMA Channel 0."]
pub struct DC0IS_W<'a> {
    w: &'a mut W,
}
impl<'a> DC0IS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `DC1IS` reader - DMA Channel 1 Interrupt Status This bit indicates an interrupt event in DMA Channel 1."]
pub struct DC1IS_R(crate::FieldReader<bool, bool>);
impl DC1IS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DC1IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC1IS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DC1IS` writer - DMA Channel 1 Interrupt Status This bit indicates an interrupt event in DMA Channel 1."]
pub struct DC1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> DC1IS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `MTLIS` reader - MTL Interrupt Status This bit indicates an interrupt event in the MTL."]
pub struct MTLIS_R(crate::FieldReader<bool, bool>);
impl MTLIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MTLIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTLIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MACIS` reader - MAC Interrupt Status This bit indicates an interrupt event in the MAC."]
pub struct MACIS_R(crate::FieldReader<bool, bool>);
impl MACIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MACIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MACIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - DMA Channel 0 Interrupt Status This bit indicates an interrupt event in DMA Channel 0."]
    #[inline(always)]
    pub fn dc0is(&self) -> DC0IS_R {
        DC0IS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA Channel 1 Interrupt Status This bit indicates an interrupt event in DMA Channel 1."]
    #[inline(always)]
    pub fn dc1is(&self) -> DC1IS_R {
        DC1IS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - MTL Interrupt Status This bit indicates an interrupt event in the MTL."]
    #[inline(always)]
    pub fn mtlis(&self) -> MTLIS_R {
        MTLIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MAC Interrupt Status This bit indicates an interrupt event in the MAC."]
    #[inline(always)]
    pub fn macis(&self) -> MACIS_R {
        MACIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Channel 0 Interrupt Status This bit indicates an interrupt event in DMA Channel 0."]
    #[inline(always)]
    pub fn dc0is(&mut self) -> DC0IS_W {
        DC0IS_W { w: self }
    }
    #[doc = "Bit 1 - DMA Channel 1 Interrupt Status This bit indicates an interrupt event in DMA Channel 1."]
    #[inline(always)]
    pub fn dc1is(&mut self) -> DC1IS_W {
        DC1IS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_intr_stat](index.html) module"]
pub struct DMA_INTR_STAT_SPEC;
impl crate::RegisterSpec for DMA_INTR_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_intr_stat::R](R) reader structure"]
impl crate::Readable for DMA_INTR_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_intr_stat::W](W) writer structure"]
impl crate::Writable for DMA_INTR_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_INTR_STAT to value 0"]
impl crate::Resettable for DMA_INTR_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
