#[doc = "Reader of register EFF_SA"]
pub type R = crate::R<u32, super::EFF_SA>;
#[doc = "Writer for register EFF_SA"]
pub type W = crate::W<u32, super::EFF_SA>;
#[doc = "Register EFF_SA `reset()`'s with value 0"]
impl crate::ResetValue for super::EFF_SA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFF_SA`"]
pub type EFF_SA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EFF_SA`"]
pub struct EFF_SA_W<'a> {
    w: &'a mut W,
}
impl<'a> EFF_SA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 2)) | (((value as u32) & 0x01ff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:10 - The start address of the table of individual Extended Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the EFF_GRP_sa register described below. The largest value that should be written to this register is 0x800, when both Extended Tables are empty and the last word (address 0x7FC) in AF Lookup Table RAM is used. For compatibility with possible future devices, please write zeroes in bits 31:11 and 1:0 of this register."]
    #[inline(always)]
    pub fn eff_sa(&self) -> EFF_SA_R {
        EFF_SA_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:10 - The start address of the table of individual Extended Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the EFF_GRP_sa register described below. The largest value that should be written to this register is 0x800, when both Extended Tables are empty and the last word (address 0x7FC) in AF Lookup Table RAM is used. For compatibility with possible future devices, please write zeroes in bits 31:11 and 1:0 of this register."]
    #[inline(always)]
    pub fn eff_sa(&mut self) -> EFF_SA_W {
        EFF_SA_W { w: self }
    }
}
