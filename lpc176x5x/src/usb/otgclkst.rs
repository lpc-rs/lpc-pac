#[doc = "Reader of register OTGCLKST"]
pub type R = crate::R<u32, super::OTGCLKST>;
#[doc = "Host clock status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOST_CLK_ON_A {
    #[doc = "0: Host clock is not available."]
    HOST_CLOCK_IS_NOT_AV = 0,
    #[doc = "1: Host clock is available."]
    HOST_CLOCK_IS_AVAILA = 1,
}
impl From<HOST_CLK_ON_A> for bool {
    #[inline(always)]
    fn from(variant: HOST_CLK_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HOST_CLK_ON`"]
pub type HOST_CLK_ON_R = crate::R<bool, HOST_CLK_ON_A>;
impl HOST_CLK_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOST_CLK_ON_A {
        match self.bits {
            false => HOST_CLK_ON_A::HOST_CLOCK_IS_NOT_AV,
            true => HOST_CLK_ON_A::HOST_CLOCK_IS_AVAILA,
        }
    }
    #[doc = "Checks if the value of the field is `HOST_CLOCK_IS_NOT_AV`"]
    #[inline(always)]
    pub fn is_host_clock_is_not_av(&self) -> bool {
        *self == HOST_CLK_ON_A::HOST_CLOCK_IS_NOT_AV
    }
    #[doc = "Checks if the value of the field is `HOST_CLOCK_IS_AVAILA`"]
    #[inline(always)]
    pub fn is_host_clock_is_availa(&self) -> bool {
        *self == HOST_CLK_ON_A::HOST_CLOCK_IS_AVAILA
    }
}
#[doc = "Device clock status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEV_CLK_ON_A {
    #[doc = "0: Device clock is not available."]
    DEVICE_CLOCK_IS_NOT_ = 0,
    #[doc = "1: Device clock is available."]
    DEVICE_CLOCK_IS_AVAI = 1,
}
impl From<DEV_CLK_ON_A> for bool {
    #[inline(always)]
    fn from(variant: DEV_CLK_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEV_CLK_ON`"]
pub type DEV_CLK_ON_R = crate::R<bool, DEV_CLK_ON_A>;
impl DEV_CLK_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEV_CLK_ON_A {
        match self.bits {
            false => DEV_CLK_ON_A::DEVICE_CLOCK_IS_NOT_,
            true => DEV_CLK_ON_A::DEVICE_CLOCK_IS_AVAI,
        }
    }
    #[doc = "Checks if the value of the field is `DEVICE_CLOCK_IS_NOT_`"]
    #[inline(always)]
    pub fn is_device_clock_is_not_(&self) -> bool {
        *self == DEV_CLK_ON_A::DEVICE_CLOCK_IS_NOT_
    }
    #[doc = "Checks if the value of the field is `DEVICE_CLOCK_IS_AVAI`"]
    #[inline(always)]
    pub fn is_device_clock_is_avai(&self) -> bool {
        *self == DEV_CLK_ON_A::DEVICE_CLOCK_IS_AVAI
    }
}
#[doc = "I2C clock status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_CLK_ON_A {
    #[doc = "0: I2C clock is not available."]
    I2C_CLOCK_IS_NOT_AVA = 0,
    #[doc = "1: I2C clock is available."]
    I2C_CLOCK_IS_AVAILAB = 1,
}
impl From<I2C_CLK_ON_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_CLK_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C_CLK_ON`"]
pub type I2C_CLK_ON_R = crate::R<bool, I2C_CLK_ON_A>;
impl I2C_CLK_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_CLK_ON_A {
        match self.bits {
            false => I2C_CLK_ON_A::I2C_CLOCK_IS_NOT_AVA,
            true => I2C_CLK_ON_A::I2C_CLOCK_IS_AVAILAB,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_CLOCK_IS_NOT_AVA`"]
    #[inline(always)]
    pub fn is_i2c_clock_is_not_ava(&self) -> bool {
        *self == I2C_CLK_ON_A::I2C_CLOCK_IS_NOT_AVA
    }
    #[doc = "Checks if the value of the field is `I2C_CLOCK_IS_AVAILAB`"]
    #[inline(always)]
    pub fn is_i2c_clock_is_availab(&self) -> bool {
        *self == I2C_CLK_ON_A::I2C_CLOCK_IS_AVAILAB
    }
}
#[doc = "OTG clock status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTG_CLK_ON_A {
    #[doc = "0: OTG clock is not available."]
    OTG_CLOCK_IS_NOT_AVA = 0,
    #[doc = "1: OTG clock is available."]
    OTG_CLOCK_IS_AVAILAB = 1,
}
impl From<OTG_CLK_ON_A> for bool {
    #[inline(always)]
    fn from(variant: OTG_CLK_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OTG_CLK_ON`"]
pub type OTG_CLK_ON_R = crate::R<bool, OTG_CLK_ON_A>;
impl OTG_CLK_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OTG_CLK_ON_A {
        match self.bits {
            false => OTG_CLK_ON_A::OTG_CLOCK_IS_NOT_AVA,
            true => OTG_CLK_ON_A::OTG_CLOCK_IS_AVAILAB,
        }
    }
    #[doc = "Checks if the value of the field is `OTG_CLOCK_IS_NOT_AVA`"]
    #[inline(always)]
    pub fn is_otg_clock_is_not_ava(&self) -> bool {
        *self == OTG_CLK_ON_A::OTG_CLOCK_IS_NOT_AVA
    }
    #[doc = "Checks if the value of the field is `OTG_CLOCK_IS_AVAILAB`"]
    #[inline(always)]
    pub fn is_otg_clock_is_availab(&self) -> bool {
        *self == OTG_CLK_ON_A::OTG_CLOCK_IS_AVAILAB
    }
}
#[doc = "AHB master clock status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_CLK_ON_A {
    #[doc = "0: AHB clock is not available."]
    AHB_CLOCK_IS_NOT_AVA = 0,
    #[doc = "1: AHB clock is available."]
    AHB_CLOCK_IS_AVAILAB = 1,
}
impl From<AHB_CLK_ON_A> for bool {
    #[inline(always)]
    fn from(variant: AHB_CLK_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AHB_CLK_ON`"]
pub type AHB_CLK_ON_R = crate::R<bool, AHB_CLK_ON_A>;
impl AHB_CLK_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_CLK_ON_A {
        match self.bits {
            false => AHB_CLK_ON_A::AHB_CLOCK_IS_NOT_AVA,
            true => AHB_CLK_ON_A::AHB_CLOCK_IS_AVAILAB,
        }
    }
    #[doc = "Checks if the value of the field is `AHB_CLOCK_IS_NOT_AVA`"]
    #[inline(always)]
    pub fn is_ahb_clock_is_not_ava(&self) -> bool {
        *self == AHB_CLK_ON_A::AHB_CLOCK_IS_NOT_AVA
    }
    #[doc = "Checks if the value of the field is `AHB_CLOCK_IS_AVAILAB`"]
    #[inline(always)]
    pub fn is_ahb_clock_is_availab(&self) -> bool {
        *self == AHB_CLK_ON_A::AHB_CLOCK_IS_AVAILAB
    }
}
impl R {
    #[doc = "Bit 0 - Host clock status."]
    #[inline(always)]
    pub fn host_clk_on(&self) -> HOST_CLK_ON_R {
        HOST_CLK_ON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Device clock status."]
    #[inline(always)]
    pub fn dev_clk_on(&self) -> DEV_CLK_ON_R {
        DEV_CLK_ON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C clock status."]
    #[inline(always)]
    pub fn i2c_clk_on(&self) -> I2C_CLK_ON_R {
        I2C_CLK_ON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OTG clock status."]
    #[inline(always)]
    pub fn otg_clk_on(&self) -> OTG_CLK_ON_R {
        OTG_CLK_ON_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AHB master clock status."]
    #[inline(always)]
    pub fn ahb_clk_on(&self) -> AHB_CLK_ON_R {
        AHB_CLK_ON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
