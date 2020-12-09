#[doc = "Reader of register CALIB"]
pub type R = crate::R<u32, super::CALIB>;
#[doc = "Writer for register CALIB"]
pub type W = crate::W<u32, super::CALIB>;
#[doc = "Register CALIB `reset()`'s with value 0x02"]
impl crate::ResetValue for super::CALIB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `CALIB`"]
pub type CALIB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CALIB`"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `CALREQD`"]
pub type CALREQD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CALREQD`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CALVALUE`"]
pub type CALVALUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CALVALUE`"]
pub struct CALVALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> CALVALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 2)) | (((value as u32) & 0x7f) << 2);
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
}
