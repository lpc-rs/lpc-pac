#[doc = "Register `DMA_CHx_CTRL` reader"]
pub struct R(crate::R<DMA_CHX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CHX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DMA_CHX_CTRL_SPEC>> for R {
    fn from(reader: crate::R<DMA_CHX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_CHx_CTRL` writer"]
pub struct W(crate::W<DMA_CHX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CHX_CTRL_SPEC>;
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
impl core::convert::From<crate::W<DMA_CHX_CTRL_SPEC>> for W {
    fn from(writer: crate::W<DMA_CHX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBLx8` reader - 8xPBL mode When this bit is set, the PBL value programmed in Bits\\[21:16\\]
in DMA Channel Transmit Control Table 780 is multiplied eight times."]
pub struct PBLX8_R(crate::FieldReader<bool, bool>);
impl PBLX8_R {
    pub(crate) fn new(bits: bool) -> Self {
        PBLX8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBLX8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBLx8` writer - 8xPBL mode When this bit is set, the PBL value programmed in Bits\\[21:16\\]
in DMA Channel Transmit Control Table 780 is multiplied eight times."]
pub struct PBLX8_W<'a> {
    w: &'a mut W,
}
impl<'a> PBLX8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `DSL` reader - Skip Length This bit specifies the Word, Dword, or Lword number (depending on the 32- bit, 64-bit, or 128-bit bus) to skip between two unchained s."]
pub struct DSL_R(crate::FieldReader<u8, u8>);
impl DSL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DSL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSL` writer - Skip Length This bit specifies the Word, Dword, or Lword number (depending on the 32- bit, 64-bit, or 128-bit bus) to skip between two unchained s."]
pub struct DSL_W<'a> {
    w: &'a mut W,
}
impl<'a> DSL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - 8xPBL mode When this bit is set, the PBL value programmed in Bits\\[21:16\\]
in DMA Channel Transmit Control Table 780 is multiplied eight times."]
    #[inline(always)]
    pub fn pblx8(&self) -> PBLX8_R {
        PBLX8_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 18:20 - Skip Length This bit specifies the Word, Dword, or Lword number (depending on the 32- bit, 64-bit, or 128-bit bus) to skip between two unchained s."]
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 18) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - 8xPBL mode When this bit is set, the PBL value programmed in Bits\\[21:16\\]
in DMA Channel Transmit Control Table 780 is multiplied eight times."]
    #[inline(always)]
    pub fn pblx8(&mut self) -> PBLX8_W {
        PBLX8_W { w: self }
    }
    #[doc = "Bits 18:20 - Skip Length This bit specifies the Word, Dword, or Lword number (depending on the 32- bit, 64-bit, or 128-bit bus) to skip between two unchained s."]
    #[inline(always)]
    pub fn dsl(&mut self) -> DSL_W {
        DSL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channelx Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_ctrl](index.html) module"]
pub struct DMA_CHX_CTRL_SPEC;
impl crate::RegisterSpec for DMA_CHX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_chx_ctrl::R](R) reader structure"]
impl crate::Readable for DMA_CHX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_chx_ctrl::W](W) writer structure"]
impl crate::Writable for DMA_CHX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_CHx_CTRL to value 0"]
impl crate::Resettable for DMA_CHX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
