#[doc = "Reader of register TRACEIDR"]
pub type R = crate::R<u32, super::TRACEIDR>;
#[doc = "Writer for register TRACEIDR"]
pub type W = crate::W<u32, super::TRACEIDR>;
#[doc = "Register TRACEIDR `reset()`'s with value 0"]
impl crate::ResetValue for super::TRACEIDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TraceID`"]
pub type TRACEID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TraceID`"]
pub struct TRACEID_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACEID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Trace ID to output onto the trace bus. On an ETM reset this field is cleared to 0x00."]
    #[inline(always)]
    pub fn trace_id(&self) -> TRACEID_R {
        TRACEID_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Trace ID to output onto the trace bus. On an ETM reset this field is cleared to 0x00."]
    #[inline(always)]
    pub fn trace_id(&mut self) -> TRACEID_W {
        TRACEID_W { w: self }
    }
}
