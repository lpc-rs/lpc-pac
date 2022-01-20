#[doc = "Register `PWREN` reader"]
pub struct R(crate::R<PWREN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWREN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWREN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWREN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWREN` writer"]
pub struct W(crate::W<PWREN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWREN_SPEC>;
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
impl From<crate::W<PWREN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWREN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POWER_ENABLE` reader - Power on/off switch for card; once power is turned on, software should wait for regulator/switch ramp-up time before trying to initialize card."]
pub struct POWER_ENABLE_R(crate::FieldReader<bool, bool>);
impl POWER_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POWER_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POWER_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POWER_ENABLE` writer - Power on/off switch for card; once power is turned on, software should wait for regulator/switch ramp-up time before trying to initialize card."]
pub struct POWER_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_ENABLE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Power on/off switch for card; once power is turned on, software should wait for regulator/switch ramp-up time before trying to initialize card."]
    #[inline(always)]
    pub fn power_enable(&self) -> POWER_ENABLE_R {
        POWER_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power on/off switch for card; once power is turned on, software should wait for regulator/switch ramp-up time before trying to initialize card."]
    #[inline(always)]
    pub fn power_enable(&mut self) -> POWER_ENABLE_W {
        POWER_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwren](index.html) module"]
pub struct PWREN_SPEC;
impl crate::RegisterSpec for PWREN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwren::R](R) reader structure"]
impl crate::Readable for PWREN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwren::W](W) writer structure"]
impl crate::Writable for PWREN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWREN to value 0"]
impl crate::Resettable for PWREN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
