#[doc = "Reader of register PIOPORCAP[%s]"]
pub type R = crate::R<u32, super::PIOPORCAP>;
#[doc = "Reader of field `PIOPORCAP`"]
pub type PIOPORCAP_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - State of PIOn_31 through PIOn_0 at power-on reset"]
    #[inline(always)]
    pub fn pioporcap(&self) -> PIOPORCAP_R {
        PIOPORCAP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
