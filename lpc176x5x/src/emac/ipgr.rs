#[doc = "Reader of register IPGR"]
pub type R = crate::R<u32, super::IPGR>;
#[doc = "Writer for register IPGR"]
pub type W = crate::W<u32, super::IPGR>;
#[doc = "Register IPGR `reset()`'s with value 0"]
impl crate::ResetValue for super::IPGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NBTOBINTEGAP2`"]
pub type NBTOBINTEGAP2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NBTOBINTEGAP2`"]
pub struct NBTOBINTEGAP2_W<'a> {
    w: &'a mut W,
}
impl<'a> NBTOBINTEGAP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `NBTOBINTEGAP1`"]
pub type NBTOBINTEGAP1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NBTOBINTEGAP1`"]
pub struct NBTOBINTEGAP1_W<'a> {
    w: &'a mut W,
}
impl<'a> NBTOBINTEGAP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - NON-BACK-TO-BACK INTER-PACKET-GAP PART2. This is a programmable field representing the Non-Back-to-Back Inter-Packet-Gap. The recommended value is 0x12 (18d), which represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode)."]
    #[inline(always)]
    pub fn nbtobintegap2(&self) -> NBTOBINTEGAP2_R {
        NBTOBINTEGAP2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - NON-BACK-TO-BACK INTER-PACKET-GAP PART1. This is a programmable field representing the optional carrierSense window referenced in IEEE 802.3/4.2.3.2.1 'Carrier Deference'. If carrier is detected during the timing of IPGR1, the MAC defers to carrier. If, however, carrier becomes active after IPGR1, the MAC continues timing IPGR2 and transmits, knowingly causing a collision, thus ensuring fair access to medium. Its range of values is 0x0 to IPGR2. The recommended value is 0xC (12d)"]
    #[inline(always)]
    pub fn nbtobintegap1(&self) -> NBTOBINTEGAP1_R {
        NBTOBINTEGAP1_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - NON-BACK-TO-BACK INTER-PACKET-GAP PART2. This is a programmable field representing the Non-Back-to-Back Inter-Packet-Gap. The recommended value is 0x12 (18d), which represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode)."]
    #[inline(always)]
    pub fn nbtobintegap2(&mut self) -> NBTOBINTEGAP2_W {
        NBTOBINTEGAP2_W { w: self }
    }
    #[doc = "Bits 8:14 - NON-BACK-TO-BACK INTER-PACKET-GAP PART1. This is a programmable field representing the optional carrierSense window referenced in IEEE 802.3/4.2.3.2.1 'Carrier Deference'. If carrier is detected during the timing of IPGR1, the MAC defers to carrier. If, however, carrier becomes active after IPGR1, the MAC continues timing IPGR2 and transmits, knowingly causing a collision, thus ensuring fair access to medium. Its range of values is 0x0 to IPGR2. The recommended value is 0xC (12d)"]
    #[inline(always)]
    pub fn nbtobintegap1(&mut self) -> NBTOBINTEGAP1_W {
        NBTOBINTEGAP1_W { w: self }
    }
}
