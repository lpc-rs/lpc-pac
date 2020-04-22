#[doc = "Reader of register SYSPLLCLKSEL"]
pub type R = crate::R<u32, super::SYSPLLCLKSEL>;
#[doc = "Writer for register SYSPLLCLKSEL"]
pub type W = crate::W<u32, super::SYSPLLCLKSEL>;
#[doc = "Register SYSPLLCLKSEL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::SYSPLLCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "System PLL clock source\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: IRC"]
    IRC = 0,
    #[doc = "1: Crystal Oscillator (SYSOSC)"]
    CRYSTAL_OSCILLATOR = 1,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<u8, SEL_A>;
impl SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            0 => SEL_A::IRC,
            1 => SEL_A::CRYSTAL_OSCILLATOR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IRC`"]
    #[inline(always)]
    pub fn is_irc(&self) -> bool {
        *self == SEL_A::IRC
    }
    #[doc = "Checks if the value of the field is `CRYSTAL_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_crystal_oscillator(&self) -> bool {
        *self == SEL_A::CRYSTAL_OSCILLATOR
    }
}
#[doc = "Write proxy for field `SEL`"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "IRC"]
    #[inline(always)]
    pub fn irc(self) -> &'a mut W {
        self.variant(SEL_A::IRC)
    }
    #[doc = "Crystal Oscillator (SYSOSC)"]
    #[inline(always)]
    pub fn crystal_oscillator(self) -> &'a mut W {
        self.variant(SEL_A::CRYSTAL_OSCILLATOR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - System PLL clock source"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System PLL clock source"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
}
