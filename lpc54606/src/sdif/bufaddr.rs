#[doc = "Register `BUFADDR` reader"]
pub struct R(crate::R<BUFADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUFADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<BUFADDR_SPEC>> for R {
    fn from(reader: crate::R<BUFADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUFADDR` writer"]
pub struct W(crate::W<BUFADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUFADDR_SPEC>;
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
impl core::convert::From<crate::W<BUFADDR_SPEC>> for W {
    fn from(writer: crate::W<BUFADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HBA` reader - Host Buffer Address Pointer."]
pub struct HBA_R(crate::FieldReader<u32, u32>);
impl HBA_R {
    pub(crate) fn new(bits: u32) -> Self {
        HBA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HBA` writer - Host Buffer Address Pointer."]
pub struct HBA_W<'a> {
    w: &'a mut W,
}
impl<'a> HBA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Host Buffer Address Pointer."]
    #[inline(always)]
    pub fn hba(&self) -> HBA_R {
        HBA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Host Buffer Address Pointer."]
    #[inline(always)]
    pub fn hba(&mut self) -> HBA_W {
        HBA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Current Buffer Descriptor Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bufaddr](index.html) module"]
pub struct BUFADDR_SPEC;
impl crate::RegisterSpec for BUFADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bufaddr::R](R) reader structure"]
impl crate::Readable for BUFADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bufaddr::W](W) writer structure"]
impl crate::Writable for BUFADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUFADDR to value 0"]
impl crate::Resettable for BUFADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
