#[doc = "Register `MAINCLKSEL` reader"]
pub struct R(crate::R<MAINCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAINCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAINCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAINCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAINCLKSEL` writer"]
pub struct W(crate::W<MAINCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAINCLKSEL_SPEC>;
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
impl From<crate::W<MAINCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAINCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "System PLL clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: FRO"]
    FRO = 0,
    #[doc = "1: External clock"]
    EXT_CLK = 1,
    #[doc = "2: Watchdog oscillator"]
    WDTOSC = 2,
    #[doc = "3: FRO_DIV"]
    FRO_DIV = 3,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEL` reader - System PLL clock source"]
pub struct SEL_R(crate::FieldReader<u8, SEL_A>);
impl SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            0 => SEL_A::FRO,
            1 => SEL_A::EXT_CLK,
            2 => SEL_A::WDTOSC,
            3 => SEL_A::FRO_DIV,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FRO`"]
    #[inline(always)]
    pub fn is_fro(&self) -> bool {
        **self == SEL_A::FRO
    }
    #[doc = "Checks if the value of the field is `EXT_CLK`"]
    #[inline(always)]
    pub fn is_ext_clk(&self) -> bool {
        **self == SEL_A::EXT_CLK
    }
    #[doc = "Checks if the value of the field is `WDTOSC`"]
    #[inline(always)]
    pub fn is_wdtosc(&self) -> bool {
        **self == SEL_A::WDTOSC
    }
    #[doc = "Checks if the value of the field is `FRO_DIV`"]
    #[inline(always)]
    pub fn is_fro_div(&self) -> bool {
        **self == SEL_A::FRO_DIV
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<u8, SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - System PLL clock source"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FRO"]
    #[inline(always)]
    pub fn fro(self) -> &'a mut W {
        self.variant(SEL_A::FRO)
    }
    #[doc = "External clock"]
    #[inline(always)]
    pub fn ext_clk(self) -> &'a mut W {
        self.variant(SEL_A::EXT_CLK)
    }
    #[doc = "Watchdog oscillator"]
    #[inline(always)]
    pub fn wdtosc(self) -> &'a mut W {
        self.variant(SEL_A::WDTOSC)
    }
    #[doc = "FRO_DIV"]
    #[inline(always)]
    pub fn fro_div(self) -> &'a mut W {
        self.variant(SEL_A::FRO_DIV)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main clock source select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mainclksel](index.html) module"]
pub struct MAINCLKSEL_SPEC;
impl crate::RegisterSpec for MAINCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mainclksel::R](R) reader structure"]
impl crate::Readable for MAINCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mainclksel::W](W) writer structure"]
impl crate::Writable for MAINCLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAINCLKSEL to value 0"]
impl crate::Resettable for MAINCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
