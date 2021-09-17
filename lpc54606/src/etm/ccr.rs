#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NumberOfAddressComparatorPairs` reader - Number of address comparator pairs. The value of these bits is b0000, indicating that address comparator pairs are not implemented."]
pub struct NUMBEROFADDRESSCOMPARATORPAIRS_R(crate::FieldReader<u8, u8>);
impl NUMBEROFADDRESSCOMPARATORPAIRS_R {
    pub(crate) fn new(bits: u8) -> Self {
        NUMBEROFADDRESSCOMPARATORPAIRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUMBEROFADDRESSCOMPARATORPAIRS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NDVC` reader - Number of data value comparators. The value of these bits is b0000, indicating that data value comparators are not implemented."]
pub struct NDVC_R(crate::FieldReader<u8, u8>);
impl NDVC_R {
    pub(crate) fn new(bits: u8) -> Self {
        NDVC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NDVC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NMMD` reader - Number of memory map decoders. The value of these bits is b00000, indicating that memory map decoder inputs are not implemented."]
pub struct NMMD_R(crate::FieldReader<u8, u8>);
impl NMMD_R {
    pub(crate) fn new(bits: u8) -> Self {
        NMMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NMMD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NC` reader - Number of counters. The value of these bits is b001, indicating that one counter is implemented."]
pub struct NC_R(crate::FieldReader<u8, u8>);
impl NC_R {
    pub(crate) fn new(bits: u8) -> Self {
        NC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SP` reader - Sequencer present. The value of this bit is 0, indicating that the sequencer is not implemented."]
pub struct SP_R(crate::FieldReader<bool, bool>);
impl SP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NEI` reader - Number of external inputs. The value of these bits is between b000 and b010, indicating the number of external inputs, from 0 to 2, implemented in the system."]
pub struct NEI_R(crate::FieldReader<u8, u8>);
impl NEI_R {
    pub(crate) fn new(bits: u8) -> Self {
        NEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NEI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NEO` reader - Number of external outputs. The value of these bits is b000, indicating that no external outputs are supported."]
pub struct NEO_R(crate::FieldReader<u8, u8>);
impl NEO_R {
    pub(crate) fn new(bits: u8) -> Self {
        NEO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NEO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFLP` reader - FIFOFULL logic present. The value of this bit is 1, indicating that FIFOFULL logic is present in the ETM. To use FIFOFULL the system must also support the function, as indicated by bit \\[8\\]
of ETMSCR."]
pub struct FFLP_R(crate::FieldReader<bool, bool>);
impl FFLP_R {
    pub(crate) fn new(bits: bool) -> Self {
        FFLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFLP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NCIDC` reader - Number of Context ID comparators. The value of these bits is b00, indicating that Context ID comparators are not implemented."]
pub struct NCIDC_R(crate::FieldReader<u8, u8>);
impl NCIDC_R {
    pub(crate) fn new(bits: u8) -> Self {
        NCIDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NCIDC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSSBP` reader - Trace start/stop block present. The value of this bit is 1, indicating that the Trace start/stop block is present."]
pub struct TSSBP_R(crate::FieldReader<bool, bool>);
impl TSSBP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSSBP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSSBP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMA` reader - Coprocessor and memory access. The value of this bit is 1, indicating that memory-mapped access to registers is supported."]
pub struct CMA_R(crate::FieldReader<bool, bool>);
impl CMA_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETMIDRP` reader - The value of this bit is 1, indicating that the ETMIDR, register 0x79, is present and defines the ETM architecture version in use."]
pub struct ETMIDRP_R(crate::FieldReader<bool, bool>);
impl ETMIDRP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETMIDRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETMIDRP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Number of address comparator pairs. The value of these bits is b0000, indicating that address comparator pairs are not implemented."]
    #[inline(always)]
    pub fn number_of_address_comparator_pairs(&self) -> NUMBEROFADDRESSCOMPARATORPAIRS_R {
        NUMBEROFADDRESSCOMPARATORPAIRS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Number of data value comparators. The value of these bits is b0000, indicating that data value comparators are not implemented."]
    #[inline(always)]
    pub fn ndvc(&self) -> NDVC_R {
        NDVC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Number of memory map decoders. The value of these bits is b00000, indicating that memory map decoder inputs are not implemented."]
    #[inline(always)]
    pub fn nmmd(&self) -> NMMD_R {
        NMMD_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - Number of counters. The value of these bits is b001, indicating that one counter is implemented."]
    #[inline(always)]
    pub fn nc(&self) -> NC_R {
        NC_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bit 16 - Sequencer present. The value of this bit is 0, indicating that the sequencer is not implemented."]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:19 - Number of external inputs. The value of these bits is between b000 and b010, indicating the number of external inputs, from 0 to 2, implemented in the system."]
    #[inline(always)]
    pub fn nei(&self) -> NEI_R {
        NEI_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Number of external outputs. The value of these bits is b000, indicating that no external outputs are supported."]
    #[inline(always)]
    pub fn neo(&self) -> NEO_R {
        NEO_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - FIFOFULL logic present. The value of this bit is 1, indicating that FIFOFULL logic is present in the ETM. To use FIFOFULL the system must also support the function, as indicated by bit \\[8\\]
of ETMSCR."]
    #[inline(always)]
    pub fn fflp(&self) -> FFLP_R {
        FFLP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Number of Context ID comparators. The value of these bits is b00, indicating that Context ID comparators are not implemented."]
    #[inline(always)]
    pub fn ncidc(&self) -> NCIDC_R {
        NCIDC_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Trace start/stop block present. The value of this bit is 1, indicating that the Trace start/stop block is present."]
    #[inline(always)]
    pub fn tssbp(&self) -> TSSBP_R {
        TSSBP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Coprocessor and memory access. The value of this bit is 1, indicating that memory-mapped access to registers is supported."]
    #[inline(always)]
    pub fn cma(&self) -> CMA_R {
        CMA_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 31 - The value of this bit is 1, indicating that the ETMIDR, register 0x79, is present and defines the ETM architecture version in use."]
    #[inline(always)]
    pub fn etmidrp(&self) -> ETMIDRP_R {
        ETMIDRP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Configuration Code Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CCR to value 0x8c80_2000"]
impl crate::Resettable for CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8c80_2000
    }
}
