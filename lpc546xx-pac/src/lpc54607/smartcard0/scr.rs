///Register `SCR` reader
pub struct R(crate::R<SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SCR` writer
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PAD` reader - A readable, writable byte.
pub struct PAD_R(crate::FieldReader<u8, u8>);
impl PAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PAD` writer - A readable, writable byte.
pub struct PAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    ///Bits 0:7 - A readable, writable byte.
    #[inline(always)]
    pub fn pad(&self) -> PAD_R {
        PAD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - A readable, writable byte.
    #[inline(always)]
    pub fn pad(&mut self) -> PAD_W {
        PAD_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Scratch Pad Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [scr](index.html) module
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [scr::R](R) reader structure
impl crate::Readable for SCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [scr::W](W) writer structure
impl crate::Writable for SCR_SPEC {
    type Writer = W;
}
///`reset()` method sets SCR to value 0
impl crate::Resettable for SCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
