#[doc = "Reader of register IIR"]
pub type R = crate::R<u32, super::IIR>;
#[doc = "Reader of field `INTSTATUS`"]
pub type INTSTATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTID`"]
pub type INTID_R = crate::R<u8, u8>;
#[doc = "Reader of field `FIFOENABLE`"]
pub type FIFOENABLE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Interrupt status."]
    #[inline(always)]
    pub fn intstatus(&self) -> INTSTATUS_R {
        INTSTATUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Interrupt identification."]
    #[inline(always)]
    pub fn intid(&self) -> INTID_R {
        INTID_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - Copies of SCInFCR\\[0\\]."]
    #[inline(always)]
    pub fn fifoenable(&self) -> FIFOENABLE_R {
        FIFOENABLE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
