///Register `FMSTATCLR` writer
pub struct W(crate::W<FMSTATCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMSTATCLR_SPEC>;
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
impl From<crate::W<FMSTATCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMSTATCLR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SIG_DONE_CLR` writer - Writing a 1 to this bits clears the signature generation completion flag (SIG_DONE) in the FMSTAT register.
pub struct SIG_DONE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SIG_DONE_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl W {
    ///Bit 2 - Writing a 1 to this bits clears the signature generation completion flag (SIG_DONE) in the FMSTAT register.
    #[inline(always)]
    pub fn sig_done_clr(&mut self) -> SIG_DONE_CLR_W {
        SIG_DONE_CLR_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Signature generation status clear register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fmstatclr](index.html) module
pub struct FMSTATCLR_SPEC;
impl crate::RegisterSpec for FMSTATCLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [fmstatclr::W](W) writer structure
impl crate::Writable for FMSTATCLR_SPEC {
    type Writer = W;
}
///`reset()` method sets FMSTATCLR to value 0
impl crate::Resettable for FMSTATCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
