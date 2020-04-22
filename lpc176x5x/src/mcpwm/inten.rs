#[doc = "Reader of register INTEN"]
pub type R = crate::R<u32, super::INTEN>;
#[doc = "Limit interrupt for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILIM0_A {
    #[doc = "0: Interrupt disabled."]
    INTERRUPT_DISABLED_ = 0,
    #[doc = "1: Interrupt enabled."]
    INTERRUPT_ENABLED_ = 1,
}
impl From<ILIM0_A> for bool {
    #[inline(always)]
    fn from(variant: ILIM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ILIM0`"]
pub type ILIM0_R = crate::R<bool, ILIM0_A>;
impl ILIM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILIM0_A {
        match self.bits {
            false => ILIM0_A::INTERRUPT_DISABLED_,
            true => ILIM0_A::INTERRUPT_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == ILIM0_A::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == ILIM0_A::INTERRUPT_ENABLED_
    }
}
#[doc = "Match interrupt for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMAT0_A {
    #[doc = "0: Interrupt disabled."]
    INTERRUPT_DISABLED_ = 0,
    #[doc = "1: Interrupt enabled."]
    INTERRUPT_ENABLED_ = 1,
}
impl From<IMAT0_A> for bool {
    #[inline(always)]
    fn from(variant: IMAT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IMAT0`"]
pub type IMAT0_R = crate::R<bool, IMAT0_A>;
impl IMAT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMAT0_A {
        match self.bits {
            false => IMAT0_A::INTERRUPT_DISABLED_,
            true => IMAT0_A::INTERRUPT_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == IMAT0_A::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == IMAT0_A::INTERRUPT_ENABLED_
    }
}
#[doc = "Capture interrupt for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICAP0_A {
    #[doc = "0: Interrupt disabled."]
    INTERRUPT_DISABLED_ = 0,
    #[doc = "1: Interrupt enabled."]
    INTERRUPT_ENABLED_ = 1,
}
impl From<ICAP0_A> for bool {
    #[inline(always)]
    fn from(variant: ICAP0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ICAP0`"]
pub type ICAP0_R = crate::R<bool, ICAP0_A>;
impl ICAP0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICAP0_A {
        match self.bits {
            false => ICAP0_A::INTERRUPT_DISABLED_,
            true => ICAP0_A::INTERRUPT_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == ICAP0_A::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == ICAP0_A::INTERRUPT_ENABLED_
    }
}
#[doc = "Limit interrupt for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILIM1_A {
    #[doc = "0: Interrupt disabled."]
    INTERRUPT_DISABLED_ = 0,
    #[doc = "1: Interrupt enabled."]
    INTERRUPT_ENABLED_ = 1,
}
impl From<ILIM1_A> for bool {
    #[inline(always)]
    fn from(variant: ILIM1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ILIM1`"]
pub type ILIM1_R = crate::R<bool, ILIM1_A>;
impl ILIM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILIM1_A {
        match self.bits {
            false => ILIM1_A::INTERRUPT_DISABLED_,
            true => ILIM1_A::INTERRUPT_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == ILIM1_A::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == ILIM1_A::INTERRUPT_ENABLED_
    }
}
#[doc = "Match interrupt for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMAT1_A {
    #[doc = "0: Interrupt disabled."]
    INTERRUPT_DISABLED_ = 0,
    #[doc = "1: Interrupt enabled."]
    INTERRUPT_ENABLED_ = 1,
}
impl From<IMAT1_A> for bool {
    #[inline(always)]
    fn from(variant: IMAT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IMAT1`"]
pub type IMAT1_R = crate::R<bool, IMAT1_A>;
impl IMAT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMAT1_A {
        match self.bits {
            false => IMAT1_A::INTERRUPT_DISABLED_,
            true => IMAT1_A::INTERRUPT_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == IMAT1_A::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == IMAT1_A::INTERRUPT_ENABLED_
    }
}
#[doc = "Capture interrupt for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICAP1_A {
    #[doc = "0: Interrupt disabled."]
    INTERRUPT_DISABLED_ = 0,
    #[doc = "1: Interrupt enabled."]
    INTERRUPT_ENABLED_ = 1,
}
impl From<ICAP1_A> for bool {
    #[inline(always)]
    fn from(variant: ICAP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ICAP1`"]
pub type ICAP1_R = crate::R<bool, ICAP1_A>;
impl ICAP1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICAP1_A {
        match self.bits {
            false => ICAP1_A::INTERRUPT_DISABLED_,
            true => ICAP1_A::INTERRUPT_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == ICAP1_A::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == ICAP1_A::INTERRUPT_ENABLED_
    }
}
#[doc = "Limit interrupt for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILIM2_A {
    #[doc = "0: Interrupt disabled."]
    INTERRUPT_DISABLED_ = 0,
    #[doc = "1: Interrupt enabled."]
    INTERRUPT_ENABLED_ = 1,
}
impl From<ILIM2_A> for bool {
    #[inline(always)]
    fn from(variant: ILIM2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ILIM2`"]
pub type ILIM2_R = crate::R<bool, ILIM2_A>;
impl ILIM2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILIM2_A {
        match self.bits {
            false => ILIM2_A::INTERRUPT_DISABLED_,
            true => ILIM2_A::INTERRUPT_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == ILIM2_A::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == ILIM2_A::INTERRUPT_ENABLED_
    }
}
#[doc = "Match interrupt for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMAT2_A {
    #[doc = "0: Interrupt disabled."]
    INTERRUPT_DISABLED_ = 0,
    #[doc = "1: Interrupt enabled."]
    INTERRUPT_ENABLED_ = 1,
}
impl From<IMAT2_A> for bool {
    #[inline(always)]
    fn from(variant: IMAT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IMAT2`"]
pub type IMAT2_R = crate::R<bool, IMAT2_A>;
impl IMAT2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMAT2_A {
        match self.bits {
            false => IMAT2_A::INTERRUPT_DISABLED_,
            true => IMAT2_A::INTERRUPT_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == IMAT2_A::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == IMAT2_A::INTERRUPT_ENABLED_
    }
}
#[doc = "Capture interrupt for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICAP2_A {
    #[doc = "0: Interrupt disabled."]
    INTERRUPT_DISABLED_ = 0,
    #[doc = "1: Interrupt enabled."]
    INTERRUPT_ENABLED_ = 1,
}
impl From<ICAP2_A> for bool {
    #[inline(always)]
    fn from(variant: ICAP2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ICAP2`"]
pub type ICAP2_R = crate::R<bool, ICAP2_A>;
impl ICAP2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICAP2_A {
        match self.bits {
            false => ICAP2_A::INTERRUPT_DISABLED_,
            true => ICAP2_A::INTERRUPT_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == ICAP2_A::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == ICAP2_A::INTERRUPT_ENABLED_
    }
}
#[doc = "Fast abort interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABORT_A {
    #[doc = "0: Interrupt disabled."]
    INTERRUPT_DISABLED_ = 0,
    #[doc = "1: Interrupt enabled."]
    INTERRUPT_ENABLED_ = 1,
}
impl From<ABORT_A> for bool {
    #[inline(always)]
    fn from(variant: ABORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ABORT`"]
pub type ABORT_R = crate::R<bool, ABORT_A>;
impl ABORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABORT_A {
        match self.bits {
            false => ABORT_A::INTERRUPT_DISABLED_,
            true => ABORT_A::INTERRUPT_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_DISABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == ABORT_A::INTERRUPT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ENABLED_`"]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == ABORT_A::INTERRUPT_ENABLED_
    }
}
impl R {
    #[doc = "Bit 0 - Limit interrupt for channel 0."]
    #[inline(always)]
    pub fn ilim0(&self) -> ILIM0_R {
        ILIM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Match interrupt for channel 0."]
    #[inline(always)]
    pub fn imat0(&self) -> IMAT0_R {
        IMAT0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Capture interrupt for channel 0."]
    #[inline(always)]
    pub fn icap0(&self) -> ICAP0_R {
        ICAP0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Limit interrupt for channel 1."]
    #[inline(always)]
    pub fn ilim1(&self) -> ILIM1_R {
        ILIM1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Match interrupt for channel 1."]
    #[inline(always)]
    pub fn imat1(&self) -> IMAT1_R {
        IMAT1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Capture interrupt for channel 1."]
    #[inline(always)]
    pub fn icap1(&self) -> ICAP1_R {
        ICAP1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Limit interrupt for channel 2."]
    #[inline(always)]
    pub fn ilim2(&self) -> ILIM2_R {
        ILIM2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Match interrupt for channel 2."]
    #[inline(always)]
    pub fn imat2(&self) -> IMAT2_R {
        IMAT2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Capture interrupt for channel 2."]
    #[inline(always)]
    pub fn icap2(&self) -> ICAP2_R {
        ICAP2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Fast abort interrupt."]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
