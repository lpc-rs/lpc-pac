///Register `HWVADTHGS` reader
pub struct R(crate::R<HWVADTHGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWVADTHGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWVADTHGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWVADTHGS_SPEC>) -> Self {
        R(reader)
    }
}
///Register `HWVADTHGS` writer
pub struct W(crate::W<HWVADTHGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWVADTHGS_SPEC>;
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
impl From<crate::W<HWVADTHGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWVADTHGS_SPEC>) -> Self {
        W(writer)
    }
}
///Field `THGS` reader - Gain value for the signal estimator. Values 0 to 14. 0 corresponds to a gain of 1.
pub struct THGS_R(crate::FieldReader<u8, u8>);
impl THGS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        THGS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THGS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `THGS` writer - Gain value for the signal estimator. Values 0 to 14. 0 corresponds to a gain of 1.
pub struct THGS_W<'a> {
    w: &'a mut W,
}
impl<'a> THGS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    ///Bits 0:3 - Gain value for the signal estimator. Values 0 to 14. 0 corresponds to a gain of 1.
    #[inline(always)]
    pub fn thgs(&self) -> THGS_R {
        THGS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - Gain value for the signal estimator. Values 0 to 14. 0 corresponds to a gain of 1.
    #[inline(always)]
    pub fn thgs(&mut self) -> THGS_W {
        THGS_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///HWVAD signal estimator gain register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hwvadthgs](index.html) module
pub struct HWVADTHGS_SPEC;
impl crate::RegisterSpec for HWVADTHGS_SPEC {
    type Ux = u32;
}
///`read()` method returns [hwvadthgs::R](R) reader structure
impl crate::Readable for HWVADTHGS_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [hwvadthgs::W](W) writer structure
impl crate::Writable for HWVADTHGS_SPEC {
    type Writer = W;
}
///`reset()` method sets HWVADTHGS to value 0x04
impl crate::Resettable for HWVADTHGS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
