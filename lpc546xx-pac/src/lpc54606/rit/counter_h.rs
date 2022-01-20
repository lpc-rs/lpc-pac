#[doc = "Register `COUNTER_H` reader"]
pub struct R(crate::R<COUNTER_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNTER_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNTER_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNTER_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COUNTER_H` writer"]
pub struct W(crate::W<COUNTER_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COUNTER_H_SPEC>;
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
impl From<crate::W<COUNTER_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COUNTER_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RICOUNTER` reader - 16 LSBs of the up counter."]
pub struct RICOUNTER_R(crate::FieldReader<u16, u16>);
impl RICOUNTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RICOUNTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RICOUNTER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RICOUNTER` writer - 16 LSBs of the up counter."]
pub struct RICOUNTER_W<'a> {
    w: &'a mut W,
}
impl<'a> RICOUNTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - 16 LSBs of the up counter."]
    #[inline(always)]
    pub fn ricounter(&self) -> RICOUNTER_R {
        RICOUNTER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 16 LSBs of the up counter."]
    #[inline(always)]
    pub fn ricounter(&mut self) -> RICOUNTER_W {
        RICOUNTER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter MSB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [counter_h](index.html) module"]
pub struct COUNTER_H_SPEC;
impl crate::RegisterSpec for COUNTER_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [counter_h::R](R) reader structure"]
impl crate::Readable for COUNTER_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [counter_h::W](W) writer structure"]
impl crate::Writable for COUNTER_H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COUNTER_H to value 0"]
impl crate::Resettable for COUNTER_H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
