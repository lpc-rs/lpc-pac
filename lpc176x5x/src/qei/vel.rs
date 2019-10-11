#[doc = "Reader of register VEL"]
pub type R = crate::R<u32, super::VEL>;
#[doc = "Reader of field `VELPC`"]
pub type VELPC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current velocity pulse count."]
    #[inline(always)]
    pub fn velpc(&self) -> VELPC_R {
        VELPC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
