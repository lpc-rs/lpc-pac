#[doc = "Reader of register BASE"]
pub type R = crate::R<u32, super::BASE>;
#[doc = "Reader of field `BASE`"]
pub type BASE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The value provided is the value of the SRAMBASEADDR\\[31:0\\]
signal."]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
