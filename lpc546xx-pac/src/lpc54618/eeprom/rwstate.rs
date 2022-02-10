///Register `RWSTATE` reader
pub struct R(crate::R<RWSTATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RWSTATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RWSTATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RWSTATE_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RWSTATE` writer
pub struct W(crate::W<RWSTATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RWSTATE_SPEC>;
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
impl From<crate::W<RWSTATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RWSTATE_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RPHASE2` reader - Wait states 2 (minus 1 encoded).
pub struct RPHASE2_R(crate::FieldReader<u8, u8>);
impl RPHASE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RPHASE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPHASE2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RPHASE2` writer - Wait states 2 (minus 1 encoded).
pub struct RPHASE2_W<'a> {
    w: &'a mut W,
}
impl<'a> RPHASE2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
///Field `RPHASE1` reader - Wait states 1 (minus 1 encoded).
pub struct RPHASE1_R(crate::FieldReader<u8, u8>);
impl RPHASE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RPHASE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPHASE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RPHASE1` writer - Wait states 1 (minus 1 encoded).
pub struct RPHASE1_W<'a> {
    w: &'a mut W,
}
impl<'a> RPHASE1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    ///Bits 0:7 - Wait states 2 (minus 1 encoded).
    #[inline(always)]
    pub fn rphase2(&self) -> RPHASE2_R {
        RPHASE2_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Wait states 1 (minus 1 encoded).
    #[inline(always)]
    pub fn rphase1(&self) -> RPHASE1_R {
        RPHASE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Wait states 2 (minus 1 encoded).
    #[inline(always)]
    pub fn rphase2(&mut self) -> RPHASE2_W {
        RPHASE2_W { w: self }
    }
    ///Bits 8:15 - Wait states 1 (minus 1 encoded).
    #[inline(always)]
    pub fn rphase1(&mut self) -> RPHASE1_W {
        RPHASE1_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EEPROM read wait state register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rwstate](index.html) module
pub struct RWSTATE_SPEC;
impl crate::RegisterSpec for RWSTATE_SPEC {
    type Ux = u32;
}
///`read()` method returns [rwstate::R](R) reader structure
impl crate::Readable for RWSTATE_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rwstate::W](W) writer structure
impl crate::Writable for RWSTATE_SPEC {
    type Writer = W;
}
///`reset()` method sets RWSTATE to value 0x0e07
impl crate::Resettable for RWSTATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0e07
    }
}
