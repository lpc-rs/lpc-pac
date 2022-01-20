#[doc = "Register `EMCSYSCTRL` reader"]
pub struct R(crate::R<EMCSYSCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMCSYSCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMCSYSCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMCSYSCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMCSYSCTRL` writer"]
pub struct W(crate::W<EMCSYSCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMCSYSCTRL_SPEC>;
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
impl From<crate::W<EMCSYSCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMCSYSCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMCSC` reader - EMC Shift Control."]
pub struct EMCSC_R(crate::FieldReader<bool, bool>);
impl EMCSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EMCSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMCSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMCSC` writer - EMC Shift Control."]
pub struct EMCSC_W<'a> {
    w: &'a mut W,
}
impl<'a> EMCSC_W<'a> {
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
#[doc = "Field `EMCRD` reader - EMC Reset Disable."]
pub struct EMCRD_R(crate::FieldReader<bool, bool>);
impl EMCRD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EMCRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMCRD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMCRD` writer - EMC Reset Disable."]
pub struct EMCRD_W<'a> {
    w: &'a mut W,
}
impl<'a> EMCRD_W<'a> {
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
#[doc = "Field `EMCBC` reader - External Memory Controller burst control."]
pub struct EMCBC_R(crate::FieldReader<bool, bool>);
impl EMCBC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EMCBC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMCBC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMCBC` writer - External Memory Controller burst control."]
pub struct EMCBC_W<'a> {
    w: &'a mut W,
}
impl<'a> EMCBC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `EMCFBCLKINSEL` reader - External Memory Controller clock select."]
pub struct EMCFBCLKINSEL_R(crate::FieldReader<bool, bool>);
impl EMCFBCLKINSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EMCFBCLKINSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMCFBCLKINSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMCFBCLKINSEL` writer - External Memory Controller clock select."]
pub struct EMCFBCLKINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EMCFBCLKINSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - EMC Shift Control."]
    #[inline(always)]
    pub fn emcsc(&self) -> EMCSC_R {
        EMCSC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EMC Reset Disable."]
    #[inline(always)]
    pub fn emcrd(&self) -> EMCRD_R {
        EMCRD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Memory Controller burst control."]
    #[inline(always)]
    pub fn emcbc(&self) -> EMCBC_R {
        EMCBC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Memory Controller clock select."]
    #[inline(always)]
    pub fn emcfbclkinsel(&self) -> EMCFBCLKINSEL_R {
        EMCFBCLKINSEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EMC Shift Control."]
    #[inline(always)]
    pub fn emcsc(&mut self) -> EMCSC_W {
        EMCSC_W { w: self }
    }
    #[doc = "Bit 1 - EMC Reset Disable."]
    #[inline(always)]
    pub fn emcrd(&mut self) -> EMCRD_W {
        EMCRD_W { w: self }
    }
    #[doc = "Bit 2 - External Memory Controller burst control."]
    #[inline(always)]
    pub fn emcbc(&mut self) -> EMCBC_W {
        EMCBC_W { w: self }
    }
    #[doc = "Bit 3 - External Memory Controller clock select."]
    #[inline(always)]
    pub fn emcfbclkinsel(&mut self) -> EMCFBCLKINSEL_W {
        EMCFBCLKINSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMC system control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emcsysctrl](index.html) module"]
pub struct EMCSYSCTRL_SPEC;
impl crate::RegisterSpec for EMCSYSCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emcsysctrl::R](R) reader structure"]
impl crate::Readable for EMCSYSCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emcsysctrl::W](W) writer structure"]
impl crate::Writable for EMCSYSCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMCSYSCTRL to value 0x01"]
impl crate::Resettable for EMCSYSCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
