#[doc = "Reader of register SFF_GRP_SA"]
pub type R = crate::R<u32, super::SFF_GRP_SA>;
#[doc = "Writer for register SFF_GRP_SA"]
pub type W = crate::W<u32, super::SFF_GRP_SA>;
#[doc = "Register SFF_GRP_SA `reset()`'s with value 0"]
impl crate::ResetValue for super::SFF_GRP_SA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SFF_GRP_SA`"]
pub type SFF_GRP_SA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SFF_GRP_SA`"]
pub struct SFF_GRP_SA_W<'a> {
    w: &'a mut W,
}
impl<'a> SFF_GRP_SA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 2)) | (((value as u32) & 0x03ff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:11 - The start address of the table of grouped Standard Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the EFF_sa register described below. The largest value that should be written to this register is 0x800, when only the Standard Individual table is used, and the last word (address 0x7FC) in AF Lookup Table RAM is used. For compatibility with possible future devices, please write zeroes in bits 31:12 and 1:0 of this register."]
    #[inline(always)]
    pub fn sff_grp_sa(&self) -> SFF_GRP_SA_R {
        SFF_GRP_SA_R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:11 - The start address of the table of grouped Standard Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the EFF_sa register described below. The largest value that should be written to this register is 0x800, when only the Standard Individual table is used, and the last word (address 0x7FC) in AF Lookup Table RAM is used. For compatibility with possible future devices, please write zeroes in bits 31:12 and 1:0 of this register."]
    #[inline(always)]
    pub fn sff_grp_sa(&mut self) -> SFF_GRP_SA_W {
        SFF_GRP_SA_W { w: self }
    }
}
