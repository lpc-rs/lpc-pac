///Register `SEQ_GDAT%s` reader
pub struct R(crate::R<SEQ_GDAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQ_GDAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQ_GDAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQ_GDAT_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RESULT` reader - This field contains the 12-bit ADC conversion result from the most recent conversion performed under conversion sequence associated with this register. The result is a binary fraction representing the voltage on the currently-selected input channel as it falls within the range of VREFP to VREFN. Zero in the field indicates that the voltage on the input pin was less than, equal to, or close to that on VREFN, while 0xFFF indicates that the voltage on the input was close to, equal to, or greater than that on VREFP. DATAVALID = 1 indicates that this result has not yet been read.
pub struct RESULT_R(crate::FieldReader<u16, u16>);
impl RESULT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RESULT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESULT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `THCMPRANGE` reader - Indicates whether the result of the last conversion performed was above, below or within the range established by the designated threshold comparison registers (THRn_LOW and THRn_HIGH).
pub struct THCMPRANGE_R(crate::FieldReader<u8, u8>);
impl THCMPRANGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        THCMPRANGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THCMPRANGE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `THCMPCROSS` reader - Indicates whether the result of the last conversion performed represented a crossing of the threshold level established by the designated LOW threshold comparison register (THRn_LOW) and, if so, in what direction the crossing occurred.
pub struct THCMPCROSS_R(crate::FieldReader<u8, u8>);
impl THCMPCROSS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        THCMPCROSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THCMPCROSS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CHN` reader - These bits contain the channel from which the RESULT bits were converted (e.g. 0000 identifies channel 0, 0001 channel 1, etc.).
pub struct CHN_R(crate::FieldReader<u8, u8>);
impl CHN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OVERRUN` reader - This bit is set if a new conversion result is loaded into the RESULT field before a previous result has been read - i.e. while the DATAVALID bit is set. This bit is cleared, along with the DATAVALID bit, whenever this register is read. This bit will contribute to an overrun interrupt/DMA trigger if the MODE bit (in SEQAA_CTRL) for the corresponding sequence is set to '0' (and if the overrun interrupt is enabled).
pub struct OVERRUN_R(crate::FieldReader<bool, bool>);
impl OVERRUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DATAVALID` reader - This bit is set to '1' at the end of each conversion when a new result is loaded into the RESULT field. It is cleared whenever this register is read. This bit will cause a conversion-complete interrupt for the corresponding sequence if the MODE bit (in SEQA_CTRL) for that sequence is set to 0 (and if the interrupt is enabled).
pub struct DATAVALID_R(crate::FieldReader<bool, bool>);
impl DATAVALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATAVALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAVALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 4:15 - This field contains the 12-bit ADC conversion result from the most recent conversion performed under conversion sequence associated with this register. The result is a binary fraction representing the voltage on the currently-selected input channel as it falls within the range of VREFP to VREFN. Zero in the field indicates that the voltage on the input pin was less than, equal to, or close to that on VREFN, while 0xFFF indicates that the voltage on the input was close to, equal to, or greater than that on VREFP. DATAVALID = 1 indicates that this result has not yet been read.
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    ///Bits 16:17 - Indicates whether the result of the last conversion performed was above, below or within the range established by the designated threshold comparison registers (THRn_LOW and THRn_HIGH).
    #[inline(always)]
    pub fn thcmprange(&self) -> THCMPRANGE_R {
        THCMPRANGE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    ///Bits 18:19 - Indicates whether the result of the last conversion performed represented a crossing of the threshold level established by the designated LOW threshold comparison register (THRn_LOW) and, if so, in what direction the crossing occurred.
    #[inline(always)]
    pub fn thcmpcross(&self) -> THCMPCROSS_R {
        THCMPCROSS_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    ///Bits 26:29 - These bits contain the channel from which the RESULT bits were converted (e.g. 0000 identifies channel 0, 0001 channel 1, etc.).
    #[inline(always)]
    pub fn chn(&self) -> CHN_R {
        CHN_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    ///Bit 30 - This bit is set if a new conversion result is loaded into the RESULT field before a previous result has been read - i.e. while the DATAVALID bit is set. This bit is cleared, along with the DATAVALID bit, whenever this register is read. This bit will contribute to an overrun interrupt/DMA trigger if the MODE bit (in SEQAA_CTRL) for the corresponding sequence is set to '0' (and if the overrun interrupt is enabled).
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - This bit is set to '1' at the end of each conversion when a new result is loaded into the RESULT field. It is cleared whenever this register is read. This bit will cause a conversion-complete interrupt for the corresponding sequence if the MODE bit (in SEQA_CTRL) for that sequence is set to 0 (and if the interrupt is enabled).
    #[inline(always)]
    pub fn datavalid(&self) -> DATAVALID_R {
        DATAVALID_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
///ADC Sequence-n Global Data register. This register contains the result of the most recent ADC conversion performed under sequence-n.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [seq_gdat](index.html) module
pub struct SEQ_GDAT_SPEC;
impl crate::RegisterSpec for SEQ_GDAT_SPEC {
    type Ux = u32;
}
///`read()` method returns [seq_gdat::R](R) reader structure
impl crate::Readable for SEQ_GDAT_SPEC {
    type Reader = R;
}
///`reset()` method sets SEQ_GDAT%s to value 0
impl crate::Resettable for SEQ_GDAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
