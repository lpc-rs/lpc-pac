#[doc = "Reader of register SEQ_GDAT%s"]
pub type R = crate::R<u32, super::SEQ_GDAT>;
#[doc = "Reader of field `RESULT`"]
pub type RESULT_R = crate::R<u16, u16>;
#[doc = "Reader of field `THCMPRANGE`"]
pub type THCMPRANGE_R = crate::R<u8, u8>;
#[doc = "Reader of field `THCMPCROSS`"]
pub type THCMPCROSS_R = crate::R<u8, u8>;
#[doc = "Reader of field `CHN`"]
pub type CHN_R = crate::R<u8, u8>;
#[doc = "Reader of field `OVERRUN`"]
pub type OVERRUN_R = crate::R<bool, bool>;
#[doc = "Reader of field `DATAVALID`"]
pub type DATAVALID_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 4:15 - This field contains the 12-bit ADC conversion result from the most recent conversion performed under conversion sequence associated with this register. The result is a binary fraction representing the voltage on the currently-selected input channel as it falls within the range of VREFP to VREFN. Zero in the field indicates that the voltage on the input pin was less than, equal to, or close to that on VREFN, while 0xFFF indicates that the voltage on the input was close to, equal to, or greater than that on VREFP. DATAVALID = 1 indicates that this result has not yet been read."]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:17 - Indicates whether the result of the last conversion performed was above, below or within the range established by the designated threshold comparison registers (THRn_LOW and THRn_HIGH)."]
    #[inline(always)]
    pub fn thcmprange(&self) -> THCMPRANGE_R {
        THCMPRANGE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Indicates whether the result of the last conversion performed represented a crossing of the threshold level established by the designated LOW threshold comparison register (THRn_LOW) and, if so, in what direction the crossing occurred."]
    #[inline(always)]
    pub fn thcmpcross(&self) -> THCMPCROSS_R {
        THCMPCROSS_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 26:29 - These bits contain the channel from which the RESULT bits were converted (e.g. 0000 identifies channel 0, 0001 channel 1, etc.)."]
    #[inline(always)]
    pub fn chn(&self) -> CHN_R {
        CHN_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - This bit is set if a new conversion result is loaded into the RESULT field before a previous result has been read - i.e. while the DATAVALID bit is set. This bit is cleared, along with the DATAVALID bit, whenever this register is read. This bit will contribute to an overrun interrupt/DMA trigger if the MODE bit (in SEQAA_CTRL) for the corresponding sequence is set to '0' (and if the overrun interrupt is enabled)."]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - This bit is set to '1' at the end of each conversion when a new result is loaded into the RESULT field. It is cleared whenever this register is read. This bit will cause a conversion-complete interrupt for the corresponding sequence if the MODE bit (in SEQA_CTRL) for that sequence is set to 0 (and if the interrupt is enabled)."]
    #[inline(always)]
    pub fn datavalid(&self) -> DATAVALID_R {
        DATAVALID_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
