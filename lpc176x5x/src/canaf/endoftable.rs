#[doc = "Reader of register ENDOFTABLE"]
pub type R = crate::R<u32, super::ENDOFTABLE>;
#[doc = "Writer for register ENDOFTABLE"]
pub type W = crate::W<u32, super::ENDOFTABLE>;
#[doc = "Register ENDOFTABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::ENDOFTABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENDOFTABLE`"]
pub type ENDOFTABLE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ENDOFTABLE`"]
pub struct ENDOFTABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDOFTABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 2)) | (((value as u32) & 0x03ff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:11 - The address above the last active address in the last active AF table. For compatibility with possible future devices, please write zeroes in bits 31:12 and 1:0 of this register. If the eFCAN bit in the AFMR is 0, the largest value that should be written to this register is 0x800, which allows the last word (address 0x7FC) in AF Lookup Table RAM to be used. If the eFCAN bit in the AFMR is 1, this value marks the start of the area of Acceptance Filter RAM, into which the Acceptance Filter will automatically receive messages for selected IDs on selected CAN buses. In this case, the maximum value that should be written to this register is 0x800 minus 6 times the value in SFF_sa. This allows 12 bytes of message storage between this address and the end of Acceptance Filter RAM, for each Standard ID that is specified between the start of Acceptance Filter RAM, and the next active AF table."]
    #[inline(always)]
    pub fn endoftable(&self) -> ENDOFTABLE_R {
        ENDOFTABLE_R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:11 - The address above the last active address in the last active AF table. For compatibility with possible future devices, please write zeroes in bits 31:12 and 1:0 of this register. If the eFCAN bit in the AFMR is 0, the largest value that should be written to this register is 0x800, which allows the last word (address 0x7FC) in AF Lookup Table RAM to be used. If the eFCAN bit in the AFMR is 1, this value marks the start of the area of Acceptance Filter RAM, into which the Acceptance Filter will automatically receive messages for selected IDs on selected CAN buses. In this case, the maximum value that should be written to this register is 0x800 minus 6 times the value in SFF_sa. This allows 12 bytes of message storage between this address and the end of Acceptance Filter RAM, for each Standard ID that is specified between the start of Acceptance Filter RAM, and the next active AF table."]
    #[inline(always)]
    pub fn endoftable(&mut self) -> ENDOFTABLE_W {
        ENDOFTABLE_W { w: self }
    }
}
