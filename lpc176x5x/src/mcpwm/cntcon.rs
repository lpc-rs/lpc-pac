#[doc = "Reader of register CNTCON"]
pub type R = crate::R<u32, super::CNTCON>;
#[doc = "Counter 0 rising edge mode, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC0MCI0_RE_A {
    #[doc = "0: A rising edge on MCI0 does not affect counter 0."]
    A_RISING_EDGE_ON_MCI = 0,
    #[doc = "1: If MODE0 is 1, counter 0 advances on a rising edge on MCI0."]
    RISING = 1,
}
impl From<TC0MCI0_RE_A> for bool {
    #[inline(always)]
    fn from(variant: TC0MCI0_RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TC0MCI0_RE`"]
pub type TC0MCI0_RE_R = crate::R<bool, TC0MCI0_RE_A>;
impl TC0MCI0_RE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC0MCI0_RE_A {
        match self.bits {
            false => TC0MCI0_RE_A::A_RISING_EDGE_ON_MCI,
            true => TC0MCI0_RE_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC0MCI0_RE_A::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == TC0MCI0_RE_A::RISING
    }
}
#[doc = "Counter 0 falling edge mode, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC0MCI0_FE_A {
    #[doc = "0: A falling edge on MCI0 does not affect counter 0."]
    A_FALLING_EDGE_ON_MC = 0,
    #[doc = "1: If MODE0 is 1, counter 0 advances on a falling edge on MCI0."]
    FALLING = 1,
}
impl From<TC0MCI0_FE_A> for bool {
    #[inline(always)]
    fn from(variant: TC0MCI0_FE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TC0MCI0_FE`"]
pub type TC0MCI0_FE_R = crate::R<bool, TC0MCI0_FE_A>;
impl TC0MCI0_FE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC0MCI0_FE_A {
        match self.bits {
            false => TC0MCI0_FE_A::A_FALLING_EDGE_ON_MC,
            true => TC0MCI0_FE_A::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC0MCI0_FE_A::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == TC0MCI0_FE_A::FALLING
    }
}
#[doc = "Counter 0 rising edge mode, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC0MCI1_RE_A {
    #[doc = "0: A rising edge on MCI1 does not affect counter 0."]
    A_RISING_EDGE_ON_MCI = 0,
    #[doc = "1: If MODE0 is 1, counter 0 advances on a rising edge on MCI1."]
    RISING = 1,
}
impl From<TC0MCI1_RE_A> for bool {
    #[inline(always)]
    fn from(variant: TC0MCI1_RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TC0MCI1_RE`"]
pub type TC0MCI1_RE_R = crate::R<bool, TC0MCI1_RE_A>;
impl TC0MCI1_RE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC0MCI1_RE_A {
        match self.bits {
            false => TC0MCI1_RE_A::A_RISING_EDGE_ON_MCI,
            true => TC0MCI1_RE_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC0MCI1_RE_A::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == TC0MCI1_RE_A::RISING
    }
}
#[doc = "Counter 0 falling edge mode, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC0MCI1_FE_A {
    #[doc = "0: A falling edge on MCI1 does not affect counter 0."]
    A_FALLING_EDGE_ON_MC = 0,
    #[doc = "1: If MODE0 is 1, counter 0 advances on a falling edge on MCI1."]
    FALLING = 1,
}
impl From<TC0MCI1_FE_A> for bool {
    #[inline(always)]
    fn from(variant: TC0MCI1_FE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TC0MCI1_FE`"]
pub type TC0MCI1_FE_R = crate::R<bool, TC0MCI1_FE_A>;
impl TC0MCI1_FE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC0MCI1_FE_A {
        match self.bits {
            false => TC0MCI1_FE_A::A_FALLING_EDGE_ON_MC,
            true => TC0MCI1_FE_A::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC0MCI1_FE_A::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == TC0MCI1_FE_A::FALLING
    }
}
#[doc = "Counter 0 rising edge mode, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC0MCI2_RE_A {
    #[doc = "0: A rising edge on MCI0 does not affect counter 0."]
    A_RISING_EDGE_ON_MCI = 0,
    #[doc = "1: If MODE0 is 1, counter 0 advances on a rising edge on MCI2."]
    RISING = 1,
}
impl From<TC0MCI2_RE_A> for bool {
    #[inline(always)]
    fn from(variant: TC0MCI2_RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TC0MCI2_RE`"]
pub type TC0MCI2_RE_R = crate::R<bool, TC0MCI2_RE_A>;
impl TC0MCI2_RE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC0MCI2_RE_A {
        match self.bits {
            false => TC0MCI2_RE_A::A_RISING_EDGE_ON_MCI,
            true => TC0MCI2_RE_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC0MCI2_RE_A::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == TC0MCI2_RE_A::RISING
    }
}
#[doc = "Counter 0 falling edge mode, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC0MCI2_FE_A {
    #[doc = "0: A falling edge on MCI0 does not affect counter 0."]
    A_FALLING_EDGE_ON_MC = 0,
    #[doc = "1: If MODE0 is 1, counter 0 advances on a falling edge on MCI2."]
    FALLLING = 1,
}
impl From<TC0MCI2_FE_A> for bool {
    #[inline(always)]
    fn from(variant: TC0MCI2_FE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TC0MCI2_FE`"]
pub type TC0MCI2_FE_R = crate::R<bool, TC0MCI2_FE_A>;
impl TC0MCI2_FE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC0MCI2_FE_A {
        match self.bits {
            false => TC0MCI2_FE_A::A_FALLING_EDGE_ON_MC,
            true => TC0MCI2_FE_A::FALLLING,
        }
    }
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC0MCI2_FE_A::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLLING`"]
    #[inline(always)]
    pub fn is_fallling(&self) -> bool {
        *self == TC0MCI2_FE_A::FALLLING
    }
}
#[doc = "Counter 1 rising edge mode, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC1MCI0_RE_A {
    #[doc = "0: A rising edge on MCI0 does not affect counter 1."]
    A_RISING_EDGE_ON_MCI = 0,
    #[doc = "1: If MODE1 is 1, counter 1 advances on a rising edge on MCI0."]
    RISING = 1,
}
impl From<TC1MCI0_RE_A> for bool {
    #[inline(always)]
    fn from(variant: TC1MCI0_RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TC1MCI0_RE`"]
pub type TC1MCI0_RE_R = crate::R<bool, TC1MCI0_RE_A>;
impl TC1MCI0_RE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC1MCI0_RE_A {
        match self.bits {
            false => TC1MCI0_RE_A::A_RISING_EDGE_ON_MCI,
            true => TC1MCI0_RE_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC1MCI0_RE_A::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == TC1MCI0_RE_A::RISING
    }
}
#[doc = "Counter 1 falling edge mode, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC1MCI0_FE_A {
    #[doc = "0: A falling edge on MCI0 does not affect counter 1."]
    A_FALLING_EDGE_ON_MC = 0,
    #[doc = "1: If MODE1 is 1, counter 1 advances on a falling edge on MCI0."]
    FALLING = 1,
}
impl From<TC1MCI0_FE_A> for bool {
    #[inline(always)]
    fn from(variant: TC1MCI0_FE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TC1MCI0_FE`"]
pub type TC1MCI0_FE_R = crate::R<bool, TC1MCI0_FE_A>;
impl TC1MCI0_FE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC1MCI0_FE_A {
        match self.bits {
            false => TC1MCI0_FE_A::A_FALLING_EDGE_ON_MC,
            true => TC1MCI0_FE_A::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC1MCI0_FE_A::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == TC1MCI0_FE_A::FALLING
    }
}
#[doc = "Counter 1 rising edge mode, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC1MCI1_RE_A {
    #[doc = "0: A rising edge on MCI1 does not affect counter 1."]
    A_RISING_EDGE_ON_MCI = 0,
    #[doc = "1: If MODE1 is 1, counter 1 advances on a rising edge on MCI1."]
    RISING = 1,
}
impl From<TC1MCI1_RE_A> for bool {
    #[inline(always)]
    fn from(variant: TC1MCI1_RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TC1MCI1_RE`"]
pub type TC1MCI1_RE_R = crate::R<bool, TC1MCI1_RE_A>;
impl TC1MCI1_RE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC1MCI1_RE_A {
        match self.bits {
            false => TC1MCI1_RE_A::A_RISING_EDGE_ON_MCI,
            true => TC1MCI1_RE_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC1MCI1_RE_A::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == TC1MCI1_RE_A::RISING
    }
}
#[doc = "Counter 1 falling edge mode, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC1MCI1_FE_A {
    #[doc = "0: A falling edge on MCI0 does not affect counter 1."]
    A_FALLING_EDGE_ON_MC = 0,
    #[doc = "1: If MODE1 is 1, counter 1 advances on a falling edge on MCI1."]
    FALLING = 1,
}
impl From<TC1MCI1_FE_A> for bool {
    #[inline(always)]
    fn from(variant: TC1MCI1_FE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TC1MCI1_FE`"]
pub type TC1MCI1_FE_R = crate::R<bool, TC1MCI1_FE_A>;
impl TC1MCI1_FE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC1MCI1_FE_A {
        match self.bits {
            false => TC1MCI1_FE_A::A_FALLING_EDGE_ON_MC,
            true => TC1MCI1_FE_A::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC1MCI1_FE_A::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == TC1MCI1_FE_A::FALLING
    }
}
#[doc = "Counter 1 rising edge mode, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC1MCI2_RE_A {
    #[doc = "0: A rising edge on MCI2 does not affect counter 1."]
    A_RISING_EDGE_ON_MCI = 0,
    #[doc = "1: If MODE1 is 1, counter 1 advances on a rising edge on MCI2."]
    RISING = 1,
}
impl From<TC1MCI2_RE_A> for bool {
    #[inline(always)]
    fn from(variant: TC1MCI2_RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TC1MCI2_RE`"]
pub type TC1MCI2_RE_R = crate::R<bool, TC1MCI2_RE_A>;
impl TC1MCI2_RE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC1MCI2_RE_A {
        match self.bits {
            false => TC1MCI2_RE_A::A_RISING_EDGE_ON_MCI,
            true => TC1MCI2_RE_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC1MCI2_RE_A::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == TC1MCI2_RE_A::RISING
    }
}
#[doc = "Counter 1 falling edge mode, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC1MCI2_FE_A {
    #[doc = "0: A falling edge on MCI2 does not affect counter 1."]
    A_FALLING_EDGE_ON_MC = 0,
    #[doc = "1: If MODE1 is 1, counter 1 advances on a falling edge on MCI2."]
    FALLING = 1,
}
impl From<TC1MCI2_FE_A> for bool {
    #[inline(always)]
    fn from(variant: TC1MCI2_FE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TC1MCI2_FE`"]
pub type TC1MCI2_FE_R = crate::R<bool, TC1MCI2_FE_A>;
impl TC1MCI2_FE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC1MCI2_FE_A {
        match self.bits {
            false => TC1MCI2_FE_A::A_FALLING_EDGE_ON_MC,
            true => TC1MCI2_FE_A::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC1MCI2_FE_A::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == TC1MCI2_FE_A::FALLING
    }
}
#[doc = "Counter 2 rising edge mode, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC2MCI0_RE_A {
    #[doc = "0: A rising edge on MCI0 does not affect counter 2."]
    A_RISING_EDGE_ON_MCI = 0,
    #[doc = "1: If MODE2 is 1, counter 2 advances on a rising edge on MCI0."]
    RISING = 1,
}
impl From<TC2MCI0_RE_A> for bool {
    #[inline(always)]
    fn from(variant: TC2MCI0_RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TC2MCI0_RE`"]
pub type TC2MCI0_RE_R = crate::R<bool, TC2MCI0_RE_A>;
impl TC2MCI0_RE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC2MCI0_RE_A {
        match self.bits {
            false => TC2MCI0_RE_A::A_RISING_EDGE_ON_MCI,
            true => TC2MCI0_RE_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC2MCI0_RE_A::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == TC2MCI0_RE_A::RISING
    }
}
#[doc = "Counter 2 falling edge mode, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC2MCI0_FE_A {
    #[doc = "0: A falling edge on MCI0 does not affect counter 2."]
    A_FALLING_EDGE_ON_MC = 0,
    #[doc = "1: If MODE2 is 1, counter 2 advances on a falling edge on MCI0."]
    FALLING = 1,
}
impl From<TC2MCI0_FE_A> for bool {
    #[inline(always)]
    fn from(variant: TC2MCI0_FE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TC2MCI0_FE`"]
pub type TC2MCI0_FE_R = crate::R<bool, TC2MCI0_FE_A>;
impl TC2MCI0_FE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC2MCI0_FE_A {
        match self.bits {
            false => TC2MCI0_FE_A::A_FALLING_EDGE_ON_MC,
            true => TC2MCI0_FE_A::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC2MCI0_FE_A::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == TC2MCI0_FE_A::FALLING
    }
}
#[doc = "Counter 2 rising edge mode, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC2MCI1_RE_A {
    #[doc = "0: A rising edge on MCI1 does not affect counter 2."]
    A_RISING_EDGE_ON_MCI = 0,
    #[doc = "1: If MODE2 is 1, counter 2 advances on a rising edge on MCI1."]
    RISING = 1,
}
impl From<TC2MCI1_RE_A> for bool {
    #[inline(always)]
    fn from(variant: TC2MCI1_RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TC2MCI1_RE`"]
pub type TC2MCI1_RE_R = crate::R<bool, TC2MCI1_RE_A>;
impl TC2MCI1_RE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC2MCI1_RE_A {
        match self.bits {
            false => TC2MCI1_RE_A::A_RISING_EDGE_ON_MCI,
            true => TC2MCI1_RE_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC2MCI1_RE_A::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == TC2MCI1_RE_A::RISING
    }
}
#[doc = "Counter 2 falling edge mode, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC2MCI1_FE_A {
    #[doc = "0: A falling edge on MCI1 does not affect counter 2."]
    A_FALLING_EDGE_ON_MC = 0,
    #[doc = "1: If MODE2 is 1, counter 2 advances on a falling edge on MCI1."]
    FALLING = 1,
}
impl From<TC2MCI1_FE_A> for bool {
    #[inline(always)]
    fn from(variant: TC2MCI1_FE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TC2MCI1_FE`"]
pub type TC2MCI1_FE_R = crate::R<bool, TC2MCI1_FE_A>;
impl TC2MCI1_FE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC2MCI1_FE_A {
        match self.bits {
            false => TC2MCI1_FE_A::A_FALLING_EDGE_ON_MC,
            true => TC2MCI1_FE_A::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC2MCI1_FE_A::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == TC2MCI1_FE_A::FALLING
    }
}
#[doc = "Counter 2 rising edge mode, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC2MCI2_RE_A {
    #[doc = "0: A rising edge on MCI2 does not affect counter 2."]
    A_RISING_EDGE_ON_MCI = 0,
    #[doc = "1: If MODE2 is 1, counter 2 advances on a rising edge on MCI2."]
    RISIING = 1,
}
impl From<TC2MCI2_RE_A> for bool {
    #[inline(always)]
    fn from(variant: TC2MCI2_RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TC2MCI2_RE`"]
pub type TC2MCI2_RE_R = crate::R<bool, TC2MCI2_RE_A>;
impl TC2MCI2_RE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC2MCI2_RE_A {
        match self.bits {
            false => TC2MCI2_RE_A::A_RISING_EDGE_ON_MCI,
            true => TC2MCI2_RE_A::RISIING,
        }
    }
    #[doc = "Checks if the value of the field is `A_RISING_EDGE_ON_MCI`"]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == TC2MCI2_RE_A::A_RISING_EDGE_ON_MCI
    }
    #[doc = "Checks if the value of the field is `RISIING`"]
    #[inline(always)]
    pub fn is_risiing(&self) -> bool {
        *self == TC2MCI2_RE_A::RISIING
    }
}
#[doc = "Counter 2 falling edge mode, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC2MCI2_FE_A {
    #[doc = "0: A falling edge on MCI2 does not affect counter 2."]
    A_FALLING_EDGE_ON_MC = 0,
    #[doc = "1: If MODE2 is 1, counter 2 advances on a falling edge on MCI2."]
    FALLING = 1,
}
impl From<TC2MCI2_FE_A> for bool {
    #[inline(always)]
    fn from(variant: TC2MCI2_FE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TC2MCI2_FE`"]
pub type TC2MCI2_FE_R = crate::R<bool, TC2MCI2_FE_A>;
impl TC2MCI2_FE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC2MCI2_FE_A {
        match self.bits {
            false => TC2MCI2_FE_A::A_FALLING_EDGE_ON_MC,
            true => TC2MCI2_FE_A::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `A_FALLING_EDGE_ON_MC`"]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == TC2MCI2_FE_A::A_FALLING_EDGE_ON_MC
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == TC2MCI2_FE_A::FALLING
    }
}
#[doc = "Channel 0 counter/timer mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTR0_A {
    #[doc = "0: Channel 0 is in timer mode."]
    CHANNEL_0_IS_IN_TIME = 0,
    #[doc = "1: Channel 0 is in counter mode."]
    CHANNEL_0_IS_IN_COUN = 1,
}
impl From<CNTR0_A> for bool {
    #[inline(always)]
    fn from(variant: CNTR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CNTR0`"]
pub type CNTR0_R = crate::R<bool, CNTR0_A>;
impl CNTR0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTR0_A {
        match self.bits {
            false => CNTR0_A::CHANNEL_0_IS_IN_TIME,
            true => CNTR0_A::CHANNEL_0_IS_IN_COUN,
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL_0_IS_IN_TIME`"]
    #[inline(always)]
    pub fn is_channel_0_is_in_time(&self) -> bool {
        *self == CNTR0_A::CHANNEL_0_IS_IN_TIME
    }
    #[doc = "Checks if the value of the field is `CHANNEL_0_IS_IN_COUN`"]
    #[inline(always)]
    pub fn is_channel_0_is_in_coun(&self) -> bool {
        *self == CNTR0_A::CHANNEL_0_IS_IN_COUN
    }
}
#[doc = "Channel 1 counter/timer mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTR1_A {
    #[doc = "0: Channel 1 is in timer mode."]
    CHANNEL_1_IS_IN_TIME = 0,
    #[doc = "1: Channel 1 is in counter mode."]
    CHANNEL_1_IS_IN_COUN = 1,
}
impl From<CNTR1_A> for bool {
    #[inline(always)]
    fn from(variant: CNTR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CNTR1`"]
pub type CNTR1_R = crate::R<bool, CNTR1_A>;
impl CNTR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTR1_A {
        match self.bits {
            false => CNTR1_A::CHANNEL_1_IS_IN_TIME,
            true => CNTR1_A::CHANNEL_1_IS_IN_COUN,
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL_1_IS_IN_TIME`"]
    #[inline(always)]
    pub fn is_channel_1_is_in_time(&self) -> bool {
        *self == CNTR1_A::CHANNEL_1_IS_IN_TIME
    }
    #[doc = "Checks if the value of the field is `CHANNEL_1_IS_IN_COUN`"]
    #[inline(always)]
    pub fn is_channel_1_is_in_coun(&self) -> bool {
        *self == CNTR1_A::CHANNEL_1_IS_IN_COUN
    }
}
#[doc = "Channel 2 counter/timer mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTR2_A {
    #[doc = "0: Channel 2 is in timer mode."]
    CHANNEL_2_IS_IN_TIME = 0,
    #[doc = "1: Channel 2 is in counter mode."]
    CHANNEL_2_IS_IN_COUN = 1,
}
impl From<CNTR2_A> for bool {
    #[inline(always)]
    fn from(variant: CNTR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CNTR2`"]
pub type CNTR2_R = crate::R<bool, CNTR2_A>;
impl CNTR2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTR2_A {
        match self.bits {
            false => CNTR2_A::CHANNEL_2_IS_IN_TIME,
            true => CNTR2_A::CHANNEL_2_IS_IN_COUN,
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL_2_IS_IN_TIME`"]
    #[inline(always)]
    pub fn is_channel_2_is_in_time(&self) -> bool {
        *self == CNTR2_A::CHANNEL_2_IS_IN_TIME
    }
    #[doc = "Checks if the value of the field is `CHANNEL_2_IS_IN_COUN`"]
    #[inline(always)]
    pub fn is_channel_2_is_in_coun(&self) -> bool {
        *self == CNTR2_A::CHANNEL_2_IS_IN_COUN
    }
}
impl R {
    #[doc = "Bit 0 - Counter 0 rising edge mode, channel 0."]
    #[inline(always)]
    pub fn tc0mci0_re(&self) -> TC0MCI0_RE_R {
        TC0MCI0_RE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Counter 0 falling edge mode, channel 0."]
    #[inline(always)]
    pub fn tc0mci0_fe(&self) -> TC0MCI0_FE_R {
        TC0MCI0_FE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Counter 0 rising edge mode, channel 1."]
    #[inline(always)]
    pub fn tc0mci1_re(&self) -> TC0MCI1_RE_R {
        TC0MCI1_RE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Counter 0 falling edge mode, channel 1."]
    #[inline(always)]
    pub fn tc0mci1_fe(&self) -> TC0MCI1_FE_R {
        TC0MCI1_FE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Counter 0 rising edge mode, channel 2."]
    #[inline(always)]
    pub fn tc0mci2_re(&self) -> TC0MCI2_RE_R {
        TC0MCI2_RE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Counter 0 falling edge mode, channel 2."]
    #[inline(always)]
    pub fn tc0mci2_fe(&self) -> TC0MCI2_FE_R {
        TC0MCI2_FE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Counter 1 rising edge mode, channel 0."]
    #[inline(always)]
    pub fn tc1mci0_re(&self) -> TC1MCI0_RE_R {
        TC1MCI0_RE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Counter 1 falling edge mode, channel 0."]
    #[inline(always)]
    pub fn tc1mci0_fe(&self) -> TC1MCI0_FE_R {
        TC1MCI0_FE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Counter 1 rising edge mode, channel 1."]
    #[inline(always)]
    pub fn tc1mci1_re(&self) -> TC1MCI1_RE_R {
        TC1MCI1_RE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Counter 1 falling edge mode, channel 1."]
    #[inline(always)]
    pub fn tc1mci1_fe(&self) -> TC1MCI1_FE_R {
        TC1MCI1_FE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Counter 1 rising edge mode, channel 2."]
    #[inline(always)]
    pub fn tc1mci2_re(&self) -> TC1MCI2_RE_R {
        TC1MCI2_RE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Counter 1 falling edge mode, channel 2."]
    #[inline(always)]
    pub fn tc1mci2_fe(&self) -> TC1MCI2_FE_R {
        TC1MCI2_FE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Counter 2 rising edge mode, channel 0."]
    #[inline(always)]
    pub fn tc2mci0_re(&self) -> TC2MCI0_RE_R {
        TC2MCI0_RE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Counter 2 falling edge mode, channel 0."]
    #[inline(always)]
    pub fn tc2mci0_fe(&self) -> TC2MCI0_FE_R {
        TC2MCI0_FE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Counter 2 rising edge mode, channel 1."]
    #[inline(always)]
    pub fn tc2mci1_re(&self) -> TC2MCI1_RE_R {
        TC2MCI1_RE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Counter 2 falling edge mode, channel 1."]
    #[inline(always)]
    pub fn tc2mci1_fe(&self) -> TC2MCI1_FE_R {
        TC2MCI1_FE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Counter 2 rising edge mode, channel 2."]
    #[inline(always)]
    pub fn tc2mci2_re(&self) -> TC2MCI2_RE_R {
        TC2MCI2_RE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Counter 2 falling edge mode, channel 2."]
    #[inline(always)]
    pub fn tc2mci2_fe(&self) -> TC2MCI2_FE_R {
        TC2MCI2_FE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Channel 0 counter/timer mode."]
    #[inline(always)]
    pub fn cntr0(&self) -> CNTR0_R {
        CNTR0_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Channel 1 counter/timer mode."]
    #[inline(always)]
    pub fn cntr1(&self) -> CNTR1_R {
        CNTR1_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Channel 2 counter/timer mode."]
    #[inline(always)]
    pub fn cntr2(&self) -> CNTR2_R {
        CNTR2_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
