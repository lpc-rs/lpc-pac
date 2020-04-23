#[doc = "Reader of register INTSTAT"]
pub type R = crate::R<u32, super::INTSTAT>;
#[doc = "Reader of field `SSA`"]
pub type SSA_R = crate::R<bool, bool>;
#[doc = "Reader of field `SSD`"]
pub type SSD_R = crate::R<bool, bool>;
#[doc = "Reader of field `MSTIDLE`"]
pub type MSTIDLE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 4 - Slave Select Assert."]
    #[inline(always)]
    pub fn ssa(&self) -> SSA_R {
        SSA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Slave Select Deassert."]
    #[inline(always)]
    pub fn ssd(&self) -> SSD_R {
        SSD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Master Idle status flag."]
    #[inline(always)]
    pub fn mstidle(&self) -> MSTIDLE_R {
        MSTIDLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
