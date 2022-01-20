#[doc = "Register `ASYNCPRESETCTRL` reader"]
pub struct R(crate::R<ASYNCPRESETCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASYNCPRESETCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASYNCPRESETCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASYNCPRESETCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASYNCPRESETCTRL` writer"]
pub struct W(crate::W<ASYNCPRESETCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASYNCPRESETCTRL_SPEC>;
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
impl From<crate::W<ASYNCPRESETCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASYNCPRESETCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTIMER3` reader - Standard counter/timer CTIMER3 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct CTIMER3_R(crate::FieldReader<bool, bool>);
impl CTIMER3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTIMER3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTIMER3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIMER3` writer - Standard counter/timer CTIMER3 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct CTIMER3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `CTIMER4` reader - Standard counter/timer CTIMER4 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct CTIMER4_R(crate::FieldReader<bool, bool>);
impl CTIMER4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTIMER4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTIMER4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIMER4` writer - Standard counter/timer CTIMER4 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct CTIMER4_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 13 - Standard counter/timer CTIMER3 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn ctimer3(&self) -> CTIMER3_R {
        CTIMER3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Standard counter/timer CTIMER4 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn ctimer4(&self) -> CTIMER4_R {
        CTIMER4_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Standard counter/timer CTIMER3 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn ctimer3(&mut self) -> CTIMER3_W {
        CTIMER3_W { w: self }
    }
    #[doc = "Bit 14 - Standard counter/timer CTIMER4 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn ctimer4(&mut self) -> CTIMER4_W {
        CTIMER4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Async peripheral reset control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asyncpresetctrl](index.html) module"]
pub struct ASYNCPRESETCTRL_SPEC;
impl crate::RegisterSpec for ASYNCPRESETCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [asyncpresetctrl::R](R) reader structure"]
impl crate::Readable for ASYNCPRESETCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asyncpresetctrl::W](W) writer structure"]
impl crate::Writable for ASYNCPRESETCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ASYNCPRESETCTRL to value 0"]
impl crate::Resettable for ASYNCPRESETCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
