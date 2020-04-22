#[doc = "Reader of register FLASHCFG"]
pub type R = crate::R<u32, super::FLASHCFG>;
#[doc = "Writer for register FLASHCFG"]
pub type W = crate::W<u32, super::FLASHCFG>;
#[doc = "Register FLASHCFG `reset()`'s with value 0x01"]
impl crate::ResetValue for super::FLASHCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Flash memory access time. FLASHTIM +1 is equal to the number of system clocks used for flash access.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASHTIM_A {
    #[doc = "0: 1 system clock flash access time."]
    ONE_SYSTEM_CLOCK_FLASH_ACCESS = 0,
    #[doc = "1: 2 system clock flash access time."]
    TWO_SYSTEM_CLOCK_FLASH_ACCESS = 1,
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
            0 => Val(FLASHTIM_A::ONE_SYSTEM_CLOCK_FLASH_ACCESS),
            1 => Val(FLASHTIM_A::TWO_SYSTEM_CLOCK_FLASH_ACCESS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ONE_SYSTEM_CLOCK_FLASH_ACCESS`"]
    #[inline(always)]
    pub fn is_one_system_clock_flash_access(&self) -> bool {
        *self == FLASHTIM_A::ONE_SYSTEM_CLOCK_FLASH_ACCESS
    }
    #[doc = "Checks if the value of the field is `TWO_SYSTEM_CLOCK_FLASH_ACCESS`"]
    #[inline(always)]
    pub fn is_two_system_clock_flash_access(&self) -> bool {
        *self == FLASHTIM_A::TWO_SYSTEM_CLOCK_FLASH_ACCESS
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
    #[doc = "1 system clock flash access time."]
    #[inline(always)]
    pub fn one_system_clock_flash_access(self) -> &'a mut W {
        self.variant(FLASHTIM_A::ONE_SYSTEM_CLOCK_FLASH_ACCESS)
    }
    #[doc = "2 system clock flash access time."]
    #[inline(always)]
    pub fn two_system_clock_flash_access(self) -> &'a mut W {
        self.variant(FLASHTIM_A::TWO_SYSTEM_CLOCK_FLASH_ACCESS)
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
