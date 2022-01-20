#[doc = "Register `MAC_DBG` reader"]
pub struct R(crate::R<MAC_DBG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_DBG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_DBG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_DBG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REPESTS` reader - MAC MII Receive Protocol Engine Status When this bit is set, it indicates that the MAC MII receive protocol engine is actively receiving data, and it is not in the Idle state."]
pub struct REPESTS_R(crate::FieldReader<bool, bool>);
impl REPESTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REPESTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REPESTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFCFCSTS` reader - MAC Receive Packet Controller FIFO Status When this bit is set, this field indicates the active state of the small FIFO Read and Write controllers of the MAC Receive Packet Controller module."]
pub struct RFCFCSTS_R(crate::FieldReader<u8, u8>);
impl RFCFCSTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RFCFCSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFCFCSTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPESTS` reader - MAC MII Transmit Protocol Engine Status When this bit is set, it indicates that the MAC or MII transmit protocol engine is actively transmitting data, and it is not in the Idle state."]
pub struct TPESTS_R(crate::FieldReader<bool, bool>);
impl TPESTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TPESTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPESTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFCSTS` reader - MAC Transmit Packet Controller Status This field indicates the state of the MAC Transmit Packet Controller module."]
pub struct TFCSTS_R(crate::FieldReader<u8, u8>);
impl TFCSTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TFCSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFCSTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - MAC MII Receive Protocol Engine Status When this bit is set, it indicates that the MAC MII receive protocol engine is actively receiving data, and it is not in the Idle state."]
    #[inline(always)]
    pub fn repests(&self) -> REPESTS_R {
        REPESTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - MAC Receive Packet Controller FIFO Status When this bit is set, this field indicates the active state of the small FIFO Read and Write controllers of the MAC Receive Packet Controller module."]
    #[inline(always)]
    pub fn rfcfcsts(&self) -> RFCFCSTS_R {
        RFCFCSTS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 16 - MAC MII Transmit Protocol Engine Status When this bit is set, it indicates that the MAC or MII transmit protocol engine is actively transmitting data, and it is not in the Idle state."]
    #[inline(always)]
    pub fn tpests(&self) -> TPESTS_R {
        TPESTS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - MAC Transmit Packet Controller Status This field indicates the state of the MAC Transmit Packet Controller module."]
    #[inline(always)]
    pub fn tfcsts(&self) -> TFCSTS_R {
        TFCSTS_R::new(((self.bits >> 17) & 0x03) as u8)
    }
}
#[doc = "MAC debug register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_dbg](index.html) module"]
pub struct MAC_DBG_SPEC;
impl crate::RegisterSpec for MAC_DBG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_dbg::R](R) reader structure"]
impl crate::Readable for MAC_DBG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MAC_DBG to value 0"]
impl crate::Resettable for MAC_DBG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
