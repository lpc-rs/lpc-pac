#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MMCTRL {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `MM_ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MM_ENAR {
    #[doc = "Monitor mode disabled."]
    MONITOR_MODE_DISABLE,
    #[doc = "The I2C module will enter monitor mode. In this mode the SDA output will be forced high. This will prevent the I2C module from outputting data of any kind (including ACK) onto the I 2C data bus. Depending on the state of the ENA_SCL bit, the output may be also forced high, preventing the module from having control over the I2C clock line."]
    THE_I2C_MODULE_WILL_,
}
impl crate::ToBits<bool> for MM_ENAR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MM_ENAR::MONITOR_MODE_DISABLE => false,
            MM_ENAR::THE_I2C_MODULE_WILL_ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MM_ENA_R = crate::FR<bool, MM_ENAR>;
impl MM_ENA_R {
    #[doc = "Checks if the value of the field is `MONITOR_MODE_DISABLE`"]
    #[inline(always)]
    pub fn is_monitor_mode_disable(&self) -> bool {
        *self == MM_ENAR::MONITOR_MODE_DISABLE
    }
    #[doc = "Checks if the value of the field is `THE_I2C_MODULE_WILL_`"]
    #[inline(always)]
    pub fn is_the_i2c_module_will_(&self) -> bool {
        *self == MM_ENAR::THE_I2C_MODULE_WILL_
    }
}
#[doc = "Values that can be written to the field `MM_ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MM_ENAW {
    #[doc = "Monitor mode disabled."]
    MONITOR_MODE_DISABLE,
    #[doc = "The I2C module will enter monitor mode. In this mode the SDA output will be forced high. This will prevent the I2C module from outputting data of any kind (including ACK) onto the I 2C data bus. Depending on the state of the ENA_SCL bit, the output may be also forced high, preventing the module from having control over the I2C clock line."]
    THE_I2C_MODULE_WILL_,
}
impl MM_ENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MM_ENAW::MONITOR_MODE_DISABLE => false,
            MM_ENAW::THE_I2C_MODULE_WILL_ => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MM_ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _MM_ENAW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MM_ENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Monitor mode disabled."]
    #[inline(always)]
    pub fn monitor_mode_disable(self) -> &'a mut W {
        self.variant(MM_ENAW::MONITOR_MODE_DISABLE)
    }
    #[doc = "The I2C module will enter monitor mode. In this mode the SDA output will be forced high. This will prevent the I2C module from outputting data of any kind (including ACK) onto the I 2C data bus. Depending on the state of the ENA_SCL bit, the output may be also forced high, preventing the module from having control over the I2C clock line."]
    #[inline(always)]
    pub fn the_i2c_module_will_(self) -> &'a mut W {
        self.variant(MM_ENAW::THE_I2C_MODULE_WILL_)
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
#[doc = "Possible values of the field `ENA_SCL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_SCLR {
    #[doc = "When this bit is cleared to 0, the SCL output will be forced high when the module is in monitor mode. As described above, this will prevent the module from having any control over the I2C clock line."]
    HIGH,
    #[doc = "When this bit is set, the I2C module may exercise the same control over the clock line that it would in normal operation. This means that, acting as a slave peripheral, the I2C module can stretch the clock line (hold it low) until it has had time to respond to an I2C interrupt.\\[1\\]"]
    NORMAL,
}
impl crate::ToBits<bool> for ENA_SCLR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ENA_SCLR::HIGH => false,
            ENA_SCLR::NORMAL => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ENA_SCL_R = crate::FR<bool, ENA_SCLR>;
