///Register `DYNAMICMRD` reader
pub struct R(crate::R<DYNAMICMRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYNAMICMRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DYNAMICMRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DYNAMICMRD_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DYNAMICMRD` writer
pub struct W(crate::W<DYNAMICMRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYNAMICMRD_SPEC>;
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
impl From<crate::W<DYNAMICMRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DYNAMICMRD_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TMRD` reader - Load mode register to active command time.
pub struct TMRD_R(crate::FieldReader<u8, u8>);
impl TMRD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TMRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMRD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TMRD` writer - Load mode register to active command time.
pub struct TMRD_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    ///Bits 0:3 - Load mode register to active command time.
    #[inline(always)]
    pub fn tmrd(&self) -> TMRD_R {
        TMRD_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - Load mode register to active command time.
    #[inline(always)]
    pub fn tmrd(&mut self) -> TMRD_W {
        TMRD_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Time for load mode register to active command
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dynamicmrd](index.html) module
pub struct DYNAMICMRD_SPEC;
impl crate::RegisterSpec for DYNAMICMRD_SPEC {
    type Ux = u32;
}
///`read()` method returns [dynamicmrd::R](R) reader structure
impl crate::Readable for DYNAMICMRD_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dynamicmrd::W](W) writer structure
impl crate::Writable for DYNAMICMRD_SPEC {
    type Writer = W;
}
///`reset()` method sets DYNAMICMRD to value 0x0f
impl crate::Resettable for DYNAMICMRD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
