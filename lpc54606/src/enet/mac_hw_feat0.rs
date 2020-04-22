#[doc = "Reader of register MAC_HW_FEAT0"]
pub type R = crate::R<u32, super::MAC_HW_FEAT0>;
#[doc = "Writer for register MAC_HW_FEAT0"]
pub type W = crate::W<u32, super::MAC_HW_FEAT0>;
#[doc = "Register MAC_HW_FEAT0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MAC_HW_FEAT0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MIISEL`"]
pub type MIISEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `HDSEL`"]
pub type HDSEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `VLHASH`"]
pub type VLHASH_R = crate::R<bool, bool>;
#[doc = "Reader of field `SMASEL`"]
pub type SMASEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RWKSEL`"]
pub type RWKSEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `MGKSEL`"]
pub type MGKSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MGKSEL`"]
pub struct MGKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MGKSEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `MMCSEL`"]
pub type MMCSEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `ARPOFFSEL`"]
pub type ARPOFFSEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSSEL`"]
pub type TSSEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `EEESEL`"]
pub type EEESEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXCOESEL`"]
pub type TXCOESEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXCOESEL`"]
pub type RXCOESEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXCOESEL`"]
pub struct RXCOESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCOESEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `TSSTSSEL`"]
pub type TSSTSSEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `ACTPHYSEL`"]
pub type ACTPHYSEL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - 10 or 100 Mbps Support."]
    #[inline(always)]
    pub fn miisel(&self) -> MIISEL_R {
        MIISEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Half-duplex Support."]
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Hash Table Based Filtering option."]
    #[inline(always)]
    pub fn vlhash(&self) -> VLHASH_R {
        VLHASH_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SMA (MDIO) Interface."]
    #[inline(always)]
    pub fn smasel(&self) -> SMASEL_R {
        SMASEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PMT Remote Wake-up Packet Detection."]
    #[inline(always)]
    pub fn rwksel(&self) -> RWKSEL_R {
        RWKSEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PMT magic packet detection."]
    #[inline(always)]
    pub fn mgksel(&self) -> MGKSEL_R {
        MGKSEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RMON Module Enable."]
    #[inline(always)]
    pub fn mmcsel(&self) -> MMCSEL_R {
        MMCSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ARP Offload Enabled."]
    #[inline(always)]
    pub fn arpoffsel(&self) -> ARPOFFSEL_R {
        ARPOFFSEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - IEEE 1588-2008 Timestamp support ."]
    #[inline(always)]
    pub fn tssel(&self) -> TSSEL_R {
        TSSEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Energy Efficient Ethernet Support ."]
    #[inline(always)]
    pub fn eeesel(&self) -> EEESEL_R {
        EEESEL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Transmit Checksum Offload Support."]
    #[inline(always)]
    pub fn txcoesel(&self) -> TXCOESEL_R {
        TXCOESEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Receive Checksum Offload Support."]
    #[inline(always)]
    pub fn rxcoesel(&self) -> RXCOESEL_R {
        RXCOESEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - Timestamp System Time Source."]
    #[inline(always)]
    pub fn tsstssel(&self) -> TSSTSSEL_R {
        TSSTSSEL_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bits 28:30 - Active PHY Selected."]
    #[inline(always)]
    pub fn actphysel(&self) -> ACTPHYSEL_R {
        ACTPHYSEL_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - PMT magic packet detection."]
    #[inline(always)]
    pub fn mgksel(&mut self) -> MGKSEL_W {
        MGKSEL_W { w: self }
    }
    #[doc = "Bit 16 - Receive Checksum Offload Support."]
    #[inline(always)]
    pub fn rxcoesel(&mut self) -> RXCOESEL_W {
        RXCOESEL_W { w: self }
    }
}
