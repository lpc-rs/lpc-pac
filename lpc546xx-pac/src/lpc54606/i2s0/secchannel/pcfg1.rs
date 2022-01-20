#[doc = "Register `PCFG1` reader"]
pub struct R(crate::R<PCFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCFG1` writer"]
pub struct W(crate::W<PCFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCFG1_SPEC>;
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
impl From<crate::W<PCFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAIRENABLE` reader - Enable for this channel pair.."]
pub struct PAIRENABLE_R(crate::FieldReader<bool, bool>);
impl PAIRENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAIRENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAIRENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAIRENABLE` writer - Enable for this channel pair.."]
pub struct PAIRENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIRENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `ONECHANNEL` reader - Single channel mode."]
pub struct ONECHANNEL_R(crate::FieldReader<bool, bool>);
impl ONECHANNEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ONECHANNEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ONECHANNEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONECHANNEL` writer - Single channel mode."]
pub struct ONECHANNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ONECHANNEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable for this channel pair.."]
    #[inline(always)]
    pub fn pairenable(&self) -> PAIRENABLE_R {
        PAIRENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 10 - Single channel mode."]
    #[inline(always)]
    pub fn onechannel(&self) -> ONECHANNEL_R {
        ONECHANNEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable for this channel pair.."]
    #[inline(always)]
    pub fn pairenable(&mut self) -> PAIRENABLE_W {
        PAIRENABLE_W { w: self }
    }
    #[doc = "Bit 10 - Single channel mode."]
    #[inline(always)]
    pub fn onechannel(&mut self) -> ONECHANNEL_W {
        ONECHANNEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register 1 for channel pair\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcfg1](index.html) module"]
pub struct PCFG1_SPEC;
impl crate::RegisterSpec for PCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcfg1::R](R) reader structure"]
impl crate::Readable for PCFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcfg1::W](W) writer structure"]
impl crate::Writable for PCFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCFG1 to value 0"]
impl crate::Resettable for PCFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
