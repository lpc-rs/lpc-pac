#[doc = "Register `CALIB` reader"]
pub struct R(crate::R<CALIB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALIB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALIB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALIB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALIB` writer"]
pub struct W(crate::W<CALIB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALIB_SPEC>;
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
impl From<crate::W<CALIB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALIB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALIB` reader - Calibration request. Setting this bit will launch an ADC calibration cycle. This bit can only be set to a '1' by software. It is cleared automatically when the calibration cycle completes."]
pub struct CALIB_R(crate::FieldReader<bool, bool>);
impl CALIB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CALIB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALIB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALIB` writer - Calibration request. Setting this bit will launch an ADC calibration cycle. This bit can only be set to a '1' by software. It is cleared automatically when the calibration cycle completes."]
pub struct CALIB_W<'a> {
    w: &'a mut W,
}
impl<'a> CALIB_W<'a> {
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
#[doc = "Field `CALREQD` reader - Calibration required. This read-only bit indicates if calibration is required when enabling the ADC. CALREQD will be '1' if no calibration has been run since the chip was powered-up and if the BYPASSCAL bit in the CTRL register is low. Software will test this bit to determine whether to initiate a calibration cycle or whether to set the ADC_INIT bit (in the STARTUP register) to launch the ADC initialization process which includes a 'dummy' conversion cycle. Note: A 'dummy' conversion cycle requires approximately 6 ADC clocks as opposed to 81 clocks required for calibration."]
pub struct CALREQD_R(crate::FieldReader<bool, bool>);
impl CALREQD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CALREQD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALREQD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALREQD` writer - Calibration required. This read-only bit indicates if calibration is required when enabling the ADC. CALREQD will be '1' if no calibration has been run since the chip was powered-up and if the BYPASSCAL bit in the CTRL register is low. Software will test this bit to determine whether to initiate a calibration cycle or whether to set the ADC_INIT bit (in the STARTUP register) to launch the ADC initialization process which includes a 'dummy' conversion cycle. Note: A 'dummy' conversion cycle requires approximately 6 ADC clocks as opposed to 81 clocks required for calibration."]
pub struct CALREQD_W<'a> {
    w: &'a mut W,
}
impl<'a> CALREQD_W<'a> {
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
#[doc = "Field `CALVALUE` reader - Calibration Value. This read-only field displays the calibration value established during last calibration cycle. This value is not typically of any use to the user."]
pub struct CALVALUE_R(crate::FieldReader<u8, u8>);
impl CALVALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CALVALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALVALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALVALUE` writer - Calibration Value. This read-only field displays the calibration value established during last calibration cycle. This value is not typically of any use to the user."]
pub struct CALVALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> CALVALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 2)) | ((value as u32 & 0x7f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Calibration request. Setting this bit will launch an ADC calibration cycle. This bit can only be set to a '1' by software. It is cleared automatically when the calibration cycle completes."]
    #[inline(always)]
    pub fn calib(&self) -> CALIB_R {
        CALIB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Calibration required. This read-only bit indicates if calibration is required when enabling the ADC. CALREQD will be '1' if no calibration has been run since the chip was powered-up and if the BYPASSCAL bit in the CTRL register is low. Software will test this bit to determine whether to initiate a calibration cycle or whether to set the ADC_INIT bit (in the STARTUP register) to launch the ADC initialization process which includes a 'dummy' conversion cycle. Note: A 'dummy' conversion cycle requires approximately 6 ADC clocks as opposed to 81 clocks required for calibration."]
    #[inline(always)]
    pub fn calreqd(&self) -> CALREQD_R {
        CALREQD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:8 - Calibration Value. This read-only field displays the calibration value established during last calibration cycle. This value is not typically of any use to the user."]
    #[inline(always)]
    pub fn calvalue(&self) -> CALVALUE_R {
        CALVALUE_R::new(((self.bits >> 2) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Calibration request. Setting this bit will launch an ADC calibration cycle. This bit can only be set to a '1' by software. It is cleared automatically when the calibration cycle completes."]
    #[inline(always)]
    pub fn calib(&mut self) -> CALIB_W {
        CALIB_W { w: self }
    }
    #[doc = "Bit 1 - Calibration required. This read-only bit indicates if calibration is required when enabling the ADC. CALREQD will be '1' if no calibration has been run since the chip was powered-up and if the BYPASSCAL bit in the CTRL register is low. Software will test this bit to determine whether to initiate a calibration cycle or whether to set the ADC_INIT bit (in the STARTUP register) to launch the ADC initialization process which includes a 'dummy' conversion cycle. Note: A 'dummy' conversion cycle requires approximately 6 ADC clocks as opposed to 81 clocks required for calibration."]
    #[inline(always)]
    pub fn calreqd(&mut self) -> CALREQD_W {
        CALREQD_W { w: self }
    }
    #[doc = "Bits 2:8 - Calibration Value. This read-only field displays the calibration value established during last calibration cycle. This value is not typically of any use to the user."]
    #[inline(always)]
    pub fn calvalue(&mut self) -> CALVALUE_W {
        CALVALUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Calibration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calib](index.html) module"]
pub struct CALIB_SPEC;
impl crate::RegisterSpec for CALIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calib::R](R) reader structure"]
impl crate::Readable for CALIB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calib::W](W) writer structure"]
impl crate::Writable for CALIB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALIB to value 0x02"]
impl crate::Resettable for CALIB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
