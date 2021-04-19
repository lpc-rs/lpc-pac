#[doc = "Register `EXTCLKSEL` reader"]
pub struct R(crate::R<EXTCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EXTCLKSEL_SPEC>> for R {
    fn from(reader: crate::R<EXTCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTCLKSEL` writer"]
pub struct W(crate::W<EXTCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTCLKSEL_SPEC>;
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
impl core::convert::From<crate::W<EXTCLKSEL_SPEC>> for W {
    fn from(writer: crate::W<EXTCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock source for external clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_A {
    #[doc = "0: System oscillator"]
    SYS_OSC = 0,
    #[doc = "1: Clk_in"]
    CLK_IN = 1,
}
impl From<SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEL` reader - Clock source for external clock"]
pub struct SEL_R(crate::FieldReader<bool, SEL_A>);
impl SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            false => SEL_A::SYS_OSC,
            true => SEL_A::CLK_IN,
        }
    }
    #[doc = "Checks if the value of the field is `SYS_OSC`"]
    #[inline(always)]
    pub fn is_sys_osc(&self) -> bool {
        **self == SEL_A::SYS_OSC
    }
    #[doc = "Checks if the value of the field is `CLK_IN`"]
    #[inline(always)]
    pub fn is_clk_in(&self) -> bool {
        **self == SEL_A::CLK_IN
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<bool, SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - Clock source for external clock"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "System oscillator"]
    #[inline(always)]
    pub fn sys_osc(self) -> &'a mut W {
        self.variant(SEL_A::SYS_OSC)
    }
    #[doc = "Clk_in"]
    #[inline(always)]
    pub fn clk_in(self) -> &'a mut W {
        self.variant(SEL_A::CLK_IN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clock source for external clock"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock source for external clock"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "external clock source select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extclksel](index.html) module"]
pub struct EXTCLKSEL_SPEC;
impl crate::RegisterSpec for EXTCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extclksel::R](R) reader structure"]
impl crate::Readable for EXTCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extclksel::W](W) writer structure"]
impl crate::Writable for EXTCLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTCLKSEL to value 0"]
impl crate::Resettable for EXTCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
