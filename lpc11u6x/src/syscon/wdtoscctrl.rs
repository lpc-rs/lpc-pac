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
    _600KHZ,
    #[doc = "1.05 MHz"]
    _1050KHZ,
    #[doc = "1.4 MHz"]
    _1400KHZ,
    #[doc = "1.75 MHz"]
    _1750KHZ,
    #[doc = "2.1 MHz"]
    _2100KHZ,
    #[doc = "2.4 MHz"]
    _2400KHZ,
    #[doc = "2.7 MHz"]
    _2700KHZ,
    #[doc = "3.0 MHz"]
    _3000KHZ,
    #[doc = "3.25 MHz"]
    _3250KHZ,
    #[doc = "3.5 MHz"]
    _3500KHZ,
    #[doc = "3.75 MHz"]
    _3750KHZ,
    #[doc = "4.0 MHz"]
    _4000KHZ,
    #[doc = "4.2 MHz"]
    _4200KHZ,
    #[doc = "4.4 MHz"]
    _4400KHZ,
    #[doc = "4.6 MHz"]
    _4600KHZ,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FREQSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FREQSELR::_600KHZ => 1,
            FREQSELR::_1050KHZ => 2,
            FREQSELR::_1400KHZ => 3,
            FREQSELR::_1750KHZ => 4,
            FREQSELR::_2100KHZ => 5,
            FREQSELR::_2400KHZ => 6,
            FREQSELR::_2700KHZ => 7,
            FREQSELR::_3000KHZ => 8,
            FREQSELR::_3250KHZ => 9,
            FREQSELR::_3500KHZ => 10,
            FREQSELR::_3750KHZ => 11,
            FREQSELR::_4000KHZ => 12,
            FREQSELR::_4200KHZ => 13,
            FREQSELR::_4400KHZ => 14,
            FREQSELR::_4600KHZ => 15,
            FREQSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FREQSELR {
        match value {
            1 => FREQSELR::_600KHZ,
            2 => FREQSELR::_1050KHZ,
            3 => FREQSELR::_1400KHZ,
            4 => FREQSELR::_1750KHZ,
            5 => FREQSELR::_2100KHZ,
            6 => FREQSELR::_2400KHZ,
            7 => FREQSELR::_2700KHZ,
            8 => FREQSELR::_3000KHZ,
            9 => FREQSELR::_3250KHZ,
            10 => FREQSELR::_3500KHZ,
            11 => FREQSELR::_3750KHZ,
            12 => FREQSELR::_4000KHZ,
            13 => FREQSELR::_4200KHZ,
            14 => FREQSELR::_4400KHZ,
            15 => FREQSELR::_4600KHZ,
            i => FREQSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_600KHZ`"]
    #[inline]
    pub fn is_600khz(&self) -> bool {
        *self == FREQSELR::_600KHZ
    }
    #[doc = "Checks if the value of the field is `_1050KHZ`"]
    #[inline]
    pub fn is_1050khz(&self) -> bool {
        *self == FREQSELR::_1050KHZ
    }
    #[doc = "Checks if the value of the field is `_1400KHZ`"]
    #[inline]
    pub fn is_1400khz(&self) -> bool {
        *self == FREQSELR::_1400KHZ
    }
    #[doc = "Checks if the value of the field is `_1750KHZ`"]
    #[inline]
    pub fn is_1750khz(&self) -> bool {
        *self == FREQSELR::_1750KHZ
    }
    #[doc = "Checks if the value of the field is `_2100KHZ`"]
    #[inline]
    pub fn is_2100khz(&self) -> bool {
        *self == FREQSELR::_2100KHZ
    }
    #[doc = "Checks if the value of the field is `_2400KHZ`"]
    #[inline]
    pub fn is_2400khz(&self) -> bool {
        *self == FREQSELR::_2400KHZ
    }
    #[doc = "Checks if the value of the field is `_2700KHZ`"]
    #[inline]
    pub fn is_2700khz(&self) -> bool {
        *self == FREQSELR::_2700KHZ
    }
    #[doc = "Checks if the value of the field is `_3000KHZ`"]
    #[inline]
    pub fn is_3000khz(&self) -> bool {
        *self == FREQSELR::_3000KHZ
    }
    #[doc = "Checks if the value of the field is `_3250KHZ`"]
    #[inline]
    pub fn is_3250khz(&self) -> bool {
        *self == FREQSELR::_3250KHZ
    }
    #[doc = "Checks if the value of the field is `_3500KHZ`"]
    #[inline]
    pub fn is_3500khz(&self) -> bool {
        *self == FREQSELR::_3500KHZ
    }
    #[doc = "Checks if the value of the field is `_3750KHZ`"]
    #[inline]
    pub fn is_3750khz(&self) -> bool {
        *self == FREQSELR::_3750KHZ
    }
    #[doc = "Checks if the value of the field is `_4000KHZ`"]
    #[inline]
    pub fn is_4000khz(&self) -> bool {
        *self == FREQSELR::_4000KHZ
    }
    #[doc = "Checks if the value of the field is `_4200KHZ`"]
    #[inline]
    pub fn is_4200khz(&self) -> bool {
        *self == FREQSELR::_4200KHZ
    }
    #[doc = "Checks if the value of the field is `_4400KHZ`"]
    #[inline]
    pub fn is_4400khz(&self) -> bool {
        *self == FREQSELR::_4400KHZ
    }
    #[doc = "Checks if the value of the field is `_4600KHZ`"]
    #[inline]
    pub fn is_4600khz(&self) -> bool {
        *self == FREQSELR::_4600KHZ
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
    _600KHZ,
    #[doc = "1.05 MHz"]
    _1050KHZ,
    #[doc = "1.4 MHz"]
    _1400KHZ,
    #[doc = "1.75 MHz"]
    _1750KHZ,
    #[doc = "2.1 MHz"]
    _2100KHZ,
    #[doc = "2.4 MHz"]
    _2400KHZ,
    #[doc = "2.7 MHz"]
    _2700KHZ,
    #[doc = "3.0 MHz"]
    _3000KHZ,
    #[doc = "3.25 MHz"]
    _3250KHZ,
    #[doc = "3.5 MHz"]
    _3500KHZ,
    #[doc = "3.75 MHz"]
    _3750KHZ,
    #[doc = "4.0 MHz"]
    _4000KHZ,
    #[doc = "4.2 MHz"]
    _4200KHZ,
    #[doc = "4.4 MHz"]
    _4400KHZ,
    #[doc = "4.6 MHz"]
    _4600KHZ,
}
impl FREQSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FREQSELW::_600KHZ => 1,
            FREQSELW::_1050KHZ => 2,
            FREQSELW::_1400KHZ => 3,
            FREQSELW::_1750KHZ => 4,
            FREQSELW::_2100KHZ => 5,
            FREQSELW::_2400KHZ => 6,
            FREQSELW::_2700KHZ => 7,
            FREQSELW::_3000KHZ => 8,
            FREQSELW::_3250KHZ => 9,
            FREQSELW::_3500KHZ => 10,
            FREQSELW::_3750KHZ => 11,
            FREQSELW::_4000KHZ => 12,
            FREQSELW::_4200KHZ => 13,
            FREQSELW::_4400KHZ => 14,
            FREQSELW::_4600KHZ => 15,
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
    pub fn _600khz(self) -> &'a mut W {
        self.variant(FREQSELW::_600KHZ)
    }
    #[doc = "1.05 MHz"]
    #[inline]
    pub fn _1050khz(self) -> &'a mut W {
        self.variant(FREQSELW::_1050KHZ)
    }
    #[doc = "1.4 MHz"]
    #[inline]
    pub fn _1400khz(self) -> &'a mut W {
        self.variant(FREQSELW::_1400KHZ)
    }
    #[doc = "1.75 MHz"]
    #[inline]
    pub fn _1750khz(self) -> &'a mut W {
        self.variant(FREQSELW::_1750KHZ)
    }
    #[doc = "2.1 MHz"]
    #[inline]
    pub fn _2100khz(self) -> &'a mut W {
        self.variant(FREQSELW::_2100KHZ)
    }
    #[doc = "2.4 MHz"]
    #[inline]
    pub fn _2400khz(self) -> &'a mut W {
        self.variant(FREQSELW::_2400KHZ)
    }
    #[doc = "2.7 MHz"]
    #[inline]
    pub fn _2700khz(self) -> &'a mut W {
        self.variant(FREQSELW::_2700KHZ)
    }
    #[doc = "3.0 MHz"]
    #[inline]
    pub fn _3000khz(self) -> &'a mut W {
        self.variant(FREQSELW::_3000KHZ)
    }
    #[doc = "3.25 MHz"]
    #[inline]
    pub fn _3250khz(self) -> &'a mut W {
        self.variant(FREQSELW::_3250KHZ)
    }
    #[doc = "3.5 MHz"]
    #[inline]
    pub fn _3500khz(self) -> &'a mut W {
        self.variant(FREQSELW::_3500KHZ)
    }
    #[doc = "3.75 MHz"]
    #[inline]
    pub fn _3750khz(self) -> &'a mut W {
        self.variant(FREQSELW::_3750KHZ)
    }
    #[doc = "4.0 MHz"]
    #[inline]
    pub fn _4000khz(self) -> &'a mut W {
        self.variant(FREQSELW::_4000KHZ)
    }
    #[doc = "4.2 MHz"]
    #[inline]
    pub fn _4200khz(self) -> &'a mut W {
        self.variant(FREQSELW::_4200KHZ)
    }
    #[doc = "4.4 MHz"]
    #[inline]
    pub fn _4400khz(self) -> &'a mut W {
        self.variant(FREQSELW::_4400KHZ)
    }
    #[doc = "4.6 MHz"]
    #[inline]
    pub fn _4600khz(self) -> &'a mut W {
        self.variant(FREQSELW::_4600KHZ)
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
        W { bits: 0 }
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