impl ENA_SCL_R {
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == ENA_SCLR::HIGH
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ENA_SCLR::NORMAL
    }
}
#[doc = "Values that can be written to the field `ENA_SCL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_SCLW {
    #[doc = "When this bit is cleared to 0, the SCL output will be forced high when the module is in monitor mode. As described above, this will prevent the module from having any control over the I2C clock line."]
    HIGH,
    #[doc = "When this bit is set, the I2C module may exercise the same control over the clock line that it would in normal operation. This means that, acting as a slave peripheral, the I2C module can stretch the clock line (hold it low) until it has had time to respond to an I2C interrupt.\\[1\\]"]
    NORMAL,
}
impl ENA_SCLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ENA_SCLW::HIGH => false,
            ENA_SCLW::NORMAL => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ENA_SCLW<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_SCLW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA_SCLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "When this bit is cleared to 0, the SCL output will be forced high when the module is in monitor mode. As described above, this will prevent the module from having any control over the I2C clock line."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ENA_SCLW::HIGH)
    }
    #[doc = "When this bit is set, the I2C module may exercise the same control over the clock line that it would in normal operation. This means that, acting as a slave peripheral, the I2C module can stretch the clock line (hold it low) until it has had time to respond to an I2C interrupt.\\[1\\]"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(ENA_SCLW::NORMAL)
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
#[doc = "Possible values of the field `MATCH_ALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MATCH_ALLR {
    #[doc = "When this bit is cleared, an interrupt will only be generated when a match occurs to one of the (up-to) four address registers described above.   That is, the module will respond as a normal slave as far as address-recognition is concerned."]
    MATCH,
    #[doc = "When this bit is set to 1 and the I2C is in monitor mode, an interrupt will be generated on ANY address received. This will enable the part to monitor all traffic on the bus."]
    ANYADDRESS,
}
impl crate::ToBits<bool> for MATCH_ALLR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MATCH_ALLR::MATCH => false,
            MATCH_ALLR::ANYADDRESS => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MATCH_ALL_R = crate::FR<bool, MATCH_ALLR>;
impl MATCH_ALL_R {
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match_(&self) -> bool {
        *self == MATCH_ALLR::MATCH
    }
    #[doc = "Checks if the value of the field is `ANYADDRESS`"]
    #[inline(always)]
    pub fn is_anyaddress(&self) -> bool {
        *self == MATCH_ALLR::ANYADDRESS
    }
}
#[doc = "Values that can be written to the field `MATCH_ALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MATCH_ALLW {
    #[doc = "When this bit is cleared, an interrupt will only be generated when a match occurs to one of the (up-to) four address registers described above.   That is, the module will respond as a normal slave as far as address-recognition is concerned."]
    MATCH,
    #[doc = "When this bit is set to 1 and the I2C is in monitor mode, an interrupt will be generated on ANY address received. This will enable the part to monitor all traffic on the bus."]
    ANYADDRESS,
}
impl MATCH_ALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MATCH_ALLW::MATCH => false,
            MATCH_ALLW::ANYADDRESS => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MATCH_ALLW<'a> {
    w: &'a mut W,
}
impl<'a> _MATCH_ALLW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MATCH_ALLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "When this bit is cleared, an interrupt will only be generated when a match occurs to one of the (up-to) four address registers described above. That is, the module will respond as a normal slave as far as address-recognition is concerned."]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(MATCH_ALLW::MATCH)
    }
    #[doc = "When this bit is set to 1 and the I2C is in monitor mode, an interrupt will be generated on ANY address received. This will enable the part to monitor all traffic on the bus."]
    #[inline(always)]
    pub fn anyaddress(self) -> &'a mut W {
        self.variant(MATCH_ALLW::ANYADDRESS)
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
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Monitor mode enable."]
    #[inline(always)]
    pub fn mm_ena(&self) -> MM_ENA_R {
        MM_ENA_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - SCL output enable."]
    #[inline(always)]
    pub fn ena_scl(&self) -> ENA_SCL_R {
        ENA_SCL_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Select interrupt register match."]
    #[inline(always)]
    pub fn match_all(&self) -> MATCH_ALL_R {
        MATCH_ALL_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Monitor mode enable."]
    #[inline(always)]
    pub fn mm_ena(&mut self) -> _MM_ENAW {
        _MM_ENAW { w: self }
    }
    #[doc = "Bit 1 - SCL output enable."]
    #[inline(always)]
    pub fn ena_scl(&mut self) -> _ENA_SCLW {
        _ENA_SCLW { w: self }
    }
    #[doc = "Bit 2 - Select interrupt register match."]
    #[inline(always)]
    pub fn match_all(&mut self) -> _MATCH_ALLW {
        _MATCH_ALLW { w: self }
    }
}
