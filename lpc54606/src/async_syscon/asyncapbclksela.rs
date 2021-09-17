#[doc = "Register `ASYNCAPBCLKSELA` reader"]
pub struct R(crate::R<ASYNCAPBCLKSELA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASYNCAPBCLKSELA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASYNCAPBCLKSELA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASYNCAPBCLKSELA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASYNCAPBCLKSELA` writer"]
pub struct W(crate::W<ASYNCAPBCLKSELA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASYNCAPBCLKSELA_SPEC>;
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
impl From<crate::W<ASYNCAPBCLKSELA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASYNCAPBCLKSELA_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `SEL` reader - Clock source for asynchronous clock source selector A"]
pub struct SEL_R(crate::FieldReader<u8, SEL_A>);
impl SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
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
        **self == SEL_A::MAIN_CLOCK
    }
    #[doc = "Checks if the value of the field is `FRO_12_MHZ`"]
    #[inline(always)]
    pub fn is_fro_12_mhz(&self) -> bool {
        **self == SEL_A::FRO_12_MHZ
    }
    #[doc = "Checks if the value of the field is `AUDIO_PLL_CLOCK`"]
    #[inline(always)]
    pub fn is_audio_pll_clock(&self) -> bool {
        **self == SEL_A::AUDIO_PLL_CLOCK
    }
    #[doc = "Checks if the value of the field is `FC6_FCLK`"]
    #[inline(always)]
    pub fn is_fc6_fclk(&self) -> bool {
        **self == SEL_A::FC6_FCLK
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<u8, SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - Clock source for asynchronous clock source selector A"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Async APB clock source select A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asyncapbclksela](index.html) module"]
pub struct ASYNCAPBCLKSELA_SPEC;
impl crate::RegisterSpec for ASYNCAPBCLKSELA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [asyncapbclksela::R](R) reader structure"]
impl crate::Readable for ASYNCAPBCLKSELA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asyncapbclksela::W](W) writer structure"]
impl crate::Writable for ASYNCAPBCLKSELA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ASYNCAPBCLKSELA to value 0"]
impl crate::Resettable for ASYNCAPBCLKSELA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
