///Register `COUNTER` reader
pub struct R(crate::R<COUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNTER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `COUNTER` writer
pub struct W(crate::W<COUNTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COUNTER_SPEC>;
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
impl From<crate::W<COUNTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COUNTER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RICOUNTER` reader - 32 LSBs of the up counter.
pub struct RICOUNTER_R(crate::FieldReader<u32, u32>);
impl RICOUNTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RICOUNTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RICOUNTER_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RICOUNTER` writer - 32 LSBs of the up counter.
pub struct RICOUNTER_W<'a> {
    w: &'a mut W,
}
impl<'a> RICOUNTER_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    ///Bits 0:31 - 32 LSBs of the up counter.
    #[inline(always)]
    pub fn ricounter(&self) -> RICOUNTER_R {
        RICOUNTER_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - 32 LSBs of the up counter.
    #[inline(always)]
    pub fn ricounter(&mut self) -> RICOUNTER_W {
        RICOUNTER_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Counter LSB register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [counter](index.html) module
pub struct COUNTER_SPEC;
impl crate::RegisterSpec for COUNTER_SPEC {
    type Ux = u32;
}
///`read()` method returns [counter::R](R) reader structure
impl crate::Readable for COUNTER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [counter::W](W) writer structure
impl crate::Writable for COUNTER_SPEC {
    type Writer = W;
}
///`reset()` method sets COUNTER to value 0
impl crate::Resettable for COUNTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
