///Register `CRSR_IMG[%s]` reader
pub struct R(crate::R<CRSR_IMG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRSR_IMG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRSR_IMG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRSR_IMG_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CRSR_IMG[%s]` writer
pub struct W(crate::W<CRSR_IMG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRSR_IMG_SPEC>;
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
impl From<crate::W<CRSR_IMG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRSR_IMG_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CRSR_IMG` reader - Cursor Image data.
pub struct CRSR_IMG_R(crate::FieldReader<u32, u32>);
impl CRSR_IMG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CRSR_IMG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRSR_IMG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CRSR_IMG` writer - Cursor Image data.
pub struct CRSR_IMG_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSR_IMG_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    ///Bits 0:31 - Cursor Image data.
    #[inline(always)]
    pub fn crsr_img(&self) -> CRSR_IMG_R {
        CRSR_IMG_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Cursor Image data.
    #[inline(always)]
    pub fn crsr_img(&mut self) -> CRSR_IMG_W {
        CRSR_IMG_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Cursor Image registers
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [crsr_img](index.html) module
pub struct CRSR_IMG_SPEC;
impl crate::RegisterSpec for CRSR_IMG_SPEC {
    type Ux = u32;
}
///`read()` method returns [crsr_img::R](R) reader structure
impl crate::Readable for CRSR_IMG_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [crsr_img::W](W) writer structure
impl crate::Writable for CRSR_IMG_SPEC {
    type Writer = W;
}
///`reset()` method sets CRSR_IMG[%s]
///to value 0
impl crate::Resettable for CRSR_IMG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
