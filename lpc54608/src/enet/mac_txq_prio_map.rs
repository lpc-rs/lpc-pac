#[doc = "Reader of register MAC_TXQ_PRIO_MAP"]
pub type R = crate::R<u32, super::MAC_TXQ_PRIO_MAP>;
#[doc = "Writer for register MAC_TXQ_PRIO_MAP"]
pub type W = crate::W<u32, super::MAC_TXQ_PRIO_MAP>;
#[doc = "Register MAC_TXQ_PRIO_MAP `reset()`'s with value 0"]
impl crate::ResetValue for super::MAC_TXQ_PRIO_MAP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PSTQ0`"]
pub type PSTQ0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PSTQ0`"]
pub struct PSTQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> PSTQ0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `PSTQ1`"]
pub type PSTQ1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PSTQ1`"]
pub struct PSTQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> PSTQ1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Priorities Selected in Transmit Queue 0 This field holds the priorities assigned to Tx Queue 0 by the software."]
    #[inline(always)]
    pub fn pstq0(&self) -> PSTQ0_R {
        PSTQ0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Priorities Selected in Transmit Queue 1 This bit is similar to the PSTQ0 bit."]
    #[inline(always)]
    pub fn pstq1(&self) -> PSTQ1_R {
        PSTQ1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priorities Selected in Transmit Queue 0 This field holds the priorities assigned to Tx Queue 0 by the software."]
    #[inline(always)]
    pub fn pstq0(&mut self) -> PSTQ0_W {
        PSTQ0_W { w: self }
    }
    #[doc = "Bits 8:15 - Priorities Selected in Transmit Queue 1 This bit is similar to the PSTQ0 bit."]
    #[inline(always)]
    pub fn pstq1(&mut self) -> PSTQ1_W {
        PSTQ1_W { w: self }
    }
}
