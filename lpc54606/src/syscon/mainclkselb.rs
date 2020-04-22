#[doc = "Reader of register MAINCLKSELB"]
pub type R = crate::R<u32, super::MAINCLKSELB>;
#[doc = "Writer for register MAINCLKSELB"]
pub type W = crate::W<u32, super::MAINCLKSELB>;
#[doc = "Register MAINCLKSELB `reset()`'s with value 0"]
impl crate::ResetValue for super::MAINCLKSELB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock source for main clock source selector B. Selects the clock source for the main clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: MAINCLKSELA. Use the clock source selected in MAINCLKSELA register."]
    MAINCLKSELA = 0,
    #[doc = "2: System PLL output (pll_clk)"]
    SYSTEM_PLL_OUTPUT = 2,
    #[doc = "3: RTC oscillator 32 kHz output (32k_clk)"]
    RTC_OSC_OUTPUT = 3,
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
    pub fn variant(&self) -> crate::Variant<u8, SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SEL_A::MAINCLKSELA),
            2 => Val(SEL_A::SYSTEM_PLL_OUTPUT),
            3 => Val(SEL_A::RTC_OSC_OUTPUT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MAINCLKSELA`"]
    #[inline(always)]
    pub fn is_mainclksela(&self) -> bool {
        *self == SEL_A::MAINCLKSELA
    }
    #[doc = "Checks if the value of the field is `SYSTEM_PLL_OUTPUT`"]
    #[inline(always)]
    pub fn is_system_pll_output(&self) -> bool {
        *self == SEL_A::SYSTEM_PLL_OUTPUT
    }
    #[doc = "Checks if the value of the field is `RTC_OSC_OUTPUT`"]
    #[inline(always)]
    pub fn is_rtc_osc_output(&self) -> bool {
        *self == SEL_A::RTC_OSC_OUTPUT
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
    #[doc = "MAINCLKSELA. Use the clock source selected in MAINCLKSELA register."]
    #[inline(always)]
    pub fn mainclksela(self) -> &'a mut W {
        self.variant(SEL_A::MAINCLKSELA)
    }
    #[doc = "System PLL output (pll_clk)"]
    #[inline(always)]
    pub fn system_pll_output(self) -> &'a mut W {
        self.variant(SEL_A::SYSTEM_PLL_OUTPUT)
    }
    #[doc = "RTC oscillator 32 kHz output (32k_clk)"]
    #[inline(always)]
    pub fn rtc_osc_output(self) -> &'a mut W {
        self.variant(SEL_A::RTC_OSC_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock source for main clock source selector B. Selects the clock source for the main clock."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source for main clock source selector B. Selects the clock source for the main clock."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
}
