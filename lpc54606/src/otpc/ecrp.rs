#[doc = "Register `ECRP` reader"]
pub struct R(crate::R<ECRP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECRP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ECRP_SPEC>> for R {
    fn from(reader: crate::R<ECRP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CRP_MASS_ERASE_DISABLE` reader - Disable or enable CRP mass erase."]
pub struct CRP_MASS_ERASE_DISABLE_R(crate::FieldReader<bool, bool>);
impl CRP_MASS_ERASE_DISABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRP_MASS_ERASE_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRP_MASS_ERASE_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IAP_PROTECTION_ENABLE` reader - This bit controls the ability to enable checking for ECRP in IAP functions."]
pub struct IAP_PROTECTION_ENABLE_R(crate::FieldReader<bool, bool>);
impl IAP_PROTECTION_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IAP_PROTECTION_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IAP_PROTECTION_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRP_ISP_DISABLE_PIN` reader - This bit controls the ability to enter ISP mode using the ISP pin."]
pub struct CRP_ISP_DISABLE_PIN_R(crate::FieldReader<bool, bool>);
impl CRP_ISP_DISABLE_PIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRP_ISP_DISABLE_PIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRP_ISP_DISABLE_PIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRP_ISP_DISABLE_IAP` reader - This bit controls the ability to re-invoke ISP using IAP routines."]
pub struct CRP_ISP_DISABLE_IAP_R(crate::FieldReader<bool, bool>);
impl CRP_ISP_DISABLE_IAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRP_ISP_DISABLE_IAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRP_ISP_DISABLE_IAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRP_ALLOW_ZERO` reader - This bit controls how 0 is treated when read as a ECRP value.."]
pub struct CRP_ALLOW_ZERO_R(crate::FieldReader<bool, bool>);
impl CRP_ALLOW_ZERO_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRP_ALLOW_ZERO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRP_ALLOW_ZERO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JTAG_DISABLE` reader - 0 => Enable SWD/JTAG; 1 => Disable SWD/JTAG.."]
pub struct JTAG_DISABLE_R(crate::FieldReader<bool, bool>);
impl JTAG_DISABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        JTAG_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JTAG_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 4 - Disable or enable CRP mass erase."]
    #[inline(always)]
    pub fn crp_mass_erase_disable(&self) -> CRP_MASS_ERASE_DISABLE_R {
        CRP_MASS_ERASE_DISABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit controls the ability to enable checking for ECRP in IAP functions."]
    #[inline(always)]
    pub fn iap_protection_enable(&self) -> IAP_PROTECTION_ENABLE_R {
        IAP_PROTECTION_ENABLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit controls the ability to enter ISP mode using the ISP pin."]
    #[inline(always)]
    pub fn crp_isp_disable_pin(&self) -> CRP_ISP_DISABLE_PIN_R {
        CRP_ISP_DISABLE_PIN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit controls the ability to re-invoke ISP using IAP routines."]
    #[inline(always)]
    pub fn crp_isp_disable_iap(&self) -> CRP_ISP_DISABLE_IAP_R {
        CRP_ISP_DISABLE_IAP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - This bit controls how 0 is treated when read as a ECRP value.."]
    #[inline(always)]
    pub fn crp_allow_zero(&self) -> CRP_ALLOW_ZERO_R {
        CRP_ALLOW_ZERO_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 31 - 0 => Enable SWD/JTAG; 1 => Disable SWD/JTAG.."]
    #[inline(always)]
    pub fn jtag_disable(&self) -> JTAG_DISABLE_R {
        JTAG_DISABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "ECRP options.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecrp](index.html) module"]
pub struct ECRP_SPEC;
impl crate::RegisterSpec for ECRP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecrp::R](R) reader structure"]
impl crate::Readable for ECRP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ECRP to value 0"]
impl crate::Resettable for ECRP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
