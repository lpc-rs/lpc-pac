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
    AT_LEAST_ONE_INTERRU,
    #[doc = "No interrupt is pending."]
    NO_INTERRUPT_IS_PEND,
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
            INTSTATUSR::AT_LEAST_ONE_INTERRU => false,
            INTSTATUSR::NO_INTERRUPT_IS_PEND => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTSTATUSR {
        match value {
            false => INTSTATUSR::AT_LEAST_ONE_INTERRU,
            true => INTSTATUSR::NO_INTERRUPT_IS_PEND,
        }
    }
    #[doc = "Checks if the value of the field is `AT_LEAST_ONE_INTERRU`"]
    #[inline]
    pub fn is_at_least_one_interru(&self) -> bool {
        *self == INTSTATUSR::AT_LEAST_ONE_INTERRU
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT_IS_PEND`"]
    #[inline]
    pub fn is_no_interrupt_is_pend(&self) -> bool {
        *self == INTSTATUSR::NO_INTERRUPT_IS_PEND
    }
}
#[doc = "Possible values of the field `INTID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTIDR {
    #[doc = "1   - Receive Line Status (RLS)."]
    RLS,
    #[doc = "2a - Receive Data Available (RDA)."]
    RDA,
    #[doc = "2b - Character Time-out Indicator (CTI)."]
    CTI,
    #[doc = "3   - THRE Interrupt."]
    THRE,
    #[doc = "4   - Modem Interrupt."]
    MODEM,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INTIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INTIDR::RLS => 3,
            INTIDR::RDA => 2,
            INTIDR::CTI => 6,
            INTIDR::THRE => 1,
            INTIDR::MODEM => 0,
            INTIDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INTIDR {
        match value {
            3 => INTIDR::RLS,
            2 => INTIDR::RDA,
            6 => INTIDR::CTI,
            1 => INTIDR::THRE,
            0 => INTIDR::MODEM,
            i => INTIDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RLS`"]
    #[inline]
    pub fn is_rls(&self) -> bool {
        *self == INTIDR::RLS
    }
    #[doc = "Checks if the value of the field is `RDA`"]
    #[inline]
    pub fn is_rda(&self) -> bool {
        *self == INTIDR::RDA
    }
    #[doc = "Checks if the value of the field is `CTI`"]
    #[inline]
    pub fn is_cti(&self) -> bool {
        *self == INTIDR::CTI
    }
    #[doc = "Checks if the value of the field is `THRE`"]
    #[inline]
    pub fn is_thre(&self) -> bool {
        *self == INTIDR::THRE
    }
    #[doc = "Checks if the value of the field is `MODEM`"]
    #[inline]
    pub fn is_modem(&self) -> bool {
        *self == INTIDR::MODEM
    }
}
#[doc = r" Value of the field"]
pub struct FIFOENABLER {
    bits: u8,
}
impl FIFOENABLER {
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
    #[doc = "Bits 1:3 - Interrupt identification. IER\\[3:1\\] identifies an interrupt corresponding to the UART1 Rx or TX FIFO. All other combinations of IER\\[3:1\\] not listed below are reserved (100,101,111)."]
    #[inline]
    pub fn intid(&self) -> INTIDR {
        INTIDR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Copies of FCR\\[0\\]."]
    #[inline]
    pub fn fifoenable(&self) -> FIFOENABLER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FIFOENABLER { bits }
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
