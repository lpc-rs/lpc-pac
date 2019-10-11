#[doc = "Reader of register TOUCH"]
pub type R = crate::R<u32, super::TOUCH>;
#[doc = "Writer for register TOUCH"]
pub type W = crate::W<u32, super::TOUCH>;
#[doc = "Register TOUCH `reset()`'s with value 0"]
impl crate::ResetValue for super::TOUCH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `XVAL`"]
pub type XVAL_R = crate::R<u8, u8>;
#[doc = "Reader of field `ISTOUCH`"]
pub type ISTOUCH_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISTO`"]
pub type ISTO_R = crate::R<bool, bool>;
#[doc = "Reader of field `SEQ`"]
pub type SEQ_R = crate::R<u8, u8>;
#[doc = "Reader of field `CHANGE`"]
pub type CHANGE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:11 - Count value reached at trigger. If timeout, will be (1 bigger than TOUT)-1; e.g. if TOUT=12, then 0xFFF."]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - Is the X that triggered this, or lowest X if more than one."]
    #[inline(always)]
    pub fn xval(&self) -> XVAL_R {
        XVAL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 1 if is Touch (by count) or 0 if is no-touch."]
    #[inline(always)]
    pub fn istouch(&self) -> ISTOUCH_R {
        ISTOUCH_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 1 if is Timeout."]
    #[inline(always)]
    pub fn isto(&self) -> ISTO_R {
        ISTO_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - Sequence number - rolling counter of polls. Changes after all selected Xs per poll (so, 0 for 1st set of Xs, then 1 for next set, etc)."]
    #[inline(always)]
    pub fn seq(&self) -> SEQ_R {
        SEQ_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - If 1, the rest of the register is 0 because the data is changing. This will only happen for 1 cycle and would never happen if using interrupts to read, unless took so long as to overrun."]
    #[inline(always)]
    pub fn change(&self) -> CHANGE_R {
        CHANGE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {}
