#[doc = "Reader of register CMDDATA"]
pub type R = crate::R<u32, super::CMDDATA>;
#[doc = "Reader of field `CMD_RDATA`"]
pub type CMD_RDATA_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Command Read Data."]
    #[inline(always)]
    pub fn cmd_rdata(&self) -> CMD_RDATA_R {
        CMD_RDATA_R::new((self.bits & 0xff) as u8)
    }
}
