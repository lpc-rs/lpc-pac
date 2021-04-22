#[doc = "Register `MAC_RXTX_STAT` reader"]
pub struct R(crate::R<MAC_RXTX_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_RXTX_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MAC_RXTX_STAT_SPEC>> for R {
    fn from(reader: crate::R<MAC_RXTX_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TJT` reader - PHY Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of PHYIS bit in MAC Interrupt Status register Table 731."]
pub struct TJT_R(crate::FieldReader<bool, bool>);
impl TJT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TJT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TJT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NCARR` reader - No Carrier When the DTXSTS bit is set in the MTL Operation Mode register Table 758, this bit indicates that the carrier signal from the PHY is not present at the end of preamble transmission."]
pub struct NCARR_R(crate::FieldReader<bool, bool>);
impl NCARR_R {
    pub(crate) fn new(bits: bool) -> Self {
        NCARR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NCARR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCARR` reader - Loss of Carrier When the DTXSTS bit is set in the MTL Operation Mode register Table 758, this bit indicates that the loss of carrier occurred during packet transmission, that is, the PHY Carrier signal was inactive for one or more transmission clock periods during packet transmission."]
pub struct LCARR_R(crate::FieldReader<bool, bool>);
impl LCARR_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCARR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCARR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXDEF` reader - Excessive Deferral When the DTXSTS bit is set in the MTL Operation Mode register Table 758 and the DC bit is set in the MAC Configuration register Table 758, this bit indicates that the transmission ended because of excessive deferral of over 24,288 bit times (155,680 when Jumbo packet is enabled)."]
pub struct EXDEF_R(crate::FieldReader<bool, bool>);
impl EXDEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXDEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXDEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCOL` reader - Late Collision When the DTXSTS bit is set in the MTL Operation Mode register Table 758, this bit indicates that the packet transmission aborted because a collision occurred after the collision window (64 bytes including Preamble in MII mode)."]
pub struct LCOL_R(crate::FieldReader<bool, bool>);
impl LCOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXCOL` reader - Excessive Collisions When the DTXSTS bit is set in the MTL Operation Mode register Table 758, this bit indicates that the transmission aborted after 16 successive collisions while attempting to transmit the current packet."]
pub struct EXCOL_R(crate::FieldReader<bool, bool>);
impl EXCOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXCOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXCOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWT` reader - Receive Watchdog Timeout This bit is set when a packet with length greater than 2,048 bytes is received (10,240 bytes when Jumbo Packet mode is enabled) and the WD bit is reset in the MAC Configuration register Table 722."]
pub struct RWT_R(crate::FieldReader<bool, bool>);
impl RWT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - PHY Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of PHYIS bit in MAC Interrupt Status register Table 731."]
    #[inline(always)]
    pub fn tjt(&self) -> TJT_R {
        TJT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - No Carrier When the DTXSTS bit is set in the MTL Operation Mode register Table 758, this bit indicates that the carrier signal from the PHY is not present at the end of preamble transmission."]
    #[inline(always)]
    pub fn ncarr(&self) -> NCARR_R {
        NCARR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Loss of Carrier When the DTXSTS bit is set in the MTL Operation Mode register Table 758, this bit indicates that the loss of carrier occurred during packet transmission, that is, the PHY Carrier signal was inactive for one or more transmission clock periods during packet transmission."]
    #[inline(always)]
    pub fn lcarr(&self) -> LCARR_R {
        LCARR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Excessive Deferral When the DTXSTS bit is set in the MTL Operation Mode register Table 758 and the DC bit is set in the MAC Configuration register Table 758, this bit indicates that the transmission ended because of excessive deferral of over 24,288 bit times (155,680 when Jumbo packet is enabled)."]
    #[inline(always)]
    pub fn exdef(&self) -> EXDEF_R {
        EXDEF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Late Collision When the DTXSTS bit is set in the MTL Operation Mode register Table 758, this bit indicates that the packet transmission aborted because a collision occurred after the collision window (64 bytes including Preamble in MII mode)."]
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Excessive Collisions When the DTXSTS bit is set in the MTL Operation Mode register Table 758, this bit indicates that the transmission aborted after 16 successive collisions while attempting to transmit the current packet."]
    #[inline(always)]
    pub fn excol(&self) -> EXCOL_R {
        EXCOL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive Watchdog Timeout This bit is set when a packet with length greater than 2,048 bytes is received (10,240 bytes when Jumbo Packet mode is enabled) and the WD bit is reset in the MAC Configuration register Table 722."]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
#[doc = "Receive Transmit Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_rxtx_stat](index.html) module"]
pub struct MAC_RXTX_STAT_SPEC;
impl crate::RegisterSpec for MAC_RXTX_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_rxtx_stat::R](R) reader structure"]
impl crate::Readable for MAC_RXTX_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MAC_RXTX_STAT to value 0"]
impl crate::Resettable for MAC_RXTX_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
