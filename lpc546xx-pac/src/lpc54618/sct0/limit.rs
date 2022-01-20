#[doc = "Register `LIMIT` reader"]
pub struct R(crate::R<LIMIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LIMIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LIMIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LIMIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LIMIT` writer"]
pub struct W(crate::W<LIMIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LIMIT_SPEC>;
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
impl From<crate::W<LIMIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LIMIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LIMMSK_L` reader - If bit n is one, event n is used as a counter limit for the L or unified counter (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub struct LIMMSK_L_R(crate::FieldReader<u16, u16>);
impl LIMMSK_L_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LIMMSK_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LIMMSK_L_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LIMMSK_L` writer - If bit n is one, event n is used as a counter limit for the L or unified counter (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub struct LIMMSK_L_W<'a> {
    w: &'a mut W,
}
impl<'a> LIMMSK_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `LIMMSK_H` reader - If bit n is one, event n is used as a counter limit for the H counter (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
pub struct LIMMSK_H_R(crate::FieldReader<u16, u16>);
impl LIMMSK_H_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LIMMSK_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LIMMSK_H_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LIMMSK_H` writer - If bit n is one, event n is used as a counter limit for the H counter (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
pub struct LIMMSK_H_W<'a> {
    w: &'a mut W,
}
impl<'a> LIMMSK_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - If bit n is one, event n is used as a counter limit for the L or unified counter (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn limmsk_l(&self) -> LIMMSK_L_R {
        LIMMSK_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - If bit n is one, event n is used as a counter limit for the H counter (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn limmsk_h(&self) -> LIMMSK_H_R {
        LIMMSK_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - If bit n is one, event n is used as a counter limit for the L or unified counter (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn limmsk_l(&mut self) -> LIMMSK_L_W {
        LIMMSK_L_W { w: self }
    }
    #[doc = "Bits 16:31 - If bit n is one, event n is used as a counter limit for the H counter (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn limmsk_h(&mut self) -> LIMMSK_H_W {
        LIMMSK_H_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT limit event select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [limit](index.html) module"]
pub struct LIMIT_SPEC;
impl crate::RegisterSpec for LIMIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [limit::R](R) reader structure"]
impl crate::Readable for LIMIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [limit::W](W) writer structure"]
impl crate::Writable for LIMIT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LIMIT to value 0"]
impl crate::Resettable for LIMIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
