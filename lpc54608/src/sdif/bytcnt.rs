#[doc = "Register `BYTCNT` reader"]
pub struct R(crate::R<BYTCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BYTCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<BYTCNT_SPEC>> for R {
    fn from(reader: crate::R<BYTCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BYTCNT` writer"]
pub struct W(crate::W<BYTCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BYTCNT_SPEC>;
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
impl core::convert::From<crate::W<BYTCNT_SPEC>> for W {
    fn from(writer: crate::W<BYTCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYTE_COUNT` reader - Number of bytes to be transferred; should be integer multiple of Block Size for block transfers."]
pub struct BYTE_COUNT_R(crate::FieldReader<u32, u32>);
impl BYTE_COUNT_R {
    pub(crate) fn new(bits: u32) -> Self {
        BYTE_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE_COUNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE_COUNT` writer - Number of bytes to be transferred; should be integer multiple of Block Size for block transfers."]
pub struct BYTE_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Number of bytes to be transferred; should be integer multiple of Block Size for block transfers."]
    #[inline(always)]
    pub fn byte_count(&self) -> BYTE_COUNT_R {
        BYTE_COUNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of bytes to be transferred; should be integer multiple of Block Size for block transfers."]
    #[inline(always)]
    pub fn byte_count(&mut self) -> BYTE_COUNT_W {
        BYTE_COUNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Byte Count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bytcnt](index.html) module"]
pub struct BYTCNT_SPEC;
impl crate::RegisterSpec for BYTCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bytcnt::R](R) reader structure"]
impl crate::Readable for BYTCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bytcnt::W](W) writer structure"]
impl crate::Writable for BYTCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BYTCNT to value 0x0200"]
impl crate::Resettable for BYTCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
