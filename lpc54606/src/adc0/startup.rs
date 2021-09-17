#[doc = "Register `STARTUP` reader"]
pub struct R(crate::R<STARTUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STARTUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STARTUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STARTUP` writer"]
pub struct W(crate::W<STARTUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTUP_SPEC>;
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
impl From<crate::W<STARTUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STARTUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_ENA` reader - ADC Enable bit. This bit can only be set to a 1 by software. It is cleared automatically whenever the ADC is powered down. This bit must not be set until at least 10 microseconds after the ADC is powered up (typically by altering a system-level ADC power control bit)."]
pub struct ADC_ENA_R(crate::FieldReader<bool, bool>);
impl ADC_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_ENA` writer - ADC Enable bit. This bit can only be set to a 1 by software. It is cleared automatically whenever the ADC is powered down. This bit must not be set until at least 10 microseconds after the ADC is powered up (typically by altering a system-level ADC power control bit)."]
pub struct ADC_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_ENA_W<'a> {
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
#[doc = "Field `ADC_INIT` reader - ADC Initialization. After enabling the ADC (setting the ADC_ENA bit), the API routine will EITHER set this bit or the CALIB bit in the CALIB register, depending on whether or not calibration is required. Setting this bit will launch the 'dummy' conversion cycle that is required if a calibration is not performed. It will also reload the stored calibration value from a previous calibration unless the BYPASSCAL bit is set. This bit should only be set AFTER the ADC_ENA bit is set and after the CALIREQD bit is tested to determine whether a calibration or an ADC dummy conversion cycle is required. It should not be set during the same write that sets the ADC_ENA bit. This bit can only be set to a '1' by software. It is cleared automatically when the 'dummy' conversion cycle completes."]
pub struct ADC_INIT_R(crate::FieldReader<bool, bool>);
impl ADC_INIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_INIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_INIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_INIT` writer - ADC Initialization. After enabling the ADC (setting the ADC_ENA bit), the API routine will EITHER set this bit or the CALIB bit in the CALIB register, depending on whether or not calibration is required. Setting this bit will launch the 'dummy' conversion cycle that is required if a calibration is not performed. It will also reload the stored calibration value from a previous calibration unless the BYPASSCAL bit is set. This bit should only be set AFTER the ADC_ENA bit is set and after the CALIREQD bit is tested to determine whether a calibration or an ADC dummy conversion cycle is required. It should not be set during the same write that sets the ADC_ENA bit. This bit can only be set to a '1' by software. It is cleared automatically when the 'dummy' conversion cycle completes."]
pub struct ADC_INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_INIT_W<'a> {
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
    #[doc = "Bit 0 - ADC Enable bit. This bit can only be set to a 1 by software. It is cleared automatically whenever the ADC is powered down. This bit must not be set until at least 10 microseconds after the ADC is powered up (typically by altering a system-level ADC power control bit)."]
    #[inline(always)]
    pub fn adc_ena(&self) -> ADC_ENA_R {
        ADC_ENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC Initialization. After enabling the ADC (setting the ADC_ENA bit), the API routine will EITHER set this bit or the CALIB bit in the CALIB register, depending on whether or not calibration is required. Setting this bit will launch the 'dummy' conversion cycle that is required if a calibration is not performed. It will also reload the stored calibration value from a previous calibration unless the BYPASSCAL bit is set. This bit should only be set AFTER the ADC_ENA bit is set and after the CALIREQD bit is tested to determine whether a calibration or an ADC dummy conversion cycle is required. It should not be set during the same write that sets the ADC_ENA bit. This bit can only be set to a '1' by software. It is cleared automatically when the 'dummy' conversion cycle completes."]
    #[inline(always)]
    pub fn adc_init(&self) -> ADC_INIT_R {
        ADC_INIT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Enable bit. This bit can only be set to a 1 by software. It is cleared automatically whenever the ADC is powered down. This bit must not be set until at least 10 microseconds after the ADC is powered up (typically by altering a system-level ADC power control bit)."]
    #[inline(always)]
    pub fn adc_ena(&mut self) -> ADC_ENA_W {
        ADC_ENA_W { w: self }
    }
    #[doc = "Bit 1 - ADC Initialization. After enabling the ADC (setting the ADC_ENA bit), the API routine will EITHER set this bit or the CALIB bit in the CALIB register, depending on whether or not calibration is required. Setting this bit will launch the 'dummy' conversion cycle that is required if a calibration is not performed. It will also reload the stored calibration value from a previous calibration unless the BYPASSCAL bit is set. This bit should only be set AFTER the ADC_ENA bit is set and after the CALIREQD bit is tested to determine whether a calibration or an ADC dummy conversion cycle is required. It should not be set during the same write that sets the ADC_ENA bit. This bit can only be set to a '1' by software. It is cleared automatically when the 'dummy' conversion cycle completes."]
    #[inline(always)]
    pub fn adc_init(&mut self) -> ADC_INIT_W {
        ADC_INIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Startup register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [startup](index.html) module"]
pub struct STARTUP_SPEC;
impl crate::RegisterSpec for STARTUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [startup::R](R) reader structure"]
impl crate::Readable for STARTUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [startup::W](W) writer structure"]
impl crate::Writable for STARTUP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STARTUP to value 0"]
impl crate::Resettable for STARTUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
