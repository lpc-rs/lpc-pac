#[doc = "Reader of register FLOWCONTROLSTATUS"]
pub type R = crate::R<u32, super::FLOWCONTROLSTATUS>;
#[doc = "Reader of field `MCC`"]
pub type MCC_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - MirrorCounterCurrent. In full duplex mode this register represents the current value of the datapath's mirror counter which counts up to the value specified by the MirrorCounter field in the FlowControlCounter register. In half duplex mode the register counts until it reaches the value of the PauseTimer bits in the FlowControlCounter register."]
    #[inline(always)]
    pub fn mcc(&self) -> MCC_R {
        MCC_R::new((self.bits & 0xffff) as u16)
    }
}
