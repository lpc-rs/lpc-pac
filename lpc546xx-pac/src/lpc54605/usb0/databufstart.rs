///Register `DATABUFSTART` reader
pub struct R(crate::R<DATABUFSTART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATABUFSTART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATABUFSTART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATABUFSTART_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DATABUFSTART` writer
pub struct W(crate::W<DATABUFSTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATABUFSTART_SPEC>;
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
impl From<crate::W<DATABUFSTART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATABUFSTART_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DA_BUF` reader - Start address of the buffer pointer page where all endpoint data buffers are located.
pub struct DA_BUF_R(crate::FieldReader<u16, u16>);
impl DA_BUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DA_BUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DA_BUF_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DA_BUF` writer - Start address of the buffer pointer page where all endpoint data buffers are located.
pub struct DA_BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> DA_BUF_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 22)) | ((value as u32 & 0x03ff) << 22);
        self.w
    }
}
impl R {
    ///Bits 22:31 - Start address of the buffer pointer page where all endpoint data buffers are located.
    #[inline(always)]
    pub fn da_buf(&self) -> DA_BUF_R {
        DA_BUF_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 22:31 - Start address of the buffer pointer page where all endpoint data buffers are located.
    #[inline(always)]
    pub fn da_buf(&mut self) -> DA_BUF_W {
        DA_BUF_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///USB Data buffer start address
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [databufstart](index.html) module
pub struct DATABUFSTART_SPEC;
impl crate::RegisterSpec for DATABUFSTART_SPEC {
    type Ux = u32;
}
///`read()` method returns [databufstart::R](R) reader structure
impl crate::Readable for DATABUFSTART_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [databufstart::W](W) writer structure
impl crate::Writable for DATABUFSTART_SPEC {
    type Writer = W;
}
///`reset()` method sets DATABUFSTART to value 0
impl crate::Resettable for DATABUFSTART_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
