///Register `_ITATBCTR0` reader
pub struct R(crate::R<_ITATBCTR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_ITATBCTR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_ITATBCTR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_ITATBCTR0_SPEC>) -> Self {
        R(reader)
    }
}
///Register `_ITATBCTR0` writer
pub struct W(crate::W<_ITATBCTR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_ITATBCTR0_SPEC>;
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
impl From<crate::W<_ITATBCTR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_ITATBCTR0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ATVALID` writer - A write to this bit sets the value of the ETM ATVALID output.
pub struct ATVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> ATVALID_W<'a> {
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
    ///Bit 0 - A write to this bit sets the value of the ETM ATVALID output.
    #[inline(always)]
    pub fn atvalid(&mut self) -> ATVALID_W {
        ATVALID_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ETM Integration Test ATB Control 0 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [_itatbctr0](index.html) module
pub struct _ITATBCTR0_SPEC;
impl crate::RegisterSpec for _ITATBCTR0_SPEC {
    type Ux = u32;
}
///`read()` method returns [_itatbctr0::R](R) reader structure
impl crate::Readable for _ITATBCTR0_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [_itatbctr0::W](W) writer structure
impl crate::Writable for _ITATBCTR0_SPEC {
    type Writer = W;
}
///`reset()` method sets _ITATBCTR0 to value 0
impl crate::Resettable for _ITATBCTR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
