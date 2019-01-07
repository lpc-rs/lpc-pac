#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::IIR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `INTSTATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSTATUSR {
    #[doc = "At least one interrupt is pending."]
    PENDING,
    #[doc = "No interrupt is pending."]
    NONE,
}
impl INTSTATUSR {
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
            INTSTATUSR::PENDING => false,
            INTSTATUSR::NONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTSTATUSR {
        match value {
            false => INTSTATUSR::PENDING,
            true => INTSTATUSR::NONE,
        }
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == INTSTATUSR::PENDING
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == INTSTATUSR::NONE
    }
}
#[doc = "Possible values of the field `INTID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTIDR {
    #[doc = "1   - Receive Line Status (RLS)."]
    RECEIVE_LINE_STATUS,
    #[doc = "2a - Receive Data Available (RDA)."]
    RECEIVE_DATA_AVAILABLE,
    #[doc = "2b - Character Time-out Indicator (CTI)."]
    CHARACTER_TIMEOUT_INDICATOR,
    #[doc = "3   - THRE Interrupt."]
    THRE_INTERRUPT,
    #[doc = "4   - Modem status"]
    MODEM_STATUS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INTIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INTIDR::RECEIVE_LINE_STATUS => 3,
            INTIDR::RECEIVE_DATA_AVAILABLE => 2,
            INTIDR::CHARACTER_TIMEOUT_INDICATOR => 6,
            INTIDR::THRE_INTERRUPT => 1,
            INTIDR::MODEM_STATUS => 0,
            INTIDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INTIDR {
        match value {
            3 => INTIDR::RECEIVE_LINE_STATUS,
            2 => INTIDR::RECEIVE_DATA_AVAILABLE,
            6 => INTIDR::CHARACTER_TIMEOUT_INDICATOR,
            1 => INTIDR::THRE_INTERRUPT,
            0 => INTIDR::MODEM_STATUS,
            i => INTIDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RECEIVE_LINE_STATUS`"]
    #[inline]
    pub fn is_receive_line_status(&self) -> bool {
        *self == INTIDR::RECEIVE_LINE_STATUS
    }
    #[doc = "Checks if the value of the field is `RECEIVE_DATA_AVAILABLE`"]
    #[inline]
    pub fn is_receive_data_available(&self) -> bool {
        *self == INTIDR::RECEIVE_DATA_AVAILABLE
    }
    #[doc = "Checks if the value of the field is `CHARACTER_TIMEOUT_INDICATOR`"]
    #[inline]
    pub fn is_character_timeout_indicator(&self) -> bool {
        *self == INTIDR::CHARACTER_TIMEOUT_INDICATOR
    }
    #[doc = "Checks if the value of the field is `THRE_INTERRUPT`"]
    #[inline]
    pub fn is_thre_interrupt(&self) -> bool {
        *self == INTIDR::THRE_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `MODEM_STATUS`"]
    #[inline]
    pub fn is_modem_status(&self) -> bool {
        *self == INTIDR::MODEM_STATUS
    }
}
#[doc = r" Value of the field"]
pub struct FIFOENR {
    bits: u8,
}
impl FIFOENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ABEOINTR {
    bits: bool,
}
impl ABEOINTR {
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
#[doc = r" Value of the field"]
pub struct ABTOINTR {
    bits: bool,
}
impl ABTOINTR {
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
    #[doc = "Bit 0 - Interrupt status. Note that IIR\\[0\\] is active low. The pending interrupt can be determined by evaluating IIR\\[3:1\\]."]
    #[inline]
    pub fn intstatus(&self) -> INTSTATUSR {
        INTSTATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:3 - Interrupt identification. IER\\[3:1\\] identifies an interrupt corresponding to the USART Rx FIFO. All other values of IER\\[3:1\\] not listed below are reserved."]
    #[inline]
    pub fn intid(&self) -> INTIDR {
        INTIDR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - These bits are equivalent to FCR\\[0\\]."]
    #[inline]
    pub fn fifoen(&self) -> FIFOENR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FIFOENR { bits }
    }
    #[doc = "Bit 8 - End of auto-baud interrupt. True if auto-baud has finished successfully and interrupt is enabled."]
    #[inline]
    pub fn abeoint(&self) -> ABEOINTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ABEOINTR { bits }
    }
    #[doc = "Bit 9 - Auto-baud time-out interrupt. True if auto-baud has timed out and interrupt is enabled."]
    #[inline]
    pub fn abtoint(&self) -> ABTOINTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ABTOINTR { bits }
    }
}
