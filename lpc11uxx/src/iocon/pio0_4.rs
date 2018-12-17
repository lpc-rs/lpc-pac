#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PIO0_4 {
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
#[doc = "Possible values of the field `FUNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUNCR {
    #[doc = "PIO0_4 (open-drain pin)."]
    PIO0_4_OPEN_DRAIN_P,
    #[doc = "I2C SCL (open-drain pin)."]
    I2C_SCL_OPEN_DRAIN_,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FUNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FUNCR::PIO0_4_OPEN_DRAIN_P => 0,
            FUNCR::I2C_SCL_OPEN_DRAIN_ => 1,
            FUNCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FUNCR {
        match value {
            0 => FUNCR::PIO0_4_OPEN_DRAIN_P,
            1 => FUNCR::I2C_SCL_OPEN_DRAIN_,
            i => FUNCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIO0_4_OPEN_DRAIN_P`"]
    #[inline]
    pub fn is_pio0_4_open_drain_p(&self) -> bool {
        *self == FUNCR::PIO0_4_OPEN_DRAIN_P
    }
    #[doc = "Checks if the value of the field is `I2C_SCL_OPEN_DRAIN_`"]
    #[inline]
    pub fn is_i2c_scl_open_drain_(&self) -> bool {
        *self == FUNCR::I2C_SCL_OPEN_DRAIN_
    }
}
#[doc = "Possible values of the field `I2CMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CMODER {
    #[doc = "Standard mode/ Fast-mode I2C."]
    STANDARD_MODE_FAST_,
    #[doc = "Standard I/O functionality"]
    STANDARD_IO_FUNCTIO,
    #[doc = "Fast-mode Plus I2C"]
    FAST_MODE_PLUS_I2C,
    #[doc = "Reserved."]
    RESERVED_3,
}
impl I2CMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            I2CMODER::STANDARD_MODE_FAST_ => 0,
            I2CMODER::STANDARD_IO_FUNCTIO => 1,
            I2CMODER::FAST_MODE_PLUS_I2C => 2,
            I2CMODER::RESERVED_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> I2CMODER {
        match value {
            0 => I2CMODER::STANDARD_MODE_FAST_,
            1 => I2CMODER::STANDARD_IO_FUNCTIO,
            2 => I2CMODER::FAST_MODE_PLUS_I2C,
            3 => I2CMODER::RESERVED_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD_MODE_FAST_`"]
    #[inline]
    pub fn is_standard_mode_fast_(&self) -> bool {
        *self == I2CMODER::STANDARD_MODE_FAST_
    }
    #[doc = "Checks if the value of the field is `STANDARD_IO_FUNCTIO`"]
    #[inline]
    pub fn is_standard_io_functio(&self) -> bool {
        *self == I2CMODER::STANDARD_IO_FUNCTIO
    }
    #[doc = "Checks if the value of the field is `FAST_MODE_PLUS_I2C`"]
    #[inline]
    pub fn is_fast_mode_plus_i2c(&self) -> bool {
        *self == I2CMODER::FAST_MODE_PLUS_I2C
    }
    #[doc = "Checks if the value of the field is `RESERVED_3`"]
    #[inline]
    pub fn is_reserved_3(&self) -> bool {
        *self == I2CMODER::RESERVED_3
    }
}
#[doc = "Values that can be written to the field `FUNC`"]
pub enum FUNCW {
    #[doc = "PIO0_4 (open-drain pin)."]
    PIO0_4_OPEN_DRAIN_P,
    #[doc = "I2C SCL (open-drain pin)."]
    I2C_SCL_OPEN_DRAIN_,
}
impl FUNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FUNCW::PIO0_4_OPEN_DRAIN_P => 0,
            FUNCW::I2C_SCL_OPEN_DRAIN_ => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FUNCW<'a> {
    w: &'a mut W,
}
impl<'a> _FUNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FUNCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PIO0_4 (open-drain pin)."]
    #[inline]
    pub fn pio0_4_open_drain_p(self) -> &'a mut W {
        self.variant(FUNCW::PIO0_4_OPEN_DRAIN_P)
    }
    #[doc = "I2C SCL (open-drain pin)."]
    #[inline]
    pub fn i2c_scl_open_drain_(self) -> &'a mut W {
        self.variant(FUNCW::I2C_SCL_OPEN_DRAIN_)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `I2CMODE`"]
pub enum I2CMODEW {
    #[doc = "Standard mode/ Fast-mode I2C."]
    STANDARD_MODE_FAST_,
    #[doc = "Standard I/O functionality"]
    STANDARD_IO_FUNCTIO,
    #[doc = "Fast-mode Plus I2C"]
    FAST_MODE_PLUS_I2C,
    #[doc = "Reserved."]
    RESERVED_3,
}
impl I2CMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            I2CMODEW::STANDARD_MODE_FAST_ => 0,
            I2CMODEW::STANDARD_IO_FUNCTIO => 1,
            I2CMODEW::FAST_MODE_PLUS_I2C => 2,
            I2CMODEW::RESERVED_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2CMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _I2CMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2CMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Standard mode/ Fast-mode I2C."]
    #[inline]
    pub fn standard_mode_fast_(self) -> &'a mut W {
        self.variant(I2CMODEW::STANDARD_MODE_FAST_)
    }
    #[doc = "Standard I/O functionality"]
    #[inline]
    pub fn standard_io_functio(self) -> &'a mut W {
        self.variant(I2CMODEW::STANDARD_IO_FUNCTIO)
    }
    #[doc = "Fast-mode Plus I2C"]
    #[inline]
    pub fn fast_mode_plus_i2c(self) -> &'a mut W {
        self.variant(I2CMODEW::FAST_MODE_PLUS_I2C)
    }
    #[doc = "Reserved."]
    #[inline]
    pub fn reserved_3(self) -> &'a mut W {
        self.variant(I2CMODEW::RESERVED_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:2 - Selects pin function. Values 0x2 to 0x7 are reserved."]
    #[inline]
    pub fn func(&self) -> FUNCR {
        FUNCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Selects I2C mode (see Section 7.3.8). Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000)."]
    #[inline]
    pub fn i2cmode(&self) -> I2CMODER {
        I2CMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 128 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Selects pin function. Values 0x2 to 0x7 are reserved."]
    #[inline]
    pub fn func(&mut self) -> _FUNCW {
        _FUNCW { w: self }
    }
    #[doc = "Bits 8:9 - Selects I2C mode (see Section 7.3.8). Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000)."]
    #[inline]
    pub fn i2cmode(&mut self) -> _I2CMODEW {
        _I2CMODEW { w: self }
    }
}
