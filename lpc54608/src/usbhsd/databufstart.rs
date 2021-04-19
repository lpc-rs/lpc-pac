#[doc = "Register `DATABUFSTART` reader"]
pub struct R(crate::R<DATABUFSTART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATABUFSTART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DATABUFSTART_SPEC>> for R {
    fn from(reader: crate::R<DATABUFSTART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATABUFSTART` writer"]
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
impl core::convert::From<crate::W<DATABUFSTART_SPEC>> for W {
    fn from(writer: crate::W<DATABUFSTART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DA_BUF` reader - Start address of the memory page where all endpoint data buffers are located."]
pub struct DA_BUF_R(crate::FieldReader<u32, u32>);
impl DA_BUF_R {
    pub(crate) fn new(bits: u32) -> Self {
        DA_BUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DA_BUF_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DA_BUF` writer - Start address of the memory page where all endpoint data buffers are located."]
pub struct DA_BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> DA_BUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Start address of the memory page where all endpoint data buffers are located."]
    #[inline(always)]
    pub fn da_buf(&self) -> DA_BUF_R {
        DA_BUF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address of the memory page where all endpoint data buffers are located."]
    #[inline(always)]
    pub fn da_buf(&mut self) -> DA_BUF_W {
        DA_BUF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Data buffer start address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [databufstart](index.html) module"]
pub struct DATABUFSTART_SPEC;
impl crate::RegisterSpec for DATABUFSTART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [databufstart::R](R) reader structure"]
impl crate::Readable for DATABUFSTART_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [databufstart::W](W) writer structure"]
impl crate::Writable for DATABUFSTART_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATABUFSTART to value 0x4100_0000"]
impl crate::Resettable for DATABUFSTART_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4100_0000
    }
}
