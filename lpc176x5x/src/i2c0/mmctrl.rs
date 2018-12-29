#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MMCTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `MM_ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MM_ENAR {
    #[doc = "Monitor mode disabled."]
    MONITOR_MODE_DISABLE,
    #[doc = "The I 2C module will enter monitor mode. In this mode the SDA output will be forced high. This will prevent the I2C module from outputting data of any kind (including ACK) onto the I2C data bus. Depending on the state of the ENA_SCL bit, the output may be also forced high, preventing the module from having control over the I2C clock line."]
    THE_I_2C_MODULE_WILL,
}
impl MM_ENAR {
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
            MM_ENAR::MONITOR_MODE_DISABLE => false,
            MM_ENAR::THE_I_2C_MODULE_WILL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MM_ENAR {
        match value {
            false => MM_ENAR::MONITOR_MODE_DISABLE,
            true => MM_ENAR::THE_I_2C_MODULE_WILL,
        }
    }
    #[doc = "Checks if the value of the field is `MONITOR_MODE_DISABLE`"]
    #[inline]
    pub fn is_monitor_mode_disable(&self) -> bool {
        *self == MM_ENAR::MONITOR_MODE_DISABLE
    }
    #[doc = "Checks if the value of the field is `THE_I_2C_MODULE_WILL`"]
    #[inline]
    pub fn is_the_i_2c_module_will(&self) -> bool {
        *self == MM_ENAR::THE_I_2C_MODULE_WILL
    }
}
#[doc = "Possible values of the field `ENA_SCL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_SCLR {
    #[doc = "When this bit is cleared to 0, the SCL output will be forced high when the module is in monitor mode. As described above, this will prevent the module from having any control over the I2C clock line."]
    WHEN_THIS_BIT_IS_CLE,
    #[doc = "When this bit is set, the I2C module may exercise the same control over the clock line that it would in normal operation. This means that, acting as a slave peripheral, the I2C module can stretch the clock line (hold it low) until it has had time to respond to an I2C interrupt.\\[1\\]"]
    WHEN_THIS_BIT_IS_SET,
}
impl ENA_SCLR {
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
            ENA_SCLR::WHEN_THIS_BIT_IS_CLE => false,
            ENA_SCLR::WHEN_THIS_BIT_IS_SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENA_SCLR {
        match value {
            false => ENA_SCLR::WHEN_THIS_BIT_IS_CLE,
            true => ENA_SCLR::WHEN_THIS_BIT_IS_SET,
        }
    }
    #[doc = "Checks if the value of the field is `WHEN_THIS_BIT_IS_CLE`"]
    #[inline]
    pub fn is_when_this_bit_is_cle(&self) -> bool {
        *self == ENA_SCLR::WHEN_THIS_BIT_IS_CLE
    }
    #[doc = "Checks if the value of the field is `WHEN_THIS_BIT_IS_SET`"]
    #[inline]
    pub fn is_when_this_bit_is_set(&self) -> bool {
        *self == ENA_SCLR::WHEN_THIS_BIT_IS_SET
    }
}
#[doc = "Possible values of the field `MATCH_ALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MATCH_ALLR {
    #[doc = "When this bit is cleared, an interrupt will only be generated when a match occurs to one of the (up-to) four address registers described above.   That is, the module will respond as a normal slave as far as address-recognition is concerned."]
    WHEN_THIS_BIT_IS_CLE,
    #[doc = "When this bit is set to 1 and the I2C is in monitor mode, an interrupt will be generated on ANY address received. This will enable the part to monitor all traffic on the bus."]
    WHEN_THIS_BIT_IS_SET,
}
impl MATCH_ALLR {
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
            MATCH_ALLR::WHEN_THIS_BIT_IS_CLE => false,
            MATCH_ALLR::WHEN_THIS_BIT_IS_SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MATCH_ALLR {
        match value {
            false => MATCH_ALLR::WHEN_THIS_BIT_IS_CLE,
            true => MATCH_ALLR::WHEN_THIS_BIT_IS_SET,
        }
    }
    #[doc = "Checks if the value of the field is `WHEN_THIS_BIT_IS_CLE`"]
    #[inline]
    pub fn is_when_this_bit_is_cle(&self) -> bool {
        *self == MATCH_ALLR::WHEN_THIS_BIT_IS_CLE
    }
    #[doc = "Checks if the value of the field is `WHEN_THIS_BIT_IS_SET`"]
    #[inline]
    pub fn is_when_this_bit_is_set(&self) -> bool {
        *self == MATCH_ALLR::WHEN_THIS_BIT_IS_SET
    }
}
#[doc = "Values that can be written to the field `MM_ENA`"]
pub enum MM_ENAW {
    #[doc = "Monitor mode disabled."]
    MONITOR_MODE_DISABLE,
    #[doc = "The I 2C module will enter monitor mode. In this mode the SDA output will be forced high. This will prevent the I2C module from outputting data of any kind (including ACK) onto the I2C data bus. Depending on the state of the ENA_SCL bit, the output may be also forced high, preventing the module from having control over the I2C clock line."]
    THE_I_2C_MODULE_WILL,
}
impl MM_ENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MM_ENAW::MONITOR_MODE_DISABLE => false,
            MM_ENAW::THE_I_2C_MODULE_WILL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MM_ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _MM_ENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MM_ENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Monitor mode disabled."]
    #[inline]
    pub fn monitor_mode_disable(self) -> &'a mut W {
        self.variant(MM_ENAW::MONITOR_MODE_DISABLE)
    }
    #[doc = "The I 2C module will enter monitor mode. In this mode the SDA output will be forced high. This will prevent the I2C module from outputting data of any kind (including ACK) onto the I2C data bus. Depending on the state of the ENA_SCL bit, the output may be also forced high, preventing the module from having control over the I2C clock line."]
    #[inline]
    pub fn the_i_2c_module_will(self) -> &'a mut W {
        self.variant(MM_ENAW::THE_I_2C_MODULE_WILL)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENA_SCL`"]
pub enum ENA_SCLW {
    #[doc = "When this bit is cleared to 0, the SCL output will be forced high when the module is in monitor mode. As described above, this will prevent the module from having any control over the I2C clock line."]
    WHEN_THIS_BIT_IS_CLE,
    #[doc = "When this bit is set, the I2C module may exercise the same control over the clock line that it would in normal operation. This means that, acting as a slave peripheral, the I2C module can stretch the clock line (hold it low) until it has had time to respond to an I2C interrupt.\\[1\\]"]
    WHEN_THIS_BIT_IS_SET,
}
impl ENA_SCLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENA_SCLW::WHEN_THIS_BIT_IS_CLE => false,
            ENA_SCLW::WHEN_THIS_BIT_IS_SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENA_SCLW<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_SCLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENA_SCLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "When this bit is cleared to 0, the SCL output will be forced high when the module is in monitor mode. As described above, this will prevent the module from having any control over the I2C clock line."]
    #[inline]
    pub fn when_this_bit_is_cle(self) -> &'a mut W {
        self.variant(ENA_SCLW::WHEN_THIS_BIT_IS_CLE)
    }
    #[doc = "When this bit is set, the I2C module may exercise the same control over the clock line that it would in normal operation. This means that, acting as a slave peripheral, the I2C module can stretch the clock line (hold it low) until it has had time to respond to an I2C interrupt.\\[1\\]"]
    #[inline]
    pub fn when_this_bit_is_set(self) -> &'a mut W {
        self.variant(ENA_SCLW::WHEN_THIS_BIT_IS_SET)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MATCH_ALL`"]
