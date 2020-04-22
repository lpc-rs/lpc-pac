#[doc = "Reader of register TCR"]
pub type R = crate::R<u32, super::TCR>;
#[doc = "Writer for register TCR"]
pub type W = crate::W<u32, super::TCR>;
#[doc = "Register TCR `reset()`'s with value 0"]
impl crate::ResetValue for super::TCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `CEN`"]
pub type CEN_R = crate::R<bool, CEN_A>;
impl CEN_R {
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
        *self == CEN_A::THE_COUNTERS_ARE_DIS
    }
    #[doc = "Checks if the value of the field is `THE_TIMER_COUNTER_AN`"]
    #[inline(always)]
    pub fn is_the_timer_counter_an(&self) -> bool {
        *self == CEN_A::THE_TIMER_COUNTER_AN
    }
}
#[doc = "Write proxy for field `CEN`"]
pub struct CEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
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
#[doc = "Reader of field `CRST`"]
pub type CRST_R = crate::R<bool, CRST_A>;
impl CRST_R {
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
        *self == CRST_A::DO_NOTHING
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CRST_A::RESET
    }
}
#[doc = "Write proxy for field `CRST`"]
pub struct CRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
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
}
