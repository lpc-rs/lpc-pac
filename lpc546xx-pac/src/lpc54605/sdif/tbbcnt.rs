///Register `TBBCNT` reader
pub struct R(crate::R<TBBCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBBCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBBCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBBCNT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TBBCNT` writer
pub struct W(crate::W<TBBCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBBCNT_SPEC>;
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
impl From<crate::W<TBBCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBBCNT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TRANS_FIFO_BYTE_COUNT` reader - Number of bytes transferred between Host/DMA memory and BIU FIFO.
pub struct TRANS_FIFO_BYTE_COUNT_R(crate::FieldReader<u32, u32>);
impl TRANS_FIFO_BYTE_COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TRANS_FIFO_BYTE_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANS_FIFO_BYTE_COUNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TRANS_FIFO_BYTE_COUNT` writer - Number of bytes transferred between Host/DMA memory and BIU FIFO.
pub struct TRANS_FIFO_BYTE_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_FIFO_BYTE_COUNT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    ///Bits 0:31 - Number of bytes transferred between Host/DMA memory and BIU FIFO.
    #[inline(always)]
    pub fn trans_fifo_byte_count(&self) -> TRANS_FIFO_BYTE_COUNT_R {
        TRANS_FIFO_BYTE_COUNT_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Number of bytes transferred between Host/DMA memory and BIU FIFO.
    #[inline(always)]
    pub fn trans_fifo_byte_count(&mut self) -> TRANS_FIFO_BYTE_COUNT_W {
        TRANS_FIFO_BYTE_COUNT_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Transferred Host to BIU-FIFO Byte Count register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tbbcnt](index.html) module
pub struct TBBCNT_SPEC;
impl crate::RegisterSpec for TBBCNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [tbbcnt::R](R) reader structure
impl crate::Readable for TBBCNT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tbbcnt::W](W) writer structure
impl crate::Writable for TBBCNT_SPEC {
    type Writer = W;
}
///`reset()` method sets TBBCNT to value 0
impl crate::Resettable for TBBCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
