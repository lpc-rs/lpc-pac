#[doc = "Reader of register ECRP"]
pub type R = crate::R<u32, super::ECRP>;
#[doc = "Reader of field `CRP_MASS_ERASE_DISABLE`"]
pub type CRP_MASS_ERASE_DISABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `IAP_PROTECTION_ENABLE`"]
pub type IAP_PROTECTION_ENABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRP_ISP_DISABLE_PIN`"]
pub type CRP_ISP_DISABLE_PIN_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRP_ISP_DISABLE_IAP`"]
pub type CRP_ISP_DISABLE_IAP_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRP_ALLOW_ZERO`"]
pub type CRP_ALLOW_ZERO_R = crate::R<bool, bool>;
#[doc = "Reader of field `JTAG_DISABLE`"]
pub type JTAG_DISABLE_R = crate::R<bool, bool>;
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
