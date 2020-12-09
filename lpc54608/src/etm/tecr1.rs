#[doc = "Reader of register TECR1"]
pub type R = crate::R<u32, super::TECR1>;
#[doc = "Writer for register TECR1"]
pub type W = crate::W<u32, super::TECR1>;
#[doc = "Register TECR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::TECR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Trace start/stop enable. The trace start/stop resource, resource 0x5F, is unaffected by the value of this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRACECONTROLENABLE_A {
    #[doc = "0: Tracing is unaffected by the trace start/stop logic."]
    TRACECONTROLENABLE_0 = 0,
    #[doc = "1: Tracing is controlled by the trace on and off addresses configured for the trace start/stop logic."]
    TRACECONTROLENABLE_1 = 1,
}
impl From<TRACECONTROLENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: TRACECONTROLENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TraceControlEnable`"]
pub type TRACECONTROLENABLE_R = crate::R<bool, TRACECONTROLENABLE_A>;
impl TRACECONTROLENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRACECONTROLENABLE_A {
        match self.bits {
            false => TRACECONTROLENABLE_A::TRACECONTROLENABLE_0,
            true => TRACECONTROLENABLE_A::TRACECONTROLENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRACECONTROLENABLE_0`"]
    #[inline(always)]
    pub fn is_trace_control_enable_0(&self) -> bool {
        *self == TRACECONTROLENABLE_A::TRACECONTROLENABLE_0
    }
    #[doc = "Checks if the value of the field is `TRACECONTROLENABLE_1`"]
    #[inline(always)]
    pub fn is_trace_control_enable_1(&self) -> bool {
        *self == TRACECONTROLENABLE_A::TRACECONTROLENABLE_1
    }
}
#[doc = "Write proxy for field `TraceControlEnable`"]
pub struct TRACECONTROLENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACECONTROLENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRACECONTROLENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tracing is unaffected by the trace start/stop logic."]
    #[inline(always)]
    pub fn trace_control_enable_0(self) -> &'a mut W {
        self.variant(TRACECONTROLENABLE_A::TRACECONTROLENABLE_0)
    }
    #[doc = "Tracing is controlled by the trace on and off addresses configured for the trace start/stop logic."]
    #[inline(always)]
    pub fn trace_control_enable_1(self) -> &'a mut W {
        self.variant(TRACECONTROLENABLE_A::TRACECONTROLENABLE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 25 - Trace start/stop enable. The trace start/stop resource, resource 0x5F, is unaffected by the value of this bit."]
    #[inline(always)]
    pub fn trace_control_enable(&self) -> TRACECONTROLENABLE_R {
        TRACECONTROLENABLE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - Trace start/stop enable. The trace start/stop resource, resource 0x5F, is unaffected by the value of this bit."]
    #[inline(always)]
    pub fn trace_control_enable(&mut self) -> TRACECONTROLENABLE_W {
        TRACECONTROLENABLE_W { w: self }
    }
}
