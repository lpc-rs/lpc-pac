///Register `CRSR_XY` reader
pub struct R(crate::R<CRSR_XY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRSR_XY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRSR_XY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRSR_XY_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CRSR_XY` writer
pub struct W(crate::W<CRSR_XY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRSR_XY_SPEC>;
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
impl From<crate::W<CRSR_XY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRSR_XY_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CRSRX` reader - X ordinate of the cursor origin measured in pixels.
pub struct CRSRX_R(crate::FieldReader<u16, u16>);
impl CRSRX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CRSRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRSRX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CRSRX` writer - X ordinate of the cursor origin measured in pixels.
pub struct CRSRX_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSRX_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
///Field `CRSRY` reader - Y ordinate of the cursor origin measured in pixels.
pub struct CRSRY_R(crate::FieldReader<u16, u16>);
impl CRSRY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CRSRY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRSRY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CRSRY` writer - Y ordinate of the cursor origin measured in pixels.
pub struct CRSRY_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSRY_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:9 - X ordinate of the cursor origin measured in pixels.
    #[inline(always)]
    pub fn crsrx(&self) -> CRSRX_R {
        CRSRX_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 16:25 - Y ordinate of the cursor origin measured in pixels.
    #[inline(always)]
    pub fn crsry(&self) -> CRSRY_R {
        CRSRY_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:9 - X ordinate of the cursor origin measured in pixels.
    #[inline(always)]
    pub fn crsrx(&mut self) -> CRSRX_W {
        CRSRX_W { w: self }
    }
    ///Bits 16:25 - Y ordinate of the cursor origin measured in pixels.
    #[inline(always)]
    pub fn crsry(&mut self) -> CRSRY_W {
        CRSRY_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Cursor XY Position register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [crsr_xy](index.html) module
pub struct CRSR_XY_SPEC;
impl crate::RegisterSpec for CRSR_XY_SPEC {
    type Ux = u32;
}
///`read()` method returns [crsr_xy::R](R) reader structure
impl crate::Readable for CRSR_XY_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [crsr_xy::W](W) writer structure
impl crate::Writable for CRSR_XY_SPEC {
    type Writer = W;
}
///`reset()` method sets CRSR_XY to value 0
impl crate::Resettable for CRSR_XY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
