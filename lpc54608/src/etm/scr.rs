#[doc = "Reader of register SCR"]
pub type R = crate::R<u32, super::SCR>;
#[doc = "Reader of field `MaximumPortSize`"]
pub type MAXIMUMPORTSIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `FIFOFULLsupported`"]
pub type FIFOFULLSUPPORTED_R = crate::R<bool, bool>;
#[doc = "Reader of field `MaximumPortSize3`"]
pub type MAXIMUMPORTSIZE3_R = crate::R<bool, bool>;
#[doc = "Reader of field `PortSizeSupported`"]
pub type PORTSIZESUPPORTED_R = crate::R<bool, bool>;
#[doc = "Reader of field `PortModeSupported`"]
pub type PORTMODESUPPORTED_R = crate::R<bool, bool>;
#[doc = "Reader of field `N`"]
pub type N_R = crate::R<u8, u8>;
#[doc = "Reader of field `NoFetchComparisons`"]
pub type NOFETCHCOMPARISONS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:2 - Maximum ETM port size bits \\[2:0\\]. These bits are used in conjunction with bit \\[9\\]. The value of these bits is b001."]
    #[inline(always)]
    pub fn maximum_port_size(&self) -> MAXIMUMPORTSIZE_R {
        MAXIMUMPORTSIZE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - FIFOFULL supported. The value of this bit is 1, indicating that FIFOFULL is supported. This bit is used in conjunction with bit \\[23\\]
of the ETMCCR."]
    #[inline(always)]
    pub fn fifofullsupported(&self) -> FIFOFULLSUPPORTED_R {
        FIFOFULLSUPPORTED_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Maximum ETM port size bit \\[3\\]. This bit is used in conjunction with bits \\[2:0\\]. Its value is 0. This has no effect on the TPIU trace port."]
    #[inline(always)]
    pub fn maximum_port_size3(&self) -> MAXIMUMPORTSIZE3_R {
        MAXIMUMPORTSIZE3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port size supported. This bit reads as 1 if the currently selected port size is supported. This has no effect on the TPIU trace port."]
    #[inline(always)]
    pub fn port_size_supported(&self) -> PORTSIZESUPPORTED_R {
        PORTSIZESUPPORTED_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port mode supported. This bit reads as 1 if the currently selected port mode is supported. This has no effect on the TPIU trace port."]
    #[inline(always)]
    pub fn port_mode_supported(&self) -> PORTMODESUPPORTED_R {
        PORTMODESUPPORTED_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - These bits give the number of supported processors minus 1. The value of these bits is b000, indicating that there is only one processor connected."]
    #[inline(always)]
    pub fn n(&self) -> N_R {
        N_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 17 - No Fetch comparisons. The value of this bit is 1, indicating that fetch comparisons are not implemented."]
    #[inline(always)]
    pub fn no_fetch_comparisons(&self) -> NOFETCHCOMPARISONS_R {
        NOFETCHCOMPARISONS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
