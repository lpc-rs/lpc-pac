#[doc = "Reader of register HCSPARAMS"]
pub type R = crate::R<u32, super::HCSPARAMS>;
#[doc = "Reader of field `N_PORTS`"]
pub type N_PORTS_R = crate::R<u8, u8>;
#[doc = "Reader of field `PPC`"]
pub type PPC_R = crate::R<bool, bool>;
#[doc = "Reader of field `P_INDICATOR`"]
pub type P_INDICATOR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:3 - This register specifies the number of physical downstream ports implemented on this host controller."]
    #[inline(always)]
    pub fn n_ports(&self) -> N_PORTS_R {
        N_PORTS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - This field indicates whether the host controller implementation includes port power control."]
    #[inline(always)]
    pub fn ppc(&self) -> PPC_R {
        PPC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - This bit indicates whether the ports support port indicator control."]
    #[inline(always)]
    pub fn p_indicator(&self) -> P_INDICATOR_R {
        P_INDICATOR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
