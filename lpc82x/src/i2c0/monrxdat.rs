#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MONRXDAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct MONRXDATR {
    bits: u8,
}
impl MONRXDATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `MONSTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONSTARTR {
    #[doc = "No detect. The monitor function has not detected a Start event on the I2C bus."]
    NO_DETECT,
    #[doc = "Start detect. The monitor function has detected a Start event on the I2C bus."]
    START_DETECT,
}
impl MONSTARTR {
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
            MONSTARTR::NO_DETECT => false,
            MONSTARTR::START_DETECT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MONSTARTR {
        match value {
            false => MONSTARTR::NO_DETECT,
            true => MONSTARTR::START_DETECT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DETECT`"]
    #[inline]
    pub fn is_no_detect(&self) -> bool {
        *self == MONSTARTR::NO_DETECT
    }
    #[doc = "Checks if the value of the field is `START_DETECT`"]
    #[inline]
    pub fn is_start_detect(&self) -> bool {
        *self == MONSTARTR::START_DETECT
    }
}
#[doc = "Possible values of the field `MONRESTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONRESTARTR {
    #[doc = "No start detect. The monitor function has not detected a Repeated Start event on the I2C bus."]
    NO_START_DETECT,
    #[doc = "Repeated start detect. The monitor function has detected a Repeated Start event on the I 2C bus."]
    REPEATED_START_DETEC,
}
impl MONRESTARTR {
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
            MONRESTARTR::NO_START_DETECT => false,
            MONRESTARTR::REPEATED_START_DETEC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MONRESTARTR {
        match value {
            false => MONRESTARTR::NO_START_DETECT,
            true => MONRESTARTR::REPEATED_START_DETEC,
        }
    }
    #[doc = "Checks if the value of the field is `NO_START_DETECT`"]
    #[inline]
    pub fn is_no_start_detect(&self) -> bool {
        *self == MONRESTARTR::NO_START_DETECT
    }
    #[doc = "Checks if the value of the field is `REPEATED_START_DETEC`"]
    #[inline]
    pub fn is_repeated_start_detec(&self) -> bool {
        *self == MONRESTARTR::REPEATED_START_DETEC
    }
}
#[doc = "Possible values of the field `MONNACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONNACKR {
    #[doc = "Acknowledged. The data currently being provided by the monitor function was acknowledged by at least one master or slave receiver."]
    ACKNOWLEDGED,
    #[doc = "Not acknowledged. The data currently being provided by the monitor function was not acknowledged by any receiver."]
    NOT_ACKNOWLEDGED,
}
impl MONNACKR {
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
            MONNACKR::ACKNOWLEDGED => false,
            MONNACKR::NOT_ACKNOWLEDGED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MONNACKR {
        match value {
            false => MONNACKR::ACKNOWLEDGED,
            true => MONNACKR::NOT_ACKNOWLEDGED,
        }
    }
    #[doc = "Checks if the value of the field is `ACKNOWLEDGED`"]
    #[inline]
    pub fn is_acknowledged(&self) -> bool {
        *self == MONNACKR::ACKNOWLEDGED
    }
    #[doc = "Checks if the value of the field is `NOT_ACKNOWLEDGED`"]
    #[inline]
    pub fn is_not_acknowledged(&self) -> bool {
        *self == MONNACKR::NOT_ACKNOWLEDGED
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Monitor function Receiver Data. This reflects every data byte that passes on the I2C pins, and adds indication of Start, Repeated Start, and data NACK."]
    #[inline]
    pub fn monrxdat(&self) -> MONRXDATR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MONRXDATR { bits }
    }
    #[doc = "Bit 8 - Monitor Received Start."]
    #[inline]
    pub fn monstart(&self) -> MONSTARTR {
        MONSTARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Monitor Received Repeated Start."]
    #[inline]
    pub fn monrestart(&self) -> MONRESTARTR {
        MONRESTARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Monitor Received NACK."]
    #[inline]
    pub fn monnack(&self) -> MONNACKR {
        MONNACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
