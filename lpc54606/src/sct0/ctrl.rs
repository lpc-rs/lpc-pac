#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x0004_0004"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0004_0004
    }
}
#[doc = "Reader of field `DOWN_L`"]
pub type DOWN_L_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOWN_L`"]
pub struct DOWN_L_W<'a> {
    w: &'a mut W,
}
impl<'a> DOWN_L_W<'a> {
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
#[doc = "Reader of field `STOP_L`"]
pub type STOP_L_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOP_L`"]
pub struct STOP_L_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_L_W<'a> {
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
#[doc = "Reader of field `HALT_L`"]
pub type HALT_L_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HALT_L`"]
pub struct HALT_L_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_L_W<'a> {
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
#[doc = "Reader of field `CLRCTR_L`"]
pub type CLRCTR_L_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRCTR_L`"]
pub struct CLRCTR_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRCTR_L_W<'a> {
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
#[doc = "L or unified counter direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIDIR_L_A {
    #[doc = "0: Up. The counter counts up to a limit condition, then is cleared to zero."]
    UP = 0,
    #[doc = "1: Up-down. The counter counts up to a limit, then counts down to a limit condition or to 0."]
    UP_DOWN = 1,
}
impl From<BIDIR_L_A> for bool {
    #[inline(always)]
    fn from(variant: BIDIR_L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BIDIR_L`"]
pub type BIDIR_L_R = crate::R<bool, BIDIR_L_A>;
impl BIDIR_L_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIDIR_L_A {
        match self.bits {
            false => BIDIR_L_A::UP,
            true => BIDIR_L_A::UP_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == BIDIR_L_A::UP
    }
    #[doc = "Checks if the value of the field is `UP_DOWN`"]
    #[inline(always)]
    pub fn is_up_down(&self) -> bool {
        *self == BIDIR_L_A::UP_DOWN
    }
}
#[doc = "Write proxy for field `BIDIR_L`"]
pub struct BIDIR_L_W<'a> {
    w: &'a mut W,
}
impl<'a> BIDIR_L_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIDIR_L_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Up. The counter counts up to a limit condition, then is cleared to zero."]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(BIDIR_L_A::UP)
    }
    #[doc = "Up-down. The counter counts up to a limit, then counts down to a limit condition or to 0."]
    #[inline(always)]
    pub fn up_down(self) -> &'a mut W {
        self.variant(BIDIR_L_A::UP_DOWN)
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
#[doc = "Reader of field `PRE_L`"]
pub type PRE_L_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRE_L`"]
pub struct PRE_L_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 5)) | (((value as u32) & 0xff) << 5);
        self.w
    }
}
#[doc = "Reader of field `DOWN_H`"]
pub type DOWN_H_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOWN_H`"]
pub struct DOWN_H_W<'a> {
    w: &'a mut W,
}
impl<'a> DOWN_H_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `STOP_H`"]
pub type STOP_H_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOP_H`"]
pub struct STOP_H_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_H_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `HALT_H`"]
pub type HALT_H_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HALT_H`"]
pub struct HALT_H_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_H_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `CLRCTR_H`"]
pub type CLRCTR_H_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRCTR_H`"]
pub struct CLRCTR_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRCTR_H_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIDIR_H_A {
    #[doc = "0: The H counter counts up to its limit condition, then is cleared to zero."]
    UP = 0,
    #[doc = "1: The H counter counts up to its limit, then counts down to a limit condition or to 0."]
    UP_DOWN = 1,
}
impl From<BIDIR_H_A> for bool {
    #[inline(always)]
    fn from(variant: BIDIR_H_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BIDIR_H`"]
pub type BIDIR_H_R = crate::R<bool, BIDIR_H_A>;
impl BIDIR_H_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIDIR_H_A {
        match self.bits {
            false => BIDIR_H_A::UP,
            true => BIDIR_H_A::UP_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == BIDIR_H_A::UP
    }
    #[doc = "Checks if the value of the field is `UP_DOWN`"]
    #[inline(always)]
    pub fn is_up_down(&self) -> bool {
        *self == BIDIR_H_A::UP_DOWN
    }
}
#[doc = "Write proxy for field `BIDIR_H`"]
pub struct BIDIR_H_W<'a> {
    w: &'a mut W,
}
impl<'a> BIDIR_H_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIDIR_H_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The H counter counts up to its limit condition, then is cleared to zero."]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(BIDIR_H_A::UP)
    }
    #[doc = "The H counter counts up to its limit, then counts down to a limit condition or to 0."]
    #[inline(always)]
    pub fn up_down(self) -> &'a mut W {
        self.variant(BIDIR_H_A::UP_DOWN)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `PRE_H`"]
pub type PRE_H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRE_H`"]
pub struct PRE_H_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 21)) | (((value as u32) & 0xff) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - This bit is 1 when the L or unified counter is counting down. Hardware sets this bit when the counter is counting up, counter limit occurs, and BIDIR = 1.Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[inline(always)]
    pub fn down_l(&self) -> DOWN_L_R {
        DOWN_L_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When this bit is 1 and HALT is 0, the L or unified counter does not run, but I/O events related to the counter can occur. If a designated start event occurs, this bit is cleared and counting resumes."]
    #[inline(always)]
    pub fn stop_l(&self) -> STOP_L_R {
        STOP_L_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - When this bit is 1, the L or unified counter does not run and no events can occur. A reset sets this bit. When the HALT_L bit is one, the STOP_L bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, only software can clear this bit to restore counter operation. This bit is set on reset."]
    #[inline(always)]
    pub fn halt_l(&self) -> HALT_L_R {
        HALT_L_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Writing a 1 to this bit clears the L or unified counter. This bit always reads as 0."]
    #[inline(always)]
    pub fn clrctr_l(&self) -> CLRCTR_L_R {
        CLRCTR_L_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - L or unified counter direction select"]
    #[inline(always)]
    pub fn bidir_l(&self) -> BIDIR_L_R {
        BIDIR_L_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:12 - Specifies the factor by which the SCT clock is prescaled to produce the L or unified counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRE_L+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[inline(always)]
    pub fn pre_l(&self) -> PRE_L_R {
        PRE_L_R::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bit 16 - This bit is 1 when the H counter is counting down. Hardware sets this bit when the counter is counting, a counter limit condition occurs, and BIDIR is 1. Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[inline(always)]
    pub fn down_h(&self) -> DOWN_H_R {
        DOWN_H_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - When this bit is 1 and HALT is 0, the H counter does not, run but I/O events related to the counter can occur. If such an event matches the mask in the Start register, this bit is cleared and counting resumes."]
    #[inline(always)]
    pub fn stop_h(&self) -> STOP_H_R {
        STOP_H_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - When this bit is 1, the H counter does not run and no events can occur. A reset sets this bit. When the HALT_H bit is one, the STOP_H bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, this bit can only be cleared by software to restore counter operation. This bit is set on reset."]
    #[inline(always)]
    pub fn halt_h(&self) -> HALT_H_R {
        HALT_H_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Writing a 1 to this bit clears the H counter. This bit always reads as 0."]
    #[inline(always)]
    pub fn clrctr_h(&self) -> CLRCTR_H_R {
        CLRCTR_H_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Direction select"]
    #[inline(always)]
    pub fn bidir_h(&self) -> BIDIR_H_R {
        BIDIR_H_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:28 - Specifies the factor by which the SCT clock is prescaled to produce the H counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRELH+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[inline(always)]
    pub fn pre_h(&self) -> PRE_H_R {
        PRE_H_R::new(((self.bits >> 21) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is 1 when the L or unified counter is counting down. Hardware sets this bit when the counter is counting up, counter limit occurs, and BIDIR = 1.Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[inline(always)]
    pub fn down_l(&mut self) -> DOWN_L_W {
        DOWN_L_W { w: self }
    }
    #[doc = "Bit 1 - When this bit is 1 and HALT is 0, the L or unified counter does not run, but I/O events related to the counter can occur. If a designated start event occurs, this bit is cleared and counting resumes."]
    #[inline(always)]
    pub fn stop_l(&mut self) -> STOP_L_W {
        STOP_L_W { w: self }
    }
    #[doc = "Bit 2 - When this bit is 1, the L or unified counter does not run and no events can occur. A reset sets this bit. When the HALT_L bit is one, the STOP_L bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, only software can clear this bit to restore counter operation. This bit is set on reset."]
    #[inline(always)]
    pub fn halt_l(&mut self) -> HALT_L_W {
        HALT_L_W { w: self }
    }
    #[doc = "Bit 3 - Writing a 1 to this bit clears the L or unified counter. This bit always reads as 0."]
    #[inline(always)]
    pub fn clrctr_l(&mut self) -> CLRCTR_L_W {
        CLRCTR_L_W { w: self }
    }
    #[doc = "Bit 4 - L or unified counter direction select"]
    #[inline(always)]
    pub fn bidir_l(&mut self) -> BIDIR_L_W {
        BIDIR_L_W { w: self }
    }
    #[doc = "Bits 5:12 - Specifies the factor by which the SCT clock is prescaled to produce the L or unified counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRE_L+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[inline(always)]
    pub fn pre_l(&mut self) -> PRE_L_W {
        PRE_L_W { w: self }
    }
    #[doc = "Bit 16 - This bit is 1 when the H counter is counting down. Hardware sets this bit when the counter is counting, a counter limit condition occurs, and BIDIR is 1. Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[inline(always)]
    pub fn down_h(&mut self) -> DOWN_H_W {
        DOWN_H_W { w: self }
    }
    #[doc = "Bit 17 - When this bit is 1 and HALT is 0, the H counter does not, run but I/O events related to the counter can occur. If such an event matches the mask in the Start register, this bit is cleared and counting resumes."]
    #[inline(always)]
    pub fn stop_h(&mut self) -> STOP_H_W {
        STOP_H_W { w: self }
    }
    #[doc = "Bit 18 - When this bit is 1, the H counter does not run and no events can occur. A reset sets this bit. When the HALT_H bit is one, the STOP_H bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, this bit can only be cleared by software to restore counter operation. This bit is set on reset."]
    #[inline(always)]
    pub fn halt_h(&mut self) -> HALT_H_W {
        HALT_H_W { w: self }
    }
    #[doc = "Bit 19 - Writing a 1 to this bit clears the H counter. This bit always reads as 0."]
    #[inline(always)]
    pub fn clrctr_h(&mut self) -> CLRCTR_H_W {
        CLRCTR_H_W { w: self }
    }
    #[doc = "Bit 20 - Direction select"]
    #[inline(always)]
    pub fn bidir_h(&mut self) -> BIDIR_H_W {
        BIDIR_H_W { w: self }
    }
    #[doc = "Bits 21:28 - Specifies the factor by which the SCT clock is prescaled to produce the H counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRELH+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[inline(always)]
    pub fn pre_h(&mut self) -> PRE_H_W {
        PRE_H_W { w: self }
    }
}
