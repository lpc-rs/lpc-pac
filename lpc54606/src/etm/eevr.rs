#[doc = "Reader of register EEVR"]
pub type R = crate::R<u32, super::EEVR>;
#[doc = "Writer for register EEVR"]
pub type W = crate::W<u32, super::EEVR>;
#[doc = "Register EEVR `reset()`'s with value 0"]
impl crate::ResetValue for super::EEVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TraceEnableEvent`"]
pub type TRACEENABLEEVENT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TraceEnableEvent`"]
pub struct TRACEENABLEEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACEENABLEEVENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - Trace Enable event."]
    #[inline(always)]
    pub fn trace_enable_event(&self) -> TRACEENABLEEVENT_R {
        TRACEENABLEEVENT_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - Trace Enable event."]
    #[inline(always)]
    pub fn trace_enable_event(&mut self) -> TRACEENABLEEVENT_W {
        TRACEENABLEEVENT_W { w: self }
    }
}
