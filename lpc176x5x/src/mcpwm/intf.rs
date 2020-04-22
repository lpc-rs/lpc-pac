#[doc = "Reader of register INTF"]
pub type R = crate::R<u32, super::INTF>;
#[doc = "Limit interrupt flag for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILIM0_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING = 1,
}
impl From<ILIM0_F_A> for bool {
    #[inline(always)]
    fn from(variant: ILIM0_F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ILIM0_F`"]
pub type ILIM0_F_R = crate::R<bool, ILIM0_F_A>;
impl ILIM0_F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILIM0_F_A {
        match self.bits {
            false => ILIM0_F_A::THIS_INTERRUPT_SOURC,
            true => ILIM0_F_A::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == ILIM0_F_A::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == ILIM0_F_A::IF_THE_CORRESPONDING
    }
}
#[doc = "Match interrupt flag for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMAT0_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING = 1,
}
impl From<IMAT0_F_A> for bool {
    #[inline(always)]
    fn from(variant: IMAT0_F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IMAT0_F`"]
pub type IMAT0_F_R = crate::R<bool, IMAT0_F_A>;
impl IMAT0_F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMAT0_F_A {
        match self.bits {
            false => IMAT0_F_A::THIS_INTERRUPT_SOURC,
            true => IMAT0_F_A::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == IMAT0_F_A::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == IMAT0_F_A::IF_THE_CORRESPONDING
    }
}
#[doc = "Capture interrupt flag for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICAP0_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING = 1,
}
impl From<ICAP0_F_A> for bool {
    #[inline(always)]
    fn from(variant: ICAP0_F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ICAP0_F`"]
pub type ICAP0_F_R = crate::R<bool, ICAP0_F_A>;
impl ICAP0_F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICAP0_F_A {
        match self.bits {
            false => ICAP0_F_A::THIS_INTERRUPT_SOURC,
            true => ICAP0_F_A::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == ICAP0_F_A::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == ICAP0_F_A::IF_THE_CORRESPONDING
    }
}
#[doc = "Limit interrupt flag for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILIM1_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING = 1,
}
impl From<ILIM1_F_A> for bool {
    #[inline(always)]
    fn from(variant: ILIM1_F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ILIM1_F`"]
pub type ILIM1_F_R = crate::R<bool, ILIM1_F_A>;
impl ILIM1_F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILIM1_F_A {
        match self.bits {
            false => ILIM1_F_A::THIS_INTERRUPT_SOURC,
            true => ILIM1_F_A::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == ILIM1_F_A::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == ILIM1_F_A::IF_THE_CORRESPONDING
    }
}
#[doc = "Match interrupt flag for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMAT1_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING = 1,
}
impl From<IMAT1_F_A> for bool {
    #[inline(always)]
    fn from(variant: IMAT1_F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IMAT1_F`"]
pub type IMAT1_F_R = crate::R<bool, IMAT1_F_A>;
impl IMAT1_F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMAT1_F_A {
        match self.bits {
            false => IMAT1_F_A::THIS_INTERRUPT_SOURC,
            true => IMAT1_F_A::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == IMAT1_F_A::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == IMAT1_F_A::IF_THE_CORRESPONDING
    }
}
#[doc = "Capture interrupt flag for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICAP1_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING = 1,
}
impl From<ICAP1_F_A> for bool {
    #[inline(always)]
    fn from(variant: ICAP1_F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ICAP1_F`"]
pub type ICAP1_F_R = crate::R<bool, ICAP1_F_A>;
impl ICAP1_F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICAP1_F_A {
        match self.bits {
            false => ICAP1_F_A::THIS_INTERRUPT_SOURC,
            true => ICAP1_F_A::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == ICAP1_F_A::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == ICAP1_F_A::IF_THE_CORRESPONDING
    }
}
#[doc = "Limit interrupt flag for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILIM2_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING = 1,
}
impl From<ILIM2_F_A> for bool {
    #[inline(always)]
    fn from(variant: ILIM2_F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ILIM2_F`"]
pub type ILIM2_F_R = crate::R<bool, ILIM2_F_A>;
impl ILIM2_F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILIM2_F_A {
        match self.bits {
            false => ILIM2_F_A::THIS_INTERRUPT_SOURC,
            true => ILIM2_F_A::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == ILIM2_F_A::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == ILIM2_F_A::IF_THE_CORRESPONDING
    }
}
#[doc = "Match interrupt flag for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMAT2_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING = 1,
}
impl From<IMAT2_F_A> for bool {
    #[inline(always)]
    fn from(variant: IMAT2_F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IMAT2_F`"]
pub type IMAT2_F_R = crate::R<bool, IMAT2_F_A>;
impl IMAT2_F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMAT2_F_A {
        match self.bits {
            false => IMAT2_F_A::THIS_INTERRUPT_SOURC,
            true => IMAT2_F_A::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == IMAT2_F_A::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == IMAT2_F_A::IF_THE_CORRESPONDING
    }
}
#[doc = "Capture interrupt flag for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICAP2_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING = 1,
}
impl From<ICAP2_F_A> for bool {
    #[inline(always)]
    fn from(variant: ICAP2_F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ICAP2_F`"]
pub type ICAP2_F_R = crate::R<bool, ICAP2_F_A>;
impl ICAP2_F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICAP2_F_A {
        match self.bits {
            false => ICAP2_F_A::THIS_INTERRUPT_SOURC,
            true => ICAP2_F_A::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == ICAP2_F_A::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == ICAP2_F_A::IF_THE_CORRESPONDING
    }
}
#[doc = "Fast abort interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABORT_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING = 1,
}
impl From<ABORT_F_A> for bool {
    #[inline(always)]
    fn from(variant: ABORT_F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ABORT_F`"]
pub type ABORT_F_R = crate::R<bool, ABORT_F_A>;
impl ABORT_F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABORT_F_A {
        match self.bits {
            false => ABORT_F_A::THIS_INTERRUPT_SOURC,
            true => ABORT_F_A::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == ABORT_F_A::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == ABORT_F_A::IF_THE_CORRESPONDING
    }
}
impl R {
    #[doc = "Bit 0 - Limit interrupt flag for channel 0."]
    #[inline(always)]
    pub fn ilim0_f(&self) -> ILIM0_F_R {
        ILIM0_F_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Match interrupt flag for channel 0."]
    #[inline(always)]
    pub fn imat0_f(&self) -> IMAT0_F_R {
        IMAT0_F_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Capture interrupt flag for channel 0."]
    #[inline(always)]
    pub fn icap0_f(&self) -> ICAP0_F_R {
        ICAP0_F_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Limit interrupt flag for channel 1."]
    #[inline(always)]
    pub fn ilim1_f(&self) -> ILIM1_F_R {
        ILIM1_F_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Match interrupt flag for channel 1."]
    #[inline(always)]
    pub fn imat1_f(&self) -> IMAT1_F_R {
        IMAT1_F_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Capture interrupt flag for channel 1."]
    #[inline(always)]
    pub fn icap1_f(&self) -> ICAP1_F_R {
        ICAP1_F_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Limit interrupt flag for channel 2."]
    #[inline(always)]
    pub fn ilim2_f(&self) -> ILIM2_F_R {
        ILIM2_F_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Match interrupt flag for channel 2."]
    #[inline(always)]
    pub fn imat2_f(&self) -> IMAT2_F_R {
        IMAT2_F_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Capture interrupt flag for channel 2."]
    #[inline(always)]
    pub fn icap2_f(&self) -> ICAP2_F_R {
        ICAP2_F_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Fast abort interrupt flag."]
    #[inline(always)]
    pub fn abort_f(&self) -> ABORT_F_R {
        ABORT_F_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
