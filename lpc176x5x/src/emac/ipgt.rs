#[doc = "Reader of register IPGT"]
pub type R = crate::R<u32, super::IPGT>;
#[doc = "Writer for register IPGT"]
pub type W = crate::W<u32, super::IPGT>;
#[doc = "Register IPGT `reset()`'s with value 0"]
impl crate::ResetValue for super::IPGT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BTOBINTEGAP`"]
pub type BTOBINTEGAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BTOBINTEGAP`"]
pub struct BTOBINTEGAP_W<'a> {
    w: &'a mut W,
}
impl<'a> BTOBINTEGAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - BACK-TO-BACK INTER-PACKET-GAP.This is a programmable field representing the nibble time offset of the minimum possible period between the end of any transmitted packet to the beginning of the next. In Full-Duplex mode, the register value should be the desired period in nibble times minus 3. In Half-Duplex mode, the register value should be the desired period in nibble times minus 6. In Full-Duplex the recommended setting is 0x15 (21d), which represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode). In Half-Duplex the recommended setting is 0x12 (18d), which also represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode)."]
    #[inline(always)]
    pub fn btobintegap(&self) -> BTOBINTEGAP_R {
        BTOBINTEGAP_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - BACK-TO-BACK INTER-PACKET-GAP.This is a programmable field representing the nibble time offset of the minimum possible period between the end of any transmitted packet to the beginning of the next. In Full-Duplex mode, the register value should be the desired period in nibble times minus 3. In Half-Duplex mode, the register value should be the desired period in nibble times minus 6. In Full-Duplex the recommended setting is 0x15 (21d), which represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode). In Half-Duplex the recommended setting is 0x12 (18d), which also represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode)."]
    #[inline(always)]
    pub fn btobintegap(&mut self) -> BTOBINTEGAP_W {
        BTOBINTEGAP_W { w: self }
    }
}
