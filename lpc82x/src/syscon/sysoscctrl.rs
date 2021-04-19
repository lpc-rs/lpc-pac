#[doc = "Register `SYSOSCCTRL` reader"]
pub struct R(crate::R<SYSOSCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSOSCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYSOSCCTRL_SPEC>> for R {
    fn from(reader: crate::R<SYSOSCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSOSCCTRL` writer"]
pub struct W(crate::W<SYSOSCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSOSCCTRL_SPEC>;
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
impl core::convert::From<crate::W<SYSOSCCTRL_SPEC>> for W {
    fn from(writer: crate::W<SYSOSCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYPASS` reader - Bypass system oscillator"]
pub struct BYPASS_R(crate::FieldReader<bool, bool>);
impl BYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYPASS` writer - Bypass system oscillator"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
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
#[doc = "Field `FREQ_RANGE` reader - oscillator low / high transconductance selection input (Active High) 1-20MHz '0' : 15-50MHz '1'"]
pub struct FREQ_RANGE_R(crate::FieldReader<bool, bool>);
impl FREQ_RANGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FREQ_RANGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREQ_RANGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREQ_RANGE` writer - oscillator low / high transconductance selection input (Active High) 1-20MHz '0' : 15-50MHz '1'"]
pub struct FREQ_RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQ_RANGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Bypass system oscillator"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - oscillator low / high transconductance selection input (Active High) 1-20MHz '0' : 15-50MHz '1'"]
    #[inline(always)]
    pub fn freq_range(&self) -> FREQ_RANGE_R {
        FREQ_RANGE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass system oscillator"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 1 - oscillator low / high transconductance selection input (Active High) 1-20MHz '0' : 15-50MHz '1'"]
    #[inline(always)]
    pub fn freq_range(&mut self) -> FREQ_RANGE_W {
        FREQ_RANGE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "system oscillator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysoscctrl](index.html) module"]
pub struct SYSOSCCTRL_SPEC;
impl crate::RegisterSpec for SYSOSCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysoscctrl::R](R) reader structure"]
impl crate::Readable for SYSOSCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysoscctrl::W](W) writer structure"]
impl crate::Writable for SYSOSCCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSOSCCTRL to value 0"]
impl crate::Resettable for SYSOSCCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
