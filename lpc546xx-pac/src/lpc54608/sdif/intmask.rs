#[doc = "Register `INTMASK` reader"]
pub struct R(crate::R<INTMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTMASK` writer"]
pub struct W(crate::W<INTMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTMASK_SPEC>;
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
impl From<crate::W<INTMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CDET` reader - Card detect."]
pub struct CDET_R(crate::FieldReader<bool, bool>);
impl CDET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDET` writer - Card detect."]
pub struct CDET_W<'a> {
    w: &'a mut W,
}
impl<'a> CDET_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `RE` reader - Response error."]
pub struct RE_R(crate::FieldReader<bool, bool>);
impl RE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RE` writer - Response error."]
pub struct RE_W<'a> {
    w: &'a mut W,
}
impl<'a> RE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CDONE` reader - Command done."]
pub struct CDONE_R(crate::FieldReader<bool, bool>);
impl CDONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDONE` writer - Command done."]
pub struct CDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> CDONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `DTO` reader - Data transfer over."]
pub struct DTO_R(crate::FieldReader<bool, bool>);
impl DTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTO` writer - Data transfer over."]
pub struct DTO_W<'a> {
    w: &'a mut W,
}
impl<'a> DTO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `TXDR` reader - Transmit FIFO data request."]
pub struct TXDR_R(crate::FieldReader<bool, bool>);
impl TXDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDR` writer - Transmit FIFO data request."]
pub struct TXDR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `RXDR` reader - Receive FIFO data request."]
pub struct RXDR_R(crate::FieldReader<bool, bool>);
impl RXDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDR` writer - Receive FIFO data request."]
pub struct RXDR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `RCRC` reader - Response CRC error."]
pub struct RCRC_R(crate::FieldReader<bool, bool>);
impl RCRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCRC` writer - Response CRC error."]
pub struct RCRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `DCRC` reader - Data CRC error."]
pub struct DCRC_R(crate::FieldReader<bool, bool>);
impl DCRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCRC` writer - Data CRC error."]
pub struct DCRC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCRC_W<'a> {
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
#[doc = "Field `RTO` reader - Response time-out."]
pub struct RTO_R(crate::FieldReader<bool, bool>);
impl RTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTO` writer - Response time-out."]
pub struct RTO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `DRTO` reader - Data read time-out."]
pub struct DRTO_R(crate::FieldReader<bool, bool>);
impl DRTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DRTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRTO` writer - Data read time-out."]
pub struct DRTO_W<'a> {
    w: &'a mut W,
}
impl<'a> DRTO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `HTO` reader - Data starvation-by-host time-out (HTO)."]
pub struct HTO_R(crate::FieldReader<bool, bool>);
impl HTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTO` writer - Data starvation-by-host time-out (HTO)."]
pub struct HTO_W<'a> {
    w: &'a mut W,
}
impl<'a> HTO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `FRUN` reader - FIFO underrun/overrun error."]
pub struct FRUN_R(crate::FieldReader<bool, bool>);
impl FRUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRUN` writer - FIFO underrun/overrun error."]
pub struct FRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRUN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `HLE` reader - Hardware locked write error."]
pub struct HLE_R(crate::FieldReader<bool, bool>);
impl HLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HLE` writer - Hardware locked write error."]
pub struct HLE_W<'a> {
    w: &'a mut W,
}
impl<'a> HLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `SBE` reader - Start-bit error."]
pub struct SBE_R(crate::FieldReader<bool, bool>);
impl SBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBE` writer - Start-bit error."]
pub struct SBE_W<'a> {
    w: &'a mut W,
}
impl<'a> SBE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `ACD` reader - Auto command done."]
pub struct ACD_R(crate::FieldReader<bool, bool>);
impl ACD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACD` writer - Auto command done."]
pub struct ACD_W<'a> {
    w: &'a mut W,
}
impl<'a> ACD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `EBE` reader - End-bit error (read)/Write no CRC."]
pub struct EBE_R(crate::FieldReader<bool, bool>);
impl EBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EBE` writer - End-bit error (read)/Write no CRC."]
pub struct EBE_W<'a> {
    w: &'a mut W,
}
impl<'a> EBE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `SDIO_INT_MASK` reader - Mask SDIO interrupt."]
pub struct SDIO_INT_MASK_R(crate::FieldReader<bool, bool>);
impl SDIO_INT_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_INT_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_INT_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_INT_MASK` writer - Mask SDIO interrupt."]
pub struct SDIO_INT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_INT_MASK_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Card detect."]
    #[inline(always)]
    pub fn cdet(&self) -> CDET_R {
        CDET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Response error."]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Command done."]
    #[inline(always)]
    pub fn cdone(&self) -> CDONE_R {
        CDONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data transfer over."]
    #[inline(always)]
    pub fn dto(&self) -> DTO_R {
        DTO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit FIFO data request."]
    #[inline(always)]
    pub fn txdr(&self) -> TXDR_R {
        TXDR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO data request."]
    #[inline(always)]
    pub fn rxdr(&self) -> RXDR_R {
        RXDR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Response CRC error."]
    #[inline(always)]
    pub fn rcrc(&self) -> RCRC_R {
        RCRC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Data CRC error."]
    #[inline(always)]
    pub fn dcrc(&self) -> DCRC_R {
        DCRC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Response time-out."]
    #[inline(always)]
    pub fn rto(&self) -> RTO_R {
        RTO_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Data read time-out."]
    #[inline(always)]
    pub fn drto(&self) -> DRTO_R {
        DRTO_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Data starvation-by-host time-out (HTO)."]
    #[inline(always)]
    pub fn hto(&self) -> HTO_R {
        HTO_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - FIFO underrun/overrun error."]
    #[inline(always)]
    pub fn frun(&self) -> FRUN_R {
        FRUN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Hardware locked write error."]
    #[inline(always)]
    pub fn hle(&self) -> HLE_R {
        HLE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Start-bit error."]
    #[inline(always)]
    pub fn sbe(&self) -> SBE_R {
        SBE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Auto command done."]
    #[inline(always)]
    pub fn acd(&self) -> ACD_R {
        ACD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - End-bit error (read)/Write no CRC."]
    #[inline(always)]
    pub fn ebe(&self) -> EBE_R {
        EBE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Mask SDIO interrupt."]
    #[inline(always)]
    pub fn sdio_int_mask(&self) -> SDIO_INT_MASK_R {
        SDIO_INT_MASK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Card detect."]
    #[inline(always)]
    pub fn cdet(&mut self) -> CDET_W {
        CDET_W { w: self }
    }
    #[doc = "Bit 1 - Response error."]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W {
        RE_W { w: self }
    }
    #[doc = "Bit 2 - Command done."]
    #[inline(always)]
    pub fn cdone(&mut self) -> CDONE_W {
        CDONE_W { w: self }
    }
    #[doc = "Bit 3 - Data transfer over."]
    #[inline(always)]
    pub fn dto(&mut self) -> DTO_W {
        DTO_W { w: self }
    }
    #[doc = "Bit 4 - Transmit FIFO data request."]
    #[inline(always)]
    pub fn txdr(&mut self) -> TXDR_W {
        TXDR_W { w: self }
    }
    #[doc = "Bit 5 - Receive FIFO data request."]
    #[inline(always)]
    pub fn rxdr(&mut self) -> RXDR_W {
        RXDR_W { w: self }
    }
    #[doc = "Bit 6 - Response CRC error."]
    #[inline(always)]
    pub fn rcrc(&mut self) -> RCRC_W {
        RCRC_W { w: self }
    }
    #[doc = "Bit 7 - Data CRC error."]
    #[inline(always)]
    pub fn dcrc(&mut self) -> DCRC_W {
        DCRC_W { w: self }
    }
    #[doc = "Bit 8 - Response time-out."]
    #[inline(always)]
    pub fn rto(&mut self) -> RTO_W {
        RTO_W { w: self }
    }
    #[doc = "Bit 9 - Data read time-out."]
    #[inline(always)]
    pub fn drto(&mut self) -> DRTO_W {
        DRTO_W { w: self }
    }
    #[doc = "Bit 10 - Data starvation-by-host time-out (HTO)."]
    #[inline(always)]
    pub fn hto(&mut self) -> HTO_W {
        HTO_W { w: self }
    }
    #[doc = "Bit 11 - FIFO underrun/overrun error."]
    #[inline(always)]
    pub fn frun(&mut self) -> FRUN_W {
        FRUN_W { w: self }
    }
    #[doc = "Bit 12 - Hardware locked write error."]
    #[inline(always)]
    pub fn hle(&mut self) -> HLE_W {
        HLE_W { w: self }
    }
    #[doc = "Bit 13 - Start-bit error."]
    #[inline(always)]
    pub fn sbe(&mut self) -> SBE_W {
        SBE_W { w: self }
    }
    #[doc = "Bit 14 - Auto command done."]
    #[inline(always)]
    pub fn acd(&mut self) -> ACD_W {
        ACD_W { w: self }
    }
    #[doc = "Bit 15 - End-bit error (read)/Write no CRC."]
    #[inline(always)]
    pub fn ebe(&mut self) -> EBE_W {
        EBE_W { w: self }
    }
    #[doc = "Bit 16 - Mask SDIO interrupt."]
    #[inline(always)]
    pub fn sdio_int_mask(&mut self) -> SDIO_INT_MASK_W {
        SDIO_INT_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intmask](index.html) module"]
pub struct INTMASK_SPEC;
impl crate::RegisterSpec for INTMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intmask::R](R) reader structure"]
impl crate::Readable for INTMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intmask::W](W) writer structure"]
impl crate::Writable for INTMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTMASK to value 0"]
impl crate::Resettable for INTMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
