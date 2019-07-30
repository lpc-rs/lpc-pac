#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CHAN_THRSEL {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `CH0_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0_THRSELR {
    #[doc = "Threshold 0. Channel 0 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 0 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl crate::ToBits<bool> for CH0_THRSELR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CH0_THRSELR::THRESHOLD_0 => false,
            CH0_THRSELR::THRESHOLD_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CH0_THRSEL_R = crate::FR<bool, CH0_THRSELR>;
impl CH0_THRSEL_R {
    #[doc = "Checks if the value of the field is `THRESHOLD_0`"]
    #[inline(always)]
    pub fn is_threshold_0(&self) -> bool {
        *self == CH0_THRSELR::THRESHOLD_0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_1`"]
    #[inline(always)]
    pub fn is_threshold_1(&self) -> bool {
        *self == CH0_THRSELR::THRESHOLD_1
    }
}
#[doc = "Possible values of the field `CH1_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1_THRSELR {
    #[doc = "Threshold 0. Channel 1 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 1 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl crate::ToBits<bool> for CH1_THRSELR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CH1_THRSELR::THRESHOLD_0 => false,
            CH1_THRSELR::THRESHOLD_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CH1_THRSEL_R = crate::FR<bool, CH1_THRSELR>;
impl CH1_THRSEL_R {
    #[doc = "Checks if the value of the field is `THRESHOLD_0`"]
    #[inline(always)]
    pub fn is_threshold_0(&self) -> bool {
        *self == CH1_THRSELR::THRESHOLD_0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_1`"]
    #[inline(always)]
    pub fn is_threshold_1(&self) -> bool {
        *self == CH1_THRSELR::THRESHOLD_1
    }
}
#[doc = "Possible values of the field `CH2_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2_THRSELR {
    #[doc = "Threshold 0. Channel 2 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 2 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl crate::ToBits<bool> for CH2_THRSELR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CH2_THRSELR::THRESHOLD_0 => false,
            CH2_THRSELR::THRESHOLD_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CH2_THRSEL_R = crate::FR<bool, CH2_THRSELR>;
impl CH2_THRSEL_R {
    #[doc = "Checks if the value of the field is `THRESHOLD_0`"]
    #[inline(always)]
    pub fn is_threshold_0(&self) -> bool {
        *self == CH2_THRSELR::THRESHOLD_0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_1`"]
    #[inline(always)]
    pub fn is_threshold_1(&self) -> bool {
        *self == CH2_THRSELR::THRESHOLD_1
    }
}
#[doc = "Possible values of the field `CH3_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3_THRSELR {
    #[doc = "Threshold 0. Channel 3 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 3 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl crate::ToBits<bool> for CH3_THRSELR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CH3_THRSELR::THRESHOLD_0 => false,
            CH3_THRSELR::THRESHOLD_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CH3_THRSEL_R = crate::FR<bool, CH3_THRSELR>;
impl CH3_THRSEL_R {
    #[doc = "Checks if the value of the field is `THRESHOLD_0`"]
    #[inline(always)]
    pub fn is_threshold_0(&self) -> bool {
        *self == CH3_THRSELR::THRESHOLD_0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_1`"]
    #[inline(always)]
    pub fn is_threshold_1(&self) -> bool {
        *self == CH3_THRSELR::THRESHOLD_1
    }
}
#[doc = "Possible values of the field `CH4_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4_THRSELR {
    #[doc = "Threshold 0. Channel 4 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 4 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl crate::ToBits<bool> for CH4_THRSELR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CH4_THRSELR::THRESHOLD_0 => false,
            CH4_THRSELR::THRESHOLD_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CH4_THRSEL_R = crate::FR<bool, CH4_THRSELR>;
impl CH4_THRSEL_R {
    #[doc = "Checks if the value of the field is `THRESHOLD_0`"]
    #[inline(always)]
    pub fn is_threshold_0(&self) -> bool {
        *self == CH4_THRSELR::THRESHOLD_0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_1`"]
    #[inline(always)]
    pub fn is_threshold_1(&self) -> bool {
        *self == CH4_THRSELR::THRESHOLD_1
    }
}
#[doc = "Possible values of the field `CH5_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5_THRSELR {
    #[doc = "Threshold 0. Channel 5 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 5 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl crate::ToBits<bool> for CH5_THRSELR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CH5_THRSELR::THRESHOLD_0 => false,
            CH5_THRSELR::THRESHOLD_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CH5_THRSEL_R = crate::FR<bool, CH5_THRSELR>;
impl CH5_THRSEL_R {
    #[doc = "Checks if the value of the field is `THRESHOLD_0`"]
    #[inline(always)]
    pub fn is_threshold_0(&self) -> bool {
        *self == CH5_THRSELR::THRESHOLD_0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_1`"]
    #[inline(always)]
    pub fn is_threshold_1(&self) -> bool {
        *self == CH5_THRSELR::THRESHOLD_1
    }
}
#[doc = "Possible values of the field `CH6_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6_THRSELR {
    #[doc = "Threshold 0. Channel 6 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 6 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl crate::ToBits<bool> for CH6_THRSELR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CH6_THRSELR::THRESHOLD_0 => false,
            CH6_THRSELR::THRESHOLD_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CH6_THRSEL_R = crate::FR<bool, CH6_THRSELR>;
impl CH6_THRSEL_R {
    #[doc = "Checks if the value of the field is `THRESHOLD_0`"]
    #[inline(always)]
    pub fn is_threshold_0(&self) -> bool {
        *self == CH6_THRSELR::THRESHOLD_0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_1`"]
    #[inline(always)]
    pub fn is_threshold_1(&self) -> bool {
        *self == CH6_THRSELR::THRESHOLD_1
    }
}
#[doc = "Possible values of the field `CH7_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7_THRSELR {
    #[doc = "Threshold 0. Channel 7 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 7 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl crate::ToBits<bool> for CH7_THRSELR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CH7_THRSELR::THRESHOLD_0 => false,
            CH7_THRSELR::THRESHOLD_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CH7_THRSEL_R = crate::FR<bool, CH7_THRSELR>;
impl CH7_THRSEL_R {
    #[doc = "Checks if the value of the field is `THRESHOLD_0`"]
    #[inline(always)]
    pub fn is_threshold_0(&self) -> bool {
        *self == CH7_THRSELR::THRESHOLD_0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_1`"]
    #[inline(always)]
    pub fn is_threshold_1(&self) -> bool {
        *self == CH7_THRSELR::THRESHOLD_1
    }
}
#[doc = "Possible values of the field `CH8_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH8_THRSELR {
    #[doc = "Threshold 0. Channel 8 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 8 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl crate::ToBits<bool> for CH8_THRSELR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CH8_THRSELR::THRESHOLD_0 => false,
            CH8_THRSELR::THRESHOLD_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CH8_THRSEL_R = crate::FR<bool, CH8_THRSELR>;
impl CH8_THRSEL_R {
    #[doc = "Checks if the value of the field is `THRESHOLD_0`"]
    #[inline(always)]
    pub fn is_threshold_0(&self) -> bool {
        *self == CH8_THRSELR::THRESHOLD_0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_1`"]
    #[inline(always)]
    pub fn is_threshold_1(&self) -> bool {
        *self == CH8_THRSELR::THRESHOLD_1
    }
}
#[doc = "Possible values of the field `CH9_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH9_THRSELR {
    #[doc = "Threshold 0. Channel 9 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 9 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl crate::ToBits<bool> for CH9_THRSELR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CH9_THRSELR::THRESHOLD_0 => false,
            CH9_THRSELR::THRESHOLD_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CH9_THRSEL_R = crate::FR<bool, CH9_THRSELR>;
impl CH9_THRSEL_R {
    #[doc = "Checks if the value of the field is `THRESHOLD_0`"]
    #[inline(always)]
    pub fn is_threshold_0(&self) -> bool {
        *self == CH9_THRSELR::THRESHOLD_0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_1`"]
    #[inline(always)]
    pub fn is_threshold_1(&self) -> bool {
        *self == CH9_THRSELR::THRESHOLD_1
    }
}
#[doc = "Possible values of the field `CH10_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH10_THRSELR {
    #[doc = "Threshold 0. Channel 10 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 10 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl crate::ToBits<bool> for CH10_THRSELR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CH10_THRSELR::THRESHOLD_0 => false,
            CH10_THRSELR::THRESHOLD_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CH10_THRSEL_R = crate::FR<bool, CH10_THRSELR>;
impl CH10_THRSEL_R {
    #[doc = "Checks if the value of the field is `THRESHOLD_0`"]
    #[inline(always)]
    pub fn is_threshold_0(&self) -> bool {
        *self == CH10_THRSELR::THRESHOLD_0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_1`"]
    #[inline(always)]
    pub fn is_threshold_1(&self) -> bool {
        *self == CH10_THRSELR::THRESHOLD_1
    }
}
#[doc = "Possible values of the field `CH11_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH11_THRSELR {
    #[doc = "Threshold 0. Channel 11 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 11 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl crate::ToBits<bool> for CH11_THRSELR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CH11_THRSELR::THRESHOLD_0 => false,
            CH11_THRSELR::THRESHOLD_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CH11_THRSEL_R = crate::FR<bool, CH11_THRSELR>;
impl CH11_THRSEL_R {
    #[doc = "Checks if the value of the field is `THRESHOLD_0`"]
    #[inline(always)]
    pub fn is_threshold_0(&self) -> bool {
        *self == CH11_THRSELR::THRESHOLD_0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_1`"]
    #[inline(always)]
    pub fn is_threshold_1(&self) -> bool {
        *self == CH11_THRSELR::THRESHOLD_1
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Threshold select by channel."]
    #[inline(always)]
    pub fn ch0_thrsel(&self) -> CH0_THRSEL_R {
        CH0_THRSEL_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Threshold select by channel."]
    #[inline(always)]
    pub fn ch1_thrsel(&self) -> CH1_THRSEL_R {
        CH1_THRSEL_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Threshold select by channel."]
    #[inline(always)]
    pub fn ch2_thrsel(&self) -> CH2_THRSEL_R {
        CH2_THRSEL_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Threshold select by channel."]
    #[inline(always)]
    pub fn ch3_thrsel(&self) -> CH3_THRSEL_R {
        CH3_THRSEL_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Threshold select by channel."]
    #[inline(always)]
    pub fn ch4_thrsel(&self) -> CH4_THRSEL_R {
        CH4_THRSEL_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Threshold select by channel."]
    #[inline(always)]
    pub fn ch5_thrsel(&self) -> CH5_THRSEL_R {
        CH5_THRSEL_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Threshold select by channel."]
    #[inline(always)]
    pub fn ch6_thrsel(&self) -> CH6_THRSEL_R {
        CH6_THRSEL_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Threshold select by channel."]
    #[inline(always)]
    pub fn ch7_thrsel(&self) -> CH7_THRSEL_R {
        CH7_THRSEL_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Threshold select by channel."]
    #[inline(always)]
    pub fn ch8_thrsel(&self) -> CH8_THRSEL_R {
        CH8_THRSEL_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Threshold select by channel."]
    #[inline(always)]
    pub fn ch9_thrsel(&self) -> CH9_THRSEL_R {
        CH9_THRSEL_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Threshold select by channel."]
    #[inline(always)]
    pub fn ch10_thrsel(&self) -> CH10_THRSEL_R {
        CH10_THRSEL_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Threshold select by channel."]
    #[inline(always)]
    pub fn ch11_thrsel(&self) -> CH11_THRSEL_R {
        CH11_THRSEL_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
}
