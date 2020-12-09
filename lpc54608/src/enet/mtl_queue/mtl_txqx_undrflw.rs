#[doc = "Reader of register MTL_TXQx_UNDRFLW"]
pub type R = crate::R<u32, super::MTL_TXQX_UNDRFLW>;
#[doc = "Reader of field `UFFRMCNT`"]
pub type UFFRMCNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `UFCNTOVF`"]
pub type UFCNTOVF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:10 - Underflow Packet Counter This field indicates the number of packets aborted by the controller because of Tx Queue Underflow."]
    #[inline(always)]
    pub fn uffrmcnt(&self) -> UFFRMCNT_R {
        UFFRMCNT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Overflow Bit for Underflow Packet Counter This bit is set every time the Tx queue Underflow Packet Counter field overflows, that is, it has crossed the maximum count."]
    #[inline(always)]
    pub fn ufcntovf(&self) -> UFCNTOVF_R {
        UFCNTOVF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
