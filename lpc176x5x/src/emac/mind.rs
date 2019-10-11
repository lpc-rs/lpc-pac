#[doc = "Reader of register MIND"]
pub type R = crate::R<u32, super::MIND>;
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCANNING`"]
pub type SCANNING_R = crate::R<bool, bool>;
#[doc = "Reader of field `NOTVALID`"]
pub type NOTVALID_R = crate::R<bool, bool>;
#[doc = "Reader of field `MIILINKFAIL`"]
pub type MIILINKFAIL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - When 1 is returned - indicates MII Mgmt is currently performing an MII Mgmt Read or Write cycle."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1 is returned - indicates a scan operation (continuous MII Mgmt Read cycles) is in progress."]
    #[inline(always)]
    pub fn scanning(&self) -> SCANNING_R {
        SCANNING_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - When 1 is returned - indicates MII Mgmt Read cycle has not completed and the Read Data is not yet valid."]
    #[inline(always)]
    pub fn notvalid(&self) -> NOTVALID_R {
        NOTVALID_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When 1 is returned - indicates that an MII Mgmt link fail has occurred."]
    #[inline(always)]
    pub fn miilinkfail(&self) -> MIILINKFAIL_R {
        MIILINKFAIL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
