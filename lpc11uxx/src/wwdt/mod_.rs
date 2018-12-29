#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MOD {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
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
impl WDENR {
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
            WDENR::STOPPED => false,
            WDENR::RUNNING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDENR {
        match value {
            false => WDENR::STOPPED,
            true => WDENR::RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `STOPPED`"]
    #[inline]
    pub fn is_stopped(&self) -> bool {
        *self == WDENR::STOPPED
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline]
    pub fn is_running(&self) -> bool {
        *self == WDENR::RUNNING
    }
}
#[doc = "Possible values of the field `WDRESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDRESETR {
    #[doc = "A watchdog timeout will not cause a chip reset."]
    INTERRUPT,
    #[doc = "A watchdog timeout will cause a chip reset."]
    RESET,
}
impl WDRESETR {
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
            WDRESETR::INTERRUPT => false,
            WDRESETR::RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDRESETR {
        match value {
            false => WDRESETR::INTERRUPT,
            true => WDRESETR::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        *self == WDRESETR::INTERRUPT
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == WDRESETR::RESET
    }
}
#[doc = r" Value of the field"]
pub struct WDTOFR {
    bits: bool,
}
impl WDTOFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct WDINTR {
    bits: bool,
}
impl WDINTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `WDPROTECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDPROTECTR {
    #[doc = "The watchdog time-out value (TC) can be changed at any time."]
    NOT_LOCKED,
    #[doc = "The watchdog time-out value (TC) can be changed only after the counter is below the value of WDWARNINT and WDWINDOW."]
    LOCKED,
}
impl WDPROTECTR {
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
            WDPROTECTR::NOT_LOCKED => false,
            WDPROTECTR::LOCKED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDPROTECTR {
        match value {
            false => WDPROTECTR::NOT_LOCKED,
            true => WDPROTECTR::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_LOCKED`"]
    #[inline]
    pub fn is_not_locked(&self) -> bool {
        *self == WDPROTECTR::NOT_LOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline]
    pub fn is_locked(&self) -> bool {
        *self == WDPROTECTR::LOCKED
    }
}
#[doc = r" Value of the field"]
pub struct LOCKR {
    bits: bool,
}
impl LOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Values that can be written to the field `WDEN`"]
pub enum WDENW {
    #[doc = "The watchdog timer is stopped."]
    STOPPED,
    #[doc = "The watchdog timer is running."]
    RUNNING,
}
impl WDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDENW::STOPPED => false,
            WDENW::RUNNING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDENW<'a> {
    w: &'a mut W,
}
impl<'a> _WDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The watchdog timer is stopped."]
    #[inline]
    pub fn stopped(self) -> &'a mut W {
        self.variant(WDENW::STOPPED)
    }
    #[doc = "The watchdog timer is running."]
    #[inline]
    pub fn running(self) -> &'a mut W {
        self.variant(WDENW::RUNNING)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WDRESET`"]
pub enum WDRESETW {
    #[doc = "A watchdog timeout will not cause a chip reset."]
    INTERRUPT,
    #[doc = "A watchdog timeout will cause a chip reset."]
    RESET,
}
impl WDRESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDRESETW::INTERRUPT => false,
            WDRESETW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _WDRESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDRESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A watchdog timeout will not cause a chip reset."]
    #[inline]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(WDRESETW::INTERRUPT)
    }
    #[doc = "A watchdog timeout will cause a chip reset."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(WDRESETW::RESET)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WDTOFW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTOFW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WDINTW<'a> {
    w: &'a mut W,
}
impl<'a> _WDINTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WDPROTECT`"]
pub enum WDPROTECTW {
    #[doc = "The watchdog time-out value (TC) can be changed at any time."]
    NOT_LOCKED,
    #[doc = "The watchdog time-out value (TC) can be changed only after the counter is below the value of WDWARNINT and WDWINDOW."]
    LOCKED,
}
impl WDPROTECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDPROTECTW::NOT_LOCKED => false,
            WDPROTECTW::LOCKED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDPROTECTW<'a> {
    w: &'a mut W,
}
impl<'a> _WDPROTECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDPROTECTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The watchdog time-out value (TC) can be changed at any time."]
    #[inline]
    pub fn not_locked(self) -> &'a mut W {
        self.variant(WDPROTECTW::NOT_LOCKED)
    }
    #[doc = "The watchdog time-out value (TC) can be changed only after the counter is below the value of WDWARNINT and WDWINDOW."]
    #[inline]
    pub fn locked(self) -> &'a mut W {
        self.variant(WDPROTECTW::LOCKED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Watchdog enable bit. Once this bit has been written with a 1 it cannot be rewritten with a 0."]
    #[inline]
    pub fn wden(&self) -> WDENR {
        WDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Watchdog reset enable bit. Once this bit has been written with a 1 it cannot be rewritten with a 0."]
    #[inline]
    pub fn wdreset(&self) -> WDRESETR {
        WDRESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT. Cleared by software. Causes a chip reset if WDRESET = 1."]
    #[inline]
    pub fn wdtof(&self) -> WDTOFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WDTOFR { bits }
    }
    #[doc = "Bit 3 - Warning interrupt flag. Set when the timer reaches the value in WDWARNINT. Cleared by software."]
    #[inline]
    pub fn wdint(&self) -> WDINTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WDINTR { bits }
    }
    #[doc = "Bit 4 - Watchdog update mode. This bit can be set once by software and is only cleared by a reset."]
    #[inline]
    pub fn wdprotect(&self) -> WDPROTECTR {
        WDPROTECTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - A 1 in this bit prevents disabling or powering down the clock source selected by bit 0 of the WDCLKSRC register and also prevents switching to a clock source that is disabled or powered down. This bit can be set once by software and is only cleared by any reset. If this bit is one and the WWDT clock source is the IRC when Deep-sleep or Power-down modes are entered, the IRC remains running thereby increasing power consumption in Deep-sleep mode and potentially preventing the part of entering Power-down mode correctly (see Section 15.7)."]
    #[inline]
    pub fn lock(&self) -> LOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LOCKR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Watchdog enable bit. Once this bit has been written with a 1 it cannot be rewritten with a 0."]
    #[inline]
    pub fn wden(&mut self) -> _WDENW {
        _WDENW { w: self }
    }
    #[doc = "Bit 1 - Watchdog reset enable bit. Once this bit has been written with a 1 it cannot be rewritten with a 0."]
    #[inline]
    pub fn wdreset(&mut self) -> _WDRESETW {
        _WDRESETW { w: self }
    }
    #[doc = "Bit 2 - Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT. Cleared by software. Causes a chip reset if WDRESET = 1."]
    #[inline]
    pub fn wdtof(&mut self) -> _WDTOFW {
        _WDTOFW { w: self }
    }
    #[doc = "Bit 3 - Warning interrupt flag. Set when the timer reaches the value in WDWARNINT. Cleared by software."]
    #[inline]
    pub fn wdint(&mut self) -> _WDINTW {
        _WDINTW { w: self }
    }
    #[doc = "Bit 4 - Watchdog update mode. This bit can be set once by software and is only cleared by a reset."]
    #[inline]
    pub fn wdprotect(&mut self) -> _WDPROTECTW {
        _WDPROTECTW { w: self }
    }
    #[doc = "Bit 5 - A 1 in this bit prevents disabling or powering down the clock source selected by bit 0 of the WDCLKSRC register and also prevents switching to a clock source that is disabled or powered down. This bit can be set once by software and is only cleared by any reset. If this bit is one and the WWDT clock source is the IRC when Deep-sleep or Power-down modes are entered, the IRC remains running thereby increasing power consumption in Deep-sleep mode and potentially preventing the part of entering Power-down mode correctly (see Section 15.7)."]
    #[inline]
    pub fn lock(&mut self) -> _LOCKW {
        _LOCKW { w: self }
    }
}
