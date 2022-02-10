///Register `PSR` reader
pub struct R(crate::R<PSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `LEC` reader - Last error code.
pub struct LEC_R(crate::FieldReader<u8, u8>);
impl LEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ACT` reader - Activity.
pub struct ACT_R(crate::FieldReader<u8, u8>);
impl ACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EP` reader - Error Passive.
pub struct EP_R(crate::FieldReader<bool, bool>);
impl EP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EW` reader - Warning status.
pub struct EW_R(crate::FieldReader<bool, bool>);
impl EW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BO` reader - Bus Off Status.
pub struct BO_R(crate::FieldReader<bool, bool>);
impl BO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DLEC` reader - Data phase last error code.
pub struct DLEC_R(crate::FieldReader<u8, u8>);
impl DLEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DLEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RESI` reader - ESI flag of the last received CAN FD message.
pub struct RESI_R(crate::FieldReader<bool, bool>);
impl RESI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RBRS` reader - BRS flag of last received CAN FD message.
pub struct RBRS_R(crate::FieldReader<bool, bool>);
impl RBRS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RBRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBRS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RFDF` reader - Received a CAN FD message.
pub struct RFDF_R(crate::FieldReader<bool, bool>);
impl RFDF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFDF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFDF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PXE` reader - Protocol exception event.
pub struct PXE_R(crate::FieldReader<bool, bool>);
impl PXE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PXE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PXE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TDCV` reader - Transmitter delay compensation value.
pub struct TDCV_R(crate::FieldReader<u8, u8>);
impl TDCV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TDCV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDCV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:2 - Last error code.
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 0x07) as u8)
    }
    ///Bits 3:4 - Activity.
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    ///Bit 5 - Error Passive.
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Warning status.
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Bus Off Status.
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 8:10 - Data phase last error code.
    #[inline(always)]
    pub fn dlec(&self) -> DLEC_R {
        DLEC_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    ///Bit 11 - ESI flag of the last received CAN FD message.
    #[inline(always)]
    pub fn resi(&self) -> RESI_R {
        RESI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - BRS flag of last received CAN FD message.
    #[inline(always)]
    pub fn rbrs(&self) -> RBRS_R {
        RBRS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Received a CAN FD message.
    #[inline(always)]
    pub fn rfdf(&self) -> RFDF_R {
        RFDF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Protocol exception event.
    #[inline(always)]
    pub fn pxe(&self) -> PXE_R {
        PXE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bits 16:22 - Transmitter delay compensation value.
    #[inline(always)]
    pub fn tdcv(&self) -> TDCV_R {
        TDCV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
///Protocol Status Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [psr](index.html) module
pub struct PSR_SPEC;
impl crate::RegisterSpec for PSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [psr::R](R) reader structure
impl crate::Readable for PSR_SPEC {
    type Reader = R;
}
///`reset()` method sets PSR to value 0x0707
impl crate::Resettable for PSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0707
    }
}
