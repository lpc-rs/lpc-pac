///Register `LCDCLKSEL` reader
pub struct R(crate::R<LCDCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LCDCLKSEL` writer
pub struct W(crate::W<LCDCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDCLKSEL_SPEC>;
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
impl From<crate::W<LCDCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
///LCD clock source select.
///
///Value on reset: 3
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    ///0: Main clock (main_clk)
    MAIN_CLOCK = 0,
    ///1: LCDCLKIN (LCDCLK_EXT)
    LCDCLKIN = 1,
    ///2: FRO 96 or 48 MHz (fro_hf)
    FRO_HF = 2,
    ///3: None, this may be selected in order to reduce power when no output is needed.
    NONE = 3,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
///Field `SEL` reader - LCD clock source select.
pub struct SEL_R(crate::FieldReader<u8, SEL_A>);
impl SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            0 => SEL_A::MAIN_CLOCK,
            1 => SEL_A::LCDCLKIN,
            2 => SEL_A::FRO_HF,
            3 => SEL_A::NONE,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `MAIN_CLOCK`
    #[inline(always)]
    pub fn is_main_clock(&self) -> bool {
        **self == SEL_A::MAIN_CLOCK
    }
    ///Checks if the value of the field is `LCDCLKIN`
    #[inline(always)]
    pub fn is_lcdclkin(&self) -> bool {
        **self == SEL_A::LCDCLKIN
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
///Field `SEL` writer - LCD clock source select.
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Main clock (main_clk)
    #[inline(always)]
    pub fn main_clock(self) -> &'a mut W {
        self.variant(SEL_A::MAIN_CLOCK)
    }
    ///LCDCLKIN (LCDCLK_EXT)
    #[inline(always)]
    pub fn lcdclkin(self) -> &'a mut W {
        self.variant(SEL_A::LCDCLKIN)
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
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    ///Bits 0:1 - LCD clock source select.
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    ///Bits 0:1 - LCD clock source select.
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
///LCD clock source select
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lcdclksel](index.html) module
pub struct LCDCLKSEL_SPEC;
impl crate::RegisterSpec for LCDCLKSEL_SPEC {
    type Ux = u32;
}
///`read()` method returns [lcdclksel::R](R) reader structure
impl crate::Readable for LCDCLKSEL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [lcdclksel::W](W) writer structure
impl crate::Writable for LCDCLKSEL_SPEC {
    type Writer = W;
}
///`reset()` method sets LCDCLKSEL to value 0x03
impl crate::Resettable for LCDCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
