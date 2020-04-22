#[doc = "Reader of register FLASHCFG"]
pub type R = crate::R<u32, super::FLASHCFG>;
#[doc = "Writer for register FLASHCFG"]
pub type W = crate::W<u32, super::FLASHCFG>;
#[doc = "Register FLASHCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASHCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Flash memory access time. FLASHTIM +1 is equal to the number of system clocks used for flash access.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASHTIM_A {
    #[doc = "0: 1 system clock flash access time (for system clock frequencies of up to 20 MHz)."]
    _1_SYSTEM_CLOCK_FLASH = 0,
    #[doc = "1: 2 system clocks flash access time (for system clock frequencies of up to 40 MHz)."]
    _2_SYSTEM_CLOCKS_FLAS = 1,
    #[doc = "2: 3 system clocks flash access time (for system clock frequencies of up to 50 MHz)."]
    _3_SYSTEM_CLOCKS_FLAS = 2,
}
impl From<FLASHTIM_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASHTIM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLASHTIM`"]
pub type FLASHTIM_R = crate::R<u8, FLASHTIM_A>;
impl FLASHTIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FLASHTIM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FLASHTIM_A::_1_SYSTEM_CLOCK_FLASH),
            1 => Val(FLASHTIM_A::_2_SYSTEM_CLOCKS_FLAS),
            2 => Val(FLASHTIM_A::_3_SYSTEM_CLOCKS_FLAS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1_SYSTEM_CLOCK_FLASH`"]
    #[inline(always)]
    pub fn is_1_system_clock_flash(&self) -> bool {
        *self == FLASHTIM_A::_1_SYSTEM_CLOCK_FLASH
    }
    #[doc = "Checks if the value of the field is `_2_SYSTEM_CLOCKS_FLAS`"]
    #[inline(always)]
    pub fn is_2_system_clocks_flas(&self) -> bool {
        *self == FLASHTIM_A::_2_SYSTEM_CLOCKS_FLAS
    }
    #[doc = "Checks if the value of the field is `_3_SYSTEM_CLOCKS_FLAS`"]
    #[inline(always)]
    pub fn is_3_system_clocks_flas(&self) -> bool {
        *self == FLASHTIM_A::_3_SYSTEM_CLOCKS_FLAS
    }
}
#[doc = "Write proxy for field `FLASHTIM`"]
pub struct FLASHTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHTIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHTIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 system clock flash access time (for system clock frequencies of up to 20 MHz)."]
    #[inline(always)]
    pub fn _1_system_clock_flash(self) -> &'a mut W {
        self.variant(FLASHTIM_A::_1_SYSTEM_CLOCK_FLASH)
    }
    #[doc = "2 system clocks flash access time (for system clock frequencies of up to 40 MHz)."]
    #[inline(always)]
    pub fn _2_system_clocks_flas(self) -> &'a mut W {
        self.variant(FLASHTIM_A::_2_SYSTEM_CLOCKS_FLAS)
    }
    #[doc = "3 system clocks flash access time (for system clock frequencies of up to 50 MHz)."]
    #[inline(always)]
    pub fn _3_system_clocks_flas(self) -> &'a mut W {
        self.variant(FLASHTIM_A::_3_SYSTEM_CLOCKS_FLAS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Flash memory access time. FLASHTIM +1 is equal to the number of system clocks used for flash access."]
    #[inline(always)]
    pub fn flashtim(&self) -> FLASHTIM_R {
        FLASHTIM_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Flash memory access time. FLASHTIM +1 is equal to the number of system clocks used for flash access."]
    #[inline(always)]
    pub fn flashtim(&mut self) -> FLASHTIM_W {
        FLASHTIM_W { w: self }
    }
}
