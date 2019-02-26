#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WDTOSCCTRL {
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
#[doc = r" Value of the field"]
pub struct DIVSELR {
    bits: u8,
}
impl DIVSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `FREQSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQSELR {
    #[doc = "0.6 MHz"]
    _0_6,
    #[doc = "1.05 MHz"]
    _1_05,
    #[doc = "1.4 MHz"]
    _1_4,
    #[doc = "1.75 MHz"]
    _1_75,
    #[doc = "2.1 MHz"]
    _2_1,
    #[doc = "2.4 MHz"]
    _2_4,
    #[doc = "2.7 MHz"]
    _2_7,
    #[doc = "3.0 MHz"]
    _3_0,
    #[doc = "3.25 MHz"]
    _3_25,
    #[doc = "3.5 MHz"]
    _3_5,
    #[doc = "3.75 MHz"]
    _3_75,
    #[doc = "4.0 MHz"]
    _4_0,
    #[doc = "4.2 MHz"]
    _4_2,
    #[doc = "4.4 MHz"]
    _4_4,
    #[doc = "4.6 MHz"]
    _4_6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FREQSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FREQSELR::_0_6 => 1,
            FREQSELR::_1_05 => 2,
            FREQSELR::_1_4 => 3,
            FREQSELR::_1_75 => 4,
            FREQSELR::_2_1 => 5,
            FREQSELR::_2_4 => 6,
            FREQSELR::_2_7 => 7,
            FREQSELR::_3_0 => 8,
            FREQSELR::_3_25 => 9,
            FREQSELR::_3_5 => 10,
            FREQSELR::_3_75 => 11,
            FREQSELR::_4_0 => 12,
            FREQSELR::_4_2 => 13,
            FREQSELR::_4_4 => 14,
            FREQSELR::_4_6 => 15,
            FREQSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FREQSELR {
        match value {
            1 => FREQSELR::_0_6,
            2 => FREQSELR::_1_05,
            3 => FREQSELR::_1_4,
            4 => FREQSELR::_1_75,
            5 => FREQSELR::_2_1,
            6 => FREQSELR::_2_4,
            7 => FREQSELR::_2_7,
            8 => FREQSELR::_3_0,
            9 => FREQSELR::_3_25,
            10 => FREQSELR::_3_5,
            11 => FREQSELR::_3_75,
            12 => FREQSELR::_4_0,
            13 => FREQSELR::_4_2,
            14 => FREQSELR::_4_4,
            15 => FREQSELR::_4_6,
            i => FREQSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0_6`"]
    #[inline]
    pub fn is_0_6(&self) -> bool {
        *self == FREQSELR::_0_6
    }
    #[doc = "Checks if the value of the field is `_1_05`"]
    #[inline]
    pub fn is_1_05(&self) -> bool {
        *self == FREQSELR::_1_05
    }
    #[doc = "Checks if the value of the field is `_1_4`"]
    #[inline]
    pub fn is_1_4(&self) -> bool {
        *self == FREQSELR::_1_4
    }
    #[doc = "Checks if the value of the field is `_1_75`"]
    #[inline]
    pub fn is_1_75(&self) -> bool {
        *self == FREQSELR::_1_75
    }
    #[doc = "Checks if the value of the field is `_2_1`"]
    #[inline]
    pub fn is_2_1(&self) -> bool {
        *self == FREQSELR::_2_1
    }
    #[doc = "Checks if the value of the field is `_2_4`"]
    #[inline]
    pub fn is_2_4(&self) -> bool {
        *self == FREQSELR::_2_4
    }
    #[doc = "Checks if the value of the field is `_2_7`"]
    #[inline]
    pub fn is_2_7(&self) -> bool {
        *self == FREQSELR::_2_7
    }
    #[doc = "Checks if the value of the field is `_3_0`"]
    #[inline]
    pub fn is_3_0(&self) -> bool {
        *self == FREQSELR::_3_0
    }
    #[doc = "Checks if the value of the field is `_3_25`"]
    #[inline]
    pub fn is_3_25(&self) -> bool {
        *self == FREQSELR::_3_25
    }
    #[doc = "Checks if the value of the field is `_3_5`"]
    #[inline]
    pub fn is_3_5(&self) -> bool {
        *self == FREQSELR::_3_5
    }
    #[doc = "Checks if the value of the field is `_3_75`"]
    #[inline]
    pub fn is_3_75(&self) -> bool {
        *self == FREQSELR::_3_75
    }
    #[doc = "Checks if the value of the field is `_4_0`"]
    #[inline]
    pub fn is_4_0(&self) -> bool {
        *self == FREQSELR::_4_0
    }
    #[doc = "Checks if the value of the field is `_4_2`"]
    #[inline]
    pub fn is_4_2(&self) -> bool {
        *self == FREQSELR::_4_2
    }
    #[doc = "Checks if the value of the field is `_4_4`"]
    #[inline]
    pub fn is_4_4(&self) -> bool {
        *self == FREQSELR::_4_4
    }
    #[doc = "Checks if the value of the field is `_4_6`"]
    #[inline]
    pub fn is_4_6(&self) -> bool {
        *self == FREQSELR::_4_6
    }
}
#[doc = r" Proxy"]
pub struct _DIVSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FREQSEL`"]
pub enum FREQSELW {
    #[doc = "0.6 MHz"]
    _0_6,
    #[doc = "1.05 MHz"]
    _1_05,
    #[doc = "1.4 MHz"]
    _1_4,
    #[doc = "1.75 MHz"]
    _1_75,
    #[doc = "2.1 MHz"]
    _2_1,
    #[doc = "2.4 MHz"]
    _2_4,
    #[doc = "2.7 MHz"]
    _2_7,
    #[doc = "3.0 MHz"]
    _3_0,
    #[doc = "3.25 MHz"]
    _3_25,
    #[doc = "3.5 MHz"]
    _3_5,
    #[doc = "3.75 MHz"]
    _3_75,
    #[doc = "4.0 MHz"]
    _4_0,
    #[doc = "4.2 MHz"]
    _4_2,
    #[doc = "4.4 MHz"]
    _4_4,
    #[doc = "4.6 MHz"]
    _4_6,
}
impl FREQSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FREQSELW::_0_6 => 1,
            FREQSELW::_1_05 => 2,
            FREQSELW::_1_4 => 3,
            FREQSELW::_1_75 => 4,
            FREQSELW::_2_1 => 5,
            FREQSELW::_2_4 => 6,
            FREQSELW::_2_7 => 7,
            FREQSELW::_3_0 => 8,
            FREQSELW::_3_25 => 9,
            FREQSELW::_3_5 => 10,
            FREQSELW::_3_75 => 11,
            FREQSELW::_4_0 => 12,
            FREQSELW::_4_2 => 13,
            FREQSELW::_4_4 => 14,
            FREQSELW::_4_6 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FREQSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FREQSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FREQSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "0.6 MHz"]
    #[inline]
    pub fn _0_6(self) -> &'a mut W {
        self.variant(FREQSELW::_0_6)
    }
    #[doc = "1.05 MHz"]
    #[inline]
    pub fn _1_05(self) -> &'a mut W {
        self.variant(FREQSELW::_1_05)
    }
    #[doc = "1.4 MHz"]
    #[inline]
    pub fn _1_4(self) -> &'a mut W {
        self.variant(FREQSELW::_1_4)
    }
    #[doc = "1.75 MHz"]
    #[inline]
    pub fn _1_75(self) -> &'a mut W {
        self.variant(FREQSELW::_1_75)
    }
    #[doc = "2.1 MHz"]
    #[inline]
    pub fn _2_1(self) -> &'a mut W {
        self.variant(FREQSELW::_2_1)
    }
    #[doc = "2.4 MHz"]
    #[inline]
    pub fn _2_4(self) -> &'a mut W {
        self.variant(FREQSELW::_2_4)
    }
    #[doc = "2.7 MHz"]
    #[inline]
    pub fn _2_7(self) -> &'a mut W {
        self.variant(FREQSELW::_2_7)
    }
    #[doc = "3.0 MHz"]
    #[inline]
    pub fn _3_0(self) -> &'a mut W {
        self.variant(FREQSELW::_3_0)
    }
    #[doc = "3.25 MHz"]
    #[inline]
    pub fn _3_25(self) -> &'a mut W {
        self.variant(FREQSELW::_3_25)
    }
    #[doc = "3.5 MHz"]
    #[inline]
    pub fn _3_5(self) -> &'a mut W {
        self.variant(FREQSELW::_3_5)
    }
    #[doc = "3.75 MHz"]
    #[inline]
    pub fn _3_75(self) -> &'a mut W {
        self.variant(FREQSELW::_3_75)
    }
    #[doc = "4.0 MHz"]
    #[inline]
    pub fn _4_0(self) -> &'a mut W {
        self.variant(FREQSELW::_4_0)
    }
    #[doc = "4.2 MHz"]
    #[inline]
    pub fn _4_2(self) -> &'a mut W {
        self.variant(FREQSELW::_4_2)
    }
    #[doc = "4.4 MHz"]
    #[inline]
    pub fn _4_4(self) -> &'a mut W {
        self.variant(FREQSELW::_4_4)
    }
    #[doc = "4.6 MHz"]
    #[inline]
    pub fn _4_6(self) -> &'a mut W {
        self.variant(FREQSELW::_4_6)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 5;
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
    #[doc = "Bits 0:4 - Select divider for Fclkana. wdt_osc_clk = Fclkana/ (2 x (1 + DIVSEL)) 00000: 2 x (1 + DIVSEL) = 2 00001: 2 x (1 + DIVSEL) = 4 to 11111: 2 x (1 + DIVSEL) = 64"]
    #[inline]
    pub fn divsel(&self) -> DIVSELR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DIVSELR { bits }
    }
    #[doc = "Bits 5:8 - Select watchdog oscillator analog output frequency (Fclkana)."]
    #[inline]
    pub fn freqsel(&self) -> FREQSELR {
        FREQSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 160 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - Select divider for Fclkana. wdt_osc_clk = Fclkana/ (2 x (1 + DIVSEL)) 00000: 2 x (1 + DIVSEL) = 2 00001: 2 x (1 + DIVSEL) = 4 to 11111: 2 x (1 + DIVSEL) = 64"]
    #[inline]
    pub fn divsel(&mut self) -> _DIVSELW {
        _DIVSELW { w: self }
    }
    #[doc = "Bits 5:8 - Select watchdog oscillator analog output frequency (Fclkana)."]
    #[inline]
    pub fn freqsel(&mut self) -> _FREQSELW {
        _FREQSELW { w: self }
    }
}
