///Register `PC` reader
pub struct R(crate::R<PC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PC_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PC` writer
pub struct W(crate::W<PC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PC_SPEC>;
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
impl From<crate::W<PC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PC_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PCVAL` reader - Prescale counter value.
pub struct PCVAL_R(crate::FieldReader<u32, u32>);
impl PCVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PCVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCVAL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PCVAL` writer - Prescale counter value.
pub struct PCVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PCVAL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    ///Bits 0:31 - Prescale counter value.
    #[inline(always)]
    pub fn pcval(&self) -> PCVAL_R {
        PCVAL_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Prescale counter value.
    #[inline(always)]
    pub fn pcval(&mut self) -> PCVAL_W {
        PCVAL_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Prescale Counter
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pc](index.html) module
pub struct PC_SPEC;
impl crate::RegisterSpec for PC_SPEC {
    type Ux = u32;
}
///`read()` method returns [pc::R](R) reader structure
impl crate::Readable for PC_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pc::W](W) writer structure
impl crate::Writable for PC_SPEC {
    type Writer = W;
}
///`reset()` method sets PC to value 0
impl crate::Resettable for PC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
