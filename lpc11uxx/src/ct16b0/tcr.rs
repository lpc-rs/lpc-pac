#[doc = "Register `TCR` reader"]
pub struct R(crate::R<TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCR` writer"]
pub struct W(crate::W<TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCR_SPEC>;
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
impl From<crate::W<TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Counter enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEN_A {
    #[doc = "0: The counters are disabled."]
    THE_COUNTERS_ARE_DIS = 0,
    #[doc = "1: The Timer Counter and Prescale Counter are enabled for counting."]
    THE_TIMER_COUNTER_AN = 1,
}
impl From<CEN_A> for bool {
    #[inline(always)]
    fn from(variant: CEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEN` reader - Counter enable."]
pub struct CEN_R(crate::FieldReader<bool, CEN_A>);
impl CEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEN_A {
        match self.bits {
            false => CEN_A::THE_COUNTERS_ARE_DIS,
            true => CEN_A::THE_TIMER_COUNTER_AN,
        }
    }
    #[doc = "Checks if the value of the field is `THE_COUNTERS_ARE_DIS`"]
    #[inline(always)]
    pub fn is_the_counters_are_dis(&self) -> bool {
        **self == CEN_A::THE_COUNTERS_ARE_DIS
    }
    #[doc = "Checks if the value of the field is `THE_TIMER_COUNTER_AN`"]
    #[inline(always)]
    pub fn is_the_timer_counter_an(&self) -> bool {
        **self == CEN_A::THE_TIMER_COUNTER_AN
    }
}
impl core::ops::Deref for CEN_R {
    type Target = crate::FieldReader<bool, CEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEN` writer - Counter enable."]
pub struct CEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The counters are disabled."]
    #[inline(always)]
    pub fn the_counters_are_dis(self) -> &'a mut W {
        self.variant(CEN_A::THE_COUNTERS_ARE_DIS)
    }
    #[doc = "The Timer Counter and Prescale Counter are enabled for counting."]
    #[inline(always)]
    pub fn the_timer_counter_an(self) -> &'a mut W {
        self.variant(CEN_A::THE_TIMER_COUNTER_AN)
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
#[doc = "Counter reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRST_A {
    #[doc = "0: Do nothing."]
    DO_NOTHING = 0,
    #[doc = "1: The Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until TCR\\[1\\]
is returned to zero."]
    RESET = 1,
}
impl From<CRST_A> for bool {
    #[inline(always)]
    fn from(variant: CRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRST` reader - Counter reset."]
pub struct CRST_R(crate::FieldReader<bool, CRST_A>);
impl CRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRST_A {
        match self.bits {
            false => CRST_A::DO_NOTHING,
            true => CRST_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `DO_NOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        **self == CRST_A::DO_NOTHING
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == CRST_A::RESET
    }
}
impl core::ops::Deref for CRST_R {
    type Target = crate::FieldReader<bool, CRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRST` writer - Counter reset."]
pub struct CRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Do nothing."]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(CRST_A::DO_NOTHING)
    }
    #[doc = "The Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until TCR\\[1\\]
is returned to zero."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Counter enable."]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Counter reset."]
    #[inline(always)]
    pub fn crst(&self) -> CRST_R {
        CRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter enable."]
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W {
        CEN_W { w: self }
    }
    #[doc = "Bit 1 - Counter reset."]
    #[inline(always)]
    pub fn crst(&mut self) -> CRST_W {
        CRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Control Register. The TCR is used to control the Timer Counter functions. The Timer Counter can be disabled or reset through the TCR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr](index.html) module"]
pub struct TCR_SPEC;
impl crate::RegisterSpec for TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcr::R](R) reader structure"]
impl crate::Readable for TCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcr::W](W) writer structure"]
impl crate::Writable for TCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
