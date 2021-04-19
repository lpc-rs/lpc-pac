#[doc = "Register `MODCFG` reader"]
pub struct R(crate::R<MODCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MODCFG_SPEC>> for R {
    fn from(reader: crate::R<MODCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODCFG` writer"]
pub struct W(crate::W<MODCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODCFG_SPEC>;
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
impl core::convert::From<crate::W<MODCFG_SPEC>> for W {
    fn from(writer: crate::W<MODCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NOC` reader - Identifies the number of channels in this MRT.(4 channels on this device.)"]
pub struct NOC_R(crate::FieldReader<u8, u8>);
impl NOC_R {
    pub(crate) fn new(bits: u8) -> Self {
        NOC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOB` reader - Identifies the number of timer bits in this MRT. (31 bits wide on this device.)"]
pub struct NOB_R(crate::FieldReader<u8, u8>);
impl NOB_R {
    pub(crate) fn new(bits: u8) -> Self {
        NOB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Identifies the number of channels in this MRT.(4 channels on this device.)"]
    #[inline(always)]
    pub fn noc(&self) -> NOC_R {
        NOC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - Identifies the number of timer bits in this MRT. (31 bits wide on this device.)"]
    #[inline(always)]
    pub fn nob(&self) -> NOB_R {
        NOB_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Configuration register. This register provides information about this particular MRT instance.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modcfg](index.html) module"]
pub struct MODCFG_SPEC;
impl crate::RegisterSpec for MODCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modcfg::R](R) reader structure"]
impl crate::Readable for MODCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [modcfg::W](W) writer structure"]
impl crate::Writable for MODCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODCFG to value 0x01f4"]
impl crate::Resettable for MODCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01f4
    }
}
