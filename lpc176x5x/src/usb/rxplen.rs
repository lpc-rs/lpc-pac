#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RXPLEN {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct PKT_LNGTHR {
    bits: u16,
}
impl PKT_LNGTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `DV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DVR {
    #[doc = "Data is invalid."]
    DATA_IS_INVALID_,
    #[doc = "Data is valid."]
    DATA_IS_VALID_,
}
impl DVR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DVR::DATA_IS_INVALID_ => false,
            DVR::DATA_IS_VALID_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DVR {
        match value {
            false => DVR::DATA_IS_INVALID_,
            true => DVR::DATA_IS_VALID_,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_IS_INVALID_`"]
    #[inline]
    pub fn is_data_is_invalid_(&self) -> bool {
        *self == DVR::DATA_IS_INVALID_
    }
    #[doc = "Checks if the value of the field is `DATA_IS_VALID_`"]
    #[inline]
    pub fn is_data_is_valid_(&self) -> bool {
        *self == DVR::DATA_IS_VALID_
    }
}
#[doc = r" Value of the field"]
pub struct PKT_RDYR {
    bits: bool,
}
impl PKT_RDYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:9 - The remaining number of bytes to be read from the currently selected endpoint's buffer. When this field decrements to 0, the RxENDPKT bit will be set in USBDevIntSt."]
    #[inline]
    pub fn pkt_lngth(&self) -> PKT_LNGTHR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PKT_LNGTHR { bits }
    }
    #[doc = "Bit 10 - Data valid. This bit is useful for isochronous endpoints. Non-isochronous endpoints do not raise an interrupt when an erroneous data packet is received. But invalid data packet can be produced with a bus reset. For isochronous endpoints, data transfer will happen even if an erroneous packet is received. In this case DV bit will not be set for the packet."]
    #[inline]
    pub fn dv(&self) -> DVR {
        DVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - The PKT_LNGTH field is valid and the packet is ready for reading."]
    #[inline]
    pub fn pkt_rdy(&self) -> PKT_RDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PKT_RDYR { bits }
    }
}
