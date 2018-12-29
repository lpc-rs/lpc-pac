#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::INTF {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `ILIM0_F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILIM0_FR {
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC,
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING,
}
impl ILIM0_FR {
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
            ILIM0_FR::THIS_INTERRUPT_SOURC => false,
            ILIM0_FR::IF_THE_CORRESPONDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ILIM0_FR {
        match value {
            false => ILIM0_FR::THIS_INTERRUPT_SOURC,
            true => ILIM0_FR::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == ILIM0_FR::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == ILIM0_FR::IF_THE_CORRESPONDING
    }
}
#[doc = "Possible values of the field `IMAT0_F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMAT0_FR {
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC,
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING,
}
impl IMAT0_FR {
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
            IMAT0_FR::THIS_INTERRUPT_SOURC => false,
            IMAT0_FR::IF_THE_CORRESPONDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IMAT0_FR {
        match value {
            false => IMAT0_FR::THIS_INTERRUPT_SOURC,
            true => IMAT0_FR::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == IMAT0_FR::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == IMAT0_FR::IF_THE_CORRESPONDING
    }
}
#[doc = "Possible values of the field `ICAP0_F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICAP0_FR {
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC,
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING,
}
impl ICAP0_FR {
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
            ICAP0_FR::THIS_INTERRUPT_SOURC => false,
            ICAP0_FR::IF_THE_CORRESPONDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ICAP0_FR {
        match value {
            false => ICAP0_FR::THIS_INTERRUPT_SOURC,
            true => ICAP0_FR::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == ICAP0_FR::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == ICAP0_FR::IF_THE_CORRESPONDING
    }
}
#[doc = "Possible values of the field `ILIM1_F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILIM1_FR {
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC,
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING,
}
impl ILIM1_FR {
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
            ILIM1_FR::THIS_INTERRUPT_SOURC => false,
            ILIM1_FR::IF_THE_CORRESPONDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ILIM1_FR {
        match value {
            false => ILIM1_FR::THIS_INTERRUPT_SOURC,
            true => ILIM1_FR::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == ILIM1_FR::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == ILIM1_FR::IF_THE_CORRESPONDING
    }
}
#[doc = "Possible values of the field `IMAT1_F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMAT1_FR {
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC,
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING,
}
impl IMAT1_FR {
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
            IMAT1_FR::THIS_INTERRUPT_SOURC => false,
            IMAT1_FR::IF_THE_CORRESPONDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IMAT1_FR {
        match value {
            false => IMAT1_FR::THIS_INTERRUPT_SOURC,
            true => IMAT1_FR::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == IMAT1_FR::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == IMAT1_FR::IF_THE_CORRESPONDING
    }
}
#[doc = "Possible values of the field `ICAP1_F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICAP1_FR {
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC,
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING,
}
impl ICAP1_FR {
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
            ICAP1_FR::THIS_INTERRUPT_SOURC => false,
            ICAP1_FR::IF_THE_CORRESPONDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ICAP1_FR {
        match value {
            false => ICAP1_FR::THIS_INTERRUPT_SOURC,
            true => ICAP1_FR::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == ICAP1_FR::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == ICAP1_FR::IF_THE_CORRESPONDING
    }
}
#[doc = "Possible values of the field `ILIM2_F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILIM2_FR {
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC,
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING,
}
impl ILIM2_FR {
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
            ILIM2_FR::THIS_INTERRUPT_SOURC => false,
            ILIM2_FR::IF_THE_CORRESPONDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ILIM2_FR {
        match value {
            false => ILIM2_FR::THIS_INTERRUPT_SOURC,
            true => ILIM2_FR::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == ILIM2_FR::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == ILIM2_FR::IF_THE_CORRESPONDING
    }
}
#[doc = "Possible values of the field `IMAT2_F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMAT2_FR {
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC,
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING,
}
impl IMAT2_FR {
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
            IMAT2_FR::THIS_INTERRUPT_SOURC => false,
            IMAT2_FR::IF_THE_CORRESPONDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IMAT2_FR {
        match value {
            false => IMAT2_FR::THIS_INTERRUPT_SOURC,
            true => IMAT2_FR::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == IMAT2_FR::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == IMAT2_FR::IF_THE_CORRESPONDING
    }
}
#[doc = "Possible values of the field `ICAP2_F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICAP2_FR {
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC,
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING,
}
impl ICAP2_FR {
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
            ICAP2_FR::THIS_INTERRUPT_SOURC => false,
            ICAP2_FR::IF_THE_CORRESPONDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ICAP2_FR {
        match value {
            false => ICAP2_FR::THIS_INTERRUPT_SOURC,
            true => ICAP2_FR::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == ICAP2_FR::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == ICAP2_FR::IF_THE_CORRESPONDING
    }
}
#[doc = "Possible values of the field `ABORT_F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABORT_FR {
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC,
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING,
}
impl ABORT_FR {
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
            ABORT_FR::THIS_INTERRUPT_SOURC => false,
            ABORT_FR::IF_THE_CORRESPONDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ABORT_FR {
        match value {
            false => ABORT_FR::THIS_INTERRUPT_SOURC,
            true => ABORT_FR::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == ABORT_FR::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == ABORT_FR::IF_THE_CORRESPONDING
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Limit interrupt flag for channel 0."]
    #[inline]
    pub fn ilim0_f(&self) -> ILIM0_FR {
        ILIM0_FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Match interrupt flag for channel 0."]
    #[inline]
    pub fn imat0_f(&self) -> IMAT0_FR {
        IMAT0_FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Capture interrupt flag for channel 0."]
    #[inline]
    pub fn icap0_f(&self) -> ICAP0_FR {
        ICAP0_FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Limit interrupt flag for channel 1."]
    #[inline]
    pub fn ilim1_f(&self) -> ILIM1_FR {
        ILIM1_FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Match interrupt flag for channel 1."]
    #[inline]
    pub fn imat1_f(&self) -> IMAT1_FR {
        IMAT1_FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Capture interrupt flag for channel 1."]
    #[inline]
    pub fn icap1_f(&self) -> ICAP1_FR {
        ICAP1_FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Limit interrupt flag for channel 2."]
    #[inline]
    pub fn ilim2_f(&self) -> ILIM2_FR {
        ILIM2_FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Match interrupt flag for channel 2."]
    #[inline]
    pub fn imat2_f(&self) -> IMAT2_FR {
        IMAT2_FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Capture interrupt flag for channel 2."]
    #[inline]
    pub fn icap2_f(&self) -> ICAP2_FR {
        ICAP2_FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Fast abort interrupt flag."]
    #[inline]
    pub fn abort_f(&self) -> ABORT_FR {
        ABORT_FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
