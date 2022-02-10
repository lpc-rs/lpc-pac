///Register `MSR[%s]` reader
pub struct R(crate::R<MSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MSR[%s]` writer
pub struct W(crate::W<MSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSR_SPEC>;
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
impl From<crate::W<MSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SHADOWW` reader - Timer counter match shadow value.
pub struct SHADOWW_R(crate::FieldReader<u32, u32>);
impl SHADOWW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SHADOWW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHADOWW_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SHADOWW` writer - Timer counter match shadow value.
pub struct SHADOWW_W<'a> {
    w: &'a mut W,
}
impl<'a> SHADOWW_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    ///Bits 0:31 - Timer counter match shadow value.
    #[inline(always)]
    pub fn shadoww(&self) -> SHADOWW_R {
        SHADOWW_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Timer counter match shadow value.
    #[inline(always)]
    pub fn shadoww(&mut self) -> SHADOWW_W {
        SHADOWW_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Match Shadow Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [msr](index.html) module
pub struct MSR_SPEC;
impl crate::RegisterSpec for MSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [msr::R](R) reader structure
impl crate::Readable for MSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [msr::W](W) writer structure
impl crate::Writable for MSR_SPEC {
    type Writer = W;
}
///`reset()` method sets MSR[%s]
///to value 0
impl crate::Resettable for MSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
