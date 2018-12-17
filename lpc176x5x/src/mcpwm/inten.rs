#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::INTEN {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `ILIM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILIM0R {
    #[doc = "Interrupt disabled."]
    INTERRUPT_DISABLED_,
    #[doc = "Interrupt enabled."]
    INTERRUPT_ENABLED_,
}
impl ILIM0R {
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
            ILIM0R::INTERRUPT_DISABLED_ => false,
            ILIM0R::INTERRUPT_ENABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ILIM0R {
        match value {
            false => ILIM0R::INTERRUPT_DISABLED_,
            true => ILIM0R::INTERRUPT_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == ILIM0R::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == ILIM0R::INTERRUPT_ENABLED_
    }
}
#[doc = "Possible values of the field `IMAT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMAT0R {
    #[doc = "Interrupt disabled."]
    INTERRUPT_DISABLED_,
    #[doc = "Interrupt enabled."]
    INTERRUPT_ENABLED_,
}
impl IMAT0R {
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
            IMAT0R::INTERRUPT_DISABLED_ => false,
            IMAT0R::INTERRUPT_ENABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IMAT0R {
        match value {
            false => IMAT0R::INTERRUPT_DISABLED_,
            true => IMAT0R::INTERRUPT_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == IMAT0R::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == IMAT0R::INTERRUPT_ENABLED_
    }
}
#[doc = "Possible values of the field `ICAP0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICAP0R {
    #[doc = "Interrupt disabled."]
    INTERRUPT_DISABLED_,
    #[doc = "Interrupt enabled."]
    INTERRUPT_ENABLED_,
}
impl ICAP0R {
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
            ICAP0R::INTERRUPT_DISABLED_ => false,
            ICAP0R::INTERRUPT_ENABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ICAP0R {
        match value {
            false => ICAP0R::INTERRUPT_DISABLED_,
            true => ICAP0R::INTERRUPT_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == ICAP0R::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == ICAP0R::INTERRUPT_ENABLED_
    }
}
#[doc = "Possible values of the field `ILIM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILIM1R {
    #[doc = "Interrupt disabled."]
    INTERRUPT_DISABLED_,
    #[doc = "Interrupt enabled."]
    INTERRUPT_ENABLED_,
}
impl ILIM1R {
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
            ILIM1R::INTERRUPT_DISABLED_ => false,
            ILIM1R::INTERRUPT_ENABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ILIM1R {
        match value {
            false => ILIM1R::INTERRUPT_DISABLED_,
            true => ILIM1R::INTERRUPT_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == ILIM1R::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == ILIM1R::INTERRUPT_ENABLED_
    }
}
#[doc = "Possible values of the field `IMAT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMAT1R {
    #[doc = "Interrupt disabled."]
    INTERRUPT_DISABLED_,
    #[doc = "Interrupt enabled."]
    INTERRUPT_ENABLED_,
}
impl IMAT1R {
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
            IMAT1R::INTERRUPT_DISABLED_ => false,
            IMAT1R::INTERRUPT_ENABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IMAT1R {
        match value {
            false => IMAT1R::INTERRUPT_DISABLED_,
            true => IMAT1R::INTERRUPT_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == IMAT1R::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == IMAT1R::INTERRUPT_ENABLED_
    }
}
#[doc = "Possible values of the field `ICAP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICAP1R {
    #[doc = "Interrupt disabled."]
    INTERRUPT_DISABLED_,
    #[doc = "Interrupt enabled."]
    INTERRUPT_ENABLED_,
}
impl ICAP1R {
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
            ICAP1R::INTERRUPT_DISABLED_ => false,
            ICAP1R::INTERRUPT_ENABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ICAP1R {
        match value {
            false => ICAP1R::INTERRUPT_DISABLED_,
            true => ICAP1R::INTERRUPT_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == ICAP1R::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == ICAP1R::INTERRUPT_ENABLED_
    }
}
#[doc = "Possible values of the field `ILIM2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILIM2R {
    #[doc = "Interrupt disabled."]
    INTERRUPT_DISABLED_,
    #[doc = "Interrupt enabled."]
    INTERRUPT_ENABLED_,
}
impl ILIM2R {
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
            ILIM2R::INTERRUPT_DISABLED_ => false,
            ILIM2R::INTERRUPT_ENABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ILIM2R {
        match value {
            false => ILIM2R::INTERRUPT_DISABLED_,
            true => ILIM2R::INTERRUPT_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == ILIM2R::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == ILIM2R::INTERRUPT_ENABLED_
    }
}
#[doc = "Possible values of the field `IMAT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMAT2R {
    #[doc = "Interrupt disabled."]
    INTERRUPT_DISABLED_,
    #[doc = "Interrupt enabled."]
    INTERRUPT_ENABLED_,
}
impl IMAT2R {
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
            IMAT2R::INTERRUPT_DISABLED_ => false,
            IMAT2R::INTERRUPT_ENABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IMAT2R {
        match value {
            false => IMAT2R::INTERRUPT_DISABLED_,
            true => IMAT2R::INTERRUPT_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == IMAT2R::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == IMAT2R::INTERRUPT_ENABLED_
    }
}
#[doc = "Possible values of the field `ICAP2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICAP2R {
    #[doc = "Interrupt disabled."]
    INTERRUPT_DISABLED_,
    #[doc = "Interrupt enabled."]
    INTERRUPT_ENABLED_,
}
impl ICAP2R {
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
            ICAP2R::INTERRUPT_DISABLED_ => false,
            ICAP2R::INTERRUPT_ENABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ICAP2R {
        match value {
            false => ICAP2R::INTERRUPT_DISABLED_,
            true => ICAP2R::INTERRUPT_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == ICAP2R::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == ICAP2R::INTERRUPT_ENABLED_
    }
}
#[doc = "Possible values of the field `ABORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABORTR {
    #[doc = "Interrupt disabled."]
    INTERRUPT_DISABLED_,
    #[doc = "Interrupt enabled."]
    INTERRUPT_ENABLED_,
}
impl ABORTR {
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
            ABORTR::INTERRUPT_DISABLED_ => false,
            ABORTR::INTERRUPT_ENABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ABORTR {
        match value {
            false => ABORTR::INTERRUPT_DISABLED_,
            true => ABORTR::INTERRUPT_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == ABORTR::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == ABORTR::INTERRUPT_ENABLED_
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Limit interrupt for channel 0."]
    #[inline]
    pub fn ilim0(&self) -> ILIM0R {
        ILIM0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Match interrupt for channel 0."]
    #[inline]
    pub fn imat0(&self) -> IMAT0R {
        IMAT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Capture interrupt for channel 0."]
    #[inline]
    pub fn icap0(&self) -> ICAP0R {
        ICAP0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Limit interrupt for channel 1."]
    #[inline]
    pub fn ilim1(&self) -> ILIM1R {
        ILIM1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Match interrupt for channel 1."]
    #[inline]
    pub fn imat1(&self) -> IMAT1R {
        IMAT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Capture interrupt for channel 1."]
    #[inline]
    pub fn icap1(&self) -> ICAP1R {
        ICAP1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Limit interrupt for channel 2."]
    #[inline]
    pub fn ilim2(&self) -> ILIM2R {
        ILIM2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Match interrupt for channel 2."]
    #[inline]
    pub fn imat2(&self) -> IMAT2R {
        IMAT2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Capture interrupt for channel 2."]
    #[inline]
    pub fn icap2(&self) -> ICAP2R {
        ICAP2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Fast abort interrupt."]
    #[inline]
    pub fn abort(&self) -> ABORTR {
        ABORTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
