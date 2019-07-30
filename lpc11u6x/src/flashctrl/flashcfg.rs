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
    #[doc = "1 system clock flash access time (for system clock frequencies of up to 20 MHz)."]
    _1_SYSTEM_CLOCK_FLASH,
    #[doc = "2 system clocks flash access time (for system clock frequencies of up to 30 MHz)."]
    _2_SYSTEM_CLOCKS_FLASH,
    #[doc = "Reserved."]
    RESERVED_0,
    #[doc = "Reserved."]
    RESERVED_1,
}
impl FLASHTIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLASHTIMR::_1_SYSTEM_CLOCK_FLASH => 0,
            FLASHTIMR::_2_SYSTEM_CLOCKS_FLASH => 1,
            FLASHTIMR::RESERVED_0 => 2,
            FLASHTIMR::RESERVED_1 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLASHTIMR {
        match value {
            0 => FLASHTIMR::_1_SYSTEM_CLOCK_FLASH,
            1 => FLASHTIMR::_2_SYSTEM_CLOCKS_FLASH,
            2 => FLASHTIMR::RESERVED_0,
            3 => FLASHTIMR::RESERVED_1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1_SYSTEM_CLOCK_FLASH`"]
    #[inline]
    pub fn is_1_system_clock_flash(&self) -> bool {
        *self == FLASHTIMR::_1_SYSTEM_CLOCK_FLASH
    }
    #[doc = "Checks if the value of the field is `_2_SYSTEM_CLOCKS_FLASH`"]
    #[inline]
    pub fn is_2_system_clocks_flash(&self) -> bool {
        *self == FLASHTIMR::_2_SYSTEM_CLOCKS_FLASH
    }
    #[doc = "Checks if the value of the field is `RESERVED_0`"]
    #[inline]
    pub fn is_reserved_0(&self) -> bool {
        *self == FLASHTIMR::RESERVED_0
    }
    #[doc = "Checks if the value of the field is `RESERVED_1`"]
    #[inline]
    pub fn is_reserved_1(&self) -> bool {
        *self == FLASHTIMR::RESERVED_1
    }
}
#[doc = "Values that can be written to the field `FLASHTIM`"]
pub enum FLASHTIMW {
    #[doc = "1 system clock flash access time (for system clock frequencies of up to 20 MHz)."]
    _1_SYSTEM_CLOCK_FLASH,
    #[doc = "2 system clocks flash access time (for system clock frequencies of up to 30 MHz)."]
    _2_SYSTEM_CLOCKS_FLASH,
    #[doc = "Reserved."]
    RESERVED_0,
    #[doc = "Reserved."]
    RESERVED_1,
}
impl FLASHTIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLASHTIMW::_1_SYSTEM_CLOCK_FLASH => 0,
            FLASHTIMW::_2_SYSTEM_CLOCKS_FLASH => 1,
            FLASHTIMW::RESERVED_0 => 2,
            FLASHTIMW::RESERVED_1 => 3,
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
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1 system clock flash access time (for system clock frequencies of up to 20 MHz)."]
    #[inline]
    pub fn _1_system_clock_flash(self) -> &'a mut W {
        self.variant(FLASHTIMW::_1_SYSTEM_CLOCK_FLASH)
    }
    #[doc = "2 system clocks flash access time (for system clock frequencies of up to 30 MHz)."]
    #[inline]
    pub fn _2_system_clocks_flash(self) -> &'a mut W {
        self.variant(FLASHTIMW::_2_SYSTEM_CLOCKS_FLASH)
    }
    #[doc = "Reserved."]
    #[inline]
    pub fn reserved_0(self) -> &'a mut W {
        self.variant(FLASHTIMW::RESERVED_0)
    }
    #[doc = "Reserved."]
    #[inline]
    pub fn reserved_1(self) -> &'a mut W {
        self.variant(FLASHTIMW::RESERVED_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:1 - Flash memory access time. FLASHTIM +1 is equal to the number of system clocks used for flash access."]
    #[inline]
    pub fn flashtim(&self) -> FLASHTIMR {
        FLASHTIMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:1 - Flash memory access time. FLASHTIM +1 is equal to the number of system clocks used for flash access."]
    #[inline]
    pub fn flashtim(&mut self) -> _FLASHTIMW {
        _FLASHTIMW { w: self }
    }
}
