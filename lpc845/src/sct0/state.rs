#[doc = "Register `STATE` reader"]
pub struct R(crate::R<STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATE` writer"]
pub struct W(crate::W<STATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATE_SPEC>;
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
impl From<crate::W<STATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STATE_L` reader - State variable."]
pub struct STATE_L_R(crate::FieldReader<u8, u8>);
impl STATE_L_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STATE_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATE_L_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATE_L` writer - State variable."]
pub struct STATE_L_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `STATE_H` reader - State variable."]
pub struct STATE_H_R(crate::FieldReader<u8, u8>);
impl STATE_H_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STATE_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATE_H_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATE_H` writer - State variable."]
pub struct STATE_H_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - State variable."]
    #[inline(always)]
    pub fn state_l(&self) -> STATE_L_R {
        STATE_L_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - State variable."]
    #[inline(always)]
    pub fn state_h(&self) -> STATE_H_R {
        STATE_H_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - State variable."]
    #[inline(always)]
    pub fn state_l(&mut self) -> STATE_L_W {
        STATE_L_W { w: self }
    }
    #[doc = "Bits 16:20 - State variable."]
    #[inline(always)]
    pub fn state_h(&mut self) -> STATE_H_W {
        STATE_H_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT state register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state](index.html) module"]
pub struct STATE_SPEC;
impl crate::RegisterSpec for STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [state::R](R) reader structure"]
impl crate::Readable for STATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [state::W](W) writer structure"]
impl crate::Writable for STATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATE to value 0"]
impl crate::Resettable for STATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
