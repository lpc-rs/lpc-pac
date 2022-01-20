#[doc = "Register `REGMODE` reader"]
pub struct R(crate::R<REGMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGMODE` writer"]
pub struct W(crate::W<REGMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGMODE_SPEC>;
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
impl From<crate::W<REGMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGMOD_L` reader - Each bit controls one match/capture register (register 0 = bit 0, register 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match register. 1 = register operates as capture register."]
pub struct REGMOD_L_R(crate::FieldReader<u16, u16>);
impl REGMOD_L_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        REGMOD_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGMOD_L_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGMOD_L` writer - Each bit controls one match/capture register (register 0 = bit 0, register 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match register. 1 = register operates as capture register."]
pub struct REGMOD_L_W<'a> {
    w: &'a mut W,
}
impl<'a> REGMOD_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `REGMOD_H` reader - Each bit controls one match/capture register (register 0 = bit 16, register 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match registers. 1 = register operates as capture registers."]
pub struct REGMOD_H_R(crate::FieldReader<u16, u16>);
impl REGMOD_H_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        REGMOD_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGMOD_H_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGMOD_H` writer - Each bit controls one match/capture register (register 0 = bit 16, register 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match registers. 1 = register operates as capture registers."]
pub struct REGMOD_H_W<'a> {
    w: &'a mut W,
}
impl<'a> REGMOD_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Each bit controls one match/capture register (register 0 = bit 0, register 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match register. 1 = register operates as capture register."]
    #[inline(always)]
    pub fn regmod_l(&self) -> REGMOD_L_R {
        REGMOD_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Each bit controls one match/capture register (register 0 = bit 16, register 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match registers. 1 = register operates as capture registers."]
    #[inline(always)]
    pub fn regmod_h(&self) -> REGMOD_H_R {
        REGMOD_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Each bit controls one match/capture register (register 0 = bit 0, register 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match register. 1 = register operates as capture register."]
    #[inline(always)]
    pub fn regmod_l(&mut self) -> REGMOD_L_W {
        REGMOD_L_W { w: self }
    }
    #[doc = "Bits 16:31 - Each bit controls one match/capture register (register 0 = bit 16, register 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match registers. 1 = register operates as capture registers."]
    #[inline(always)]
    pub fn regmod_h(&mut self) -> REGMOD_H_W {
        REGMOD_H_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT match/capture mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regmode](index.html) module"]
pub struct REGMODE_SPEC;
impl crate::RegisterSpec for REGMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [regmode::R](R) reader structure"]
impl crate::Readable for REGMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [regmode::W](W) writer structure"]
impl crate::Writable for REGMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REGMODE to value 0"]
impl crate::Resettable for REGMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
