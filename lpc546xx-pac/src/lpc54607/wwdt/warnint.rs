///Register `WARNINT` reader
pub struct R(crate::R<WARNINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WARNINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WARNINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WARNINT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `WARNINT` writer
pub struct W(crate::W<WARNINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WARNINT_SPEC>;
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
impl From<crate::W<WARNINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WARNINT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WARNINT` reader - Watchdog warning interrupt compare value.
pub struct WARNINT_R(crate::FieldReader<u16, u16>);
impl WARNINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WARNINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WARNINT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WARNINT` writer - Watchdog warning interrupt compare value.
pub struct WARNINT_W<'a> {
    w: &'a mut W,
}
impl<'a> WARNINT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    ///Bits 0:9 - Watchdog warning interrupt compare value.
    #[inline(always)]
    pub fn warnint(&self) -> WARNINT_R {
        WARNINT_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:9 - Watchdog warning interrupt compare value.
    #[inline(always)]
    pub fn warnint(&mut self) -> WARNINT_W {
        WARNINT_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Watchdog Warning Interrupt compare value.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [warnint](index.html) module
pub struct WARNINT_SPEC;
impl crate::RegisterSpec for WARNINT_SPEC {
    type Ux = u32;
}
///`read()` method returns [warnint::R](R) reader structure
impl crate::Readable for WARNINT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [warnint::W](W) writer structure
impl crate::Writable for WARNINT_SPEC {
    type Writer = W;
}
///`reset()` method sets WARNINT to value 0
impl crate::Resettable for WARNINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
