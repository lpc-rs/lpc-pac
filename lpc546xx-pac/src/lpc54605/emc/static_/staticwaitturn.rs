///Register `STATICWAITTURN` reader
pub struct R(crate::R<STATICWAITTURN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATICWAITTURN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATICWAITTURN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATICWAITTURN_SPEC>) -> Self {
        R(reader)
    }
}
///Register `STATICWAITTURN` writer
pub struct W(crate::W<STATICWAITTURN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATICWAITTURN_SPEC>;
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
impl From<crate::W<STATICWAITTURN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATICWAITTURN_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WAITTURN` reader - Bus turn-around cycles.
pub struct WAITTURN_R(crate::FieldReader<u8, u8>);
impl WAITTURN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WAITTURN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAITTURN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WAITTURN` writer - Bus turn-around cycles.
pub struct WAITTURN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITTURN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    ///Bits 0:3 - Bus turn-around cycles.
    #[inline(always)]
    pub fn waitturn(&self) -> WAITTURN_R {
        WAITTURN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - Bus turn-around cycles.
    #[inline(always)]
    pub fn waitturn(&mut self) -> WAITTURN_W {
        WAITTURN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Number of bus turnaround cycles EMC_CSx
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [staticwaitturn](index.html) module
pub struct STATICWAITTURN_SPEC;
impl crate::RegisterSpec for STATICWAITTURN_SPEC {
    type Ux = u32;
}
///`read()` method returns [staticwaitturn::R](R) reader structure
impl crate::Readable for STATICWAITTURN_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [staticwaitturn::W](W) writer structure
impl crate::Writable for STATICWAITTURN_SPEC {
    type Writer = W;
}
///`reset()` method sets STATICWAITTURN to value 0x0f
impl crate::Resettable for STATICWAITTURN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
