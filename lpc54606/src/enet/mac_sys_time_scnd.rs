#[doc = "Reader of register MAC_SYS_TIME_SCND"]
pub type R = crate::R<u32, super::MAC_SYS_TIME_SCND>;
#[doc = "Reader of field `TSS`"]
pub type TSS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Time stamp second The value in this field indicates the current value in seconds of the System Time maintained by the MAC."]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
