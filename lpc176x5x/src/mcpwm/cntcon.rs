#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CNTCON {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `TC0MCI0_RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC0MCI0_RER {
    #[doc = "A rising edge on MCI0 does not affect counter 0."]
    A_RISING_EDGE_ON_MCI,
    #[doc = "If MODE0 is 1, counter 0 advances on a rising edge on MCI0."]
    RISING,
}
impl TC0MCI0_RER {
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
            TC0MCI0_RER::A_RISING_EDGE_ON_MCI => false,
            TC0MCI0_RER::RISING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TC0MCI0_RER {
        match value {
            false => TC0MCI0_RER::A_RISING_EDGE_ON_MCI,
            true => TC0MCI0_RER::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC0MCI0_RER::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == TC0MCI0_RER::RISING
    }
}
#[doc = "Possible values of the field `TC0MCI0_FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC0MCI0_FER {
    #[doc = "A falling edge on MCI0 does not affect counter 0."]
    A_FALLING_EDGE_ON_MC,
    #[doc = "If MODE0 is 1, counter 0 advances on a falling edge on MCI0."]
    FALLING,
}
impl TC0MCI0_FER {
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
            TC0MCI0_FER::A_FALLING_EDGE_ON_MC => false,
            TC0MCI0_FER::FALLING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TC0MCI0_FER {
        match value {
            false => TC0MCI0_FER::A_FALLING_EDGE_ON_MC,
            true => TC0MCI0_FER::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC0MCI0_FER::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == TC0MCI0_FER::FALLING
    }
}
#[doc = "Possible values of the field `TC0MCI1_RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC0MCI1_RER {
    #[doc = "A rising edge on MCI1 does not affect counter 0."]
    A_RISING_EDGE_ON_MCI,
    #[doc = "If MODE0 is 1, counter 0 advances on a rising edge on MCI1."]
    RISING,
}
impl TC0MCI1_RER {
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
            TC0MCI1_RER::A_RISING_EDGE_ON_MCI => false,
            TC0MCI1_RER::RISING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TC0MCI1_RER {
        match value {
            false => TC0MCI1_RER::A_RISING_EDGE_ON_MCI,
            true => TC0MCI1_RER::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC0MCI1_RER::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == TC0MCI1_RER::RISING
    }
}
#[doc = "Possible values of the field `TC0MCI1_FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC0MCI1_FER {
    #[doc = "A falling edge on MCI1 does not affect counter 0."]
    A_FALLING_EDGE_ON_MC,
    #[doc = "If MODE0 is 1, counter 0 advances on a falling edge on MCI1."]
    FALLING,
}
impl TC0MCI1_FER {
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
            TC0MCI1_FER::A_FALLING_EDGE_ON_MC => false,
            TC0MCI1_FER::FALLING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TC0MCI1_FER {
        match value {
            false => TC0MCI1_FER::A_FALLING_EDGE_ON_MC,
            true => TC0MCI1_FER::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC0MCI1_FER::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == TC0MCI1_FER::FALLING
    }
}
#[doc = "Possible values of the field `TC0MCI2_RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC0MCI2_RER {
    #[doc = "A rising edge on MCI0 does not affect counter 0."]
    A_RISING_EDGE_ON_MCI,
    #[doc = "If MODE0 is 1, counter 0 advances on a rising edge on MCI2."]
    RISING,
}
impl TC0MCI2_RER {
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
            TC0MCI2_RER::A_RISING_EDGE_ON_MCI => false,
            TC0MCI2_RER::RISING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TC0MCI2_RER {
        match value {
            false => TC0MCI2_RER::A_RISING_EDGE_ON_MCI,
            true => TC0MCI2_RER::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC0MCI2_RER::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == TC0MCI2_RER::RISING
    }
}
#[doc = "Possible values of the field `TC0MCI2_FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC0MCI2_FER {
    #[doc = "A falling edge on MCI0 does not affect counter 0."]
    A_FALLING_EDGE_ON_MC,
    #[doc = "If MODE0 is 1, counter 0 advances on a falling edge on MCI2."]
    FALLLING,
}
impl TC0MCI2_FER {
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
            TC0MCI2_FER::A_FALLING_EDGE_ON_MC => false,
            TC0MCI2_FER::FALLLING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TC0MCI2_FER {
        match value {
            false => TC0MCI2_FER::A_FALLING_EDGE_ON_MC,
            true => TC0MCI2_FER::FALLLING,
        }
    }
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC0MCI2_FER::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLLING`"]
    #[inline]
    pub fn is_fallling(&self) -> bool {
        *self == TC0MCI2_FER::FALLLING
    }
}
#[doc = "Possible values of the field `TC1MCI0_RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC1MCI0_RER {
    #[doc = "A rising edge on MCI0 does not affect counter 1."]
    A_RISING_EDGE_ON_MCI,
    #[doc = "If MODE1 is 1, counter 1 advances on a rising edge on MCI0."]
    RISING,
}
impl TC1MCI0_RER {
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
            TC1MCI0_RER::A_RISING_EDGE_ON_MCI => false,
            TC1MCI0_RER::RISING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TC1MCI0_RER {
        match value {
            false => TC1MCI0_RER::A_RISING_EDGE_ON_MCI,
            true => TC1MCI0_RER::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC1MCI0_RER::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == TC1MCI0_RER::RISING
    }
}
#[doc = "Possible values of the field `TC1MCI0_FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC1MCI0_FER {
    #[doc = "A falling edge on MCI0 does not affect counter 1."]
    A_FALLING_EDGE_ON_MC,
    #[doc = "If MODE1 is 1, counter 1 advances on a falling edge on MCI0."]
    FALLING,
}
impl TC1MCI0_FER {
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
            TC1MCI0_FER::A_FALLING_EDGE_ON_MC => false,
            TC1MCI0_FER::FALLING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TC1MCI0_FER {
        match value {
            false => TC1MCI0_FER::A_FALLING_EDGE_ON_MC,
            true => TC1MCI0_FER::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC1MCI0_FER::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == TC1MCI0_FER::FALLING
    }
}
#[doc = "Possible values of the field `TC1MCI1_RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC1MCI1_RER {
    #[doc = "A rising edge on MCI1 does not affect counter 1."]
    A_RISING_EDGE_ON_MCI,
    #[doc = "If MODE1 is 1, counter 1 advances on a rising edge on MCI1."]
    RISING,
}
impl TC1MCI1_RER {
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
            TC1MCI1_RER::A_RISING_EDGE_ON_MCI => false,
            TC1MCI1_RER::RISING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TC1MCI1_RER {
        match value {
            false => TC1MCI1_RER::A_RISING_EDGE_ON_MCI,
            true => TC1MCI1_RER::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC1MCI1_RER::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == TC1MCI1_RER::RISING
    }
}
#[doc = "Possible values of the field `TC1MCI1_FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC1MCI1_FER {
    #[doc = "A falling edge on MCI0 does not affect counter 1."]
    A_FALLING_EDGE_ON_MC,
    #[doc = "If MODE1 is 1, counter 1 advances on a falling edge on MCI1."]
    FALLING,
}
impl TC1MCI1_FER {
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
            TC1MCI1_FER::A_FALLING_EDGE_ON_MC => false,
            TC1MCI1_FER::FALLING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TC1MCI1_FER {
        match value {
            false => TC1MCI1_FER::A_FALLING_EDGE_ON_MC,
            true => TC1MCI1_FER::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC1MCI1_FER::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == TC1MCI1_FER::FALLING
    }
}
#[doc = "Possible values of the field `TC1MCI2_RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC1MCI2_RER {
    #[doc = "A rising edge on MCI2 does not affect counter 1."]
    A_RISING_EDGE_ON_MCI,
    #[doc = "If MODE1 is 1, counter 1 advances on a rising edge on MCI2."]
    RISING,
}
impl TC1MCI2_RER {
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
            TC1MCI2_RER::A_RISING_EDGE_ON_MCI => false,
            TC1MCI2_RER::RISING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TC1MCI2_RER {
        match value {
            false => TC1MCI2_RER::A_RISING_EDGE_ON_MCI,
            true => TC1MCI2_RER::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC1MCI2_RER::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == TC1MCI2_RER::RISING
    }
}
#[doc = "Possible values of the field `TC1MCI2_FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC1MCI2_FER {
    #[doc = "A falling edge on MCI2 does not affect counter 1."]
    A_FALLING_EDGE_ON_MC,
    #[doc = "If MODE1 is 1, counter 1 advances on a falling edge on MCI2."]
    FALLING,
}
impl TC1MCI2_FER {
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
            TC1MCI2_FER::A_FALLING_EDGE_ON_MC => false,
            TC1MCI2_FER::FALLING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TC1MCI2_FER {
        match value {
            false => TC1MCI2_FER::A_FALLING_EDGE_ON_MC,
            true => TC1MCI2_FER::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC1MCI2_FER::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == TC1MCI2_FER::FALLING
    }
}
#[doc = "Possible values of the field `TC2MCI0_RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC2MCI0_RER {
    #[doc = "A rising edge on MCI0 does not affect counter 2."]
    A_RISING_EDGE_ON_MCI,
    #[doc = "If MODE2 is 1, counter 2 advances on a rising edge on MCI0."]
    RISING,
}
impl TC2MCI0_RER {
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
            TC2MCI0_RER::A_RISING_EDGE_ON_MCI => false,
            TC2MCI0_RER::RISING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TC2MCI0_RER {
        match value {
            false => TC2MCI0_RER::A_RISING_EDGE_ON_MCI,
            true => TC2MCI0_RER::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC2MCI0_RER::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == TC2MCI0_RER::RISING
    }
}
#[doc = "Possible values of the field `TC2MCI0_FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC2MCI0_FER {
    #[doc = "A falling edge on MCI0 does not affect counter 2."]
    A_FALLING_EDGE_ON_MC,
    #[doc = "If MODE2 is 1, counter 2 advances on a falling edge on MCI0."]
    FALLING,
}
impl TC2MCI0_FER {
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
            TC2MCI0_FER::A_FALLING_EDGE_ON_MC => false,
            TC2MCI0_FER::FALLING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TC2MCI0_FER {
        match value {
            false => TC2MCI0_FER::A_FALLING_EDGE_ON_MC,
            true => TC2MCI0_FER::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC2MCI0_FER::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == TC2MCI0_FER::FALLING
    }
}
#[doc = "Possible values of the field `TC2MCI1_RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC2MCI1_RER {
    #[doc = "A rising edge on MCI1 does not affect counter 2."]
    A_RISING_EDGE_ON_MCI,
    #[doc = "If MODE2 is 1, counter 2 advances on a rising edge on MCI1."]
    RISING,
}
impl TC2MCI1_RER {
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
            TC2MCI1_RER::A_RISING_EDGE_ON_MCI => false,
            TC2MCI1_RER::RISING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TC2MCI1_RER {
        match value {
            false => TC2MCI1_RER::A_RISING_EDGE_ON_MCI,
            true => TC2MCI1_RER::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC2MCI1_RER::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == TC2MCI1_RER::RISING
    }
}
#[doc = "Possible values of the field `TC2MCI1_FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC2MCI1_FER {
    #[doc = "A falling edge on MCI1 does not affect counter 2."]
    A_FALLING_EDGE_ON_MC,
    #[doc = "If MODE2 is 1, counter 2 advances on a falling edge on MCI1."]
    FALLING,
}
impl TC2MCI1_FER {
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
            TC2MCI1_FER::A_FALLING_EDGE_ON_MC => false,
            TC2MCI1_FER::FALLING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TC2MCI1_FER {
        match value {
            false => TC2MCI1_FER::A_FALLING_EDGE_ON_MC,
            true => TC2MCI1_FER::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC2MCI1_FER::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == TC2MCI1_FER::FALLING
    }
}
#[doc = "Possible values of the field `TC2MCI2_RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC2MCI2_RER {
    #[doc = "A rising edge on MCI2 does not affect counter 2."]
    A_RISING_EDGE_ON_MCI,
    #[doc = "If MODE2 is 1, counter 2 advances on a rising edge on MCI2."]
    RISIING,
}
impl TC2MCI2_RER {
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
            TC2MCI2_RER::A_RISING_EDGE_ON_MCI => false,
            TC2MCI2_RER::RISIING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TC2MCI2_RER {
        match value {
            false => TC2MCI2_RER::A_RISING_EDGE_ON_MCI,
            true => TC2MCI2_RER::RISIING,
        }
    }
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC2MCI2_RER::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISIING`"]
    #[inline]
    pub fn is_risiing(&self) -> bool {
        *self == TC2MCI2_RER::RISIING
    }
}
#[doc = "Possible values of the field `TC2MCI2_FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC2MCI2_FER {
    #[doc = "A falling edge on MCI2 does not affect counter 2."]
    A_FALLING_EDGE_ON_MC,
    #[doc = "If MODE2 is 1, counter 2 advances on a falling edge on MCI2."]
    FALLING,
}
impl TC2MCI2_FER {
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
            TC2MCI2_FER::A_FALLING_EDGE_ON_MC => false,
            TC2MCI2_FER::FALLING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TC2MCI2_FER {
        match value {
            false => TC2MCI2_FER::A_FALLING_EDGE_ON_MC,
            true => TC2MCI2_FER::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC2MCI2_FER::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == TC2MCI2_FER::FALLING
    }
}
#[doc = "Possible values of the field `CNTR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTR0R {
    #[doc = "Channel 0 is in timer mode."]
    CHANNEL_0_IS_IN_TIME,
    #[doc = "Channel 0 is in counter mode."]
    CHANNEL_0_IS_IN_COUN,
}
impl CNTR0R {
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
            CNTR0R::CHANNEL_0_IS_IN_TIME => false,
            CNTR0R::CHANNEL_0_IS_IN_COUN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNTR0R {
        match value {
            false => CNTR0R::CHANNEL_0_IS_IN_TIME,
            true => CNTR0R::CHANNEL_0_IS_IN_COUN,
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL_0_IS_IN_TIME`"]
    #[inline]
    pub fn is_channel_0_is_in_time(&self) -> bool {
        *self == CNTR0R::CHANNEL_0_IS_IN_TIME
    }
    #[doc = "Checks if the value of the field is `CHANNEL_0_IS_IN_COUN`"]
    #[inline]
    pub fn is_channel_0_is_in_coun(&self) -> bool {
        *self == CNTR0R::CHANNEL_0_IS_IN_COUN
    }
}
#[doc = "Possible values of the field `CNTR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTR1R {
    #[doc = "Channel 1 is in timer mode."]
    CHANNEL_1_IS_IN_TIME,
    #[doc = "Channel 1 is in counter mode."]
    CHANNEL_1_IS_IN_COUN,
}
impl CNTR1R {
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
            CNTR1R::CHANNEL_1_IS_IN_TIME => false,
            CNTR1R::CHANNEL_1_IS_IN_COUN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNTR1R {
        match value {
            false => CNTR1R::CHANNEL_1_IS_IN_TIME,
            true => CNTR1R::CHANNEL_1_IS_IN_COUN,
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL_1_IS_IN_TIME`"]
    #[inline]
    pub fn is_channel_1_is_in_time(&self) -> bool {
        *self == CNTR1R::CHANNEL_1_IS_IN_TIME
    }
    #[doc = "Checks if the value of the field is `CHANNEL_1_IS_IN_COUN`"]
    #[inline]
    pub fn is_channel_1_is_in_coun(&self) -> bool {
        *self == CNTR1R::CHANNEL_1_IS_IN_COUN
    }
}
#[doc = "Possible values of the field `CNTR2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTR2R {
    #[doc = "Channel 2 is in timer mode."]
    CHANNEL_2_IS_IN_TIME,
    #[doc = "Channel 2 is in counter mode."]
    CHANNEL_2_IS_IN_COUN,
}
impl CNTR2R {
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
            CNTR2R::CHANNEL_2_IS_IN_TIME => false,
            CNTR2R::CHANNEL_2_IS_IN_COUN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNTR2R {
        match value {
            false => CNTR2R::CHANNEL_2_IS_IN_TIME,
            true => CNTR2R::CHANNEL_2_IS_IN_COUN,
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL_2_IS_IN_TIME`"]
    #[inline]
    pub fn is_channel_2_is_in_time(&self) -> bool {
        *self == CNTR2R::CHANNEL_2_IS_IN_TIME
    }
    #[doc = "Checks if the value of the field is `CHANNEL_2_IS_IN_COUN`"]
    #[inline]
    pub fn is_channel_2_is_in_coun(&self) -> bool {
        *self == CNTR2R::CHANNEL_2_IS_IN_COUN
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Counter 0 rising edge mode, channel 0."]
    #[inline]
    pub fn tc0mci0_re(&self) -> TC0MCI0_RER {
        TC0MCI0_RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Counter 0 falling edge mode, channel 0."]
    #[inline]
    pub fn tc0mci0_fe(&self) -> TC0MCI0_FER {
        TC0MCI0_FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Counter 0 rising edge mode, channel 1."]
    #[inline]
    pub fn tc0mci1_re(&self) -> TC0MCI1_RER {
        TC0MCI1_RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Counter 0 falling edge mode, channel 1."]
    #[inline]
    pub fn tc0mci1_fe(&self) -> TC0MCI1_FER {
        TC0MCI1_FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Counter 0 rising edge mode, channel 2."]
    #[inline]
    pub fn tc0mci2_re(&self) -> TC0MCI2_RER {
        TC0MCI2_RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Counter 0 falling edge mode, channel 2."]
    #[inline]
    pub fn tc0mci2_fe(&self) -> TC0MCI2_FER {
        TC0MCI2_FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Counter 1 rising edge mode, channel 0."]
    #[inline]
    pub fn tc1mci0_re(&self) -> TC1MCI0_RER {
        TC1MCI0_RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Counter 1 falling edge mode, channel 0."]
    #[inline]
    pub fn tc1mci0_fe(&self) -> TC1MCI0_FER {
        TC1MCI0_FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Counter 1 rising edge mode, channel 1."]
    #[inline]
    pub fn tc1mci1_re(&self) -> TC1MCI1_RER {
        TC1MCI1_RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Counter 1 falling edge mode, channel 1."]
    #[inline]
    pub fn tc1mci1_fe(&self) -> TC1MCI1_FER {
        TC1MCI1_FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Counter 1 rising edge mode, channel 2."]
    #[inline]
    pub fn tc1mci2_re(&self) -> TC1MCI2_RER {
        TC1MCI2_RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Counter 1 falling edge mode, channel 2."]
    #[inline]
    pub fn tc1mci2_fe(&self) -> TC1MCI2_FER {
        TC1MCI2_FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Counter 2 rising edge mode, channel 0."]
    #[inline]
    pub fn tc2mci0_re(&self) -> TC2MCI0_RER {
        TC2MCI0_RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Counter 2 falling edge mode, channel 0."]
    #[inline]
    pub fn tc2mci0_fe(&self) -> TC2MCI0_FER {
        TC2MCI0_FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Counter 2 rising edge mode, channel 1."]
    #[inline]
    pub fn tc2mci1_re(&self) -> TC2MCI1_RER {
        TC2MCI1_RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Counter 2 falling edge mode, channel 1."]
    #[inline]
    pub fn tc2mci1_fe(&self) -> TC2MCI1_FER {
        TC2MCI1_FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Counter 2 rising edge mode, channel 2."]
    #[inline]
    pub fn tc2mci2_re(&self) -> TC2MCI2_RER {
        TC2MCI2_RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Counter 2 falling edge mode, channel 2."]
    #[inline]
    pub fn tc2mci2_fe(&self) -> TC2MCI2_FER {
        TC2MCI2_FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Channel 0 counter/timer mode."]
    #[inline]
    pub fn cntr0(&self) -> CNTR0R {
        CNTR0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Channel 1 counter/timer mode."]
    #[inline]
    pub fn cntr1(&self) -> CNTR1R {
        CNTR1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Channel 2 counter/timer mode."]
    #[inline]
    pub fn cntr2(&self) -> CNTR2R {
        CNTR2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
