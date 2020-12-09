#[doc = "Reader of register FCLKSEL[%s]"]
pub type R = crate::R<u32, super::FCLKSEL>;
#[doc = "Writer for register FCLKSEL[%s]"]
pub type W = crate::W<u32, super::FCLKSEL>;
#[doc = "Register FCLKSEL[%s]
`reset()`'s with value 0x07"]
impl crate::ResetValue for super::FCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
#[doc = "Flexcomm clock source selection. One per Flexcomm.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: FRO 12 MHz (fro_12m)"]
    FRO_12_MHZ = 0,
    #[doc = "1: FRO HF DIV (fro_hf_div)"]
    FRO_HF_DIV = 1,
    #[doc = "2: Audio PLL clock (audio_pll_clk)"]
    AUDIO_PLL_OUTPUT = 2,
    #[doc = "3: MCLK pin input, when selected in IOCON (mclk_in)"]
    MCLK_INPUT = 3,
    #[doc = "4: FRG clock, the output of the fractional rate generator (frg_clk)"]
    FRG_CLOCK_OUTPUT = 4,
    #[doc = "7: None, this may be selected in order to reduce power when no output is needed."]
    NONE = 7,
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
            0 => Val(SEL_A::FRO_12_MHZ),
            1 => Val(SEL_A::FRO_HF_DIV),
            2 => Val(SEL_A::AUDIO_PLL_OUTPUT),
            3 => Val(SEL_A::MCLK_INPUT),
            4 => Val(SEL_A::FRG_CLOCK_OUTPUT),
            7 => Val(SEL_A::NONE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FRO_12_MHZ`"]
    #[inline(always)]
    pub fn is_fro_12_mhz(&self) -> bool {
        *self == SEL_A::FRO_12_MHZ
    }
    #[doc = "Checks if the value of the field is `FRO_HF_DIV`"]
    #[inline(always)]
    pub fn is_fro_hf_div(&self) -> bool {
        *self == SEL_A::FRO_HF_DIV
    }
    #[doc = "Checks if the value of the field is `AUDIO_PLL_OUTPUT`"]
    #[inline(always)]
    pub fn is_audio_pll_output(&self) -> bool {
        *self == SEL_A::AUDIO_PLL_OUTPUT
    }
    #[doc = "Checks if the value of the field is `MCLK_INPUT`"]
    #[inline(always)]
    pub fn is_mclk_input(&self) -> bool {
        *self == SEL_A::MCLK_INPUT
    }
    #[doc = "Checks if the value of the field is `FRG_CLOCK_OUTPUT`"]
    #[inline(always)]
    pub fn is_frg_clock_output(&self) -> bool {
        *self == SEL_A::FRG_CLOCK_OUTPUT
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SEL_A::NONE
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
    #[doc = "FRO 12 MHz (fro_12m)"]
    #[inline(always)]
    pub fn fro_12_mhz(self) -> &'a mut W {
        self.variant(SEL_A::FRO_12_MHZ)
    }
    #[doc = "FRO HF DIV (fro_hf_div)"]
    #[inline(always)]
    pub fn fro_hf_div(self) -> &'a mut W {
        self.variant(SEL_A::FRO_HF_DIV)
    }
    #[doc = "Audio PLL clock (audio_pll_clk)"]
    #[inline(always)]
    pub fn audio_pll_output(self) -> &'a mut W {
        self.variant(SEL_A::AUDIO_PLL_OUTPUT)
    }
    #[doc = "MCLK pin input, when selected in IOCON (mclk_in)"]
    #[inline(always)]
    pub fn mclk_input(self) -> &'a mut W {
        self.variant(SEL_A::MCLK_INPUT)
    }
    #[doc = "FRG clock, the output of the fractional rate generator (frg_clk)"]
    #[inline(always)]
    pub fn frg_clock_output(self) -> &'a mut W {
        self.variant(SEL_A::FRG_CLOCK_OUTPUT)
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SEL_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Flexcomm clock source selection. One per Flexcomm."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Flexcomm clock source selection. One per Flexcomm."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
}
