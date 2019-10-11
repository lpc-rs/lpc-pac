#[doc = "Reader of register TXSTATUS"]
pub type R = crate::R<u32, super::TXSTATUS>;
#[doc = "Writer for register TXSTATUS"]
pub type W = crate::W<u32, super::TXSTATUS>;
#[doc = "Register TXSTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::TXSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXSTAT`"]
pub type TXSTAT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TXSTAT`"]
pub struct TXSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSTAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - TxStatus. MSBs of transmit status base address."]
    #[inline(always)]
    pub fn txstat(&self) -> TXSTAT_R {
        TXSTAT_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - TxStatus. MSBs of transmit status base address."]
    #[inline(always)]
    pub fn txstat(&mut self) -> TXSTAT_W {
        TXSTAT_W { w: self }
    }
}
