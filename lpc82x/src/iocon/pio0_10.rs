#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PIO0_10 {
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
    INPUT_NOT_INVERTED,
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    INPUT_INVERTED_HIGH,
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
            INVR::INPUT_NOT_INVERTED => false,
            INVR::INPUT_INVERTED_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVR {
        match value {
            false => INVR::INPUT_NOT_INVERTED,
            true => INVR::INPUT_INVERTED_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT_NOT_INVERTED`"]
    #[inline]
    pub fn is_input_not_inverted(&self) -> bool {
        *self == INVR::INPUT_NOT_INVERTED
    }
    #[doc = "Checks if the value of the field is `INPUT_INVERTED_HIGH`"]
    #[inline]
    pub fn is_input_inverted_high(&self) -> bool {
        *self == INVR::INPUT_INVERTED_HIGH
    }
}
#[doc = "Possible values of the field `I2CMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CMODER {
    #[doc = "Standard mode/ Fast-mode I2C."]
    STANDARD_FAST,
    #[doc = "Standard I/O functionality"]
    STANDARD_IO,
    #[doc = "Fast-mode Plus I2C"]
    FAST_MODE_PLUS_I2C,
}
impl I2CMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            I2CMODER::STANDARD_FAST => 0,
            I2CMODER::STANDARD_IO => 1,
            I2CMODER::FAST_MODE_PLUS_I2C => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> I2CMODER {
        match value {
            0 => I2CMODER::STANDARD_FAST,
            1 => I2CMODER::STANDARD_IO,
            2 => I2CMODER::FAST_MODE_PLUS_I2C,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD_FAST`"]
    #[inline]
    pub fn is_standard_fast(&self) -> bool {
        *self == I2CMODER::STANDARD_FAST
    }
    #[doc = "Checks if the value of the field is `STANDARD_IO`"]
    #[inline]
    pub fn is_standard_io(&self) -> bool {
        *self == I2CMODER::STANDARD_IO
    }
    #[doc = "Checks if the value of the field is `FAST_MODE_PLUS_I2C`"]
    #[inline]
    pub fn is_fast_mode_plus_i2c(&self) -> bool {
        *self == I2CMODER::FAST_MODE_PLUS_I2C
    }
}
#[doc = "Possible values of the field `S_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S_MODER {
    #[doc = "Bypass input filter."]
    BYPASS_INPUT_FILTER,
    #[doc = "1 clock cycle. Input pulses shorter than one filter clock are rejected."]
    _1_CLOCK_CYCLE,
    #[doc = "2 clock cycles. Input pulses shorter than two filter clocks are rejected."]
    _2_CLOCK_CYCLES,
    #[doc = "3 clock cycles. Input pulses shorter than three filter clocks are rejected."]
    _3_CLOCK_CYCLES,
}
impl S_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            S_MODER::BYPASS_INPUT_FILTER => 0,
            S_MODER::_1_CLOCK_CYCLE => 1,
            S_MODER::_2_CLOCK_CYCLES => 2,
            S_MODER::_3_CLOCK_CYCLES => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> S_MODER {
        match value {
            0 => S_MODER::BYPASS_INPUT_FILTER,
            1 => S_MODER::_1_CLOCK_CYCLE,
            2 => S_MODER::_2_CLOCK_CYCLES,
            3 => S_MODER::_3_CLOCK_CYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS_INPUT_FILTER`"]
    #[inline]
    pub fn is_bypass_input_filter(&self) -> bool {
        *self == S_MODER::BYPASS_INPUT_FILTER
    }
    #[doc = "Checks if the value of the field is `_1_CLOCK_CYCLE`"]
    #[inline]
    pub fn is_1_clock_cycle(&self) -> bool {
        *self == S_MODER::_1_CLOCK_CYCLE
    }
    #[doc = "Checks if the value of the field is `_2_CLOCK_CYCLES`"]
    #[inline]
    pub fn is_2_clock_cycles(&self) -> bool {
        *self == S_MODER::_2_CLOCK_CYCLES
    }
    #[doc = "Checks if the value of the field is `_3_CLOCK_CYCLES`"]
    #[inline]
    pub fn is_3_clock_cycles(&self) -> bool {
        *self == S_MODER::_3_CLOCK_CYCLES
    }
}
#[doc = "Possible values of the field `CLK_DIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_DIVR {
    #[doc = "IOCONCLKDIV0."]
    IOCONCLKDIV0,
    #[doc = "IOCONCLKDIV1."]
    IOCONCLKDIV1,
    #[doc = "IOCONCLKDIV2."]
    IOCONCLKDIV2,
    #[doc = "IOCONCLKDIV3."]
    IOCONCLKDIV3,
    #[doc = "IOCONCLKDIV4."]
    IOCONCLKDIV4,
    #[doc = "IOCONCLKDIV5."]
    IOCONCLKDIV5,
    #[doc = "IOCONCLKDIV6."]
    IOCONCLKDIV6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLK_DIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLK_DIVR::IOCONCLKDIV0 => 0,
            CLK_DIVR::IOCONCLKDIV1 => 1,
            CLK_DIVR::IOCONCLKDIV2 => 2,
            CLK_DIVR::IOCONCLKDIV3 => 3,
            CLK_DIVR::IOCONCLKDIV4 => 4,
            CLK_DIVR::IOCONCLKDIV5 => 5,
            CLK_DIVR::IOCONCLKDIV6 => 6,
            CLK_DIVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLK_DIVR {
        match value {
            0 => CLK_DIVR::IOCONCLKDIV0,
            1 => CLK_DIVR::IOCONCLKDIV1,
            2 => CLK_DIVR::IOCONCLKDIV2,
            3 => CLK_DIVR::IOCONCLKDIV3,
            4 => CLK_DIVR::IOCONCLKDIV4,
            5 => CLK_DIVR::IOCONCLKDIV5,
            6 => CLK_DIVR::IOCONCLKDIV6,
            i => CLK_DIVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IOCONCLKDIV0`"]
    #[inline]
    pub fn is_ioconclkdiv0(&self) -> bool {
        *self == CLK_DIVR::IOCONCLKDIV0
    }
    #[doc = "Checks if the value of the field is `IOCONCLKDIV1`"]
    #[inline]
    pub fn is_ioconclkdiv1(&self) -> bool {
        *self == CLK_DIVR::IOCONCLKDIV1
    }
    #[doc = "Checks if the value of the field is `IOCONCLKDIV2`"]
    #[inline]
    pub fn is_ioconclkdiv2(&self) -> bool {
        *self == CLK_DIVR::IOCONCLKDIV2
    }
    #[doc = "Checks if the value of the field is `IOCONCLKDIV3`"]
    #[inline]
    pub fn is_ioconclkdiv3(&self) -> bool {
        *self == CLK_DIVR::IOCONCLKDIV3
    }
    #[doc = "Checks if the value of the field is `IOCONCLKDIV4`"]
    #[inline]
    pub fn is_ioconclkdiv4(&self) -> bool {
        *self == CLK_DIVR::IOCONCLKDIV4
    }
    #[doc = "Checks if the value of the field is `IOCONCLKDIV5`"]
    #[inline]
    pub fn is_ioconclkdiv5(&self) -> bool {
        *self == CLK_DIVR::IOCONCLKDIV5
    }
    #[doc = "Checks if the value of the field is `IOCONCLKDIV6`"]
    #[inline]
    pub fn is_ioconclkdiv6(&self) -> bool {
        *self == CLK_DIVR::IOCONCLKDIV6
    }
}
#[doc = "Values that can be written to the field `INV`"]
pub enum INVW {
    #[doc = "Input not inverted (HIGH on pin reads as 1; LOW on pin reads as 0)."]
    INPUT_NOT_INVERTED,
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    INPUT_INVERTED_HIGH,
}
impl INVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVW::INPUT_NOT_INVERTED => false,
            INVW::INPUT_INVERTED_HIGH => true,
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
    pub fn input_not_inverted(self) -> &'a mut W {
        self.variant(INVW::INPUT_NOT_INVERTED)
    }
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    #[inline]
    pub fn input_inverted_high(self) -> &'a mut W {
        self.variant(INVW::INPUT_INVERTED_HIGH)
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
    STANDARD_FAST,
    #[doc = "Standard I/O functionality"]
    STANDARD_IO,
    #[doc = "Fast-mode Plus I2C"]
    FAST_MODE_PLUS_I2C,
}
impl I2CMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            I2CMODEW::STANDARD_FAST => 0,
            I2CMODEW::STANDARD_IO => 1,
            I2CMODEW::FAST_MODE_PLUS_I2C => 2,
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
    pub fn standard_fast(self) -> &'a mut W {
        self.variant(I2CMODEW::STANDARD_FAST)
    }
    #[doc = "Standard I/O functionality"]
    #[inline]
    pub fn standard_io(self) -> &'a mut W {
        self.variant(I2CMODEW::STANDARD_IO)
    }
    #[doc = "Fast-mode Plus I2C"]
    #[inline]
    pub fn fast_mode_plus_i2c(self) -> &'a mut W {
        self.variant(I2CMODEW::FAST_MODE_PLUS_I2C)
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
    BYPASS_INPUT_FILTER,
    #[doc = "1 clock cycle. Input pulses shorter than one filter clock are rejected."]
    _1_CLOCK_CYCLE,
    #[doc = "2 clock cycles. Input pulses shorter than two filter clocks are rejected."]
    _2_CLOCK_CYCLES,
    #[doc = "3 clock cycles. Input pulses shorter than three filter clocks are rejected."]
    _3_CLOCK_CYCLES,
}
impl S_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            S_MODEW::BYPASS_INPUT_FILTER => 0,
            S_MODEW::_1_CLOCK_CYCLE => 1,
            S_MODEW::_2_CLOCK_CYCLES => 2,
            S_MODEW::_3_CLOCK_CYCLES => 3,
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
    pub fn bypass_input_filter(self) -> &'a mut W {
        self.variant(S_MODEW::BYPASS_INPUT_FILTER)
    }
    #[doc = "1 clock cycle. Input pulses shorter than one filter clock are rejected."]
    #[inline]
    pub fn _1_clock_cycle(self) -> &'a mut W {
        self.variant(S_MODEW::_1_CLOCK_CYCLE)
    }
    #[doc = "2 clock cycles. Input pulses shorter than two filter clocks are rejected."]
    #[inline]
    pub fn _2_clock_cycles(self) -> &'a mut W {
        self.variant(S_MODEW::_2_CLOCK_CYCLES)
    }
    #[doc = "3 clock cycles. Input pulses shorter than three filter clocks are rejected."]
    #[inline]
    pub fn _3_clock_cycles(self) -> &'a mut W {
        self.variant(S_MODEW::_3_CLOCK_CYCLES)
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
    #[doc = "IOCONCLKDIV0."]
    IOCONCLKDIV0,
    #[doc = "IOCONCLKDIV1."]
    IOCONCLKDIV1,
    #[doc = "IOCONCLKDIV2."]
    IOCONCLKDIV2,
    #[doc = "IOCONCLKDIV3."]
    IOCONCLKDIV3,
    #[doc = "IOCONCLKDIV4."]
    IOCONCLKDIV4,
    #[doc = "IOCONCLKDIV5."]
    IOCONCLKDIV5,
    #[doc = "IOCONCLKDIV6."]
    IOCONCLKDIV6,
}
impl CLK_DIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLK_DIVW::IOCONCLKDIV0 => 0,
            CLK_DIVW::IOCONCLKDIV1 => 1,
            CLK_DIVW::IOCONCLKDIV2 => 2,
            CLK_DIVW::IOCONCLKDIV3 => 3,
            CLK_DIVW::IOCONCLKDIV4 => 4,
            CLK_DIVW::IOCONCLKDIV5 => 5,
            CLK_DIVW::IOCONCLKDIV6 => 6,
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
    #[doc = "IOCONCLKDIV0."]
    #[inline]
    pub fn ioconclkdiv0(self) -> &'a mut W {
        self.variant(CLK_DIVW::IOCONCLKDIV0)
    }
    #[doc = "IOCONCLKDIV1."]
    #[inline]
    pub fn ioconclkdiv1(self) -> &'a mut W {
        self.variant(CLK_DIVW::IOCONCLKDIV1)
    }
    #[doc = "IOCONCLKDIV2."]
    #[inline]
    pub fn ioconclkdiv2(self) -> &'a mut W {
        self.variant(CLK_DIVW::IOCONCLKDIV2)
    }
    #[doc = "IOCONCLKDIV3."]
    #[inline]
    pub fn ioconclkdiv3(self) -> &'a mut W {
        self.variant(CLK_DIVW::IOCONCLKDIV3)
    }
    #[doc = "IOCONCLKDIV4."]
    #[inline]
    pub fn ioconclkdiv4(self) -> &'a mut W {
        self.variant(CLK_DIVW::IOCONCLKDIV4)
    }
    #[doc = "IOCONCLKDIV5."]
    #[inline]
    pub fn ioconclkdiv5(self) -> &'a mut W {
        self.variant(CLK_DIVW::IOCONCLKDIV5)
    }
    #[doc = "IOCONCLKDIV6."]
    #[inline]
    pub fn ioconclkdiv6(self) -> &'a mut W {
        self.variant(CLK_DIVW::IOCONCLKDIV6)
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
    #[doc = "Bits 8:9 - Selects I2C mode. Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000)."]
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
    #[doc = "Bits 8:9 - Selects I2C mode. Select Standard mode (I2CMODE = 00, default) or Standard I/O functionality (I2CMODE = 01) if the pin function is GPIO (FUNC = 000)."]
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
