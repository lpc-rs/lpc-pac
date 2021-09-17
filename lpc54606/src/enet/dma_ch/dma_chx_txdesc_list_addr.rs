#[doc = "Register `DMA_CHx_TXDESC_LIST_ADDR` reader"]
pub struct R(crate::R<DMA_CHX_TXDESC_LIST_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CHX_TXDESC_LIST_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CHX_TXDESC_LIST_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CHX_TXDESC_LIST_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_CHx_TXDESC_LIST_ADDR` writer"]
pub struct W(crate::W<DMA_CHX_TXDESC_LIST_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CHX_TXDESC_LIST_ADDR_SPEC>;
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
impl From<crate::W<DMA_CHX_TXDESC_LIST_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CHX_TXDESC_LIST_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STL` reader - Start of transmit list This field contains the base address of the first in the Transmit list."]
pub struct STL_R(crate::FieldReader<u32, u32>);
impl STL_R {
    pub(crate) fn new(bits: u32) -> Self {
        STL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STL` writer - Start of transmit list This field contains the base address of the first in the Transmit list."]
pub struct STL_W<'a> {
    w: &'a mut W,
}
impl<'a> STL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | ((value as u32 & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Start of transmit list This field contains the base address of the first in the Transmit list."]
    #[inline(always)]
    pub fn stl(&self) -> STL_R {
        STL_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Start of transmit list This field contains the base address of the first in the Transmit list."]
    #[inline(always)]
    pub fn stl(&mut self) -> STL_W {
        STL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chx_txdesc_list_addr](index.html) module"]
pub struct DMA_CHX_TXDESC_LIST_ADDR_SPEC;
impl crate::RegisterSpec for DMA_CHX_TXDESC_LIST_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_chx_txdesc_list_addr::R](R) reader structure"]
impl crate::Readable for DMA_CHX_TXDESC_LIST_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_chx_txdesc_list_addr::W](W) writer structure"]
impl crate::Writable for DMA_CHX_TXDESC_LIST_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_CHx_TXDESC_LIST_ADDR to value 0"]
impl crate::Resettable for DMA_CHX_TXDESC_LIST_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
