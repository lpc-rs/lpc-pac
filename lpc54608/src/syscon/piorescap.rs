#[doc = "Reader of register PIORESCAP[%s]"]
pub type R = crate::R<u32, super::PIORESCAP>;
#[doc = "Reader of field `PIORESCAP`"]
pub type PIORESCAP_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - State of PIOn_31 through PIOn_0 for resets other than POR."]
    #[inline(always)]
    pub fn piorescap(&self) -> PIORESCAP_R {
        PIORESCAP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
