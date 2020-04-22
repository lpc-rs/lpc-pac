#[doc = "Reader of register RXPLEN"]
pub type R = crate::R<u32, super::RXPLEN>;
#[doc = "Reader of field `PKT_LNGTH`"]
pub type PKT_LNGTH_R = crate::R<u16, u16>;
#[doc = "Data valid. This bit is useful for isochronous endpoints. Non-isochronous endpoints do not raise an interrupt when an erroneous data packet is received. But invalid data packet can be produced with a bus reset. For isochronous endpoints, data transfer will happen even if an erroneous packet is received. In this case DV bit will not be set for the packet.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DV_A {
    #[doc = "0: Data is invalid."]
    DATA_IS_INVALID_ = 0,
    #[doc = "1: Data is valid."]
    DATA_IS_VALID_ = 1,
}
impl From<DV_A> for bool {
    #[inline(always)]
    fn from(variant: DV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DV`"]
pub type DV_R = crate::R<bool, DV_A>;
impl DV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DV_A {
        match self.bits {
            false => DV_A::DATA_IS_INVALID_,
            true => DV_A::DATA_IS_VALID_,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_IS_INVALID_`"]
    #[inline(always)]
    pub fn is_data_is_invalid_(&self) -> bool {
        *self == DV_A::DATA_IS_INVALID_
    }
    #[doc = "Checks if the value of the field is `DATA_IS_VALID_`"]
    #[inline(always)]
    pub fn is_data_is_valid_(&self) -> bool {
        *self == DV_A::DATA_IS_VALID_
    }
}
#[doc = "Reader of field `PKT_RDY`"]
pub type PKT_RDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:9 - The remaining number of bytes to be read from the currently selected endpoint's buffer. When this field decrements to 0, the RxENDPKT bit will be set in USBDevIntSt."]
    #[inline(always)]
    pub fn pkt_lngth(&self) -> PKT_LNGTH_R {
        PKT_LNGTH_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Data valid. This bit is useful for isochronous endpoints. Non-isochronous endpoints do not raise an interrupt when an erroneous data packet is received. But invalid data packet can be produced with a bus reset. For isochronous endpoints, data transfer will happen even if an erroneous packet is received. In this case DV bit will not be set for the packet."]
    #[inline(always)]
    pub fn dv(&self) -> DV_R {
        DV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - The PKT_LNGTH field is valid and the packet is ready for reading."]
    #[inline(always)]
    pub fn pkt_rdy(&self) -> PKT_RDY_R {
        PKT_RDY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
