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
    #[doc = "1 system clock flash access time."]
    ONE_SYSTEM_CLOCK_FLASH_ACCESS,
    #[doc = "2 system clock flash access time."]
    TWO_SYSTEM_CLOCK_FLASH_ACCESS,
    #[doc = "3 system clock flash access time."]
    THREE_SYSTEM_CLOCK_FLASH_ACCESS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FLASHTIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLASHTIMR::ONE_SYSTEM_CLOCK_FLASH_ACCESS => 0,
            FLASHTIMR::TWO_SYSTEM_CLOCK_FLASH_ACCESS => 1,
            FLASHTIMR::THREE_SYSTEM_CLOCK_FLASH_ACCESS => 2,
            FLASHTIMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLASHTIMR {
        match value {
            0 => FLASHTIMR::ONE_SYSTEM_CLOCK_FLASH_ACCESS,
            1 => FLASHTIMR::TWO_SYSTEM_CLOCK_FLASH_ACCESS,
            2 => FLASHTIMR::THREE_SYSTEM_CLOCK_FLASH_ACCESS,
            i => FLASHTIMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ONE_SYSTEM_CLOCK_FLASH_ACCESS`"]
    #[inline]
    pub fn is_one_system_clock_flash_access(&self) -> bool {
        *self == FLASHTIMR::ONE_SYSTEM_CLOCK_FLASH_ACCESS
    }
    #[doc = "Checks if the value of the field is `TWO_SYSTEM_CLOCK_FLASH_ACCESS`"]
    #[inline]
    pub fn is_two_system_clock_flash_access(&self) -> bool {
        *self == FLASHTIMR::TWO_SYSTEM_CLOCK_FLASH_ACCESS
    }
    #[doc = "Checks if the value of the field is `THREE_SYSTEM_CLOCK_FLASH_ACCESS`"]
    #[inline]
    pub fn is_three_system_clock_flash_access(&self) -> bool {
        *self == FLASHTIMR::THREE_SYSTEM_CLOCK_FLASH_ACCESS
    }
}
#[doc = "Values that can be written to the field `FLASHTIM`"]
pub enum FLASHTIMW {
    #[doc = "1 system clock flash access time."]
    ONE_SYSTEM_CLOCK_FLASH_ACCESS,
    #[doc = "2 system clock flash access time."]
    TWO_SYSTEM_CLOCK_FLASH_ACCESS,
    #[doc = "3 system clock flash access time."]
    THREE_SYSTEM_CLOCK_FLASH_ACCESS,
}
impl FLASHTIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLASHTIMW::ONE_SYSTEM_CLOCK_FLASH_ACCESS => 0,
            FLASHTIMW::TWO_SYSTEM_CLOCK_FLASH_ACCESS => 1,
            FLASHTIMW::THREE_SYSTEM_CLOCK_FLASH_ACCESS => 2,
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
    #[doc = "1 system clock flash access time."]
    #[inline]
    pub fn one_system_clock_flash_access(self) -> &'a mut W {
        self.variant(FLASHTIMW::ONE_SYSTEM_CLOCK_FLASH_ACCESS)
    }
    #[doc = "2 system clock flash access time."]
    #[inline]
    pub fn two_system_clock_flash_access(self) -> &'a mut W {
        self.variant(FLASHTIMW::TWO_SYSTEM_CLOCK_FLASH_ACCESS)
    }
    #[doc = "3 system clock flash access time."]
    #[inline]
    pub fn three_system_clock_flash_access(self) -> &'a mut W {
        self.variant(FLASHTIMW::THREE_SYSTEM_CLOCK_FLASH_ACCESS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
        W { bits: 2 }
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
