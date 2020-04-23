#[doc = "Reader of register CCR"]
pub type R = crate::R<u32, super::CCR>;
#[doc = "Reader of field `NumberOfAddressComparatorPairs`"]
pub type NUMBEROFADDRESSCOMPARATORPAIRS_R = crate::R<u8, u8>;
#[doc = "Reader of field `NDVC`"]
pub type NDVC_R = crate::R<u8, u8>;
#[doc = "Reader of field `NMMD`"]
pub type NMMD_R = crate::R<u8, u8>;
#[doc = "Reader of field `NC`"]
pub type NC_R = crate::R<u8, u8>;
#[doc = "Reader of field `SP`"]
pub type SP_R = crate::R<bool, bool>;
#[doc = "Reader of field `NEI`"]
pub type NEI_R = crate::R<u8, u8>;
#[doc = "Reader of field `NEO`"]
pub type NEO_R = crate::R<u8, u8>;
#[doc = "Reader of field `FFLP`"]
pub type FFLP_R = crate::R<bool, bool>;
#[doc = "Reader of field `NCIDC`"]
pub type NCIDC_R = crate::R<u8, u8>;
#[doc = "Reader of field `TSSBP`"]
pub type TSSBP_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMA`"]
pub type CMA_R = crate::R<bool, bool>;
#[doc = "Reader of field `ETMIDRP`"]
pub type ETMIDRP_R = crate::R<bool, bool>;
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
