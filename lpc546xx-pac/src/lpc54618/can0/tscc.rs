///Register `TSCC` reader
pub struct R(crate::R<TSCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSCC_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TSCC` writer
pub struct W(crate::W<TSCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSCC_SPEC>;
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
impl From<crate::W<TSCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSCC_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TSS` reader - Timestamp select.
pub struct TSS_R(crate::FieldReader<u8, u8>);
impl TSS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TSS` writer - Timestamp select.
pub struct TSS_W<'a> {
    w: &'a mut W,
}
impl<'a> TSS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
///Field `TCP` reader - Timestamp counter prescaler Configures the timestamp and timeout counters time unit in multiple of CAN bit times.
pub struct TCP_R(crate::FieldReader<u8, u8>);
impl TCP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TCP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TCP` writer - Timestamp counter prescaler Configures the timestamp and timeout counters time unit in multiple of CAN bit times.
pub struct TCP_W<'a> {
    w: &'a mut W,
}
impl<'a> TCP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:1 - Timestamp select.
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 0x03) as u8)
    }
    ///Bits 16:19 - Timestamp counter prescaler Configures the timestamp and timeout counters time unit in multiple of CAN bit times.
    #[inline(always)]
    pub fn tcp(&self) -> TCP_R {
        TCP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:1 - Timestamp select.
    #[inline(always)]
    pub fn tss(&mut self) -> TSS_W {
        TSS_W { w: self }
    }
    ///Bits 16:19 - Timestamp counter prescaler Configures the timestamp and timeout counters time unit in multiple of CAN bit times.
    #[inline(always)]
    pub fn tcp(&mut self) -> TCP_W {
        TCP_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Timestamp Counter Configuration
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tscc](index.html) module
pub struct TSCC_SPEC;
impl crate::RegisterSpec for TSCC_SPEC {
    type Ux = u32;
}
///`read()` method returns [tscc::R](R) reader structure
impl crate::Readable for TSCC_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tscc::W](W) writer structure
impl crate::Writable for TSCC_SPEC {
    type Writer = W;
}
///`reset()` method sets TSCC to value 0
impl crate::Resettable for TSCC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
