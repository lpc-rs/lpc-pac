#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLASHCFG {
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
#[doc = "Possible values of the field `FLASHTIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHTIMR {
    #[doc = "Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock."]
    _1CLK,
    #[doc = "Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock."]
    _2CLK,
    #[doc = "Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock."]
    _3CLK,
    #[doc = "Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock."]
    _4CLK,
    #[doc = "Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock. Use for up to 120 Mhz for LPC1759 and LPC1769 only."]
    _5CLK,
    #[doc = "Flash accesses use 6 CPU clocks. This safe setting will work under any conditions."]
    _6CLK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FLASHTIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLASHTIMR::_1CLK => 0,
            FLASHTIMR::_2CLK => 1,
            FLASHTIMR::_3CLK => 2,
            FLASHTIMR::_4CLK => 3,
            FLASHTIMR::_5CLK => 4,
            FLASHTIMR::_6CLK => 5,
            FLASHTIMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLASHTIMR {
        match value {
            0 => FLASHTIMR::_1CLK,
            1 => FLASHTIMR::_2CLK,
            2 => FLASHTIMR::_3CLK,
            3 => FLASHTIMR::_4CLK,
            4 => FLASHTIMR::_5CLK,
            5 => FLASHTIMR::_6CLK,
            i => FLASHTIMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1CLK`"]
    #[inline]
    pub fn is_1clk(&self) -> bool {
        *self == FLASHTIMR::_1CLK
    }
    #[doc = "Checks if the value of the field is `_2CLK`"]
    #[inline]
    pub fn is_2clk(&self) -> bool {
        *self == FLASHTIMR::_2CLK
    }
    #[doc = "Checks if the value of the field is `_3CLK`"]
    #[inline]
    pub fn is_3clk(&self) -> bool {
        *self == FLASHTIMR::_3CLK
    }
    #[doc = "Checks if the value of the field is `_4CLK`"]
    #[inline]
    pub fn is_4clk(&self) -> bool {
        *self == FLASHTIMR::_4CLK
    }
    #[doc = "Checks if the value of the field is `_5CLK`"]
    #[inline]
    pub fn is_5clk(&self) -> bool {
        *self == FLASHTIMR::_5CLK
    }
    #[doc = "Checks if the value of the field is `_6CLK`"]
    #[inline]
    pub fn is_6clk(&self) -> bool {
        *self == FLASHTIMR::_6CLK
    }
}
#[doc = "Values that can be written to the field `FLASHTIM`"]
pub enum FLASHTIMW {
    #[doc = "Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock."]
    _1CLK,
    #[doc = "Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock."]
    _2CLK,
    #[doc = "Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock."]
    _3CLK,
    #[doc = "Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock."]
    _4CLK,
    #[doc = "Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock. Use for up to 120 Mhz for LPC1759 and LPC1769 only."]
    _5CLK,
    #[doc = "Flash accesses use 6 CPU clocks. This safe setting will work under any conditions."]
    _6CLK,
}
impl FLASHTIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLASHTIMW::_1CLK => 0,
            FLASHTIMW::_2CLK => 1,
            FLASHTIMW::_3CLK => 2,
            FLASHTIMW::_4CLK => 3,
            FLASHTIMW::_5CLK => 4,
            FLASHTIMW::_6CLK => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASHTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASHTIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASHTIMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock."]
    #[inline]
    pub fn _1clk(self) -> &'a mut W {
        self.variant(FLASHTIMW::_1CLK)
    }
    #[doc = "Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock."]
    #[inline]
    pub fn _2clk(self) -> &'a mut W {
        self.variant(FLASHTIMW::_2CLK)
    }
    #[doc = "Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock."]
    #[inline]
    pub fn _3clk(self) -> &'a mut W {
        self.variant(FLASHTIMW::_3CLK)
    }
    #[doc = "Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock."]
    #[inline]
    pub fn _4clk(self) -> &'a mut W {
        self.variant(FLASHTIMW::_4CLK)
    }
    #[doc = "Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock. Use for up to 120 Mhz for LPC1759 and LPC1769 only."]
    #[inline]
    pub fn _5clk(self) -> &'a mut W {
        self.variant(FLASHTIMW::_5CLK)
    }
    #[doc = "Flash accesses use 6 CPU clocks. This safe setting will work under any conditions."]
    #[inline]
    pub fn _6clk(self) -> &'a mut W {
        self.variant(FLASHTIMW::_6CLK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
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
    #[doc = "Bits 12:15 - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. Other values are reserved."]
    #[inline]
    pub fn flashtim(&self) -> FLASHTIMR {
        FLASHTIMR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 12346 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 12:15 - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. Other values are reserved."]
    #[inline]
    pub fn flashtim(&mut self) -> _FLASHTIMW {
        _FLASHTIMW { w: self }
    }
}
