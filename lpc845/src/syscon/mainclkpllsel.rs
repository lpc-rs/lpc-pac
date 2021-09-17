#[doc = "Register `MAINCLKPLLSEL` reader"]
pub struct R(crate::R<MAINCLKPLLSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAINCLKPLLSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAINCLKPLLSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAINCLKPLLSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAINCLKPLLSEL` writer"]
pub struct W(crate::W<MAINCLKPLLSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAINCLKPLLSEL_SPEC>;
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
impl From<crate::W<MAINCLKPLLSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAINCLKPLLSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "System PLL clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: main_clk_pre_pll"]
    MAIN_CLK_PRE_PLL = 0,
    #[doc = "1: sys pll"]
    SYS_PLL = 1,
    #[doc = "2: none"]
    SEL_2 = 2,
    #[doc = "3: none"]
    SEL_3 = 3,
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
    pub(crate) fn new(bits: u8) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            0 => SEL_A::MAIN_CLK_PRE_PLL,
            1 => SEL_A::SYS_PLL,
            2 => SEL_A::SEL_2,
            3 => SEL_A::SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MAIN_CLK_PRE_PLL`"]
    #[inline(always)]
    pub fn is_main_clk_pre_pll(&self) -> bool {
        **self == SEL_A::MAIN_CLK_PRE_PLL
    }
    #[doc = "Checks if the value of the field is `SYS_PLL`"]
    #[inline(always)]
    pub fn is_sys_pll(&self) -> bool {
        **self == SEL_A::SYS_PLL
    }
    #[doc = "Checks if the value of the field is `SEL_2`"]
    #[inline(always)]
    pub fn is_sel_2(&self) -> bool {
        **self == SEL_A::SEL_2
    }
    #[doc = "Checks if the value of the field is `SEL_3`"]
    #[inline(always)]
    pub fn is_sel_3(&self) -> bool {
        **self == SEL_A::SEL_3
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
    #[doc = "main_clk_pre_pll"]
    #[inline(always)]
    pub fn main_clk_pre_pll(self) -> &'a mut W {
        self.variant(SEL_A::MAIN_CLK_PRE_PLL)
    }
    #[doc = "sys pll"]
    #[inline(always)]
    pub fn sys_pll(self) -> &'a mut W {
        self.variant(SEL_A::SYS_PLL)
    }
    #[doc = "none"]
    #[inline(always)]
    pub fn sel_2(self) -> &'a mut W {
        self.variant(SEL_A::SEL_2)
    }
    #[doc = "none"]
    #[inline(always)]
    pub fn sel_3(self) -> &'a mut W {
        self.variant(SEL_A::SEL_3)
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
#[doc = "Main clock source select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mainclkpllsel](index.html) module"]
pub struct MAINCLKPLLSEL_SPEC;
impl crate::RegisterSpec for MAINCLKPLLSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mainclkpllsel::R](R) reader structure"]
impl crate::Readable for MAINCLKPLLSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mainclkpllsel::W](W) writer structure"]
impl crate::Writable for MAINCLKPLLSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAINCLKPLLSEL to value 0"]
impl crate::Resettable for MAINCLKPLLSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
