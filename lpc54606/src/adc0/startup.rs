#[doc = "Reader of register STARTUP"]
pub type R = crate::R<u32, super::STARTUP>;
#[doc = "Writer for register STARTUP"]
pub type W = crate::W<u32, super::STARTUP>;
#[doc = "Register STARTUP `reset()`'s with value 0"]
impl crate::ResetValue for super::STARTUP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_ENA`"]
pub type ADC_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_ENA`"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ADC_INIT`"]
pub type ADC_INIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_INIT`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
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
}
