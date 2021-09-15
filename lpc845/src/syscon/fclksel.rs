#[doc = "Register `FCLKSEL[%s]` reader"]
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
#[doc = "Register `FCLKSEL[%s]` writer"]
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
#[doc = "Peripheral clock source\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: FRO"]
    FRO = 0,
    #[doc = "1: main clock"]
    MAIN_CLK = 1,
    #[doc = "2: Frg0clk"]
    FRG0CLK = 2,
    #[doc = "3: Frg1clk"]
    FRG1CLK = 3,
    #[doc = "4: FRO_DIV"]
    FRO_DIV = 4,
    #[doc = "7: none"]
    NONE = 7,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEL` reader - Peripheral clock source"]
pub struct SEL_R(crate::FieldReader<u8, SEL_A>);
impl SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEL_A> {
        match self.bits {
            0 => Some(SEL_A::FRO),
            1 => Some(SEL_A::MAIN_CLK),
            2 => Some(SEL_A::FRG0CLK),
            3 => Some(SEL_A::FRG1CLK),
            4 => Some(SEL_A::FRO_DIV),
            7 => Some(SEL_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FRO`"]
    #[inline(always)]
    pub fn is_fro(&self) -> bool {
        **self == SEL_A::FRO
    }
    #[doc = "Checks if the value of the field is `MAIN_CLK`"]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        **self == SEL_A::MAIN_CLK
    }
    #[doc = "Checks if the value of the field is `FRG0CLK`"]
    #[inline(always)]
    pub fn is_frg0clk(&self) -> bool {
        **self == SEL_A::FRG0CLK
    }
    #[doc = "Checks if the value of the field is `FRG1CLK`"]
    #[inline(always)]
    pub fn is_frg1clk(&self) -> bool {
        **self == SEL_A::FRG1CLK
    }
    #[doc = "Checks if the value of the field is `FRO_DIV`"]
    #[inline(always)]
    pub fn is_fro_div(&self) -> bool {
        **self == SEL_A::FRO_DIV
    }
    #[doc = "Checks if the value of the field is `NONE`"]
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
#[doc = "Field `SEL` writer - Peripheral clock source"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FRO"]
    #[inline(always)]
    pub fn fro(self) -> &'a mut W {
        self.variant(SEL_A::FRO)
    }
    #[doc = "main clock"]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut W {
        self.variant(SEL_A::MAIN_CLK)
    }
    #[doc = "Frg0clk"]
    #[inline(always)]
    pub fn frg0clk(self) -> &'a mut W {
        self.variant(SEL_A::FRG0CLK)
    }
    #[doc = "Frg1clk"]
    #[inline(always)]
    pub fn frg1clk(self) -> &'a mut W {
        self.variant(SEL_A::FRG1CLK)
    }
    #[doc = "FRO_DIV"]
    #[inline(always)]
    pub fn fro_div(self) -> &'a mut W {
        self.variant(SEL_A::FRO_DIV)
    }
    #[doc = "none"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SEL_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Peripheral clock source"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Peripheral clock source"]
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
#[doc = "peripheral clock source select register. FCLK0SEL~FCLK4SEL are for UART0~UART4 clock source select register. FCLK5SEL~FCLK8SEL are for I2C0~I2C3 clock source select register. FCLK9SEL~FCLK10SEL are for SPI0~SPI1 clock source select register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fclksel](index.html) module"]
pub struct FCLKSEL_SPEC;
impl crate::RegisterSpec for FCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fclksel::R](R) reader structure"]
impl crate::Readable for FCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fclksel::W](W) writer structure"]
impl crate::Writable for FCLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCLKSEL[%s]
to value 0x07"]
impl crate::Resettable for FCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
