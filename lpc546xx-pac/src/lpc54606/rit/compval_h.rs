///Register `COMPVAL_H` reader
pub struct R(crate::R<COMPVAL_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMPVAL_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMPVAL_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMPVAL_H_SPEC>) -> Self {
        R(reader)
    }
}
///Register `COMPVAL_H` writer
pub struct W(crate::W<COMPVAL_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMPVAL_H_SPEC>;
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
impl From<crate::W<COMPVAL_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMPVAL_H_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RICOMP` reader - Compare value MSB register.
pub struct RICOMP_R(crate::FieldReader<u16, u16>);
impl RICOMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RICOMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RICOMP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RICOMP` writer - Compare value MSB register.
pub struct RICOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> RICOMP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Compare value MSB register.
    #[inline(always)]
    pub fn ricomp(&self) -> RICOMP_R {
        RICOMP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Compare value MSB register.
    #[inline(always)]
    pub fn ricomp(&mut self) -> RICOMP_W {
        RICOMP_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Compare value MSB register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [compval_h](index.html) module
pub struct COMPVAL_H_SPEC;
impl crate::RegisterSpec for COMPVAL_H_SPEC {
    type Ux = u32;
}
///`read()` method returns [compval_h::R](R) reader structure
impl crate::Readable for COMPVAL_H_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [compval_h::W](W) writer structure
impl crate::Writable for COMPVAL_H_SPEC {
    type Writer = W;
}
///`reset()` method sets COMPVAL_H to value 0xffff
impl crate::Resettable for COMPVAL_H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
