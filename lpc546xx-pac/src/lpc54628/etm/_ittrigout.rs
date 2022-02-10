///Register `_ITTRIGOUT` reader
pub struct R(crate::R<_ITTRIGOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_ITTRIGOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_ITTRIGOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_ITTRIGOUT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `_ITTRIGOUT` writer
pub struct W(crate::W<_ITTRIGOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_ITTRIGOUT_SPEC>;
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
impl From<crate::W<_ITTRIGOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_ITTRIGOUT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TRIGGER` writer - A write to this bit sets the TRIGGER output.
pub struct TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGER_W<'a> {
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
impl W {
    ///Bit 0 - A write to this bit sets the TRIGGER output.
    #[inline(always)]
    pub fn trigger(&mut self) -> TRIGGER_W {
        TRIGGER_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Integration Test Trigger Out Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [_ittrigout](index.html) module
pub struct _ITTRIGOUT_SPEC;
impl crate::RegisterSpec for _ITTRIGOUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [_ittrigout::R](R) reader structure
impl crate::Readable for _ITTRIGOUT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [_ittrigout::W](W) writer structure
impl crate::Writable for _ITTRIGOUT_SPEC {
    type Writer = W;
}
///`reset()` method sets _ITTRIGOUT to value 0
impl crate::Resettable for _ITTRIGOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
