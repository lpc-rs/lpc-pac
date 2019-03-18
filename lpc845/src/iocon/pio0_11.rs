#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PIO0_11 {
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
#[doc = "Possible values of the field `INV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVR {
    #[doc = "Input not inverted (HIGH on pin reads as 1; LOW on pin reads as 0)."]
    NOT_INVERTED,
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    INVERTED,
}
impl INVR {
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
            INVR::NOT_INVERTED => false,
            INVR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVR {
        match value {
            false => INVR::NOT_INVERTED,
            true => INVR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INVERTED`"]
    #[inline]
    pub fn is_not_inverted(&self) -> bool {
        *self == INVR::NOT_INVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == INVR::INVERTED
    }
}
#[doc = "Possible values of the field `I2CMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CMODER {
    #[doc = "Standard mode/ Fast-mode I2C."]
    STANDARAD_I2C,
    #[doc = "Standard GPIO functionality. Requires external pull-up for GPIO output function."]
    STANDARD_GPIO,
    #[doc = "Fast-mode Plus I2C"]
    FAST_PLUS_I2C,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl I2CMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            I2CMODER::STANDARAD_I2C => 0,
            I2CMODER::STANDARD_GPIO => 1,
            I2CMODER::FAST_PLUS_I2C => 2,
            I2CMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> I2CMODER {
        match value {
            0 => I2CMODER::STANDARAD_I2C,
            1 => I2CMODER::STANDARD_GPIO,
            2 => I2CMODER::FAST_PLUS_I2C,
            i => I2CMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARAD_I2C`"]
    #[inline]
    pub fn is_standarad_i2c(&self) -> bool {
        *self == I2CMODER::STANDARAD_I2C
    }
    #[doc = "Checks if the value of the field is `STANDARD_GPIO`"]
    #[inline]
    pub fn is_standard_gpio(&self) -> bool {
        *self == I2CMODER::STANDARD_GPIO
    }
    #[doc = "Checks if the value of the field is `FAST_PLUS_I2C`"]
    #[inline]
    pub fn is_fast_plus_i2c(&self) -> bool {
        *self == I2CMODER::FAST_PLUS_I2C
    }
}
#[doc = "Possible values of the field `S_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S_MODER {
    #[doc = "Bypass input filter."]
    S_MODE_0,
    #[doc = "1 clock cycle. Input pulses shorter than one filter clock are rejected."]
    S_MODE_1,
    #[doc = "2 clock cycles. Input pulses shorter than two filter clocks are rejected."]
    S_MODE_2,
    #[doc = "3 clock cycles. Input pulses shorter than three filter clocks are rejected."]
    S_MODE_3,
}
impl S_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            S_MODER::S_MODE_0 => 0,
            S_MODER::S_MODE_1 => 1,
            S_MODER::S_MODE_2 => 2,
            S_MODER::S_MODE_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> S_MODER {
        match value {
            0 => S_MODER::S_MODE_0,
            1 => S_MODER::S_MODE_1,
            2 => S_MODER::S_MODE_2,
            3 => S_MODER::S_MODE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `S_MODE_0`"]
    #[inline]
    pub fn is_s_mode_0(&self) -> bool {
        *self == S_MODER::S_MODE_0
    }
    #[doc = "Checks if the value of the field is `S_MODE_1`"]
    #[inline]
    pub fn is_s_mode_1(&self) -> bool {
        *self == S_MODER::S_MODE_1
    }
    #[doc = "Checks if the value of the field is `S_MODE_2`"]
    #[inline]
    pub fn is_s_mode_2(&self) -> bool {
        *self == S_MODER::S_MODE_2
    }
    #[doc = "Checks if the value of the field is `S_MODE_3`"]
    #[inline]
    pub fn is_s_mode_3(&self) -> bool {
        *self == S_MODER::S_MODE_3
    }
}
#[doc = "Possible values of the field `CLK_DIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_DIVR {
    #[doc = "IOCONCLKDIV0"]
    CLK_DIV_0,
    #[doc = "IOCONCLKDIV1"]
    CLK_DIV_1,
    #[doc = "IOCONCLKDIV2"]
    CLK_DIV_2,
    #[doc = "IOCONCLKDIV3"]
    CLK_DIV_3,
    #[doc = "IOCONCLKDIV4"]
    CLK_DIV_4,
    #[doc = "IOCONCLKDIV5"]
    CLK_DIV_5,
    #[doc = "IOCONCLKDIV6"]
    CLK_DIV_6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLK_DIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLK_DIVR::CLK_DIV_0 => 0,
            CLK_DIVR::CLK_DIV_1 => 1,
            CLK_DIVR::CLK_DIV_2 => 2,
            CLK_DIVR::CLK_DIV_3 => 3,
            CLK_DIVR::CLK_DIV_4 => 4,
            CLK_DIVR::CLK_DIV_5 => 5,
            CLK_DIVR::CLK_DIV_6 => 6,
            CLK_DIVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLK_DIVR {
        match value {
            0 => CLK_DIVR::CLK_DIV_0,
            1 => CLK_DIVR::CLK_DIV_1,
            2 => CLK_DIVR::CLK_DIV_2,
            3 => CLK_DIVR::CLK_DIV_3,
            4 => CLK_DIVR::CLK_DIV_4,
            5 => CLK_DIVR::CLK_DIV_5,
            6 => CLK_DIVR::CLK_DIV_6,
            i => CLK_DIVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_0`"]
    #[inline]
    pub fn is_clk_div_0(&self) -> bool {
        *self == CLK_DIVR::CLK_DIV_0
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_1`"]
    #[inline]
    pub fn is_clk_div_1(&self) -> bool {
        *self == CLK_DIVR::CLK_DIV_1
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_2`"]
    #[inline]
    pub fn is_clk_div_2(&self) -> bool {
        *self == CLK_DIVR::CLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_3`"]
    #[inline]
    pub fn is_clk_div_3(&self) -> bool {
        *self == CLK_DIVR::CLK_DIV_3
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_4`"]
    #[inline]
    pub fn is_clk_div_4(&self) -> bool {
        *self == CLK_DIVR::CLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_5`"]
    #[inline]
    pub fn is_clk_div_5(&self) -> bool {
        *self == CLK_DIVR::CLK_DIV_5
    }
    #[doc = "Checks if the value of the field is `CLK_DIV_6`"]
    #[inline]
    pub fn is_clk_div_6(&self) -> bool {
        *self == CLK_DIVR::CLK_DIV_6
    }
}
#[doc = "Values that can be written to the field `INV`"]
pub enum INVW {
    #[doc = "Input not inverted (HIGH on pin reads as 1; LOW on pin reads as 0)."]
    NOT_INVERTED,
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    INVERTED,
}
impl INVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVW::NOT_INVERTED => false,
            INVW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVW<'a> {
    w: &'a mut W,
}
impl<'a> _INVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input not inverted (HIGH on pin reads as 1; LOW on pin reads as 0)."]
    #[inline]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(INVW::NOT_INVERTED)
    }
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(INVW::INVERTED)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `I2CMODE`"]
pub enum I2CMODEW {
    #[doc = "Standard mode/ Fast-mode I2C."]
    STANDARAD_I2C,
    #[doc = "Standard GPIO functionality. Requires external pull-up for GPIO output function."]
    STANDARD_GPIO,
    #[doc = "Fast-mode Plus I2C"]
    FAST_PLUS_I2C,
}
impl I2CMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            I2CMODEW::STANDARAD_I2C => 0,
            I2CMODEW::STANDARD_GPIO => 1,
            I2CMODEW::FAST_PLUS_I2C => 2,
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
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Standard mode/ Fast-mode I2C."]
    #[inline]
    pub fn standarad_i2c(self) -> &'a mut W {
        self.variant(I2CMODEW::STANDARAD_I2C)
    }
    #[doc = "Standard GPIO functionality. Requires external pull-up for GPIO output function."]
    #[inline]
    pub fn standard_gpio(self) -> &'a mut W {
        self.variant(I2CMODEW::STANDARD_GPIO)
    }
    #[doc = "Fast-mode Plus I2C"]
    #[inline]
    pub fn fast_plus_i2c(self) -> &'a mut W {
        self.variant(I2CMODEW::FAST_PLUS_I2C)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `S_MODE`"]
pub enum S_MODEW {
    #[doc = "Bypass input filter."]
    S_MODE_0,
    #[doc = "1 clock cycle. Input pulses shorter than one filter clock are rejected."]
    S_MODE_1,
    #[doc = "2 clock cycles. Input pulses shorter than two filter clocks are rejected."]
    S_MODE_2,
    #[doc = "3 clock cycles. Input pulses shorter than three filter clocks are rejected."]
    S_MODE_3,
}
impl S_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            S_MODEW::S_MODE_0 => 0,
            S_MODEW::S_MODE_1 => 1,
            S_MODEW::S_MODE_2 => 2,
            S_MODEW::S_MODE_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _S_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S_MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Bypass input filter."]
    #[inline]
    pub fn s_mode_0(self) -> &'a mut W {
        self.variant(S_MODEW::S_MODE_0)
    }
    #[doc = "1 clock cycle. Input pulses shorter than one filter clock are rejected."]
    #[inline]
    pub fn s_mode_1(self) -> &'a mut W {
        self.variant(S_MODEW::S_MODE_1)
    }
    #[doc = "2 clock cycles. Input pulses shorter than two filter clocks are rejected."]
    #[inline]
    pub fn s_mode_2(self) -> &'a mut W {
        self.variant(S_MODEW::S_MODE_2)
    }
    #[doc = "3 clock cycles. Input pulses shorter than three filter clocks are rejected."]
    #[inline]
    pub fn s_mode_3(self) -> &'a mut W {
        self.variant(S_MODEW::S_MODE_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLK_DIV`"]
pub enum CLK_DIVW {
    #[doc = "IOCONCLKDIV0"]
    CLK_DIV_0,
    #[doc = "IOCONCLKDIV1"]
    CLK_DIV_1,
    #[doc = "IOCONCLKDIV2"]
    CLK_DIV_2,
    #[doc = "IOCONCLKDIV3"]
    CLK_DIV_3,
    #[doc = "IOCONCLKDIV4"]
    CLK_DIV_4,
    #[doc = "IOCONCLKDIV5"]
    CLK_DIV_5,
    #[doc = "IOCONCLKDIV6"]
    CLK_DIV_6,
}
impl CLK_DIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLK_DIVW::CLK_DIV_0 => 0,
            CLK_DIVW::CLK_DIV_1 => 1,
            CLK_DIVW::CLK_DIV_2 => 2,
            CLK_DIVW::CLK_DIV_3 => 3,
            CLK_DIVW::CLK_DIV_4 => 4,
            CLK_DIVW::CLK_DIV_5 => 5,
            CLK_DIVW::CLK_DIV_6 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLK_DIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_DIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLK_DIVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "IOCONCLKDIV0"]
    #[inline]
    pub fn clk_div_0(self) -> &'a mut W {
        self.variant(CLK_DIVW::CLK_DIV_0)
    }
    #[doc = "IOCONCLKDIV1"]
    #[inline]
    pub fn clk_div_1(self) -> &'a mut W {
        self.variant(CLK_DIVW::CLK_DIV_1)
    }
    #[doc = "IOCONCLKDIV2"]
    #[inline]
    pub fn clk_div_2(self) -> &'a mut W {
        self.variant(CLK_DIVW::CLK_DIV_2)
    }
    #[doc = "IOCONCLKDIV3"]
    #[inline]
    pub fn clk_div_3(self) -> &'a mut W {
        self.variant(CLK_DIVW::CLK_DIV_3)
    }
    #[doc = "IOCONCLKDIV4"]
    #[inline]
    pub fn clk_div_4(self) -> &'a mut W {
        self.variant(CLK_DIVW::CLK_DIV_4)
    }
    #[doc = "IOCONCLKDIV5"]
    #[inline]
    pub fn clk_div_5(self) -> &'a mut W {
        self.variant(CLK_DIVW::CLK_DIV_5)
    }
    #[doc = "IOCONCLKDIV6"]
    #[inline]
    pub fn clk_div_6(self) -> &'a mut W {
        self.variant(CLK_DIVW::CLK_DIV_6)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 13;
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
    #[doc = "Bit 6 - Invert input"]
    #[inline]
    pub fn inv(&self) -> INVR {
        INVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - Selects I2C mode."]
    #[inline]
    pub fn i2cmode(&self) -> I2CMODER {
        I2CMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:12 - Digital filter sample mode."]
    #[inline]
    pub fn s_mode(&self) -> S_MODER {
        S_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 13:15 - Select peripheral clock divider for input filter sampling clock. Value 0x7 is reserved."]
    #[inline]
    pub fn clk_div(&self) -> CLK_DIVR {
        CLK_DIVR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
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
    #[doc = "Bit 6 - Invert input"]
    #[inline]
    pub fn inv(&mut self) -> _INVW {
        _INVW { w: self }
    }
    #[doc = "Bits 8:9 - Selects I2C mode."]
    #[inline]
    pub fn i2cmode(&mut self) -> _I2CMODEW {
        _I2CMODEW { w: self }
    }
    #[doc = "Bits 11:12 - Digital filter sample mode."]
    #[inline]
    pub fn s_mode(&mut self) -> _S_MODEW {
        _S_MODEW { w: self }
    }
    #[doc = "Bits 13:15 - Select peripheral clock divider for input filter sampling clock. Value 0x7 is reserved."]
    #[inline]
    pub fn clk_div(&mut self) -> _CLK_DIVW {
        _CLK_DIVW { w: self }
    }
}
