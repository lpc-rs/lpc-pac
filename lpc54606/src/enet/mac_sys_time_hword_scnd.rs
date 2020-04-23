#[doc = "Reader of register MAC_SYS_TIME_HWORD_SCND"]
pub type R = crate::R<u32, super::MAC_SYS_TIME_HWORD_SCND>;
#[doc = "Writer for register MAC_SYS_TIME_HWORD_SCND"]
pub type W = crate::W<u32, super::MAC_SYS_TIME_HWORD_SCND>;
#[doc = "Register MAC_SYS_TIME_HWORD_SCND `reset()`'s with value 0"]
impl crate::ResetValue for super::MAC_SYS_TIME_HWORD_SCND {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSHWR`"]
pub type TSHWR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TSHWR`"]
pub struct TSHWR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSHWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Time stamp higher word Contains the most significant 16-bits of the Time stamp seconds value."]
    #[inline(always)]
    pub fn tshwr(&self) -> TSHWR_R {
        TSHWR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Time stamp higher word Contains the most significant 16-bits of the Time stamp seconds value."]
    #[inline(always)]
    pub fn tshwr(&mut self) -> TSHWR_W {
        TSHWR_W { w: self }
    }
}
