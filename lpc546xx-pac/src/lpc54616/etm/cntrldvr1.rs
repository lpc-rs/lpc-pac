///Register `CNTRLDVR1` reader
pub struct R(crate::R<CNTRLDVR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTRLDVR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTRLDVR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTRLDVR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CNTRLDVR1` writer
pub struct W(crate::W<CNTRLDVR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTRLDVR1_SPEC>;
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
impl From<crate::W<CNTRLDVR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTRLDVR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IntitialCount` reader - Initial count.
pub struct INTITIALCOUNT_R(crate::FieldReader<u16, u16>);
impl INTITIALCOUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        INTITIALCOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTITIALCOUNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `IntitialCount` writer - Initial count.
pub struct INTITIALCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> INTITIALCOUNT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Initial count.
    #[inline(always)]
    pub fn intitial_count(&self) -> INTITIALCOUNT_R {
        INTITIALCOUNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Initial count.
    #[inline(always)]
    pub fn intitial_count(&mut self) -> INTITIALCOUNT_W {
        INTITIALCOUNT_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Free-running counter reload value
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cntrldvr1](index.html) module
pub struct CNTRLDVR1_SPEC;
impl crate::RegisterSpec for CNTRLDVR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [cntrldvr1::R](R) reader structure
impl crate::Readable for CNTRLDVR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cntrldvr1::W](W) writer structure
impl crate::Writable for CNTRLDVR1_SPEC {
    type Writer = W;
}
///`reset()` method sets CNTRLDVR1 to value 0
impl crate::Resettable for CNTRLDVR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
