///Register `UPBASE` reader
pub struct R(crate::R<UPBASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UPBASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UPBASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UPBASE_SPEC>) -> Self {
        R(reader)
    }
}
///Register `UPBASE` writer
pub struct W(crate::W<UPBASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UPBASE_SPEC>;
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
impl From<crate::W<UPBASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UPBASE_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LCDUPBASE` reader - LCD upper panel base address.
pub struct LCDUPBASE_R(crate::FieldReader<u32, u32>);
impl LCDUPBASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        LCDUPBASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDUPBASE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LCDUPBASE` writer - LCD upper panel base address.
pub struct LCDUPBASE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDUPBASE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | ((value as u32 & 0x1fff_ffff) << 3);
        self.w
    }
}
impl R {
    ///Bits 3:31 - LCD upper panel base address.
    #[inline(always)]
    pub fn lcdupbase(&self) -> LCDUPBASE_R {
        LCDUPBASE_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    ///Bits 3:31 - LCD upper panel base address.
    #[inline(always)]
    pub fn lcdupbase(&mut self) -> LCDUPBASE_W {
        LCDUPBASE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Upper Panel Frame Base Address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [upbase](index.html) module
pub struct UPBASE_SPEC;
impl crate::RegisterSpec for UPBASE_SPEC {
    type Ux = u32;
}
///`read()` method returns [upbase::R](R) reader structure
impl crate::Readable for UPBASE_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [upbase::W](W) writer structure
impl crate::Writable for UPBASE_SPEC {
    type Writer = W;
}
///`reset()` method sets UPBASE to value 0
impl crate::Resettable for UPBASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
