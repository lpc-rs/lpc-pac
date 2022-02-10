///Register `FRGCLKSEL` reader
pub struct R(crate::R<FRGCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRGCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRGCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRGCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FRGCLKSEL` writer
pub struct W(crate::W<FRGCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRGCLKSEL_SPEC>;
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
impl From<crate::W<FRGCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRGCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
///Fractional Rate Generator clock source select.
///
///Value on reset: 7
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    ///0: Main clock (main_clk)
    MAIN_CLOCK = 0,
    ///1: System PLL output (pll_clk)
    SYSTEM_PLL_OUTPUT = 1,
    ///2: FRO 12 MHz (fro_12m)
    FRO_12_MHZ = 2,
    ///3: FRO 96 or 48 MHz (fro_hf)
    FRO_HF = 3,
    ///7: None, this may be selected in order to reduce power when no output is needed.
    NONE = 7,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
///Field `SEL` reader - Fractional Rate Generator clock source select.
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
            0 => Some(SEL_A::MAIN_CLOCK),
            1 => Some(SEL_A::SYSTEM_PLL_OUTPUT),
            2 => Some(SEL_A::FRO_12_MHZ),
            3 => Some(SEL_A::FRO_HF),
            7 => Some(SEL_A::NONE),
            _ => None,
        }
    }
    ///Checks if the value of the field is `MAIN_CLOCK`
    #[inline(always)]
    pub fn is_main_clock(&self) -> bool {
        **self == SEL_A::MAIN_CLOCK
    }
    ///Checks if the value of the field is `SYSTEM_PLL_OUTPUT`
    #[inline(always)]
    pub fn is_system_pll_output(&self) -> bool {
        **self == SEL_A::SYSTEM_PLL_OUTPUT
    }
    ///Checks if the value of the field is `FRO_12_MHZ`
    #[inline(always)]
    pub fn is_fro_12_mhz(&self) -> bool {
        **self == SEL_A::FRO_12_MHZ
    }
    ///Checks if the value of the field is `FRO_HF`
    #[inline(always)]
    pub fn is_fro_hf(&self) -> bool {
        **self == SEL_A::FRO_HF
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
///Field `SEL` writer - Fractional Rate Generator clock source select.
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Main clock (main_clk)
    #[inline(always)]
    pub fn main_clock(self) -> &'a mut W {
        self.variant(SEL_A::MAIN_CLOCK)
    }
    ///System PLL output (pll_clk)
    #[inline(always)]
    pub fn system_pll_output(self) -> &'a mut W {
        self.variant(SEL_A::SYSTEM_PLL_OUTPUT)
    }
    ///FRO 12 MHz (fro_12m)
    #[inline(always)]
    pub fn fro_12_mhz(self) -> &'a mut W {
        self.variant(SEL_A::FRO_12_MHZ)
    }
    ///FRO 96 or 48 MHz (fro_hf)
    #[inline(always)]
    pub fn fro_hf(self) -> &'a mut W {
        self.variant(SEL_A::FRO_HF)
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
    ///Bits 0:2 - Fractional Rate Generator clock source select.
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    ///Bits 0:2 - Fractional Rate Generator clock source select.
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
///Fractional Rate Generator clock source select
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [frgclksel](index.html) module
pub struct FRGCLKSEL_SPEC;
impl crate::RegisterSpec for FRGCLKSEL_SPEC {
    type Ux = u32;
}
///`read()` method returns [frgclksel::R](R) reader structure
impl crate::Readable for FRGCLKSEL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [frgclksel::W](W) writer structure
impl crate::Writable for FRGCLKSEL_SPEC {
    type Writer = W;
}
///`reset()` method sets FRGCLKSEL to value 0x07
impl crate::Resettable for FRGCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
