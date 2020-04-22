#[doc = "Reader of register IRQ_FLAG"]
pub type R = crate::R<u32, super::IRQ_FLAG>;
#[doc = "Writer for register IRQ_FLAG"]
pub type W = crate::W<u32, super::IRQ_FLAG>;
#[doc = "Register IRQ_FLAG `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQ_FLAG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Monitors the interrupt flag of TIMER0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GFLAG0_A {
    #[doc = "0: No pending interrupt. Writing a zero is equivalent to no operation."]
    NO_PENDING_INTERRUPT = 0,
    #[doc = "1: Pending interrupt. The interrupt is pending because TIMER0 has reached the end of the time interval. If the INTEN bit in the CONTROL0 register is also set to 1, the interrupt for timer channel 0 and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    PENDING_INTERRUPT = 1,
}
impl From<GFLAG0_A> for bool {
    #[inline(always)]
    fn from(variant: GFLAG0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GFLAG0`"]
pub type GFLAG0_R = crate::R<bool, GFLAG0_A>;
impl GFLAG0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GFLAG0_A {
        match self.bits {
            false => GFLAG0_A::NO_PENDING_INTERRUPT,
            true => GFLAG0_A::PENDING_INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_pending_interrupt(&self) -> bool {
        *self == GFLAG0_A::NO_PENDING_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING_INTERRUPT`"]
    #[inline(always)]
    pub fn is_pending_interrupt(&self) -> bool {
        *self == GFLAG0_A::PENDING_INTERRUPT
    }
}
#[doc = "Write proxy for field `GFLAG0`"]
pub struct GFLAG0_W<'a> {
    w: &'a mut W,
}
impl<'a> GFLAG0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GFLAG0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    #[inline(always)]
    pub fn no_pending_interrupt(self) -> &'a mut W {
        self.variant(GFLAG0_A::NO_PENDING_INTERRUPT)
    }
    #[doc = "Pending interrupt. The interrupt is pending because TIMER0 has reached the end of the time interval. If the INTEN bit in the CONTROL0 register is also set to 1, the interrupt for timer channel 0 and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    #[inline(always)]
    pub fn pending_interrupt(self) -> &'a mut W {
        self.variant(GFLAG0_A::PENDING_INTERRUPT)
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
#[doc = "Reader of field `GFLAG1`"]
pub type GFLAG1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GFLAG1`"]
pub struct GFLAG1_W<'a> {
    w: &'a mut W,
}
impl<'a> GFLAG1_W<'a> {
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
#[doc = "Reader of field `GFLAG2`"]
pub type GFLAG2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GFLAG2`"]
pub struct GFLAG2_W<'a> {
    w: &'a mut W,
}
impl<'a> GFLAG2_W<'a> {
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
#[doc = "Reader of field `GFLAG3`"]
pub type GFLAG3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GFLAG3`"]
pub struct GFLAG3_W<'a> {
    w: &'a mut W,
}
impl<'a> GFLAG3_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Monitors the interrupt flag of TIMER0."]
    #[inline(always)]
    pub fn gflag0(&self) -> GFLAG0_R {
        GFLAG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Monitors the interrupt flag of TIMER1. See description of channel 0."]
    #[inline(always)]
    pub fn gflag1(&self) -> GFLAG1_R {
        GFLAG1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Monitors the interrupt flag of TIMER2. See description of channel 0."]
    #[inline(always)]
    pub fn gflag2(&self) -> GFLAG2_R {
        GFLAG2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Monitors the interrupt flag of TIMER3. See description of channel 0."]
    #[inline(always)]
    pub fn gflag3(&self) -> GFLAG3_R {
        GFLAG3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Monitors the interrupt flag of TIMER0."]
    #[inline(always)]
    pub fn gflag0(&mut self) -> GFLAG0_W {
        GFLAG0_W { w: self }
    }
    #[doc = "Bit 1 - Monitors the interrupt flag of TIMER1. See description of channel 0."]
    #[inline(always)]
    pub fn gflag1(&mut self) -> GFLAG1_W {
        GFLAG1_W { w: self }
    }
    #[doc = "Bit 2 - Monitors the interrupt flag of TIMER2. See description of channel 0."]
    #[inline(always)]
    pub fn gflag2(&mut self) -> GFLAG2_W {
        GFLAG2_W { w: self }
    }
    #[doc = "Bit 3 - Monitors the interrupt flag of TIMER3. See description of channel 0."]
    #[inline(always)]
    pub fn gflag3(&mut self) -> GFLAG3_W {
        GFLAG3_W { w: self }
    }
}
