#[doc = "Register `BLKSIZ` reader"]
pub struct R(crate::R<BLKSIZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLKSIZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLKSIZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLKSIZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLKSIZ` writer"]
pub struct W(crate::W<BLKSIZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLKSIZ_SPEC>;
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
impl From<crate::W<BLKSIZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLKSIZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLOCK_SIZE` reader - Block size."]
pub struct BLOCK_SIZE_R(crate::FieldReader<u16, u16>);
impl BLOCK_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BLOCK_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLOCK_SIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLOCK_SIZE` writer - Block size."]
pub struct BLOCK_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Block size."]
    #[inline(always)]
    pub fn block_size(&self) -> BLOCK_SIZE_R {
        BLOCK_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Block size."]
    #[inline(always)]
    pub fn block_size(&mut self) -> BLOCK_SIZE_W {
        BLOCK_SIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block Size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blksiz](index.html) module"]
pub struct BLKSIZ_SPEC;
impl crate::RegisterSpec for BLKSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blksiz::R](R) reader structure"]
impl crate::Readable for BLKSIZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blksiz::W](W) writer structure"]
impl crate::Writable for BLKSIZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLKSIZ to value 0x0200"]
impl crate::Resettable for BLKSIZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
