#[doc = "Register `MAINCLKSELA` reader"]
pub struct R(crate::R<MAINCLKSELA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAINCLKSELA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAINCLKSELA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAINCLKSELA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAINCLKSELA` writer"]
pub struct W(crate::W<MAINCLKSELA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAINCLKSELA_SPEC>;
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
impl From<crate::W<MAINCLKSELA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAINCLKSELA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock source for main clock source selector A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: FRO 12 MHz (fro_12m)"]
    FRO_12_MHZ = 0,
    #[doc = "1: CLKIN (clk_in)"]
    CLKIN = 1,
    #[doc = "2: Watchdog oscillator (wdt_clk)"]
    WATCHDOG_OSCILLATOR = 2,
    #[doc = "3: FRO 96 or 48 MHz (fro_hf)"]
    FRO_HF = 3,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEL` reader - Clock source for main clock source selector A"]
pub struct SEL_R(crate::FieldReader<u8, SEL_A>);
impl SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            0 => SEL_A::FRO_12_MHZ,
            1 => SEL_A::CLKIN,
            2 => SEL_A::WATCHDOG_OSCILLATOR,
            3 => SEL_A::FRO_HF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FRO_12_MHZ`"]
    #[inline(always)]
    pub fn is_fro_12_mhz(&self) -> bool {
        **self == SEL_A::FRO_12_MHZ
    }
    #[doc = "Checks if the value of the field is `CLKIN`"]
    #[inline(always)]
    pub fn is_clkin(&self) -> bool {
        **self == SEL_A::CLKIN
    }
    #[doc = "Checks if the value of the field is `WATCHDOG_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_watchdog_oscillator(&self) -> bool {
        **self == SEL_A::WATCHDOG_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `FRO_HF`"]
    #[inline(always)]
    pub fn is_fro_hf(&self) -> bool {
        **self == SEL_A::FRO_HF
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<u8, SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - Clock source for main clock source selector A"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FRO 12 MHz (fro_12m)"]
    #[inline(always)]
    pub fn fro_12_mhz(self) -> &'a mut W {
        self.variant(SEL_A::FRO_12_MHZ)
    }
    #[doc = "CLKIN (clk_in)"]
    #[inline(always)]
    pub fn clkin(self) -> &'a mut W {
        self.variant(SEL_A::CLKIN)
    }
    #[doc = "Watchdog oscillator (wdt_clk)"]
    #[inline(always)]
    pub fn watchdog_oscillator(self) -> &'a mut W {
        self.variant(SEL_A::WATCHDOG_OSCILLATOR)
    }
    #[doc = "FRO 96 or 48 MHz (fro_hf)"]
    #[inline(always)]
    pub fn fro_hf(self) -> &'a mut W {
        self.variant(SEL_A::FRO_HF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock source for main clock source selector A"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source for main clock source selector A"]
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
#[doc = "Main clock source select A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mainclksela](index.html) module"]
pub struct MAINCLKSELA_SPEC;
impl crate::RegisterSpec for MAINCLKSELA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mainclksela::R](R) reader structure"]
impl crate::Readable for MAINCLKSELA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mainclksela::W](W) writer structure"]
impl crate::Writable for MAINCLKSELA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAINCLKSELA to value 0"]
impl crate::Resettable for MAINCLKSELA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