pub enum MATCH_ALLW {
    #[doc = "When this bit is cleared, an interrupt will only be generated when a match occurs to one of the (up-to) four address registers described above.   That is, the module will respond as a normal slave as far as address-recognition is concerned."]
    WHEN_THIS_BIT_IS_CLE,
    #[doc = "When this bit is set to 1 and the I2C is in monitor mode, an interrupt will be generated on ANY address received. This will enable the part to monitor all traffic on the bus."]
    WHEN_THIS_BIT_IS_SET,
}
impl MATCH_ALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MATCH_ALLW::WHEN_THIS_BIT_IS_CLE => false,
            MATCH_ALLW::WHEN_THIS_BIT_IS_SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MATCH_ALLW<'a> {
    w: &'a mut W,
}
impl<'a> _MATCH_ALLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MATCH_ALLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "When this bit is cleared, an interrupt will only be generated when a match occurs to one of the (up-to) four address registers described above. That is, the module will respond as a normal slave as far as address-recognition is concerned."]
    #[inline]
    pub fn when_this_bit_is_cle(self) -> &'a mut W {
        self.variant(MATCH_ALLW::WHEN_THIS_BIT_IS_CLE)
    }
    #[doc = "When this bit is set to 1 and the I2C is in monitor mode, an interrupt will be generated on ANY address received. This will enable the part to monitor all traffic on the bus."]
    #[inline]
    pub fn when_this_bit_is_set(self) -> &'a mut W {
        self.variant(MATCH_ALLW::WHEN_THIS_BIT_IS_SET)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Monitor mode enable."]
    #[inline]
    pub fn mm_ena(&self) -> MM_ENAR {
        MM_ENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - SCL output enable."]
    #[inline]
    pub fn ena_scl(&self) -> ENA_SCLR {
        ENA_SCLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Select interrupt register match."]
    #[inline]
    pub fn match_all(&self) -> MATCH_ALLR {
        MATCH_ALLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Monitor mode enable."]
    #[inline]
    pub fn mm_ena(&mut self) -> _MM_ENAW {
        _MM_ENAW { w: self }
    }
    #[doc = "Bit 1 - SCL output enable."]
    #[inline]
    pub fn ena_scl(&mut self) -> _ENA_SCLW {
        _ENA_SCLW { w: self }
    }
    #[doc = "Bit 2 - Select interrupt register match."]
    #[inline]
    pub fn match_all(&mut self) -> _MATCH_ALLW {
        _MATCH_ALLW { w: self }
    }
}
