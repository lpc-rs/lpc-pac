#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::OTGCLKST {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `HOST_CLK_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOST_CLK_ONR {
    #[doc = "Host clock is not available."]
    HOST_CLOCK_IS_NOT_AV,
    #[doc = "Host clock is available."]
    HOST_CLOCK_IS_AVAILA,
}
impl HOST_CLK_ONR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            HOST_CLK_ONR::HOST_CLOCK_IS_NOT_AV => false,
            HOST_CLK_ONR::HOST_CLOCK_IS_AVAILA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HOST_CLK_ONR {
        match value {
            false => HOST_CLK_ONR::HOST_CLOCK_IS_NOT_AV,
            true => HOST_CLK_ONR::HOST_CLOCK_IS_AVAILA,
        }
    }
    #[doc = "Checks if the value of the field is `HOST_CLOCK_IS_NOT_AV`"]
    #[inline]
    pub fn is_host_clock_is_not_av(&self) -> bool {
        *self == HOST_CLK_ONR::HOST_CLOCK_IS_NOT_AV
    }
    #[doc = "Checks if the value of the field is `HOST_CLOCK_IS_AVAILA`"]
    #[inline]
    pub fn is_host_clock_is_availa(&self) -> bool {
        *self == HOST_CLK_ONR::HOST_CLOCK_IS_AVAILA
    }
}
#[doc = "Possible values of the field `DEV_CLK_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEV_CLK_ONR {
    #[doc = "Device clock is not available."]
    DEVICE_CLOCK_IS_NOT_,
    #[doc = "Device clock is available."]
    DEVICE_CLOCK_IS_AVAI,
}
impl DEV_CLK_ONR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DEV_CLK_ONR::DEVICE_CLOCK_IS_NOT_ => false,
            DEV_CLK_ONR::DEVICE_CLOCK_IS_AVAI => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEV_CLK_ONR {
        match value {
            false => DEV_CLK_ONR::DEVICE_CLOCK_IS_NOT_,
            true => DEV_CLK_ONR::DEVICE_CLOCK_IS_AVAI,
        }
    }
    #[doc = "Checks if the value of the field is `DEVICE_CLOCK_IS_NOT_`"]
    #[inline]
    pub fn is_device_clock_is_not_(&self) -> bool {
        *self == DEV_CLK_ONR::DEVICE_CLOCK_IS_NOT_
    }
    #[doc = "Checks if the value of the field is `DEVICE_CLOCK_IS_AVAI`"]
    #[inline]
    pub fn is_device_clock_is_avai(&self) -> bool {
        *self == DEV_CLK_ONR::DEVICE_CLOCK_IS_AVAI
    }
}
#[doc = "Possible values of the field `I2C_CLK_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_CLK_ONR {
    #[doc = "I2C clock is not available."]
    I2C_CLOCK_IS_NOT_AVA,
    #[doc = "I2C clock is available."]
    I2C_CLOCK_IS_AVAILAB,
}
impl I2C_CLK_ONR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            I2C_CLK_ONR::I2C_CLOCK_IS_NOT_AVA => false,
            I2C_CLK_ONR::I2C_CLOCK_IS_AVAILAB => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2C_CLK_ONR {
        match value {
            false => I2C_CLK_ONR::I2C_CLOCK_IS_NOT_AVA,
            true => I2C_CLK_ONR::I2C_CLOCK_IS_AVAILAB,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_CLOCK_IS_NOT_AVA`"]
    #[inline]
    pub fn is_i2c_clock_is_not_ava(&self) -> bool {
        *self == I2C_CLK_ONR::I2C_CLOCK_IS_NOT_AVA
    }
    #[doc = "Checks if the value of the field is `I2C_CLOCK_IS_AVAILAB`"]
    #[inline]
    pub fn is_i2c_clock_is_availab(&self) -> bool {
        *self == I2C_CLK_ONR::I2C_CLOCK_IS_AVAILAB
    }
}
#[doc = "Possible values of the field `OTG_CLK_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTG_CLK_ONR {
    #[doc = "OTG clock is not available."]
    OTG_CLOCK_IS_NOT_AVA,
    #[doc = "OTG clock is available."]
    OTG_CLOCK_IS_AVAILAB,
}
impl OTG_CLK_ONR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            OTG_CLK_ONR::OTG_CLOCK_IS_NOT_AVA => false,
            OTG_CLK_ONR::OTG_CLOCK_IS_AVAILAB => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OTG_CLK_ONR {
        match value {
            false => OTG_CLK_ONR::OTG_CLOCK_IS_NOT_AVA,
            true => OTG_CLK_ONR::OTG_CLOCK_IS_AVAILAB,
        }
    }
    #[doc = "Checks if the value of the field is `OTG_CLOCK_IS_NOT_AVA`"]
    #[inline]
    pub fn is_otg_clock_is_not_ava(&self) -> bool {
        *self == OTG_CLK_ONR::OTG_CLOCK_IS_NOT_AVA
    }
    #[doc = "Checks if the value of the field is `OTG_CLOCK_IS_AVAILAB`"]
    #[inline]
    pub fn is_otg_clock_is_availab(&self) -> bool {
        *self == OTG_CLK_ONR::OTG_CLOCK_IS_AVAILAB
    }
}
#[doc = "Possible values of the field `AHB_CLK_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_CLK_ONR {
    #[doc = "AHB clock is not available."]
    AHB_CLOCK_IS_NOT_AVA,
    #[doc = "AHB clock is available."]
    AHB_CLOCK_IS_AVAILAB,
}
impl AHB_CLK_ONR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AHB_CLK_ONR::AHB_CLOCK_IS_NOT_AVA => false,
            AHB_CLK_ONR::AHB_CLOCK_IS_AVAILAB => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AHB_CLK_ONR {
        match value {
            false => AHB_CLK_ONR::AHB_CLOCK_IS_NOT_AVA,
            true => AHB_CLK_ONR::AHB_CLOCK_IS_AVAILAB,
        }
    }
    #[doc = "Checks if the value of the field is `AHB_CLOCK_IS_NOT_AVA`"]
    #[inline]
    pub fn is_ahb_clock_is_not_ava(&self) -> bool {
        *self == AHB_CLK_ONR::AHB_CLOCK_IS_NOT_AVA
    }
    #[doc = "Checks if the value of the field is `AHB_CLOCK_IS_AVAILAB`"]
    #[inline]
    pub fn is_ahb_clock_is_availab(&self) -> bool {
        *self == AHB_CLK_ONR::AHB_CLOCK_IS_AVAILAB
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Host clock status."]
    #[inline]
    pub fn host_clk_on(&self) -> HOST_CLK_ONR {
        HOST_CLK_ONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Device clock status."]
    #[inline]
    pub fn dev_clk_on(&self) -> DEV_CLK_ONR {
        DEV_CLK_ONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - I2C clock status."]
    #[inline]
    pub fn i2c_clk_on(&self) -> I2C_CLK_ONR {
        I2C_CLK_ONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - OTG clock status."]
    #[inline]
    pub fn otg_clk_on(&self) -> OTG_CLK_ONR {
        OTG_CLK_ONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - AHB master clock status."]
    #[inline]
    pub fn ahb_clk_on(&self) -> AHB_CLK_ONR {
        AHB_CLK_ONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
