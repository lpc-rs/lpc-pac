///Register `BACKENDPWR` reader
pub struct R(crate::R<BACKENDPWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BACKENDPWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BACKENDPWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BACKENDPWR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BACKENDPWR` writer
pub struct W(crate::W<BACKENDPWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BACKENDPWR_SPEC>;
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
impl From<crate::W<BACKENDPWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BACKENDPWR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BACKENDPWR` reader - Back-end Power control for card application.
pub struct BACKENDPWR_R(crate::FieldReader<bool, bool>);
impl BACKENDPWR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BACKENDPWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKENDPWR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BACKENDPWR` writer - Back-end Power control for card application.
pub struct BACKENDPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKENDPWR_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    ///Bit 0 - Back-end Power control for card application.
    #[inline(always)]
    pub fn backendpwr(&self) -> BACKENDPWR_R {
        BACKENDPWR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Back-end Power control for card application.
    #[inline(always)]
    pub fn backendpwr(&mut self) -> BACKENDPWR_W {
        BACKENDPWR_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Power control
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [backendpwr](index.html) module
pub struct BACKENDPWR_SPEC;
impl crate::RegisterSpec for BACKENDPWR_SPEC {
    type Ux = u32;
}
///`read()` method returns [backendpwr::R](R) reader structure
impl crate::Readable for BACKENDPWR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [backendpwr::W](W) writer structure
impl crate::Writable for BACKENDPWR_SPEC {
    type Writer = W;
}
///`reset()` method sets BACKENDPWR to value 0
impl crate::Resettable for BACKENDPWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
