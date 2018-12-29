#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRESETCTRL {
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
#[doc = "Possible values of the field `SSP0_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP0_RST_NR {
    #[doc = "Resets the SSP0 peripheral."]
    RESETS_THE_SSP0_PERI,
    #[doc = "SSP0 reset de-asserted."]
    SSP0_RESET_DE_ASSERT,
}
impl SSP0_RST_NR {
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
            SSP0_RST_NR::RESETS_THE_SSP0_PERI => false,
            SSP0_RST_NR::SSP0_RESET_DE_ASSERT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSP0_RST_NR {
        match value {
            false => SSP0_RST_NR::RESETS_THE_SSP0_PERI,
            true => SSP0_RST_NR::SSP0_RESET_DE_ASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `RESETS_THE_SSP0_PERI`"]
    #[inline]
    pub fn is_resets_the_ssp0_peri(&self) -> bool {
        *self == SSP0_RST_NR::RESETS_THE_SSP0_PERI
    }
    #[doc = "Checks if the value of the field is `SSP0_RESET_DE_ASSERT`"]
    #[inline]
    pub fn is_ssp0_reset_de_assert(&self) -> bool {
        *self == SSP0_RST_NR::SSP0_RESET_DE_ASSERT
    }
}
#[doc = "Possible values of the field `I2C_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_RST_NR {
    #[doc = "Resets the I2C peripheral."]
    RESETS_THE_I2C_PERIP,
    #[doc = "I2C reset de-asserted."]
    I2C_RESET_DE_ASSERTE,
}
impl I2C_RST_NR {
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
            I2C_RST_NR::RESETS_THE_I2C_PERIP => false,
            I2C_RST_NR::I2C_RESET_DE_ASSERTE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2C_RST_NR {
        match value {
            false => I2C_RST_NR::RESETS_THE_I2C_PERIP,
            true => I2C_RST_NR::I2C_RESET_DE_ASSERTE,
        }
    }
    #[doc = "Checks if the value of the field is `RESETS_THE_I2C_PERIP`"]
    #[inline]
    pub fn is_resets_the_i2c_perip(&self) -> bool {
        *self == I2C_RST_NR::RESETS_THE_I2C_PERIP
    }
    #[doc = "Checks if the value of the field is `I2C_RESET_DE_ASSERTE`"]
    #[inline]
    pub fn is_i2c_reset_de_asserte(&self) -> bool {
        *self == I2C_RST_NR::I2C_RESET_DE_ASSERTE
    }
}
#[doc = "Possible values of the field `SSP1_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP1_RST_NR {
    #[doc = "Resets the SSP1 peripheral."]
    RESETS_THE_SSP1_PERI,
    #[doc = "SSP1 reset de-asserted."]
    SSP1_RESET_DE_ASSERT,
}
impl SSP1_RST_NR {
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
            SSP1_RST_NR::RESETS_THE_SSP1_PERI => false,
            SSP1_RST_NR::SSP1_RESET_DE_ASSERT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSP1_RST_NR {
        match value {
            false => SSP1_RST_NR::RESETS_THE_SSP1_PERI,
            true => SSP1_RST_NR::SSP1_RESET_DE_ASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `RESETS_THE_SSP1_PERI`"]
    #[inline]
    pub fn is_resets_the_ssp1_peri(&self) -> bool {
        *self == SSP1_RST_NR::RESETS_THE_SSP1_PERI
    }
    #[doc = "Checks if the value of the field is `SSP1_RESET_DE_ASSERT`"]
    #[inline]
    pub fn is_ssp1_reset_de_assert(&self) -> bool {
        *self == SSP1_RST_NR::SSP1_RESET_DE_ASSERT
    }
}
#[doc = "Values that can be written to the field `SSP0_RST_N`"]
pub enum SSP0_RST_NW {
    #[doc = "Resets the SSP0 peripheral."]
    RESETS_THE_SSP0_PERI,
    #[doc = "SSP0 reset de-asserted."]
    SSP0_RESET_DE_ASSERT,
}
impl SSP0_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSP0_RST_NW::RESETS_THE_SSP0_PERI => false,
            SSP0_RST_NW::SSP0_RESET_DE_ASSERT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSP0_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _SSP0_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSP0_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Resets the SSP0 peripheral."]
    #[inline]
    pub fn resets_the_ssp0_peri(self) -> &'a mut W {
        self.variant(SSP0_RST_NW::RESETS_THE_SSP0_PERI)
    }
    #[doc = "SSP0 reset de-asserted."]
    #[inline]
    pub fn ssp0_reset_de_assert(self) -> &'a mut W {
        self.variant(SSP0_RST_NW::SSP0_RESET_DE_ASSERT)
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
#[doc = "Values that can be written to the field `I2C_RST_N`"]
pub enum I2C_RST_NW {
    #[doc = "Resets the I2C peripheral."]
    RESETS_THE_I2C_PERIP,
    #[doc = "I2C reset de-asserted."]
    I2C_RESET_DE_ASSERTE,
}
impl I2C_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2C_RST_NW::RESETS_THE_I2C_PERIP => false,
            I2C_RST_NW::I2C_RESET_DE_ASSERTE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2C_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Resets the I2C peripheral."]
    #[inline]
    pub fn resets_the_i2c_perip(self) -> &'a mut W {
        self.variant(I2C_RST_NW::RESETS_THE_I2C_PERIP)
    }
    #[doc = "I2C reset de-asserted."]
    #[inline]
    pub fn i2c_reset_de_asserte(self) -> &'a mut W {
        self.variant(I2C_RST_NW::I2C_RESET_DE_ASSERTE)
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
#[doc = "Values that can be written to the field `SSP1_RST_N`"]
pub enum SSP1_RST_NW {
    #[doc = "Resets the SSP1 peripheral."]
    RESETS_THE_SSP1_PERI,
    #[doc = "SSP1 reset de-asserted."]
    SSP1_RESET_DE_ASSERT,
}
impl SSP1_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSP1_RST_NW::RESETS_THE_SSP1_PERI => false,
            SSP1_RST_NW::SSP1_RESET_DE_ASSERT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSP1_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _SSP1_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSP1_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Resets the SSP1 peripheral."]
    #[inline]
    pub fn resets_the_ssp1_peri(self) -> &'a mut W {
        self.variant(SSP1_RST_NW::RESETS_THE_SSP1_PERI)
    }
    #[doc = "SSP1 reset de-asserted."]
    #[inline]
    pub fn ssp1_reset_de_assert(self) -> &'a mut W {
        self.variant(SSP1_RST_NW::SSP1_RESET_DE_ASSERT)
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
    #[doc = "Bit 0 - SSP0 reset control"]
    #[inline]
    pub fn ssp0_rst_n(&self) -> SSP0_RST_NR {
        SSP0_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - I2C reset control"]
    #[inline]
    pub fn i2c_rst_n(&self) -> I2C_RST_NR {
        I2C_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - SSP1 reset control"]
    #[inline]
    pub fn ssp1_rst_n(&self) -> SSP1_RST_NR {
        SSP1_RST_NR::_from({
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
    #[doc = "Bit 0 - SSP0 reset control"]
    #[inline]
    pub fn ssp0_rst_n(&mut self) -> _SSP0_RST_NW {
        _SSP0_RST_NW { w: self }
    }
    #[doc = "Bit 1 - I2C reset control"]
    #[inline]
    pub fn i2c_rst_n(&mut self) -> _I2C_RST_NW {
        _I2C_RST_NW { w: self }
    }
    #[doc = "Bit 2 - SSP1 reset control"]
    #[inline]
    pub fn ssp1_rst_n(&mut self) -> _SSP1_RST_NW {
        _SSP1_RST_NW { w: self }
    }
}
