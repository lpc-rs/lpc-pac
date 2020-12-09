#[doc = "Reader of register CONFIG"]
pub type R = crate::R<u32, super::CONFIG>;
#[doc = "Writer for register CONFIG"]
pub type W = crate::W<u32, super::CONFIG>;
#[doc = "Register CONFIG `reset()`'s with value 0x1e00"]
impl crate::ResetValue for super::CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1e00
    }
}
#[doc = "SCT operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNIFY_A {
    #[doc = "0: The SCT operates as two 16-bit counters named COUNTER_L and COUNTER_H."]
    DUAL_COUNTER = 0,
    #[doc = "1: The SCT operates as a unified 32-bit counter."]
    UNIFIED_COUNTER = 1,
}
impl From<UNIFY_A> for bool {
    #[inline(always)]
    fn from(variant: UNIFY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UNIFY`"]
pub type UNIFY_R = crate::R<bool, UNIFY_A>;
impl UNIFY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNIFY_A {
        match self.bits {
            false => UNIFY_A::DUAL_COUNTER,
            true => UNIFY_A::UNIFIED_COUNTER,
        }
    }
    #[doc = "Checks if the value of the field is `DUAL_COUNTER`"]
    #[inline(always)]
    pub fn is_dual_counter(&self) -> bool {
        *self == UNIFY_A::DUAL_COUNTER
    }
    #[doc = "Checks if the value of the field is `UNIFIED_COUNTER`"]
    #[inline(always)]
    pub fn is_unified_counter(&self) -> bool {
        *self == UNIFY_A::UNIFIED_COUNTER
    }
}
#[doc = "Write proxy for field `UNIFY`"]
pub struct UNIFY_W<'a> {
    w: &'a mut W,
}
impl<'a> UNIFY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNIFY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The SCT operates as two 16-bit counters named COUNTER_L and COUNTER_H."]
    #[inline(always)]
    pub fn dual_counter(self) -> &'a mut W {
        self.variant(UNIFY_A::DUAL_COUNTER)
    }
    #[doc = "The SCT operates as a unified 32-bit counter."]
    #[inline(always)]
    pub fn unified_counter(self) -> &'a mut W {
        self.variant(UNIFY_A::UNIFIED_COUNTER)
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
#[doc = "SCT clock mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKMODE_A {
    #[doc = "0: System Clock Mode. The system clock clocks the entire SCT module including the counter(s) and counter prescalers."]
    SYSTEM_CLOCK_MODE = 0,
    #[doc = "1: Sampled System Clock Mode. The system clock clocks the SCT module, but the counter and prescalers are only enabled to count when the designated edge is detected on the input selected by the CKSEL field. The minimum pulse width on the selected clock-gate input is 1 bus clock period. This mode is the high-performance, sampled-clock mode."]
    SAMPLED_SYSTEM_CLOCK_MODE = 1,
    #[doc = "2: SCT Input Clock Mode. The input/edge selected by the CKSEL field clocks the SCT module, including the counters and prescalers, after first being synchronized to the system clock. The minimum pulse width on the clock input is 1 bus clock period. This mode is the low-power, sampled-clock mode."]
    SCT_INPUT_CLOCK_MODE = 2,
    #[doc = "3: Asynchronous Mode. The entire SCT module is clocked directly by the input/edge selected by the CKSEL field. In this mode, the SCT outputs are switched synchronously to the SCT input clock - not the system clock. The input clock rate must be at least half the system clock rate and can be the same or faster than the system clock."]
    ASYNCHRONOUS_MODE = 3,
}
impl From<CLKMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKMODE`"]
pub type CLKMODE_R = crate::R<u8, CLKMODE_A>;
impl CLKMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKMODE_A {
        match self.bits {
            0 => CLKMODE_A::SYSTEM_CLOCK_MODE,
            1 => CLKMODE_A::SAMPLED_SYSTEM_CLOCK_MODE,
            2 => CLKMODE_A::SCT_INPUT_CLOCK_MODE,
            3 => CLKMODE_A::ASYNCHRONOUS_MODE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYSTEM_CLOCK_MODE`"]
    #[inline(always)]
    pub fn is_system_clock_mode(&self) -> bool {
        *self == CLKMODE_A::SYSTEM_CLOCK_MODE
    }
    #[doc = "Checks if the value of the field is `SAMPLED_SYSTEM_CLOCK_MODE`"]
    #[inline(always)]
    pub fn is_sampled_system_clock_mode(&self) -> bool {
        *self == CLKMODE_A::SAMPLED_SYSTEM_CLOCK_MODE
    }
    #[doc = "Checks if the value of the field is `SCT_INPUT_CLOCK_MODE`"]
    #[inline(always)]
    pub fn is_sct_input_clock_mode(&self) -> bool {
        *self == CLKMODE_A::SCT_INPUT_CLOCK_MODE
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS_MODE`"]
    #[inline(always)]
    pub fn is_asynchronous_mode(&self) -> bool {
        *self == CLKMODE_A::ASYNCHRONOUS_MODE
    }
}
#[doc = "Write proxy for field `CLKMODE`"]
pub struct CLKMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "System Clock Mode. The system clock clocks the entire SCT module including the counter(s) and counter prescalers."]
    #[inline(always)]
    pub fn system_clock_mode(self) -> &'a mut W {
        self.variant(CLKMODE_A::SYSTEM_CLOCK_MODE)
    }
    #[doc = "Sampled System Clock Mode. The system clock clocks the SCT module, but the counter and prescalers are only enabled to count when the designated edge is detected on the input selected by the CKSEL field. The minimum pulse width on the selected clock-gate input is 1 bus clock period. This mode is the high-performance, sampled-clock mode."]
    #[inline(always)]
    pub fn sampled_system_clock_mode(self) -> &'a mut W {
        self.variant(CLKMODE_A::SAMPLED_SYSTEM_CLOCK_MODE)
    }
    #[doc = "SCT Input Clock Mode. The input/edge selected by the CKSEL field clocks the SCT module, including the counters and prescalers, after first being synchronized to the system clock. The minimum pulse width on the clock input is 1 bus clock period. This mode is the low-power, sampled-clock mode."]
    #[inline(always)]
    pub fn sct_input_clock_mode(self) -> &'a mut W {
        self.variant(CLKMODE_A::SCT_INPUT_CLOCK_MODE)
    }
    #[doc = "Asynchronous Mode. The entire SCT module is clocked directly by the input/edge selected by the CKSEL field. In this mode, the SCT outputs are switched synchronously to the SCT input clock - not the system clock. The input clock rate must be at least half the system clock rate and can be the same or faster than the system clock."]
    #[inline(always)]
    pub fn asynchronous_mode(self) -> &'a mut W {
        self.variant(CLKMODE_A::ASYNCHRONOUS_MODE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "SCT clock select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKSEL_A {
    #[doc = "0: Rising edges on input 0."]
    INPUT_0_RISING_EDGES = 0,
    #[doc = "1: Falling edges on input 0."]
    INPUT_0_FALLING_EDGE = 1,
    #[doc = "2: Rising edges on input 1."]
    INPUT_1_RISING_EDGES = 2,
    #[doc = "3: Falling edges on input 1."]
    INPUT_1_FALLING_EDGE = 3,
    #[doc = "4: Rising edges on input 2."]
    INPUT_2_RISING_EDGES = 4,
    #[doc = "5: Falling edges on input 2."]
    INPUT_2_FALLING_EDGE = 5,
    #[doc = "6: Rising edges on input 3."]
    INPUT_3_RISING_EDGES = 6,
    #[doc = "7: Falling edges on input 3."]
    INPUT_3_FALLING_EDGE = 7,
    #[doc = "8: Rising edges on input 4."]
    INPUT_4_RISING_EDGES = 8,
    #[doc = "9: Falling edges on input 4."]
    INPUT_4_FALLING_EDGE = 9,
    #[doc = "10: Rising edges on input 5."]
    INPUT_5_RISING_EDGES = 10,
    #[doc = "11: Falling edges on input 5."]
    INPUT_5_FALLING_EDGE = 11,
    #[doc = "12: Rising edges on input 6."]
    INPUT_6_RISING_EDGES = 12,
    #[doc = "13: Falling edges on input 6."]
    INPUT_6_FALLING_EDGE = 13,
    #[doc = "14: Rising edges on input 7."]
    INPUT_7_RISING_EDGES = 14,
    #[doc = "15: Falling edges on input 7."]
    INPUT_7_FALLING_EDGE = 15,
}
impl From<CKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CKSEL`"]
pub type CKSEL_R = crate::R<u8, CKSEL_A>;
impl CKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKSEL_A {
        match self.bits {
            0 => CKSEL_A::INPUT_0_RISING_EDGES,
            1 => CKSEL_A::INPUT_0_FALLING_EDGE,
            2 => CKSEL_A::INPUT_1_RISING_EDGES,
            3 => CKSEL_A::INPUT_1_FALLING_EDGE,
            4 => CKSEL_A::INPUT_2_RISING_EDGES,
            5 => CKSEL_A::INPUT_2_FALLING_EDGE,
            6 => CKSEL_A::INPUT_3_RISING_EDGES,
            7 => CKSEL_A::INPUT_3_FALLING_EDGE,
            8 => CKSEL_A::INPUT_4_RISING_EDGES,
            9 => CKSEL_A::INPUT_4_FALLING_EDGE,
            10 => CKSEL_A::INPUT_5_RISING_EDGES,
            11 => CKSEL_A::INPUT_5_FALLING_EDGE,
            12 => CKSEL_A::INPUT_6_RISING_EDGES,
            13 => CKSEL_A::INPUT_6_FALLING_EDGE,
            14 => CKSEL_A::INPUT_7_RISING_EDGES,
            15 => CKSEL_A::INPUT_7_FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT_0_RISING_EDGES`"]
    #[inline(always)]
    pub fn is_input_0_rising_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_0_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_0_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_input_0_falling_edge(&self) -> bool {
        *self == CKSEL_A::INPUT_0_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `INPUT_1_RISING_EDGES`"]
    #[inline(always)]
    pub fn is_input_1_rising_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_1_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_1_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_input_1_falling_edge(&self) -> bool {
        *self == CKSEL_A::INPUT_1_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `INPUT_2_RISING_EDGES`"]
    #[inline(always)]
    pub fn is_input_2_rising_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_2_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_2_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_input_2_falling_edge(&self) -> bool {
        *self == CKSEL_A::INPUT_2_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `INPUT_3_RISING_EDGES`"]
    #[inline(always)]
    pub fn is_input_3_rising_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_3_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_3_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_input_3_falling_edge(&self) -> bool {
        *self == CKSEL_A::INPUT_3_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `INPUT_4_RISING_EDGES`"]
    #[inline(always)]
    pub fn is_input_4_rising_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_4_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_4_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_input_4_falling_edge(&self) -> bool {
        *self == CKSEL_A::INPUT_4_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `INPUT_5_RISING_EDGES`"]
    #[inline(always)]
    pub fn is_input_5_rising_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_5_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_5_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_input_5_falling_edge(&self) -> bool {
        *self == CKSEL_A::INPUT_5_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `INPUT_6_RISING_EDGES`"]
    #[inline(always)]
    pub fn is_input_6_rising_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_6_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_6_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_input_6_falling_edge(&self) -> bool {
        *self == CKSEL_A::INPUT_6_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `INPUT_7_RISING_EDGES`"]
    #[inline(always)]
    pub fn is_input_7_rising_edges(&self) -> bool {
        *self == CKSEL_A::INPUT_7_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_7_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_input_7_falling_edge(&self) -> bool {
        *self == CKSEL_A::INPUT_7_FALLING_EDGE
    }
}
#[doc = "Write proxy for field `CKSEL`"]
pub struct CKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Rising edges on input 0."]
    #[inline(always)]
    pub fn input_0_rising_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_0_RISING_EDGES)
    }
    #[doc = "Falling edges on input 0."]
    #[inline(always)]
    pub fn input_0_falling_edge(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_0_FALLING_EDGE)
    }
    #[doc = "Rising edges on input 1."]
    #[inline(always)]
    pub fn input_1_rising_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_1_RISING_EDGES)
    }
    #[doc = "Falling edges on input 1."]
    #[inline(always)]
    pub fn input_1_falling_edge(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_1_FALLING_EDGE)
    }
    #[doc = "Rising edges on input 2."]
    #[inline(always)]
    pub fn input_2_rising_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_2_RISING_EDGES)
    }
    #[doc = "Falling edges on input 2."]
    #[inline(always)]
    pub fn input_2_falling_edge(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_2_FALLING_EDGE)
    }
    #[doc = "Rising edges on input 3."]
    #[inline(always)]
    pub fn input_3_rising_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_3_RISING_EDGES)
    }
    #[doc = "Falling edges on input 3."]
    #[inline(always)]
    pub fn input_3_falling_edge(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_3_FALLING_EDGE)
    }
    #[doc = "Rising edges on input 4."]
    #[inline(always)]
    pub fn input_4_rising_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_4_RISING_EDGES)
    }
    #[doc = "Falling edges on input 4."]
    #[inline(always)]
    pub fn input_4_falling_edge(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_4_FALLING_EDGE)
    }
    #[doc = "Rising edges on input 5."]
    #[inline(always)]
    pub fn input_5_rising_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_5_RISING_EDGES)
    }
    #[doc = "Falling edges on input 5."]
    #[inline(always)]
    pub fn input_5_falling_edge(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_5_FALLING_EDGE)
    }
    #[doc = "Rising edges on input 6."]
    #[inline(always)]
    pub fn input_6_rising_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_6_RISING_EDGES)
    }
    #[doc = "Falling edges on input 6."]
    #[inline(always)]
    pub fn input_6_falling_edge(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_6_FALLING_EDGE)
    }
    #[doc = "Rising edges on input 7."]
    #[inline(always)]
    pub fn input_7_rising_edges(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_7_RISING_EDGES)
    }
    #[doc = "Falling edges on input 7."]
    #[inline(always)]
    pub fn input_7_falling_edge(self) -> &'a mut W {
        self.variant(CKSEL_A::INPUT_7_FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | (((value as u32) & 0x0f) << 3);
        self.w
    }
}
#[doc = "Reader of field `NORELOAD_L`"]
pub type NORELOAD_L_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NORELOAD_L`"]
pub struct NORELOAD_L_W<'a> {
    w: &'a mut W,
}
impl<'a> NORELOAD_L_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `NORELOAD_H`"]
pub type NORELOAD_H_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NORELOAD_H`"]
pub struct NORELOAD_H_W<'a> {
    w: &'a mut W,
}
impl<'a> NORELOAD_H_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `INSYNC`"]
pub type INSYNC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSYNC`"]
pub struct INSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> INSYNC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 9)) | (((value as u32) & 0x0f) << 9);
        self.w
    }
}
#[doc = "Reader of field `AUTOLIMIT_L`"]
pub type AUTOLIMIT_L_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOLIMIT_L`"]
pub struct AUTOLIMIT_L_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOLIMIT_L_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `AUTOLIMIT_H`"]
pub type AUTOLIMIT_H_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOLIMIT_H`"]
pub struct AUTOLIMIT_H_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOLIMIT_H_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SCT operation"]
    #[inline(always)]
    pub fn unify(&self) -> UNIFY_R {
        UNIFY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - SCT clock mode"]
    #[inline(always)]
    pub fn clkmode(&self) -> CLKMODE_R {
        CLKMODE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:6 - SCT clock select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register."]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - A 1 in this bit prevents the lower match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline(always)]
    pub fn noreload_l(&self) -> NORELOAD_L_R {
        NORELOAD_L_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - A 1 in this bit prevents the higher match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline(always)]
    pub fn noreload_h(&self) -> NORELOAD_H_R {
        NORELOAD_H_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:12 - Synchronization for input N (bit 9 = input 0, bit 10 = input 1,, bit 12 = input 3); all other bits are reserved. A 1 in one of these bits subjects the corresponding input to synchronization to the SCT clock, before it is used to create an event. If an input is known to already be synchronous to the SCT clock, this bit may be set to 0 for faster input response. (Note: The SCT clock is the system clock for CKMODEs 0-2. It is the selected, asynchronous SCT input clock for CKMODE3). Note that the INSYNC field only affects inputs used for event generation. It does not apply to the clock input specified in the CKSEL field."]
    #[inline(always)]
    pub fn insync(&self) -> INSYNC_R {
        INSYNC_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 17 - A one in this bit causes a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline(always)]
    pub fn autolimit_l(&self) -> AUTOLIMIT_L_R {
        AUTOLIMIT_L_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - A one in this bit will cause a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline(always)]
    pub fn autolimit_h(&self) -> AUTOLIMIT_H_R {
        AUTOLIMIT_H_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCT operation"]
    #[inline(always)]
    pub fn unify(&mut self) -> UNIFY_W {
        UNIFY_W { w: self }
    }
    #[doc = "Bits 1:2 - SCT clock mode"]
    #[inline(always)]
    pub fn clkmode(&mut self) -> CLKMODE_W {
        CLKMODE_W { w: self }
    }
    #[doc = "Bits 3:6 - SCT clock select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register."]
    #[inline(always)]
    pub fn cksel(&mut self) -> CKSEL_W {
        CKSEL_W { w: self }
    }
    #[doc = "Bit 7 - A 1 in this bit prevents the lower match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline(always)]
    pub fn noreload_l(&mut self) -> NORELOAD_L_W {
        NORELOAD_L_W { w: self }
    }
    #[doc = "Bit 8 - A 1 in this bit prevents the higher match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline(always)]
    pub fn noreload_h(&mut self) -> NORELOAD_H_W {
        NORELOAD_H_W { w: self }
    }
    #[doc = "Bits 9:12 - Synchronization for input N (bit 9 = input 0, bit 10 = input 1,, bit 12 = input 3); all other bits are reserved. A 1 in one of these bits subjects the corresponding input to synchronization to the SCT clock, before it is used to create an event. If an input is known to already be synchronous to the SCT clock, this bit may be set to 0 for faster input response. (Note: The SCT clock is the system clock for CKMODEs 0-2. It is the selected, asynchronous SCT input clock for CKMODE3). Note that the INSYNC field only affects inputs used for event generation. It does not apply to the clock input specified in the CKSEL field."]
    #[inline(always)]
    pub fn insync(&mut self) -> INSYNC_W {
        INSYNC_W { w: self }
    }
    #[doc = "Bit 17 - A one in this bit causes a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline(always)]
    pub fn autolimit_l(&mut self) -> AUTOLIMIT_L_W {
        AUTOLIMIT_L_W { w: self }
    }
    #[doc = "Bit 18 - A one in this bit will cause a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline(always)]
    pub fn autolimit_h(&mut self) -> AUTOLIMIT_H_W {
        AUTOLIMIT_H_W { w: self }
    }
}
