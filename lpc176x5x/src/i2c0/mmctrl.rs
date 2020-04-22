#[doc = "Reader of register MMCTRL"]
pub type R = crate::R<u32, super::MMCTRL>;
#[doc = "Writer for register MMCTRL"]
pub type W = crate::W<u32, super::MMCTRL>;
#[doc = "Register MMCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::MMCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Monitor mode enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MM_ENA_A {
    #[doc = "0: Monitor mode disabled."]
    MONITOR_MODE_DISABLE = 0,
    #[doc = "1: The I 2C module will enter monitor mode. In this mode the SDA output will be forced high. This will prevent the I2C module from outputting data of any kind (including ACK) onto the I2C data bus. Depending on the state of the ENA_SCL bit, the output may be also forced high, preventing the module from having control over the I2C clock line."]
    THE_I_2C_MODULE_WILL = 1,
}
impl From<MM_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: MM_ENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MM_ENA`"]
pub type MM_ENA_R = crate::R<bool, MM_ENA_A>;
impl MM_ENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MM_ENA_A {
        match self.bits {
            false => MM_ENA_A::MONITOR_MODE_DISABLE,
            true => MM_ENA_A::THE_I_2C_MODULE_WILL,
        }
    }
    #[doc = "Checks if the value of the field is `MONITOR_MODE_DISABLE`"]
    #[inline(always)]
    pub fn is_monitor_mode_disable(&self) -> bool {
        *self == MM_ENA_A::MONITOR_MODE_DISABLE
    }
    #[doc = "Checks if the value of the field is `THE_I_2C_MODULE_WILL`"]
    #[inline(always)]
    pub fn is_the_i_2c_module_will(&self) -> bool {
        *self == MM_ENA_A::THE_I_2C_MODULE_WILL
    }
}
#[doc = "Write proxy for field `MM_ENA`"]
pub struct MM_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> MM_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MM_ENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Monitor mode disabled."]
    #[inline(always)]
    pub fn monitor_mode_disable(self) -> &'a mut W {
        self.variant(MM_ENA_A::MONITOR_MODE_DISABLE)
    }
    #[doc = "The I 2C module will enter monitor mode. In this mode the SDA output will be forced high. This will prevent the I2C module from outputting data of any kind (including ACK) onto the I2C data bus. Depending on the state of the ENA_SCL bit, the output may be also forced high, preventing the module from having control over the I2C clock line."]
    #[inline(always)]
    pub fn the_i_2c_module_will(self) -> &'a mut W {
        self.variant(MM_ENA_A::THE_I_2C_MODULE_WILL)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "SCL output enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_SCL_A {
    #[doc = "0: When this bit is cleared to 0, the SCL output will be forced high when the module is in monitor mode. As described above, this will prevent the module from having any control over the I2C clock line."]
    WHEN_THIS_BIT_IS_CLE = 0,
    #[doc = "1: When this bit is set, the I2C module may exercise the same control over the clock line that it would in normal operation. This means that, acting as a slave peripheral, the I2C module can stretch the clock line (hold it low) until it has had time to respond to an I2C interrupt.\\[1\\]"]
    WHEN_THIS_BIT_IS_SET = 1,
}
impl From<ENA_SCL_A> for bool {
    #[inline(always)]
    fn from(variant: ENA_SCL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENA_SCL`"]
