///Register `TXFQS` reader
pub struct R(crate::R<TXFQS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXFQS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXFQS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXFQS_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TXFQS` writer
pub struct W(crate::W<TXFQS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXFQS_SPEC>;
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
impl From<crate::W<TXFQS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXFQS_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TFGI` reader - Tx FIFO get index.
pub struct TFGI_R(crate::FieldReader<u8, u8>);
impl TFGI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TFGI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFGI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TFGI` writer - Tx FIFO get index.
pub struct TFGI_W<'a> {
    w: &'a mut W,
}
impl<'a> TFGI_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
///Field `TFQPI` reader - Tx FIFO/queue put index.
pub struct TFQPI_R(crate::FieldReader<u8, u8>);
impl TFQPI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TFQPI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFQPI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TFQPI` writer - Tx FIFO/queue put index.
pub struct TFQPI_W<'a> {
    w: &'a mut W,
}
impl<'a> TFQPI_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
///Field `TFQF` reader - Tx FIFO/queue full.
pub struct TFQF_R(crate::FieldReader<bool, bool>);
impl TFQF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TFQF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFQF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TFQF` writer - Tx FIFO/queue full.
pub struct TFQF_W<'a> {
    w: &'a mut W,
}
impl<'a> TFQF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
impl R {
    ///Bits 8:12 - Tx FIFO get index.
    #[inline(always)]
    pub fn tfgi(&self) -> TFGI_R {
        TFGI_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:20 - Tx FIFO/queue put index.
    #[inline(always)]
    pub fn tfqpi(&self) -> TFQPI_R {
        TFQPI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bit 21 - Tx FIFO/queue full.
    #[inline(always)]
    pub fn tfqf(&self) -> TFQF_R {
        TFQF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    ///Bits 8:12 - Tx FIFO get index.
    #[inline(always)]
    pub fn tfgi(&mut self) -> TFGI_W {
        TFGI_W { w: self }
    }
    ///Bits 16:20 - Tx FIFO/queue put index.
    #[inline(always)]
    pub fn tfqpi(&mut self) -> TFQPI_W {
        TFQPI_W { w: self }
    }
    ///Bit 21 - Tx FIFO/queue full.
    #[inline(always)]
    pub fn tfqf(&mut self) -> TFQF_W {
        TFQF_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Tx FIFO/Queue Status
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [txfqs](index.html) module
pub struct TXFQS_SPEC;
impl crate::RegisterSpec for TXFQS_SPEC {
    type Ux = u32;
}
///`read()` method returns [txfqs::R](R) reader structure
impl crate::Readable for TXFQS_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [txfqs::W](W) writer structure
impl crate::Writable for TXFQS_SPEC {
    type Writer = W;
}
///`reset()` method sets TXFQS to value 0
impl crate::Resettable for TXFQS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
