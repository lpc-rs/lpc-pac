#[doc = "Reader of register CCR"]
pub type R = crate::R<u32, super::CCR>;
#[doc = "Writer for register CCR"]
pub type W = crate::W<u32, super::CCR>;
#[doc = "Register CCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Capture on PWMn_CAP0 rising edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0_R_A {
    #[doc = "0: Disabled. This feature is disabled."]
    DISABLED_THIS_FEATU = 0,
    #[doc = "1: Rising edge. A synchronously sampled rising edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of the TC."]
    RISING_EDGE_A_SYNCH = 1,
}
impl From<CAP0_R_A> for bool {
    #[inline(always)]
    fn from(variant: CAP0_R_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAP0_R`"]
pub type CAP0_R_R = crate::R<bool, CAP0_R_A>;
impl CAP0_R_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP0_R_A {
        match self.bits {
            false => CAP0_R_A::DISABLED_THIS_FEATU,
            true => CAP0_R_A::RISING_EDGE_A_SYNCH,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_THIS_FEATU`"]
    #[inline(always)]
    pub fn is_disabled_this_featu(&self) -> bool {
        *self == CAP0_R_A::DISABLED_THIS_FEATU
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE_A_SYNCH`"]
    #[inline(always)]
    pub fn is_rising_edge_a_synch(&self) -> bool {
        *self == CAP0_R_A::RISING_EDGE_A_SYNCH
    }
}
#[doc = "Write proxy for field `CAP0_R`"]
pub struct CAP0_R_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0_R_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP0_R_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn disabled_this_featu(self) -> &'a mut W {
        self.variant(CAP0_R_A::DISABLED_THIS_FEATU)
    }
    #[doc = "Rising edge. A synchronously sampled rising edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of the TC."]
    #[inline(always)]
    pub fn rising_edge_a_synch(self) -> &'a mut W {
        self.variant(CAP0_R_A::RISING_EDGE_A_SYNCH)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Capture on PWMn_CAP0 falling edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0_F_A {
    #[doc = "0: Disabled. This feature is disabled."]
    DISABLED_THIS_FEATU = 0,
    #[doc = "1: Falling edge. A synchronously sampled falling edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of TC."]
    FALLING_EDGE_A_SYNC = 1,
}
impl From<CAP0_F_A> for bool {
    #[inline(always)]
    fn from(variant: CAP0_F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAP0_F`"]
pub type CAP0_F_R = crate::R<bool, CAP0_F_A>;
impl CAP0_F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP0_F_A {
        match self.bits {
            false => CAP0_F_A::DISABLED_THIS_FEATU,
            true => CAP0_F_A::FALLING_EDGE_A_SYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_THIS_FEATU`"]
    #[inline(always)]
    pub fn is_disabled_this_featu(&self) -> bool {
        *self == CAP0_F_A::DISABLED_THIS_FEATU
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE_A_SYNC`"]
    #[inline(always)]
    pub fn is_falling_edge_a_sync(&self) -> bool {
        *self == CAP0_F_A::FALLING_EDGE_A_SYNC
    }
}
#[doc = "Write proxy for field `CAP0_F`"]
pub struct CAP0_F_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0_F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP0_F_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn disabled_this_featu(self) -> &'a mut W {
        self.variant(CAP0_F_A::DISABLED_THIS_FEATU)
    }
    #[doc = "Falling edge. A synchronously sampled falling edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn falling_edge_a_sync(self) -> &'a mut W {
        self.variant(CAP0_F_A::FALLING_EDGE_A_SYNC)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Interrupt on PWMn_CAP0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0_I_A {
    #[doc = "0: Disabled. This feature is disabled."]
    DISABLED_THIS_FEATU = 0,
    #[doc = "1: Interrupt. A CR0 load due to a PWMn_CAP0 event will generate an interrupt."]
    INTERRUPT_A_CR0_LOA = 1,
}
impl From<CAP0_I_A> for bool {
    #[inline(always)]
    fn from(variant: CAP0_I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAP0_I`"]
pub type CAP0_I_R = crate::R<bool, CAP0_I_A>;
impl CAP0_I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP0_I_A {
        match self.bits {
            false => CAP0_I_A::DISABLED_THIS_FEATU,
            true => CAP0_I_A::INTERRUPT_A_CR0_LOA,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_THIS_FEATU`"]
    #[inline(always)]
    pub fn is_disabled_this_featu(&self) -> bool {
        *self == CAP0_I_A::DISABLED_THIS_FEATU
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_A_CR0_LOA`"]
    #[inline(always)]
    pub fn is_interrupt_a_cr0_loa(&self) -> bool {
        *self == CAP0_I_A::INTERRUPT_A_CR0_LOA
    }
}
#[doc = "Write proxy for field `CAP0_I`"]
pub struct CAP0_I_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0_I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP0_I_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn disabled_this_featu(self) -> &'a mut W {
        self.variant(CAP0_I_A::DISABLED_THIS_FEATU)
    }
    #[doc = "Interrupt. A CR0 load due to a PWMn_CAP0 event will generate an interrupt."]
    #[inline(always)]
    pub fn interrupt_a_cr0_loa(self) -> &'a mut W {
        self.variant(CAP0_I_A::INTERRUPT_A_CR0_LOA)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Capture on PWMn_CAP1 rising edge. Reserved for PWM0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1_R_A {
    #[doc = "0: Disabled. This feature is disabled."]
    DISABLED_THIS_FEATU = 0,
    #[doc = "1: Rising edge. A synchronously sampled rising edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of the TC."]
    RISING_EDGE_A_SYNCH = 1,
}
impl From<CAP1_R_A> for bool {
    #[inline(always)]
    fn from(variant: CAP1_R_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAP1_R`"]
pub type CAP1_R_R = crate::R<bool, CAP1_R_A>;
impl CAP1_R_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP1_R_A {
        match self.bits {
            false => CAP1_R_A::DISABLED_THIS_FEATU,
            true => CAP1_R_A::RISING_EDGE_A_SYNCH,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_THIS_FEATU`"]
    #[inline(always)]
    pub fn is_disabled_this_featu(&self) -> bool {
        *self == CAP1_R_A::DISABLED_THIS_FEATU
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE_A_SYNCH`"]
    #[inline(always)]
    pub fn is_rising_edge_a_synch(&self) -> bool {
        *self == CAP1_R_A::RISING_EDGE_A_SYNCH
    }
}
#[doc = "Write proxy for field `CAP1_R`"]
pub struct CAP1_R_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1_R_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP1_R_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn disabled_this_featu(self) -> &'a mut W {
        self.variant(CAP1_R_A::DISABLED_THIS_FEATU)
    }
    #[doc = "Rising edge. A synchronously sampled rising edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of the TC."]
    #[inline(always)]
    pub fn rising_edge_a_synch(self) -> &'a mut W {
        self.variant(CAP1_R_A::RISING_EDGE_A_SYNCH)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Capture on PWMn_CAP1 falling edge. Reserved for PWM0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1_F_A {
    #[doc = "0: Disabled. This feature is disabled."]
    DISABLED_THIS_FEATU = 0,
    #[doc = "1: Falling edge. A synchronously sampled falling edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of TC."]
    FALLING_EDGE_A_SYNC = 1,
}
impl From<CAP1_F_A> for bool {
    #[inline(always)]
    fn from(variant: CAP1_F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAP1_F`"]
pub type CAP1_F_R = crate::R<bool, CAP1_F_A>;
impl CAP1_F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP1_F_A {
        match self.bits {
            false => CAP1_F_A::DISABLED_THIS_FEATU,
            true => CAP1_F_A::FALLING_EDGE_A_SYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_THIS_FEATU`"]
    #[inline(always)]
    pub fn is_disabled_this_featu(&self) -> bool {
        *self == CAP1_F_A::DISABLED_THIS_FEATU
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE_A_SYNC`"]
    #[inline(always)]
    pub fn is_falling_edge_a_sync(&self) -> bool {
        *self == CAP1_F_A::FALLING_EDGE_A_SYNC
    }
}
#[doc = "Write proxy for field `CAP1_F`"]
pub struct CAP1_F_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1_F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP1_F_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn disabled_this_featu(self) -> &'a mut W {
        self.variant(CAP1_F_A::DISABLED_THIS_FEATU)
    }
    #[doc = "Falling edge. A synchronously sampled falling edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn falling_edge_a_sync(self) -> &'a mut W {
        self.variant(CAP1_F_A::FALLING_EDGE_A_SYNC)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Interrupt on PWMn_CAP1 event. Reserved for PWM0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1_I_A {
    #[doc = "0: Disabled. This feature is disabled."]
    DISABLED_THIS_FEATU = 0,
    #[doc = "1: Interrupt. A CR1 load due to a PWMn_CAP1 event will generate an interrupt."]
    INTERRUPT_A_CR1_LOA = 1,
}
impl From<CAP1_I_A> for bool {
    #[inline(always)]
    fn from(variant: CAP1_I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAP1_I`"]
pub type CAP1_I_R = crate::R<bool, CAP1_I_A>;
impl CAP1_I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP1_I_A {
        match self.bits {
            false => CAP1_I_A::DISABLED_THIS_FEATU,
            true => CAP1_I_A::INTERRUPT_A_CR1_LOA,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_THIS_FEATU`"]
    #[inline(always)]
    pub fn is_disabled_this_featu(&self) -> bool {
        *self == CAP1_I_A::DISABLED_THIS_FEATU
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_A_CR1_LOA`"]
    #[inline(always)]
    pub fn is_interrupt_a_cr1_loa(&self) -> bool {
        *self == CAP1_I_A::INTERRUPT_A_CR1_LOA
    }
}
#[doc = "Write proxy for field `CAP1_I`"]
pub struct CAP1_I_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1_I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP1_I_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn disabled_this_featu(self) -> &'a mut W {
        self.variant(CAP1_I_A::DISABLED_THIS_FEATU)
    }
    #[doc = "Interrupt. A CR1 load due to a PWMn_CAP1 event will generate an interrupt."]
    #[inline(always)]
    pub fn interrupt_a_cr1_loa(self) -> &'a mut W {
        self.variant(CAP1_I_A::INTERRUPT_A_CR1_LOA)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Capture on PWMn_CAP0 rising edge"]
    #[inline(always)]
    pub fn cap0_r(&self) -> CAP0_R_R {
        CAP0_R_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Capture on PWMn_CAP0 falling edge"]
    #[inline(always)]
    pub fn cap0_f(&self) -> CAP0_F_R {
        CAP0_F_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt on PWMn_CAP0 event"]
    #[inline(always)]
    pub fn cap0_i(&self) -> CAP0_I_R {
        CAP0_I_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Capture on PWMn_CAP1 rising edge. Reserved for PWM0."]
    #[inline(always)]
    pub fn cap1_r(&self) -> CAP1_R_R {
        CAP1_R_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Capture on PWMn_CAP1 falling edge. Reserved for PWM0."]
    #[inline(always)]
    pub fn cap1_f(&self) -> CAP1_F_R {
        CAP1_F_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt on PWMn_CAP1 event. Reserved for PWM0."]
    #[inline(always)]
    pub fn cap1_i(&self) -> CAP1_I_R {
        CAP1_I_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture on PWMn_CAP0 rising edge"]
    #[inline(always)]
    pub fn cap0_r(&mut self) -> CAP0_R_W {
        CAP0_R_W { w: self }
    }
    #[doc = "Bit 1 - Capture on PWMn_CAP0 falling edge"]
    #[inline(always)]
    pub fn cap0_f(&mut self) -> CAP0_F_W {
        CAP0_F_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt on PWMn_CAP0 event"]
    #[inline(always)]
    pub fn cap0_i(&mut self) -> CAP0_I_W {
        CAP0_I_W { w: self }
    }
    #[doc = "Bit 3 - Capture on PWMn_CAP1 rising edge. Reserved for PWM0."]
    #[inline(always)]
    pub fn cap1_r(&mut self) -> CAP1_R_W {
        CAP1_R_W { w: self }
    }
    #[doc = "Bit 4 - Capture on PWMn_CAP1 falling edge. Reserved for PWM0."]
    #[inline(always)]
    pub fn cap1_f(&mut self) -> CAP1_F_W {
        CAP1_F_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt on PWMn_CAP1 event. Reserved for PWM0."]
    #[inline(always)]
    pub fn cap1_i(&mut self) -> CAP1_I_W {
        CAP1_I_W { w: self }
    }
}
