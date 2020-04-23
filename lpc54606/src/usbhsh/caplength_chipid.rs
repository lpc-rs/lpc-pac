#[doc = "Reader of register CAPLENGTH_CHIPID"]
pub type R = crate::R<u32, super::CAPLENGTH_CHIPID>;
#[doc = "Reader of field `CAPLENGTH`"]
pub type CAPLENGTH_R = crate::R<u8, u8>;
#[doc = "Reader of field `CHIPID`"]
pub type CHIPID_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:7 - Capability Length: This is used as an offset."]
    #[inline(always)]
    pub fn caplength(&self) -> CAPLENGTH_R {
        CAPLENGTH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Chip identification: indicates major and minor revision of the IP: \\[31:24\\]
= Major revision \\[23:16\\]
= Minor revision Major revisions used: 0x01: USB2."]
    #[inline(always)]
    pub fn chipid(&self) -> CHIPID_R {
        CHIPID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
