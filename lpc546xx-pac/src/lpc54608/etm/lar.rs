///Register `LAR` reader
pub struct R(crate::R<LAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LAR` writer
pub struct W(crate::W<LAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAR_SPEC>;
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
impl From<crate::W<LAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WriteAccessCode` reader - Write Access Code. A write of 0xC5ACCE55 enables further write access to this device. An invalid write will have the affect of removing write access.
pub struct WRITEACCESSCODE_R(crate::FieldReader<u32, u32>);
impl WRITEACCESSCODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        WRITEACCESSCODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRITEACCESSCODE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WriteAccessCode` writer - Write Access Code. A write of 0xC5ACCE55 enables further write access to this device. An invalid write will have the affect of removing write access.
pub struct WRITEACCESSCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITEACCESSCODE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    ///Bits 0:31 - Write Access Code. A write of 0xC5ACCE55 enables further write access to this device. An invalid write will have the affect of removing write access.
    #[inline(always)]
    pub fn write_access_code(&self) -> WRITEACCESSCODE_R {
        WRITEACCESSCODE_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Write Access Code. A write of 0xC5ACCE55 enables further write access to this device. An invalid write will have the affect of removing write access.
    #[inline(always)]
    pub fn write_access_code(&mut self) -> WRITEACCESSCODE_W {
        WRITEACCESSCODE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Lock Access Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lar](index.html) module
pub struct LAR_SPEC;
impl crate::RegisterSpec for LAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [lar::R](R) reader structure
impl crate::Readable for LAR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [lar::W](W) writer structure
impl crate::Writable for LAR_SPEC {
    type Writer = W;
}
///`reset()` method sets LAR to value 0
impl crate::Resettable for LAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
