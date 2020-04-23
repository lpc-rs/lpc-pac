#[doc = "Reader of register MAC_SYS_TIME_SCND_UPD"]
pub type R = crate::R<u32, super::MAC_SYS_TIME_SCND_UPD>;
#[doc = "Writer for register MAC_SYS_TIME_SCND_UPD"]
pub type W = crate::W<u32, super::MAC_SYS_TIME_SCND_UPD>;
#[doc = "Register MAC_SYS_TIME_SCND_UPD `reset()`'s with value 0"]
impl crate::ResetValue for super::MAC_SYS_TIME_SCND_UPD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSS`"]
pub type TSS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TSS`"]
pub struct TSS_W<'a> {
    w: &'a mut W,
}
impl<'a> TSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Time stamp second The value in this field indicates the time, in seconds, to be initialized or added to the system time."]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Time stamp second The value in this field indicates the time, in seconds, to be initialized or added to the system time."]
    #[inline(always)]
    pub fn tss(&mut self) -> TSS_W {
        TSS_W { w: self }
    }
}
