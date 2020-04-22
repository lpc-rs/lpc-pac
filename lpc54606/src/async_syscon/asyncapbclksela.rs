#[doc = "Reader of register ASYNCAPBCLKSELA"]
pub type R = crate::R<u32, super::ASYNCAPBCLKSELA>;
#[doc = "Writer for register ASYNCAPBCLKSELA"]
pub type W = crate::W<u32, super::ASYNCAPBCLKSELA>;
#[doc = "Register ASYNCAPBCLKSELA `reset()`'s with value 0"]
impl crate::ResetValue for super::ASYNCAPBCLKSELA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock source for asynchronous clock source selector A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: Main clock (main_clk)"]
    MAIN_CLOCK = 0,
    #[doc = "1: FRO 12 MHz (fro_12m)"]
    FRO_12_MHZ = 1,
    #[doc = "2: Audio PLL clock.(AUDPLL_BYPASS)"]
    AUDIO_PLL_CLOCK = 2,
    #[doc = "3: fc6 fclk (fc6_fclk)"]
    FC6_FCLK = 3,
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
            0 => SEL_A::MAIN_CLOCK,
            1 => SEL_A::FRO_12_MHZ,
            2 => SEL_A::AUDIO_PLL_CLOCK,
            3 => SEL_A::FC6_FCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MAIN_CLOCK`"]
    #[inline(always)]
    pub fn is_main_clock(&self) -> bool {
        *self == SEL_A::MAIN_CLOCK
    }
    #[doc = "Checks if the value of the field is `FRO_12_MHZ`"]
    #[inline(always)]
    pub fn is_fro_12_mhz(&self) -> bool {
        *self == SEL_A::FRO_12_MHZ
    }
    #[doc = "Checks if the value of the field is `AUDIO_PLL_CLOCK`"]
    #[inline(always)]
    pub fn is_audio_pll_clock(&self) -> bool {
        *self == SEL_A::AUDIO_PLL_CLOCK
    }
    #[doc = "Checks if the value of the field is `FC6_FCLK`"]
    #[inline(always)]
    pub fn is_fc6_fclk(&self) -> bool {
        *self == SEL_A::FC6_FCLK
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
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Main clock (main_clk)"]
    #[inline(always)]
    pub fn main_clock(self) -> &'a mut W {
        self.variant(SEL_A::MAIN_CLOCK)
    }
    #[doc = "FRO 12 MHz (fro_12m)"]
    #[inline(always)]
    pub fn fro_12_mhz(self) -> &'a mut W {
        self.variant(SEL_A::FRO_12_MHZ)
    }
    #[doc = "Audio PLL clock.(AUDPLL_BYPASS)"]
    #[inline(always)]
    pub fn audio_pll_clock(self) -> &'a mut W {
        self.variant(SEL_A::AUDIO_PLL_CLOCK)
    }
    #[doc = "fc6 fclk (fc6_fclk)"]
    #[inline(always)]
    pub fn fc6_fclk(self) -> &'a mut W {
        self.variant(SEL_A::FC6_FCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock source for asynchronous clock source selector A"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source for asynchronous clock source selector A"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
}
