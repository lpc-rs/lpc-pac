#[doc = "Reader of register FLOWCONTROLCOUNTER"]
pub type R = crate::R<u32, super::FLOWCONTROLCOUNTER>;
#[doc = "Writer for register FLOWCONTROLCOUNTER"]
pub type W = crate::W<u32, super::FLOWCONTROLCOUNTER>;
#[doc = "Register FLOWCONTROLCOUNTER `reset()`'s with value 0"]
impl crate::ResetValue for super::FLOWCONTROLCOUNTER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MC`"]
pub type MC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MC`"]
pub struct MC_W<'a> {
    w: &'a mut W,
}
impl<'a> MC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `PT`"]
pub type PT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PT`"]
pub struct PT_W<'a> {
    w: &'a mut W,
}
impl<'a> PT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - MirrorCounter. In full duplex mode the MirrorCounter specifies the number of cycles before re-issuing the Pause control frame."]
    #[inline(always)]
    pub fn mc(&self) -> MC_R {
        MC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - PauseTimer. In full-duplex mode the PauseTimer specifies the value that is inserted into the pause timer field of a pause flow control frame. In half duplex mode the PauseTimer specifies the number of backpressure cycles."]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MirrorCounter. In full duplex mode the MirrorCounter specifies the number of cycles before re-issuing the Pause control frame."]
    #[inline(always)]
    pub fn mc(&mut self) -> MC_W {
        MC_W { w: self }
    }
    #[doc = "Bits 16:31 - PauseTimer. In full-duplex mode the PauseTimer specifies the value that is inserted into the pause timer field of a pause flow control frame. In half duplex mode the PauseTimer specifies the number of backpressure cycles."]
    #[inline(always)]
    pub fn pt(&mut self) -> PT_W {
        PT_W { w: self }
    }
}
