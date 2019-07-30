#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0x0004_0004
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type DOWN_L_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DOWN_LW<'a> {
    w: &'a mut W,
}
impl<'a> _DOWN_LW<'a> {
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
#[doc = r"Reader of the field"]
pub type STOP_L_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _STOP_LW<'a> {
    w: &'a mut W,
}
impl<'a> _STOP_LW<'a> {
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
#[doc = r"Reader of the field"]
pub type HALT_L_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _HALT_LW<'a> {
    w: &'a mut W,
}
impl<'a> _HALT_LW<'a> {
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
#[doc = r"Reader of the field"]
pub type CLRCTR_L_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CLRCTR_LW<'a> {
    w: &'a mut W,
}
impl<'a> _CLRCTR_LW<'a> {
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
#[doc = "Possible values of the field `BIDIR_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIDIR_LR {
    #[doc = "The counter counts up to its limit condition, then is cleared to zero."]
    THE_COUNTER_COUNTS_UP_AND_CLEAR,
    #[doc = "The counter counts up to its limit, then counts down to a limit condition or to 0."]
    THE_COUNTER_COUNTS_UP_AND_DOWN,
}
impl crate::ToBits<bool> for BIDIR_LR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            BIDIR_LR::THE_COUNTER_COUNTS_UP_AND_CLEAR => false,
            BIDIR_LR::THE_COUNTER_COUNTS_UP_AND_DOWN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BIDIR_L_R = crate::FR<bool, BIDIR_LR>;
impl BIDIR_L_R {
    #[doc = "Checks if the value of the field is `THE_COUNTER_COUNTS_UP_AND_CLEAR`"]
    #[inline(always)]
    pub fn is_the_counter_counts_up_and_clear(&self) -> bool {
        *self == BIDIR_LR::THE_COUNTER_COUNTS_UP_AND_CLEAR
    }
    #[doc = "Checks if the value of the field is `THE_COUNTER_COUNTS_UP_AND_DOWN`"]
    #[inline(always)]
    pub fn is_the_counter_counts_up_and_down(&self) -> bool {
        *self == BIDIR_LR::THE_COUNTER_COUNTS_UP_AND_DOWN
    }
}
#[doc = "Values that can be written to the field `BIDIR_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIDIR_LW {
    #[doc = "The counter counts up to its limit condition, then is cleared to zero."]
    THE_COUNTER_COUNTS_UP_AND_CLEAR,
    #[doc = "The counter counts up to its limit, then counts down to a limit condition or to 0."]
    THE_COUNTER_COUNTS_UP_AND_DOWN,
}
impl BIDIR_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            BIDIR_LW::THE_COUNTER_COUNTS_UP_AND_CLEAR => false,
            BIDIR_LW::THE_COUNTER_COUNTS_UP_AND_DOWN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _BIDIR_LW<'a> {
    w: &'a mut W,
}
impl<'a> _BIDIR_LW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIDIR_LW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The counter counts up to its limit condition, then is cleared to zero."]
    #[inline(always)]
    pub fn the_counter_counts_up_and_clear(self) -> &'a mut W {
        self.variant(BIDIR_LW::THE_COUNTER_COUNTS_UP_AND_CLEAR)
    }
    #[doc = "The counter counts up to its limit, then counts down to a limit condition or to 0."]
    #[inline(always)]
    pub fn the_counter_counts_up_and_down(self) -> &'a mut W {
        self.variant(BIDIR_LW::THE_COUNTER_COUNTS_UP_AND_DOWN)
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
#[doc = r"Reader of the field"]
pub type PRE_L_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PRE_LW<'a> {
    w: &'a mut W,
}
impl<'a> _PRE_LW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 5)) | (((value as u32) & 0xff) << 5);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DOWN_H_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DOWN_HW<'a> {
    w: &'a mut W,
}
impl<'a> _DOWN_HW<'a> {
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
#[doc = r"Reader of the field"]
pub type STOP_H_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _STOP_HW<'a> {
    w: &'a mut W,
}
impl<'a> _STOP_HW<'a> {
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
#[doc = r"Reader of the field"]
pub type HALT_H_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _HALT_HW<'a> {
    w: &'a mut W,
}
impl<'a> _HALT_HW<'a> {
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
#[doc = r"Reader of the field"]
pub type CLRCTR_H_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CLRCTR_HW<'a> {
    w: &'a mut W,
}
impl<'a> _CLRCTR_HW<'a> {
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
#[doc = "Possible values of the field `BIDIR_H`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIDIR_HR {
    #[doc = "The H counter counts up to its limit condition, then is cleared to zero."]
    THE_H_COUNTER_COUNTS_UP_AND_CLEAR,
    #[doc = "The H counter counts up to its limit, then counts down to a limit condition or to 0."]
    THE_H_COUNTER_COUNTS_UP_AND_DOWN,
}
impl crate::ToBits<bool> for BIDIR_HR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            BIDIR_HR::THE_H_COUNTER_COUNTS_UP_AND_CLEAR => false,
            BIDIR_HR::THE_H_COUNTER_COUNTS_UP_AND_DOWN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BIDIR_H_R = crate::FR<bool, BIDIR_HR>;
