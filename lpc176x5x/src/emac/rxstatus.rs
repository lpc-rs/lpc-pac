#[doc = "Reader of register RXSTATUS"]
pub type R = crate::R<u32, super::RXSTATUS>;
#[doc = "Writer for register RXSTATUS"]
pub type W = crate::W<u32, super::RXSTATUS>;
#[doc = "Register RXSTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::RXSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXSTATUS`"]
pub type RXSTATUS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RXSTATUS`"]
pub struct RXSTATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - MSBs of receive status base address."]
    #[inline(always)]
    pub fn rxstatus(&self) -> RXSTATUS_R {
        RXSTATUS_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 3:31 - MSBs of receive status base address."]
    #[inline(always)]
    pub fn rxstatus(&mut self) -> RXSTATUS_W {
        RXSTATUS_W { w: self }
    }
}
