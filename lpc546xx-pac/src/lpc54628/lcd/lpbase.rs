#[doc = "Register `LPBASE` reader"]
pub struct R(crate::R<LPBASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPBASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPBASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPBASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPBASE` writer"]
pub struct W(crate::W<LPBASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPBASE_SPEC>;
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
impl From<crate::W<LPBASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPBASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDLPBASE` reader - LCD lower panel base address."]
pub struct LCDLPBASE_R(crate::FieldReader<u32, u32>);
impl LCDLPBASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        LCDLPBASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDLPBASE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDLPBASE` writer - LCD lower panel base address."]
pub struct LCDLPBASE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDLPBASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | ((value as u32 & 0x1fff_ffff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - LCD lower panel base address."]
    #[inline(always)]
    pub fn lcdlpbase(&self) -> LCDLPBASE_R {
        LCDLPBASE_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 3:31 - LCD lower panel base address."]
    #[inline(always)]
    pub fn lcdlpbase(&mut self) -> LCDLPBASE_W {
        LCDLPBASE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lower Panel Frame Base Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpbase](index.html) module"]
pub struct LPBASE_SPEC;
impl crate::RegisterSpec for LPBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpbase::R](R) reader structure"]
impl crate::Readable for LPBASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpbase::W](W) writer structure"]
impl crate::Writable for LPBASE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPBASE to value 0"]
impl crate::Resettable for LPBASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