pub type ENA_SCL_R = crate::R<bool, ENA_SCL_A>;
impl ENA_SCL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA_SCL_A {
        match self.bits {
            false => ENA_SCL_A::WHEN_THIS_BIT_IS_CLE,
            true => ENA_SCL_A::WHEN_THIS_BIT_IS_SET,
        }
    }
    #[doc = "Checks if the value of the field is `WHEN_THIS_BIT_IS_CLE`"]
    #[inline(always)]
    pub fn is_when_this_bit_is_cle(&self) -> bool {
        *self == ENA_SCL_A::WHEN_THIS_BIT_IS_CLE
    }
    #[doc = "Checks if the value of the field is `WHEN_THIS_BIT_IS_SET`"]
    #[inline(always)]
    pub fn is_when_this_bit_is_set(&self) -> bool {
        *self == ENA_SCL_A::WHEN_THIS_BIT_IS_SET
    }
}
#[doc = "Write proxy for field `ENA_SCL`"]
pub struct ENA_SCL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_SCL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA_SCL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "When this bit is cleared to 0, the SCL output will be forced high when the module is in monitor mode. As described above, this will prevent the module from having any control over the I2C clock line."]
    #[inline(always)]
    pub fn when_this_bit_is_cle(self) -> &'a mut W {
        self.variant(ENA_SCL_A::WHEN_THIS_BIT_IS_CLE)
    }
    #[doc = "When this bit is set, the I2C module may exercise the same control over the clock line that it would in normal operation. This means that, acting as a slave peripheral, the I2C module can stretch the clock line (hold it low) until it has had time to respond to an I2C interrupt.\\[1\\]"]
    #[inline(always)]
    pub fn when_this_bit_is_set(self) -> &'a mut W {
        self.variant(ENA_SCL_A::WHEN_THIS_BIT_IS_SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Select interrupt register match.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MATCH_ALL_A {
    #[doc = "0: When this bit is cleared, an interrupt will only be generated when a match occurs to one of the (up-to) four address registers described above.   That is, the module will respond as a normal slave as far as address-recognition is concerned."]
    WHEN_THIS_BIT_IS_CLE = 0,
    #[doc = "1: When this bit is set to 1 and the I2C is in monitor mode, an interrupt will be generated on ANY address received. This will enable the part to monitor all traffic on the bus."]
    WHEN_THIS_BIT_IS_SET = 1,
}
impl From<MATCH_ALL_A> for bool {
    #[inline(always)]
    fn from(variant: MATCH_ALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MATCH_ALL`"]
pub type MATCH_ALL_R = crate::R<bool, MATCH_ALL_A>;
impl MATCH_ALL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MATCH_ALL_A {
        match self.bits {
            false => MATCH_ALL_A::WHEN_THIS_BIT_IS_CLE,
            true => MATCH_ALL_A::WHEN_THIS_BIT_IS_SET,
        }
    }
    #[doc = "Checks if the value of the field is `WHEN_THIS_BIT_IS_CLE`"]
    #[inline(always)]
    pub fn is_when_this_bit_is_cle(&self) -> bool {
        *self == MATCH_ALL_A::WHEN_THIS_BIT_IS_CLE
    }
    #[doc = "Checks if the value of the field is `WHEN_THIS_BIT_IS_SET`"]
    #[inline(always)]
    pub fn is_when_this_bit_is_set(&self) -> bool {
        *self == MATCH_ALL_A::WHEN_THIS_BIT_IS_SET
    }
}
#[doc = "Write proxy for field `MATCH_ALL`"]
pub struct MATCH_ALL_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCH_ALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MATCH_ALL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "When this bit is cleared, an interrupt will only be generated when a match occurs to one of the (up-to) four address registers described above. That is, the module will respond as a normal slave as far as address-recognition is concerned."]
    #[inline(always)]
    pub fn when_this_bit_is_cle(self) -> &'a mut W {
        self.variant(MATCH_ALL_A::WHEN_THIS_BIT_IS_CLE)
    }
    #[doc = "When this bit is set to 1 and the I2C is in monitor mode, an interrupt will be generated on ANY address received. This will enable the part to monitor all traffic on the bus."]
    #[inline(always)]
    pub fn when_this_bit_is_set(self) -> &'a mut W {
        self.variant(MATCH_ALL_A::WHEN_THIS_BIT_IS_SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Monitor mode enable."]
    #[inline(always)]
    pub fn mm_ena(&self) -> MM_ENA_R {
        MM_ENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SCL output enable."]
    #[inline(always)]
    pub fn ena_scl(&self) -> ENA_SCL_R {
        ENA_SCL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Select interrupt register match."]
    #[inline(always)]
    pub fn match_all(&self) -> MATCH_ALL_R {
        MATCH_ALL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Monitor mode enable."]
    #[inline(always)]
    pub fn mm_ena(&mut self) -> MM_ENA_W {
        MM_ENA_W { w: self }
    }
    #[doc = "Bit 1 - SCL output enable."]
    #[inline(always)]
    pub fn ena_scl(&mut self) -> ENA_SCL_W {
        ENA_SCL_W { w: self }
    }
    #[doc = "Bit 2 - Select interrupt register match."]
    #[inline(always)]
    pub fn match_all(&mut self) -> MATCH_ALL_W {
        MATCH_ALL_W { w: self }
    }
}
