#[doc = "Reader of register SFF_SA"]
pub type R = crate::R<u32, super::SFF_SA>;
#[doc = "Writer for register SFF_SA"]
pub type W = crate::W<u32, super::SFF_SA>;
#[doc = "Register SFF_SA `reset()`'s with value 0"]
impl crate::ResetValue for super::SFF_SA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SFF_SA`"]
pub type SFF_SA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SFF_SA`"]
pub struct SFF_SA_W<'a> {
    w: &'a mut W,
}
impl<'a> SFF_SA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 2)) | (((value as u32) & 0x01ff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:10 - The start address of the table of individual Standard Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the SFF_GRP_sa register described below. For compatibility with possible future devices, write zeroes in bits 31:11 and 1:0 of this register. If the eFCAN bit in the AFMR is 1, this value also indicates the size of the table of Standard IDs which the Acceptance Filter will search and (if found) automatically store received messages in Acceptance Filter RAM."]
    #[inline(always)]
    pub fn sff_sa(&self) -> SFF_SA_R {
        SFF_SA_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:10 - The start address of the table of individual Standard Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the SFF_GRP_sa register described below. For compatibility with possible future devices, write zeroes in bits 31:11 and 1:0 of this register. If the eFCAN bit in the AFMR is 1, this value also indicates the size of the table of Standard IDs which the Acceptance Filter will search and (if found) automatically store received messages in Acceptance Filter RAM."]
    #[inline(always)]
    pub fn sff_sa(&mut self) -> SFF_SA_W {
        SFF_SA_W { w: self }
    }
}
