#[doc = "Register `CMDARG` reader"]
pub struct R(crate::R<CMDARG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDARG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CMDARG_SPEC>> for R {
    fn from(reader: crate::R<CMDARG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDARG` writer"]
pub struct W(crate::W<CMDARG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDARG_SPEC>;
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
impl core::convert::From<crate::W<CMDARG_SPEC>> for W {
    fn from(writer: crate::W<CMDARG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD_ARG` reader - Value indicates command argument to be passed to card."]
pub struct CMD_ARG_R(crate::FieldReader<u32, u32>);
impl CMD_ARG_R {
    pub(crate) fn new(bits: u32) -> Self {
        CMD_ARG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMD_ARG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD_ARG` writer - Value indicates command argument to be passed to card."]
pub struct CMD_ARG_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_ARG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Value indicates command argument to be passed to card."]
    #[inline(always)]
    pub fn cmd_arg(&self) -> CMD_ARG_R {
        CMD_ARG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value indicates command argument to be passed to card."]
    #[inline(always)]
    pub fn cmd_arg(&mut self) -> CMD_ARG_W {
        CMD_ARG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Argument register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdarg](index.html) module"]
pub struct CMDARG_SPEC;
impl crate::RegisterSpec for CMDARG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdarg::R](R) reader structure"]
impl crate::Readable for CMDARG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdarg::W](W) writer structure"]
impl crate::Writable for CMDARG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMDARG to value 0"]
impl crate::Resettable for CMDARG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
