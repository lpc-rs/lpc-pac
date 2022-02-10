///Register `FCLKSEL[%s]` reader
pub struct R(crate::R<FCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FCLKSEL[%s]` writer
pub struct W(crate::W<FCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
///Flexcomm clock source selection. One per Flexcomm.
///
///Value on reset: 7
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    ///0: FRO 12 MHz (fro_12m)
    FRO_12_MHZ = 0,
    ///1: FRO HF DIV (fro_hf_div)
    FRO_HF_DIV = 1,
    ///2: Audio PLL clock (audio_pll_clk)
    AUDIO_PLL_OUTPUT = 2,
    ///3: MCLK pin input, when selected in IOCON (mclk_in)
    MCLK_INPUT = 3,
    ///4: FRG clock, the output of the fractional rate generator (frg_clk)
    FRG_CLOCK_OUTPUT = 4,
    ///7: None, this may be selected in order to reduce power when no output is needed.
    NONE = 7,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
///Field `SEL` reader - Flexcomm clock source selection. One per Flexcomm.
pub struct SEL_R(crate::FieldReader<u8, SEL_A>);
impl SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SEL_A> {
        match self.bits {
            0 => Some(SEL_A::FRO_12_MHZ),
            1 => Some(SEL_A::FRO_HF_DIV),
            2 => Some(SEL_A::AUDIO_PLL_OUTPUT),
            3 => Some(SEL_A::MCLK_INPUT),
            4 => Some(SEL_A::FRG_CLOCK_OUTPUT),
            7 => Some(SEL_A::NONE),
            _ => None,
        }
    }
    ///Checks if the value of the field is `FRO_12_MHZ`
    #[inline(always)]
    pub fn is_fro_12_mhz(&self) -> bool {
        **self == SEL_A::FRO_12_MHZ
    }
    ///Checks if the value of the field is `FRO_HF_DIV`
    #[inline(always)]
    pub fn is_fro_hf_div(&self) -> bool {
        **self == SEL_A::FRO_HF_DIV
    }
    ///Checks if the value of the field is `AUDIO_PLL_OUTPUT`
    #[inline(always)]
    pub fn is_audio_pll_output(&self) -> bool {
        **self == SEL_A::AUDIO_PLL_OUTPUT
    }
    ///Checks if the value of the field is `MCLK_INPUT`
    #[inline(always)]
    pub fn is_mclk_input(&self) -> bool {
        **self == SEL_A::MCLK_INPUT
    }
    ///Checks if the value of the field is `FRG_CLOCK_OUTPUT`
    #[inline(always)]
    pub fn is_frg_clock_output(&self) -> bool {
        **self == SEL_A::FRG_CLOCK_OUTPUT
    }
    ///Checks if the value of the field is `NONE`
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == SEL_A::NONE
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<u8, SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SEL` writer - Flexcomm clock source selection. One per Flexcomm.
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///FRO 12 MHz (fro_12m)
    #[inline(always)]
    pub fn fro_12_mhz(self) -> &'a mut W {
        self.variant(SEL_A::FRO_12_MHZ)
    }
    ///FRO HF DIV (fro_hf_div)
    #[inline(always)]
    pub fn fro_hf_div(self) -> &'a mut W {
        self.variant(SEL_A::FRO_HF_DIV)
    }
    ///Audio PLL clock (audio_pll_clk)
    #[inline(always)]
    pub fn audio_pll_output(self) -> &'a mut W {
        self.variant(SEL_A::AUDIO_PLL_OUTPUT)
    }
    ///MCLK pin input, when selected in IOCON (mclk_in)
    #[inline(always)]
    pub fn mclk_input(self) -> &'a mut W {
        self.variant(SEL_A::MCLK_INPUT)
    }
    ///FRG clock, the output of the fractional rate generator (frg_clk)
    #[inline(always)]
    pub fn frg_clock_output(self) -> &'a mut W {
        self.variant(SEL_A::FRG_CLOCK_OUTPUT)
    }
    ///None, this may be selected in order to reduce power when no output is needed.
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SEL_A::NONE)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    ///Bits 0:2 - Flexcomm clock source selection. One per Flexcomm.
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    ///Bits 0:2 - Flexcomm clock source selection. One per Flexcomm.
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Flexcomm 0 clock source select
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fclksel](index.html) module
pub struct FCLKSEL_SPEC;
impl crate::RegisterSpec for FCLKSEL_SPEC {
    type Ux = u32;
}
///`read()` method returns [fclksel::R](R) reader structure
impl crate::Readable for FCLKSEL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fclksel::W](W) writer structure
impl crate::Writable for FCLKSEL_SPEC {
    type Writer = W;
}
///`reset()` method sets FCLKSEL[%s]
///to value 0x07
impl crate::Resettable for FCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
