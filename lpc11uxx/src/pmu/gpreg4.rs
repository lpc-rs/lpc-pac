#[doc = "Register `GPREG4` reader"]
pub struct R(crate::R<GPREG4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPREG4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPREG4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPREG4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPREG4` writer"]
pub struct W(crate::W<GPREG4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPREG4_SPEC>;
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
impl From<crate::W<GPREG4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPREG4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "WAKEUP pin hysteresis enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUPHYS_A {
    #[doc = "0: Hysteresis for WAKEUP pin disabled."]
    DISABLED = 0,
    #[doc = "1: Hysteresis for WAKEUP pin enabled."]
    ENABLED = 1,
}
impl From<WAKEUPHYS_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUPHYS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUPHYS` reader - WAKEUP pin hysteresis enable"]
pub struct WAKEUPHYS_R(crate::FieldReader<bool, WAKEUPHYS_A>);
impl WAKEUPHYS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPHYS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUPHYS_A {
        match self.bits {
            false => WAKEUPHYS_A::DISABLED,
            true => WAKEUPHYS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WAKEUPHYS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WAKEUPHYS_A::ENABLED
    }
}
impl core::ops::Deref for WAKEUPHYS_R {
    type Target = crate::FieldReader<bool, WAKEUPHYS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPHYS` writer - WAKEUP pin hysteresis enable"]
pub struct WAKEUPHYS_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPHYS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEUPHYS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Hysteresis for WAKEUP pin disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAKEUPHYS_A::DISABLED)
    }
    #[doc = "Hysteresis for WAKEUP pin enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAKEUPHYS_A::ENABLED)
    }
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
#[doc = "Field `GPDATA` reader - Data retained during Deep power-down mode."]
pub struct GPDATA_R(crate::FieldReader<u32, u32>);
impl GPDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        GPDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPDATA` writer - Data retained during Deep power-down mode."]
pub struct GPDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x001f_ffff << 11)) | ((value as u32 & 0x001f_ffff) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 10 - WAKEUP pin hysteresis enable"]
    #[inline(always)]
    pub fn wakeuphys(&self) -> WAKEUPHYS_R {
        WAKEUPHYS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:31 - Data retained during Deep power-down mode."]
    #[inline(always)]
    pub fn gpdata(&self) -> GPDATA_R {
        GPDATA_R::new(((self.bits >> 11) & 0x001f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 10 - WAKEUP pin hysteresis enable"]
    #[inline(always)]
    pub fn wakeuphys(&mut self) -> WAKEUPHYS_W {
        WAKEUPHYS_W { w: self }
    }
    #[doc = "Bits 11:31 - Data retained during Deep power-down mode."]
    #[inline(always)]
    pub fn gpdata(&mut self) -> GPDATA_W {
        GPDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General purpose register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpreg4](index.html) module"]
pub struct GPREG4_SPEC;
impl crate::RegisterSpec for GPREG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpreg4::R](R) reader structure"]
impl crate::Readable for GPREG4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpreg4::W](W) writer structure"]
impl crate::Writable for GPREG4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPREG4 to value 0"]
impl crate::Resettable for GPREG4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
