///Register `FMSSTOP` reader
pub struct R(crate::R<FMSSTOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMSSTOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMSSTOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMSSTOP_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FMSSTOP` writer
pub struct W(crate::W<FMSSTOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMSSTOP_SPEC>;
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
impl From<crate::W<FMSSTOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMSSTOP_SPEC>) -> Self {
        W(writer)
    }
}
///Field `STOP` reader - Stop address for signature generation (the word specified by STOP is included in the address range).
pub struct STOP_R(crate::FieldReader<u32, u32>);
impl STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `STOP` writer - Stop address for signature generation (the word specified by STOP is included in the address range).
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | (value as u32 & 0x0001_ffff);
        self.w
    }
}
///Field `SIG_START` reader - When this bit is written to 1, signature generation starts.
pub struct SIG_START_R(crate::FieldReader<bool, bool>);
impl SIG_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIG_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIG_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SIG_START` writer - When this bit is written to 1, signature generation starts.
pub struct SIG_START_W<'a> {
    w: &'a mut W,
}
impl<'a> SIG_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
impl R {
    ///Bits 0:16 - Stop address for signature generation (the word specified by STOP is included in the address range).
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new((self.bits & 0x0001_ffff) as u32)
    }
    ///Bit 17 - When this bit is written to 1, signature generation starts.
    #[inline(always)]
    pub fn sig_start(&self) -> SIG_START_R {
        SIG_START_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:16 - Stop address for signature generation (the word specified by STOP is included in the address range).
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    ///Bit 17 - When this bit is written to 1, signature generation starts.
    #[inline(always)]
    pub fn sig_start(&mut self) -> SIG_START_W {
        SIG_START_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Signature stop-address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fmsstop](index.html) module
pub struct FMSSTOP_SPEC;
impl crate::RegisterSpec for FMSSTOP_SPEC {
    type Ux = u32;
}
///`read()` method returns [fmsstop::R](R) reader structure
impl crate::Readable for FMSSTOP_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fmsstop::W](W) writer structure
impl crate::Writable for FMSSTOP_SPEC {
    type Writer = W;
}
///`reset()` method sets FMSSTOP to value 0
impl crate::Resettable for FMSSTOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
