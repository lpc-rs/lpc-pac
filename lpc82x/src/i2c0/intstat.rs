#[doc = "Reader of register INTSTAT"]
pub type R = crate::R<u32, super::INTSTAT>;
#[doc = "Reader of field `MSTPENDING`"]
pub type MSTPENDING_R = crate::R<bool, bool>;
#[doc = "Reader of field `MSTARBLOSS`"]
pub type MSTARBLOSS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MSTSTSTPERR`"]
pub type MSTSTSTPERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLVPENDING`"]
pub type SLVPENDING_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLVNOTSTR`"]
pub type SLVNOTSTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLVDESEL`"]
pub type SLVDESEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `MONRDY`"]
pub type MONRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `MONOV`"]
pub type MONOV_R = crate::R<bool, bool>;
#[doc = "Reader of field `MONIDLE`"]
pub type MONIDLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `EVENTTIMEOUT`"]
pub type EVENTTIMEOUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCLTIMEOUT`"]
pub type SCLTIMEOUT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Master Pending."]
    #[inline(always)]
    pub fn mstpending(&self) -> MSTPENDING_R {
        MSTPENDING_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Master Arbitration Loss flag."]
    #[inline(always)]
    pub fn mstarbloss(&self) -> MSTARBLOSS_R {
        MSTARBLOSS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Master Start/Stop Error flag."]
    #[inline(always)]
    pub fn mstststperr(&self) -> MSTSTSTPERR_R {
        MSTSTSTPERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Slave Pending."]
    #[inline(always)]
    pub fn slvpending(&self) -> SLVPENDING_R {
        SLVPENDING_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Slave Not Stretching status."]
    #[inline(always)]
    pub fn slvnotstr(&self) -> SLVNOTSTR_R {
        SLVNOTSTR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Slave Deselected flag."]
    #[inline(always)]
    pub fn slvdesel(&self) -> SLVDESEL_R {
        SLVDESEL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Monitor Ready."]
    #[inline(always)]
    pub fn monrdy(&self) -> MONRDY_R {
        MONRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Monitor Overflow flag."]
    #[inline(always)]
    pub fn monov(&self) -> MONOV_R {
        MONOV_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Monitor Idle flag."]
    #[inline(always)]
    pub fn monidle(&self) -> MONIDLE_R {
        MONIDLE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Event time-out Interrupt flag."]
    #[inline(always)]
    pub fn eventtimeout(&self) -> EVENTTIMEOUT_R {
        EVENTTIMEOUT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - SCL time-out Interrupt flag."]
    #[inline(always)]
    pub fn scltimeout(&self) -> SCLTIMEOUT_R {
        SCLTIMEOUT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
