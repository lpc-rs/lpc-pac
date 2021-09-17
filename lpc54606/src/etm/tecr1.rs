#[doc = "Register `TECR1` reader"]
pub struct R(crate::R<TECR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TECR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TECR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TECR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TECR1` writer"]
pub struct W(crate::W<TECR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TECR1_SPEC>;
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
impl From<crate::W<TECR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TECR1_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `TraceControlEnable` reader - Trace start/stop enable. The trace start/stop resource, resource 0x5F, is unaffected by the value of this bit."]
pub struct TRACECONTROLENABLE_R(crate::FieldReader<bool, TRACECONTROLENABLE_A>);
impl TRACECONTROLENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRACECONTROLENABLE_R(crate::FieldReader::new(bits))
    }
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
        **self == TRACECONTROLENABLE_A::TRACECONTROLENABLE_0
    }
    #[doc = "Checks if the value of the field is `TRACECONTROLENABLE_1`"]
    #[inline(always)]
    pub fn is_trace_control_enable_1(&self) -> bool {
        **self == TRACECONTROLENABLE_A::TRACECONTROLENABLE_1
    }
}
impl core::ops::Deref for TRACECONTROLENABLE_R {
    type Target = crate::FieldReader<bool, TRACECONTROLENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TraceControlEnable` writer - Trace start/stop enable. The trace start/stop resource, resource 0x5F, is unaffected by the value of this bit."]
pub struct TRACECONTROLENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACECONTROLENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRACECONTROLENABLE_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trace Enable Control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tecr1](index.html) module"]
pub struct TECR1_SPEC;
impl crate::RegisterSpec for TECR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tecr1::R](R) reader structure"]
impl crate::Readable for TECR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tecr1::W](W) writer structure"]
impl crate::Writable for TECR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TECR1 to value 0"]
impl crate::Resettable for TECR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