impl BIDIR_H_R {
    #[doc = "Checks if the value of the field is `THE_H_COUNTER_COUNTS_UP_AND_CLEAR`"]
    #[inline(always)]
    pub fn is_the_h_counter_counts_up_and_clear(&self) -> bool {
        *self == BIDIR_HR::THE_H_COUNTER_COUNTS_UP_AND_CLEAR
    }
    #[doc = "Checks if the value of the field is `THE_H_COUNTER_COUNTS_UP_AND_DOWN`"]
    #[inline(always)]
    pub fn is_the_h_counter_counts_up_and_down(&self) -> bool {
        *self == BIDIR_HR::THE_H_COUNTER_COUNTS_UP_AND_DOWN
    }
}
#[doc = "Values that can be written to the field `BIDIR_H`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIDIR_HW {
    #[doc = "The H counter counts up to its limit condition, then is cleared to zero."]
    THE_H_COUNTER_COUNTS_UP_AND_CLEAR,
    #[doc = "The H counter counts up to its limit, then counts down to a limit condition or to 0."]
    THE_H_COUNTER_COUNTS_UP_AND_DOWN,
}
impl BIDIR_HW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            BIDIR_HW::THE_H_COUNTER_COUNTS_UP_AND_CLEAR => false,
            BIDIR_HW::THE_H_COUNTER_COUNTS_UP_AND_DOWN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _BIDIR_HW<'a> {
    w: &'a mut W,
}
impl<'a> _BIDIR_HW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIDIR_HW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The H counter counts up to its limit condition, then is cleared to zero."]
    #[inline(always)]
    pub fn the_h_counter_counts_up_and_clear(self) -> &'a mut W {
        self.variant(BIDIR_HW::THE_H_COUNTER_COUNTS_UP_AND_CLEAR)
    }
    #[doc = "The H counter counts up to its limit, then counts down to a limit condition or to 0."]
    #[inline(always)]
    pub fn the_h_counter_counts_up_and_down(self) -> &'a mut W {
        self.variant(BIDIR_HW::THE_H_COUNTER_COUNTS_UP_AND_DOWN)
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
#[doc = r"Reader of the field"]
pub type PRE_H_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PRE_HW<'a> {
    w: &'a mut W,
}
impl<'a> _PRE_HW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 21)) | (((value as u32) & 0xff) << 21);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - This bit is 1 when the L or unified counter is counting down. Hardware sets this bit when the counter limit is reached and BIDIR is 1. Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[inline(always)]
    pub fn down_l(&self) -> DOWN_L_R {
        DOWN_L_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - When this bit is 1 and HALT is 0, the L or unified counter does not run, but I/O events related to the counter can occur. If such an event matches the mask in the Start register, this bit is cleared and counting resumes."]
    #[inline(always)]
    pub fn stop_l(&self) -> STOP_L_R {
        STOP_L_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - When this bit is 1, the L or unified counter does not run and no events can occur. A reset sets this bit. When the HALT_L bit is one, the STOP_L bit is cleared. If you want to remove the halt condition and keep the SCT in the stop condition (not running), then you can change the halt and stop condition with one single write to this register. Once set, only software can clear this bit to restore counter operation."]
    #[inline(always)]
    pub fn halt_l(&self) -> HALT_L_R {
        HALT_L_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Writing a 1 to this bit clears the L or unified counter. This bit always reads as 0."]
    #[inline(always)]
    pub fn clrctr_l(&self) -> CLRCTR_L_R {
        CLRCTR_L_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - L or unified counter direction select"]
    #[inline(always)]
    pub fn bidir_l(&self) -> BIDIR_L_R {
        BIDIR_L_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:12 - Specifies the factor by which the SCT clock is prescaled to produce the L or unified counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRE_L+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[inline(always)]
    pub fn pre_l(&self) -> PRE_L_R {
        PRE_L_R::new(((self.bits() >> 5) & 0xff) as u8)
    }
    #[doc = "Bit 16 - This bit is 1 when the H counter is counting down. Hardware sets this bit when the counter limit is reached and BIDIR is 1. Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[inline(always)]
    pub fn down_h(&self) -> DOWN_H_R {
        DOWN_H_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - When this bit is 1 and HALT is 0, the H counter does not, run but I/O events related to the counter can occur. If such an event matches the mask in the Start register, this bit is cleared and counting resumes."]
    #[inline(always)]
    pub fn stop_h(&self) -> STOP_H_R {
        STOP_H_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - When this bit is 1, the H counter does not run and no events can occur. A reset sets this bit. When the HALT_H bit is one, the STOP_H bit is cleared. If you want to remove the halt condition and keep the SCT in the stop condition (not running), then you can change the halt and stop condition with one single write to this register. Once set, this bit can only be cleared by software to restore counter operation."]
    #[inline(always)]
    pub fn halt_h(&self) -> HALT_H_R {
        HALT_H_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Writing a 1 to this bit clears the H counter. This bit always reads as 0."]
    #[inline(always)]
    pub fn clrctr_h(&self) -> CLRCTR_H_R {
        CLRCTR_H_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Direction select"]
    #[inline(always)]
    pub fn bidir_h(&self) -> BIDIR_H_R {
        BIDIR_H_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:28 - Specifies the factor by which the SCT clock is prescaled to produce the H counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRELH+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[inline(always)]
    pub fn pre_h(&self) -> PRE_H_R {
        PRE_H_R::new(((self.bits() >> 21) & 0xff) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - This bit is 1 when the L or unified counter is counting down. Hardware sets this bit when the counter limit is reached and BIDIR is 1. Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[inline(always)]
    pub fn down_l(&mut self) -> _DOWN_LW {
        _DOWN_LW { w: self }
    }
    #[doc = "Bit 1 - When this bit is 1 and HALT is 0, the L or unified counter does not run, but I/O events related to the counter can occur. If such an event matches the mask in the Start register, this bit is cleared and counting resumes."]
    #[inline(always)]
    pub fn stop_l(&mut self) -> _STOP_LW {
        _STOP_LW { w: self }
    }
    #[doc = "Bit 2 - When this bit is 1, the L or unified counter does not run and no events can occur. A reset sets this bit. When the HALT_L bit is one, the STOP_L bit is cleared. If you want to remove the halt condition and keep the SCT in the stop condition (not running), then you can change the halt and stop condition with one single write to this register. Once set, only software can clear this bit to restore counter operation."]
    #[inline(always)]
    pub fn halt_l(&mut self) -> _HALT_LW {
        _HALT_LW { w: self }
    }
    #[doc = "Bit 3 - Writing a 1 to this bit clears the L or unified counter. This bit always reads as 0."]
    #[inline(always)]
    pub fn clrctr_l(&mut self) -> _CLRCTR_LW {
        _CLRCTR_LW { w: self }
    }
    #[doc = "Bit 4 - L or unified counter direction select"]
    #[inline(always)]
    pub fn bidir_l(&mut self) -> _BIDIR_LW {
        _BIDIR_LW { w: self }
    }
    #[doc = "Bits 5:12 - Specifies the factor by which the SCT clock is prescaled to produce the L or unified counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRE_L+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[inline(always)]
    pub fn pre_l(&mut self) -> _PRE_LW {
        _PRE_LW { w: self }
    }
    #[doc = "Bit 16 - This bit is 1 when the H counter is counting down. Hardware sets this bit when the counter limit is reached and BIDIR is 1. Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[inline(always)]
    pub fn down_h(&mut self) -> _DOWN_HW {
        _DOWN_HW { w: self }
    }
    #[doc = "Bit 17 - When this bit is 1 and HALT is 0, the H counter does not, run but I/O events related to the counter can occur. If such an event matches the mask in the Start register, this bit is cleared and counting resumes."]
    #[inline(always)]
    pub fn stop_h(&mut self) -> _STOP_HW {
        _STOP_HW { w: self }
    }
    #[doc = "Bit 18 - When this bit is 1, the H counter does not run and no events can occur. A reset sets this bit. When the HALT_H bit is one, the STOP_H bit is cleared. If you want to remove the halt condition and keep the SCT in the stop condition (not running), then you can change the halt and stop condition with one single write to this register. Once set, this bit can only be cleared by software to restore counter operation."]
    #[inline(always)]
    pub fn halt_h(&mut self) -> _HALT_HW {
        _HALT_HW { w: self }
    }
    #[doc = "Bit 19 - Writing a 1 to this bit clears the H counter. This bit always reads as 0."]
    #[inline(always)]
    pub fn clrctr_h(&mut self) -> _CLRCTR_HW {
        _CLRCTR_HW { w: self }
    }
    #[doc = "Bit 20 - Direction select"]
    #[inline(always)]
    pub fn bidir_h(&mut self) -> _BIDIR_HW {
        _BIDIR_HW { w: self }
    }
    #[doc = "Bits 21:28 - Specifies the factor by which the SCT clock is prescaled to produce the H counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRELH+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[inline(always)]
    pub fn pre_h(&mut self) -> _PRE_HW {
        _PRE_HW { w: self }
    }
}
