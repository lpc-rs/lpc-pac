#[doc = "Reader of register DMAINTST"]
pub type R = crate::R<u32, super::DMAINTST>;
#[doc = "End of Transfer Interrupt bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOT_A {
    #[doc = "0: All bits in the USBEoTIntSt register are 0."]
    ALL_BITS_IN_THE_USBE = 0,
    #[doc = "1: At least one bit in the USBEoTIntSt is set."]
    AT_LEAST_ONE_BIT_IN_ = 1,
}
impl From<EOT_A> for bool {
    #[inline(always)]
    fn from(variant: EOT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EOT`"]
pub type EOT_R = crate::R<bool, EOT_A>;
impl EOT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOT_A {
        match self.bits {
            false => EOT_A::ALL_BITS_IN_THE_USBE,
            true => EOT_A::AT_LEAST_ONE_BIT_IN_,
        }
    }
    #[doc = "Checks if the value of the field is `ALL_BITS_IN_THE_USBE`"]
    #[inline(always)]
    pub fn is_all_bits_in_the_usbe(&self) -> bool {
        *self == EOT_A::ALL_BITS_IN_THE_USBE
    }
    #[doc = "Checks if the value of the field is `AT_LEAST_ONE_BIT_IN_`"]
    #[inline(always)]
    pub fn is_at_least_one_bit_in_(&self) -> bool {
        *self == EOT_A::AT_LEAST_ONE_BIT_IN_
    }
}
#[doc = "New DD Request Interrupt bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDDR_A {
    #[doc = "0: All bits in the USBNDDRIntSt register are 0."]
    ALL_BITS_IN_THE_USBN = 0,
    #[doc = "1: At least one bit in the USBNDDRIntSt is set."]
    AT_LEAST_ONE_BIT_IN_ = 1,
}
impl From<NDDR_A> for bool {
    #[inline(always)]
    fn from(variant: NDDR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NDDR`"]
pub type NDDR_R = crate::R<bool, NDDR_A>;
impl NDDR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NDDR_A {
        match self.bits {
            false => NDDR_A::ALL_BITS_IN_THE_USBN,
            true => NDDR_A::AT_LEAST_ONE_BIT_IN_,
        }
    }
    #[doc = "Checks if the value of the field is `ALL_BITS_IN_THE_USBN`"]
    #[inline(always)]
    pub fn is_all_bits_in_the_usbn(&self) -> bool {
        *self == NDDR_A::ALL_BITS_IN_THE_USBN
    }
    #[doc = "Checks if the value of the field is `AT_LEAST_ONE_BIT_IN_`"]
    #[inline(always)]
    pub fn is_at_least_one_bit_in_(&self) -> bool {
        *self == NDDR_A::AT_LEAST_ONE_BIT_IN_
    }
}
#[doc = "System Error Interrupt bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR_A {
    #[doc = "0: All bits in the USBSysErrIntSt register are 0."]
    ALL_BITS_IN_THE_USBS = 0,
    #[doc = "1: At least one bit in the USBSysErrIntSt is set."]
    AT_LEAST_ONE_BIT_IN_ = 1,
}
impl From<ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERR`"]
pub type ERR_R = crate::R<bool, ERR_A>;
impl ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR_A {
        match self.bits {
            false => ERR_A::ALL_BITS_IN_THE_USBS,
            true => ERR_A::AT_LEAST_ONE_BIT_IN_,
        }
    }
    #[doc = "Checks if the value of the field is `ALL_BITS_IN_THE_USBS`"]
    #[inline(always)]
    pub fn is_all_bits_in_the_usbs(&self) -> bool {
        *self == ERR_A::ALL_BITS_IN_THE_USBS
    }
    #[doc = "Checks if the value of the field is `AT_LEAST_ONE_BIT_IN_`"]
    #[inline(always)]
    pub fn is_at_least_one_bit_in_(&self) -> bool {
        *self == ERR_A::AT_LEAST_ONE_BIT_IN_
    }
}
impl R {
    #[doc = "Bit 0 - End of Transfer Interrupt bit."]
    #[inline(always)]
    pub fn eot(&self) -> EOT_R {
        EOT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - New DD Request Interrupt bit."]
    #[inline(always)]
    pub fn nddr(&self) -> NDDR_R {
        NDDR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - System Error Interrupt bit."]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
