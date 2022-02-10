///Register `PID` reader
pub struct R(crate::R<PID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PID_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PID` writer
pub struct W(crate::W<PID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PID_SPEC>;
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
impl From<crate::W<PID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PID_SPEC>) -> Self {
        W(writer)
    }
}
///Field `Minor_Rev` reader - Minor revision of module implementation.
pub struct MINOR_REV_R(crate::FieldReader<u8, u8>);
impl MINOR_REV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MINOR_REV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINOR_REV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `Major_Rev` reader - Major revision of module implementation.
pub struct MAJOR_REV_R(crate::FieldReader<u8, u8>);
impl MAJOR_REV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAJOR_REV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAJOR_REV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ID` reader - Module identifier for the selected function.
pub struct ID_R(crate::FieldReader<u16, u16>);
impl ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ID` writer - Module identifier for the selected function.
pub struct ID_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    ///Bits 8:11 - Minor revision of module implementation.
    #[inline(always)]
    pub fn minor_rev(&self) -> MINOR_REV_R {
        MINOR_REV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Major revision of module implementation.
    #[inline(always)]
    pub fn major_rev(&self) -> MAJOR_REV_R {
        MAJOR_REV_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:31 - Module identifier for the selected function.
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 16:31 - Module identifier for the selected function.
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Peripheral identification register.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pid](index.html) module
pub struct PID_SPEC;
impl crate::RegisterSpec for PID_SPEC {
    type Ux = u32;
}
///`read()` method returns [pid::R](R) reader structure
impl crate::Readable for PID_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pid::W](W) writer structure
impl crate::Writable for PID_SPEC {
    type Writer = W;
}
///`reset()` method sets PID to value 0
impl crate::Resettable for PID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
