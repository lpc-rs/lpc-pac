#[doc = "Register `MAC_HW_FEAT0` reader"]
pub struct R(crate::R<MAC_HW_FEAT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_HW_FEAT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_HW_FEAT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_HW_FEAT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_HW_FEAT0` writer"]
pub struct W(crate::W<MAC_HW_FEAT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_HW_FEAT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MAC_HW_FEAT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_HW_FEAT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIISEL` reader - 10 or 100 Mbps Support."]
pub struct MIISEL_R(crate::FieldReader<bool, bool>);
impl MIISEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        MIISEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIISEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDSEL` reader - Half-duplex Support."]
pub struct HDSEL_R(crate::FieldReader<bool, bool>);
impl HDSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HDSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLHASH` reader - Hash Table Based Filtering option."]
pub struct VLHASH_R(crate::FieldReader<bool, bool>);
impl VLHASH_R {
    pub(crate) fn new(bits: bool) -> Self {
        VLHASH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VLHASH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMASEL` reader - SMA (MDIO) Interface."]
pub struct SMASEL_R(crate::FieldReader<bool, bool>);
impl SMASEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMASEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMASEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWKSEL` reader - PMT Remote Wake-up Packet Detection."]
pub struct RWKSEL_R(crate::FieldReader<bool, bool>);
impl RWKSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWKSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWKSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MGKSEL` reader - PMT magic packet detection."]
pub struct MGKSEL_R(crate::FieldReader<bool, bool>);
impl MGKSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        MGKSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MGKSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MGKSEL` writer - PMT magic packet detection."]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `MMCSEL` reader - RMON Module Enable."]
pub struct MMCSEL_R(crate::FieldReader<bool, bool>);
impl MMCSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        MMCSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMCSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARPOFFSEL` reader - ARP Offload Enabled."]
pub struct ARPOFFSEL_R(crate::FieldReader<bool, bool>);
impl ARPOFFSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARPOFFSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARPOFFSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSSEL` reader - IEEE 1588-2008 Timestamp support ."]
pub struct TSSEL_R(crate::FieldReader<bool, bool>);
impl TSSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEESEL` reader - Energy Efficient Ethernet Support ."]
pub struct EEESEL_R(crate::FieldReader<bool, bool>);
impl EEESEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EEESEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EEESEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXCOESEL` reader - Transmit Checksum Offload Support."]
pub struct TXCOESEL_R(crate::FieldReader<bool, bool>);
impl TXCOESEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXCOESEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCOESEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXCOESEL` reader - Receive Checksum Offload Support."]
pub struct RXCOESEL_R(crate::FieldReader<bool, bool>);
impl RXCOESEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXCOESEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXCOESEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXCOESEL` writer - Receive Checksum Offload Support."]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `TSSTSSEL` reader - Timestamp System Time Source."]
pub struct TSSTSSEL_R(crate::FieldReader<u8, u8>);
impl TSSTSSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TSSTSSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSSTSSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTPHYSEL` reader - Active PHY Selected."]
pub struct ACTPHYSEL_R(crate::FieldReader<u8, u8>);
impl ACTPHYSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACTPHYSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTPHYSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC hardware feature register 0x0201\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_hw_feat0](index.html) module"]
pub struct MAC_HW_FEAT0_SPEC;
impl crate::RegisterSpec for MAC_HW_FEAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_hw_feat0::R](R) reader structure"]
impl crate::Readable for MAC_HW_FEAT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_hw_feat0::W](W) writer structure"]
impl crate::Writable for MAC_HW_FEAT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_HW_FEAT0 to value 0"]
impl crate::Resettable for MAC_HW_FEAT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
