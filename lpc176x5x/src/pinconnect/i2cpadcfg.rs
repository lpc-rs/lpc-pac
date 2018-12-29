#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::I2CPADCFG {
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
#[doc = "Possible values of the field `SDADRV0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDADRV0R {
    #[doc = "Standard. The SDA0 pin is in the standard drive mode."]
    STANDARD,
    #[doc = "Fast-mode plus. The SDA0 pin is in Fast Mode Plus drive mode."]
    FAST_MODE_PLUS,
}
impl SDADRV0R {
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
            SDADRV0R::STANDARD => false,
            SDADRV0R::FAST_MODE_PLUS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDADRV0R {
        match value {
            false => SDADRV0R::STANDARD,
            true => SDADRV0R::FAST_MODE_PLUS,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline]
    pub fn is_standard(&self) -> bool {
        *self == SDADRV0R::STANDARD
    }
    #[doc = "Checks if the value of the field is `FAST_MODE_PLUS`"]
    #[inline]
    pub fn is_fast_mode_plus(&self) -> bool {
        *self == SDADRV0R::FAST_MODE_PLUS
    }
}
#[doc = "Possible values of the field `SDAI2C0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDAI2C0R {
    #[doc = "Enabled. The SDA0 pin has I2C glitch filtering and slew rate control enabled."]
    ENABLED,
    #[doc = "Disabled. The SDA0 pin has I2C glitch filtering and slew rate control disabled."]
    DISABLED,
}
impl SDAI2C0R {
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
            SDAI2C0R::ENABLED => false,
            SDAI2C0R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDAI2C0R {
        match value {
            false => SDAI2C0R::ENABLED,
            true => SDAI2C0R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SDAI2C0R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SDAI2C0R::DISABLED
    }
}
#[doc = "Possible values of the field `SCLDRV0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLDRV0R {
    #[doc = "Standard. The SCL0 pin is in the standard drive mode."]
    STANDARD,
    #[doc = "Fast-mode plus. The SCL0 pin is in Fast Mode Plus drive mode."]
    FAST_MODE_PLUS,
}
impl SCLDRV0R {
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
            SCLDRV0R::STANDARD => false,
            SCLDRV0R::FAST_MODE_PLUS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCLDRV0R {
        match value {
            false => SCLDRV0R::STANDARD,
            true => SCLDRV0R::FAST_MODE_PLUS,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline]
    pub fn is_standard(&self) -> bool {
        *self == SCLDRV0R::STANDARD
    }
    #[doc = "Checks if the value of the field is `FAST_MODE_PLUS`"]
    #[inline]
    pub fn is_fast_mode_plus(&self) -> bool {
        *self == SCLDRV0R::FAST_MODE_PLUS
    }
}
#[doc = "Possible values of the field `SCLI2C0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLI2C0R {
    #[doc = "Enabled. The SCL0 pin has I2C glitch filtering and slew rate control enabled."]
    ENABLED,
    #[doc = "Disabled. The SCL0 pin has I2C glitch filtering and slew rate control disabled."]
    DISABLED,
}
impl SCLI2C0R {
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
            SCLI2C0R::ENABLED => false,
            SCLI2C0R::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCLI2C0R {
        match value {
            false => SCLI2C0R::ENABLED,
            true => SCLI2C0R::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SCLI2C0R::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SCLI2C0R::DISABLED
    }
}
#[doc = "Values that can be written to the field `SDADRV0`"]
pub enum SDADRV0W {
    #[doc = "Standard. The SDA0 pin is in the standard drive mode."]
    STANDARD,
    #[doc = "Fast-mode plus. The SDA0 pin is in Fast Mode Plus drive mode."]
    FAST_MODE_PLUS,
}
impl SDADRV0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDADRV0W::STANDARD => false,
            SDADRV0W::FAST_MODE_PLUS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDADRV0W<'a> {
    w: &'a mut W,
}
impl<'a> _SDADRV0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDADRV0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard. The SDA0 pin is in the standard drive mode."]
    #[inline]
    pub fn standard(self) -> &'a mut W {
        self.variant(SDADRV0W::STANDARD)
    }
    #[doc = "Fast-mode plus. The SDA0 pin is in Fast Mode Plus drive mode."]
    #[inline]
    pub fn fast_mode_plus(self) -> &'a mut W {
        self.variant(SDADRV0W::FAST_MODE_PLUS)
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
#[doc = "Values that can be written to the field `SDAI2C0`"]
pub enum SDAI2C0W {
    #[doc = "Enabled. The SDA0 pin has I2C glitch filtering and slew rate control enabled."]
    ENABLED,
    #[doc = "Disabled. The SDA0 pin has I2C glitch filtering and slew rate control disabled."]
    DISABLED,
}
impl SDAI2C0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDAI2C0W::ENABLED => false,
            SDAI2C0W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDAI2C0W<'a> {
    w: &'a mut W,
}
impl<'a> _SDAI2C0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDAI2C0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled. The SDA0 pin has I2C glitch filtering and slew rate control enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SDAI2C0W::ENABLED)
    }
    #[doc = "Disabled. The SDA0 pin has I2C glitch filtering and slew rate control disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SDAI2C0W::DISABLED)
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
#[doc = "Values that can be written to the field `SCLDRV0`"]
pub enum SCLDRV0W {
    #[doc = "Standard. The SCL0 pin is in the standard drive mode."]
    STANDARD,
    #[doc = "Fast-mode plus. The SCL0 pin is in Fast Mode Plus drive mode."]
    FAST_MODE_PLUS,
}
impl SCLDRV0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCLDRV0W::STANDARD => false,
            SCLDRV0W::FAST_MODE_PLUS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCLDRV0W<'a> {
    w: &'a mut W,
}
impl<'a> _SCLDRV0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCLDRV0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard. The SCL0 pin is in the standard drive mode."]
    #[inline]
    pub fn standard(self) -> &'a mut W {
        self.variant(SCLDRV0W::STANDARD)
    }
    #[doc = "Fast-mode plus. The SCL0 pin is in Fast Mode Plus drive mode."]
    #[inline]
    pub fn fast_mode_plus(self) -> &'a mut W {
        self.variant(SCLDRV0W::FAST_MODE_PLUS)
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
#[doc = "Values that can be written to the field `SCLI2C0`"]
pub enum SCLI2C0W {
    #[doc = "Enabled. The SCL0 pin has I2C glitch filtering and slew rate control enabled."]
    ENABLED,
    #[doc = "Disabled. The SCL0 pin has I2C glitch filtering and slew rate control disabled."]
    DISABLED,
}
impl SCLI2C0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCLI2C0W::ENABLED => false,
            SCLI2C0W::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCLI2C0W<'a> {
    w: &'a mut W,
}
impl<'a> _SCLI2C0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCLI2C0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled. The SCL0 pin has I2C glitch filtering and slew rate control enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCLI2C0W::ENABLED)
    }
    #[doc = "Disabled. The SCL0 pin has I2C glitch filtering and slew rate control disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCLI2C0W::DISABLED)
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
        const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Drive mode control for the SDA0 pin, P0.27."]
    #[inline]
    pub fn sdadrv0(&self) -> SDADRV0R {
        SDADRV0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - I 2C filter mode control for the SDA0 pin, P0.27."]
    #[inline]
    pub fn sdai2c0(&self) -> SDAI2C0R {
        SDAI2C0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Drive mode control for the SCL0 pin, P0.28."]
    #[inline]
    pub fn scldrv0(&self) -> SCLDRV0R {
        SCLDRV0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - I 2C filter mode control for the SCL0 pin, P0.28."]
    #[inline]
    pub fn scli2c0(&self) -> SCLI2C0R {
        SCLI2C0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Drive mode control for the SDA0 pin, P0.27."]
    #[inline]
    pub fn sdadrv0(&mut self) -> _SDADRV0W {
        _SDADRV0W { w: self }
    }
    #[doc = "Bit 1 - I 2C filter mode control for the SDA0 pin, P0.27."]
    #[inline]
    pub fn sdai2c0(&mut self) -> _SDAI2C0W {
        _SDAI2C0W { w: self }
    }
    #[doc = "Bit 2 - Drive mode control for the SCL0 pin, P0.28."]
    #[inline]
    pub fn scldrv0(&mut self) -> _SCLDRV0W {
        _SCLDRV0W { w: self }
    }
    #[doc = "Bit 3 - I 2C filter mode control for the SCL0 pin, P0.28."]
    #[inline]
    pub fn scli2c0(&mut self) -> _SCLI2C0W {
        _SCLI2C0W { w: self }
    }
}
