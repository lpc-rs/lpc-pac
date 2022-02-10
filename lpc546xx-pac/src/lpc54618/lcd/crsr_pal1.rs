///Register `CRSR_PAL1` reader
pub struct R(crate::R<CRSR_PAL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRSR_PAL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRSR_PAL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRSR_PAL1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CRSR_PAL1` writer
pub struct W(crate::W<CRSR_PAL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRSR_PAL1_SPEC>;
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
impl From<crate::W<CRSR_PAL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRSR_PAL1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RED` reader - Red color component.
pub struct RED_R(crate::FieldReader<u8, u8>);
impl RED_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RED` writer - Red color component.
pub struct RED_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
///Field `GREEN` reader - Green color component.
pub struct GREEN_R(crate::FieldReader<u8, u8>);
impl GREEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GREEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GREEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `GREEN` writer - Green color component.
pub struct GREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GREEN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
///Field `BLUE` reader - Blue color component.
pub struct BLUE_R(crate::FieldReader<u8, u8>);
impl BLUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BLUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BLUE` writer - Blue color component.
pub struct BLUE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLUE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:7 - Red color component.
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Green color component.
    #[inline(always)]
    pub fn green(&self) -> GREEN_R {
        GREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Blue color component.
    #[inline(always)]
    pub fn blue(&self) -> BLUE_R {
        BLUE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Red color component.
    #[inline(always)]
    pub fn red(&mut self) -> RED_W {
        RED_W { w: self }
    }
    ///Bits 8:15 - Green color component.
    #[inline(always)]
    pub fn green(&mut self) -> GREEN_W {
        GREEN_W { w: self }
    }
    ///Bits 16:23 - Blue color component.
    #[inline(always)]
    pub fn blue(&mut self) -> BLUE_W {
        BLUE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Cursor Palette register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [crsr_pal1](index.html) module
pub struct CRSR_PAL1_SPEC;
impl crate::RegisterSpec for CRSR_PAL1_SPEC {
    type Ux = u32;
}
///`read()` method returns [crsr_pal1::R](R) reader structure
impl crate::Readable for CRSR_PAL1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [crsr_pal1::W](W) writer structure
impl crate::Writable for CRSR_PAL1_SPEC {
    type Writer = W;
}
///`reset()` method sets CRSR_PAL1 to value 0
impl crate::Resettable for CRSR_PAL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
