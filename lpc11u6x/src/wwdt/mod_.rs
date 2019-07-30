#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MOD {
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
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `WDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDENR {
    #[doc = "The watchdog timer is stopped."]
    STOPPED,
    #[doc = "The watchdog timer is running."]
    RUNNING,
}
impl crate::ToBits<bool> for WDENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WDENR::STOPPED => false,
            WDENR::RUNNING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WDEN_R = crate::FR<bool, WDENR>;
impl WDEN_R {
    #[doc = "Checks if the value of the field is `STOPPED`"]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == WDENR::STOPPED
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == WDENR::RUNNING
    }
}
#[doc = "Values that can be written to the field `WDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDENW {
    #[doc = "The watchdog timer is stopped."]
    STOPPED,
    #[doc = "The watchdog timer is running."]
    RUNNING,
}
impl WDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WDENW::STOPPED => false,
            WDENW::RUNNING => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WDENW<'a> {
    w: &'a mut W,
}
impl<'a> _WDENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The watchdog timer is stopped."]
    #[inline(always)]
    pub fn stopped(self) -> &'a mut W {
        self.variant(WDENW::STOPPED)
    }
    #[doc = "The watchdog timer is running."]
    #[inline(always)]
    pub fn running(self) -> &'a mut W {
        self.variant(WDENW::RUNNING)
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
#[doc = "Possible values of the field `WDRESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDRESETR {
    #[doc = "A watchdog timeout will not cause a chip reset."]
    INTERRUPT,
}
impl crate::ToBits<bool> for WDRESETR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WDRESETR::INTERRUPT => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WDRESET_R = crate::FR<bool, WDRESETR>;
impl WDRESET_R {
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == WDRESETR::INTERRUPT
    }
}
#[doc = "Values that can be written to the field `WDRESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDRESETW {
    #[doc = "A watchdog timeout will not cause a chip reset."]
    INTERRUPT,
}
impl WDRESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WDRESETW::INTERRUPT => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WDRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _WDRESETW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDRESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A watchdog timeout will not cause a chip reset."]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(WDRESETW::INTERRUPT)
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
#[doc = r"Reader of the field"]
pub type WDTOF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WDTOFW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTOFW<'a> {
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
pub type WDINT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WDINTW<'a> {
    w: &'a mut W,
}
impl<'a> _WDINTW<'a> {
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
#[doc = "Possible values of the field `WDPROTECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDPROTECTR {
    #[doc = "The watchdog time-out value (TC) can be changed at any time."]
    NOT_LOCKED,
    #[doc = "The watchdog time-out value (TC) can be changed only after the counter is below the value of WDWARNINT and WDWINDOW."]
    LOCKED,
}
impl crate::ToBits<bool> for WDPROTECTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WDPROTECTR::NOT_LOCKED => false,
            WDPROTECTR::LOCKED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WDPROTECT_R = crate::FR<bool, WDPROTECTR>;
impl WDPROTECT_R {
    #[doc = "Checks if the value of the field is `NOT_LOCKED`"]
    #[inline(always)]
    pub fn is_not_locked(&self) -> bool {
        *self == WDPROTECTR::NOT_LOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == WDPROTECTR::LOCKED
    }
}
#[doc = "Values that can be written to the field `WDPROTECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDPROTECTW {
    #[doc = "The watchdog time-out value (TC) can be changed at any time."]
    NOT_LOCKED,
    #[doc = "The watchdog time-out value (TC) can be changed only after the counter is below the value of WDWARNINT and WDWINDOW."]
    LOCKED,
}
impl WDPROTECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WDPROTECTW::NOT_LOCKED => false,
            WDPROTECTW::LOCKED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WDPROTECTW<'a> {
    w: &'a mut W,
}
impl<'a> _WDPROTECTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDPROTECTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The watchdog time-out value (TC) can be changed at any time."]
    #[inline(always)]
    pub fn not_locked(self) -> &'a mut W {
        self.variant(WDPROTECTW::NOT_LOCKED)
    }
    #[doc = "The watchdog time-out value (TC) can be changed only after the counter is below the value of WDWARNINT and WDWINDOW."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(WDPROTECTW::LOCKED)
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
pub type LOCK_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKW<'a> {
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
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Watchdog enable bit. Once this bit has been written with a 1 it cannot be rewritten with a 0."]
    #[inline(always)]
    pub fn wden(&self) -> WDEN_R {
        WDEN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Watchdog reset enable bit. Once this bit has been written with a 1 it cannot be rewritten with a 0."]
    #[inline(always)]
    pub fn wdreset(&self) -> WDRESET_R {
        WDRESET_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT. Cleared by software. Causes a chip reset if WDRESET = 1."]
    #[inline(always)]
    pub fn wdtof(&self) -> WDTOF_R {
        WDTOF_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Warning interrupt flag. Set when the timer reaches the value in WDWARNINT. Cleared by software."]
    #[inline(always)]
    pub fn wdint(&self) -> WDINT_R {
        WDINT_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Watchdog update mode. This bit can be set once by software and is only cleared by a reset."]
    #[inline(always)]
    pub fn wdprotect(&self) -> WDPROTECT_R {
        WDPROTECT_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - A 1 in this bit prevents disabling or powering down the clock source selected by bit 0 of the WDCLKSRC register and also prevents switching to a clock source that is disabled or powered down. This bit can be set once by software and is only cleared by any reset. If this bit is one and the WWDT clock source is the IRC when Deep-sleep or Power-down modes are entered, the IRC remains running thereby increasing power consumption in Deep-sleep mode and potentially preventing the part of entering Power-down mode correctly (see Section 15.7)."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Watchdog enable bit. Once this bit has been written with a 1 it cannot be rewritten with a 0."]
    #[inline(always)]
    pub fn wden(&mut self) -> _WDENW {
        _WDENW { w: self }
    }
    #[doc = "Bit 1 - Watchdog reset enable bit. Once this bit has been written with a 1 it cannot be rewritten with a 0."]
    #[inline(always)]
    pub fn wdreset(&mut self) -> _WDRESETW {
        _WDRESETW { w: self }
    }
    #[doc = "Bit 2 - Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT. Cleared by software. Causes a chip reset if WDRESET = 1."]
    #[inline(always)]
    pub fn wdtof(&mut self) -> _WDTOFW {
        _WDTOFW { w: self }
    }
    #[doc = "Bit 3 - Warning interrupt flag. Set when the timer reaches the value in WDWARNINT. Cleared by software."]
    #[inline(always)]
    pub fn wdint(&mut self) -> _WDINTW {
        _WDINTW { w: self }
    }
    #[doc = "Bit 4 - Watchdog update mode. This bit can be set once by software and is only cleared by a reset."]
    #[inline(always)]
    pub fn wdprotect(&mut self) -> _WDPROTECTW {
        _WDPROTECTW { w: self }
    }
    #[doc = "Bit 5 - A 1 in this bit prevents disabling or powering down the clock source selected by bit 0 of the WDCLKSRC register and also prevents switching to a clock source that is disabled or powered down. This bit can be set once by software and is only cleared by any reset. If this bit is one and the WWDT clock source is the IRC when Deep-sleep or Power-down modes are entered, the IRC remains running thereby increasing power consumption in Deep-sleep mode and potentially preventing the part of entering Power-down mode correctly (see Section 15.7)."]
    #[inline(always)]
    pub fn lock(&mut self) -> _LOCKW {
        _LOCKW { w: self }
    }
}
