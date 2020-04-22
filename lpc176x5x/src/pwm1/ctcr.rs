#[doc = "Reader of register CTCR"]
pub type R = crate::R<u32, super::CTCR>;
#[doc = "Writer for register CTCR"]
pub type W = crate::W<u32, super::CTCR>;
#[doc = "Register CTCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CTCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Counter/ Timer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MOD_A {
    #[doc = "0: Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale register."]
    TIMER_MODE_THE_TC_I = 0,
    #[doc = "1: Rising edge counter Mode: the TC is incremented on rising edges of the PWM_CAP input selected by bits 3:2."]
    RISING_EDGE_COUNTER_ = 1,
    #[doc = "2: Falling edge counter Mode: the TC is incremented on falling edges of the PWM_CAP input selected by bits 3:2."]
    FALLING_EDGE_COUNTER = 2,
    #[doc = "3: Dual edge counter Mode: the TC is incremented on both edges of the PWM_CAP input selected by bits 3:2."]
    DUAL_EDGE_COUNTER_MO = 3,
}
impl From<MOD_A> for u8 {
    #[inline(always)]
    fn from(variant: MOD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MOD`"]
pub type MOD_R = crate::R<u8, MOD_A>;
impl MOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOD_A {
        match self.bits {
            0 => MOD_A::TIMER_MODE_THE_TC_I,
            1 => MOD_A::RISING_EDGE_COUNTER_,
            2 => MOD_A::FALLING_EDGE_COUNTER,
            3 => MOD_A::DUAL_EDGE_COUNTER_MO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_MODE_THE_TC_I`"]
    #[inline(always)]
    pub fn is_timer_mode_the_tc_i(&self) -> bool {
        *self == MOD_A::TIMER_MODE_THE_TC_I
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE_COUNTER_`"]
    #[inline(always)]
    pub fn is_rising_edge_counter_(&self) -> bool {
        *self == MOD_A::RISING_EDGE_COUNTER_
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE_COUNTER`"]
    #[inline(always)]
    pub fn is_falling_edge_counter(&self) -> bool {
        *self == MOD_A::FALLING_EDGE_COUNTER
    }
    #[doc = "Checks if the value of the field is `DUAL_EDGE_COUNTER_MO`"]
    #[inline(always)]
    pub fn is_dual_edge_counter_mo(&self) -> bool {
        *self == MOD_A::DUAL_EDGE_COUNTER_MO
    }
}
#[doc = "Write proxy for field `MOD`"]
pub struct MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale register."]
    #[inline(always)]
    pub fn timer_mode_the_tc_i(self) -> &'a mut W {
        self.variant(MOD_A::TIMER_MODE_THE_TC_I)
    }
    #[doc = "Rising edge counter Mode: the TC is incremented on rising edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn rising_edge_counter_(self) -> &'a mut W {
        self.variant(MOD_A::RISING_EDGE_COUNTER_)
    }
    #[doc = "Falling edge counter Mode: the TC is incremented on falling edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn falling_edge_counter(self) -> &'a mut W {
        self.variant(MOD_A::FALLING_EDGE_COUNTER)
    }
    #[doc = "Dual edge counter Mode: the TC is incremented on both edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn dual_edge_counter_mo(self) -> &'a mut W {
        self.variant(MOD_A::DUAL_EDGE_COUNTER_MO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CIS_A {
    #[doc = "0: For PWM0: 00 = PWM0_CAP0 (Other combinations are reserved) For PWM1: 00 = PWM1_CAP0, 01 = PWM1_CAP1 (Other combinations are reserved)"]
    FOR_PWM0_00_EQ_PWM0_ = 0,
}
impl From<CIS_A> for u8 {
    #[inline(always)]
    fn from(variant: CIS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CIS`"]
pub type CIS_R = crate::R<u8, CIS_A>;
impl CIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CIS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CIS_A::FOR_PWM0_00_EQ_PWM0_),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FOR_PWM0_00_EQ_PWM0_`"]
    #[inline(always)]
    pub fn is_for_pwm0_00_eq_pwm0_(&self) -> bool {
        *self == CIS_A::FOR_PWM0_00_EQ_PWM0_
    }
}
#[doc = "Write proxy for field `CIS`"]
pub struct CIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CIS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "For PWM0: 00 = PWM0_CAP0 (Other combinations are reserved) For PWM1: 00 = PWM1_CAP0, 01 = PWM1_CAP1 (Other combinations are reserved)"]
    #[inline(always)]
    pub fn for_pwm0_00_eq_pwm0_(self) -> &'a mut W {
        self.variant(CIS_A::FOR_PWM0_00_EQ_PWM0_)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Counter/ Timer Mode"]
    #[inline(always)]
    pub fn mod_(&self) -> MOD_R {
        MOD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved."]
    #[inline(always)]
    pub fn cis(&self) -> CIS_R {
        CIS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Counter/ Timer Mode"]
    #[inline(always)]
    pub fn mod_(&mut self) -> MOD_W {
        MOD_W { w: self }
    }
    #[doc = "Bits 2:3 - Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved."]
    #[inline(always)]
    pub fn cis(&mut self) -> CIS_W {
        CIS_W { w: self }
    }
}
