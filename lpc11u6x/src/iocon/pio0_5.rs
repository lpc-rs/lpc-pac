#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PIO0_5 {
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
        0x80
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type FUNC_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _FUNCW<'a> {
    w: &'a mut W,
}
impl<'a> _FUNCW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Possible values of the field `I2CMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CMODER {
    #[doc = "Standard mode/ Fast-mode I2C."]
    STANDARD_MODE_FAST,
    #[doc = "Standard I/O functionality"]
    STANDARD_IO_FUNCTIO,
    #[doc = "Fast-mode Plus I2C"]
    FAST_MODE_PLUS_I2C,
}
impl crate::ToBits<u8> for I2CMODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            I2CMODER::STANDARD_MODE_FAST => 0,
            I2CMODER::STANDARD_IO_FUNCTIO => 1,
            I2CMODER::FAST_MODE_PLUS_I2C => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type I2CMODE_R = crate::FR<u8, I2CMODER>;
impl I2CMODE_R {
    #[doc = "Checks if the value of the field is `STANDARD_MODE_FAST`"]
    #[inline(always)]
    pub fn is_standard_mode_fast(&self) -> bool {
        *self == I2CMODER::STANDARD_MODE_FAST
    }
    #[doc = "Checks if the value of the field is `STANDARD_IO_FUNCTIO`"]
    #[inline(always)]
    pub fn is_standard_io_functio(&self) -> bool {
        *self == I2CMODER::STANDARD_IO_FUNCTIO
    }
    #[doc = "Checks if the value of the field is `FAST_MODE_PLUS_I2C`"]
    #[inline(always)]
    pub fn is_fast_mode_plus_i2c(&self) -> bool {
        *self == I2CMODER::FAST_MODE_PLUS_I2C
    }
}
#[doc = "Values that can be written to the field `I2CMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CMODEW {
    #[doc = "Standard mode/ Fast-mode I2C."]
    STANDARD_MODE_FAST,
    #[doc = "Standard I/O functionality"]
    STANDARD_IO_FUNCTIO,
    #[doc = "Fast-mode Plus I2C"]
    FAST_MODE_PLUS_I2C,
}
impl I2CMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            I2CMODEW::STANDARD_MODE_FAST => 0,
            I2CMODEW::STANDARD_IO_FUNCTIO => 1,
            I2CMODEW::FAST_MODE_PLUS_I2C => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _I2CMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _I2CMODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2CMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Standard mode/ Fast-mode I2C."]
    #[inline(always)]
    pub fn standard_mode_fast(self) -> &'a mut W {
        self.variant(I2CMODEW::STANDARD_MODE_FAST)
    }
    #[doc = "Standard I/O functionality"]
    #[inline(always)]
    pub fn standard_io_functio(self) -> &'a mut W {
        self.variant(I2CMODEW::STANDARD_IO_FUNCTIO)
    }
    #[doc = "Fast-mode Plus I2C"]
    #[inline(always)]
    pub fn fast_mode_plus_i2c(self) -> &'a mut W {
        self.variant(I2CMODEW::FAST_MODE_PLUS_I2C)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Selects pin function."]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits() & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - Selects I2C mode (see Section 7.3.8). Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000)."]
    #[inline(always)]
    pub fn i2cmode(&self) -> I2CMODE_R {
        I2CMODE_R::new(((self.bits() >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Selects pin function."]
    #[inline(always)]
    pub fn func(&mut self) -> _FUNCW {
        _FUNCW { w: self }
    }
    #[doc = "Bits 8:9 - Selects I2C mode (see Section 7.3.8). Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000)."]
    #[inline(always)]
    pub fn i2cmode(&mut self) -> _I2CMODEW {
        _I2CMODEW { w: self }
    }
}
