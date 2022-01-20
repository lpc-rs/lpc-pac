#[doc = "Register `USB1CLKSTAT` reader"]
pub struct R(crate::R<USB1CLKSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB1CLKSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB1CLKSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB1CLKSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB1CLKSTAT` writer"]
pub struct W(crate::W<USB1CLKSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB1CLKSTAT_SPEC>;
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
impl From<crate::W<USB1CLKSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB1CLKSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEV_NEED_CLKST` reader - USB1 Device USB1_NEEDCLK signal status."]
pub struct DEV_NEED_CLKST_R(crate::FieldReader<bool, bool>);
impl DEV_NEED_CLKST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEV_NEED_CLKST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEV_NEED_CLKST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEV_NEED_CLKST` writer - USB1 Device USB1_NEEDCLK signal status."]
pub struct DEV_NEED_CLKST_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_NEED_CLKST_W<'a> {
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
#[doc = "Field `HOST_NEED_CLKST` reader - USB1 Device host USB1_NEEDCLK signal status."]
pub struct HOST_NEED_CLKST_R(crate::FieldReader<bool, bool>);
impl HOST_NEED_CLKST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_NEED_CLKST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_NEED_CLKST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_NEED_CLKST` writer - USB1 Device host USB1_NEEDCLK signal status."]
pub struct HOST_NEED_CLKST_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_NEED_CLKST_W<'a> {
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
    #[doc = "Bit 0 - USB1 Device USB1_NEEDCLK signal status."]
    #[inline(always)]
    pub fn dev_need_clkst(&self) -> DEV_NEED_CLKST_R {
        DEV_NEED_CLKST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB1 Device host USB1_NEEDCLK signal status."]
    #[inline(always)]
    pub fn host_need_clkst(&self) -> HOST_NEED_CLKST_R {
        HOST_NEED_CLKST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB1 Device USB1_NEEDCLK signal status."]
    #[inline(always)]
    pub fn dev_need_clkst(&mut self) -> DEV_NEED_CLKST_W {
        DEV_NEED_CLKST_W { w: self }
    }
    #[doc = "Bit 1 - USB1 Device host USB1_NEEDCLK signal status."]
    #[inline(always)]
    pub fn host_need_clkst(&mut self) -> HOST_NEED_CLKST_W {
        HOST_NEED_CLKST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB1 clock status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1clkstat](index.html) module"]
pub struct USB1CLKSTAT_SPEC;
impl crate::RegisterSpec for USB1CLKSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb1clkstat::R](R) reader structure"]
impl crate::Readable for USB1CLKSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb1clkstat::W](W) writer structure"]
impl crate::Writable for USB1CLKSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB1CLKSTAT to value 0"]
impl crate::Resettable for USB1CLKSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
