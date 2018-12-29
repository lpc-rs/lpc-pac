#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ACR {
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
#[doc = "Possible values of the field `START`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTR {
    #[doc = "Auto-baud stop (auto-baud is not running)."]
    AUTO_BAUD_STOP_AUTO,
    #[doc = "Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion."]
    AUTO_BAUD_START_AUT,
}
impl STARTR {
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
            STARTR::AUTO_BAUD_STOP_AUTO => false,
            STARTR::AUTO_BAUD_START_AUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STARTR {
        match value {
            false => STARTR::AUTO_BAUD_STOP_AUTO,
            true => STARTR::AUTO_BAUD_START_AUT,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO_BAUD_STOP_AUTO`"]
    #[inline]
    pub fn is_auto_baud_stop_auto(&self) -> bool {
        *self == STARTR::AUTO_BAUD_STOP_AUTO
    }
    #[doc = "Checks if the value of the field is `AUTO_BAUD_START_AUT`"]
    #[inline]
    pub fn is_auto_baud_start_aut(&self) -> bool {
        *self == STARTR::AUTO_BAUD_START_AUT
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Mode 0."]
    MODE_0_,
    #[doc = "Mode 1."]
    MODE_1_,
}
impl MODER {
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
            MODER::MODE_0_ => false,
            MODER::MODE_1_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MODER {
        match value {
            false => MODER::MODE_0_,
            true => MODER::MODE_1_,
        }
    }
    #[doc = "Checks if the value of the field is `MODE_0_`"]
    #[inline]
    pub fn is_mode_0_(&self) -> bool {
        *self == MODER::MODE_0_
    }
    #[doc = "Checks if the value of the field is `MODE_1_`"]
    #[inline]
    pub fn is_mode_1_(&self) -> bool {
        *self == MODER::MODE_1_
    }
}
#[doc = "Possible values of the field `AUTORESTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTORESTARTR {
    #[doc = "No restart."]
    NO_RESTART_,
    #[doc = "Restart in case of time-out (counter restarts at next UARTn Rx falling edge)"]
    RESTART_IN_CASE_OF_T,
}
impl AUTORESTARTR {
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
            AUTORESTARTR::NO_RESTART_ => false,
            AUTORESTARTR::RESTART_IN_CASE_OF_T => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUTORESTARTR {
        match value {
            false => AUTORESTARTR::NO_RESTART_,
            true => AUTORESTARTR::RESTART_IN_CASE_OF_T,
        }
    }
    #[doc = "Checks if the value of the field is `NO_RESTART_`"]
    #[inline]
    pub fn is_no_restart_(&self) -> bool {
        *self == AUTORESTARTR::NO_RESTART_
    }
    #[doc = "Checks if the value of the field is `RESTART_IN_CASE_OF_T`"]
    #[inline]
    pub fn is_restart_in_case_of_t(&self) -> bool {
        *self == AUTORESTARTR::RESTART_IN_CASE_OF_T
    }
}
#[doc = "Possible values of the field `ABEOINTCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABEOINTCLRR {
    #[doc = "No impact."]
    NO_IMPACT_,
    #[doc = "Clear the corresponding interrupt in the IIR."]
    CLEAR_THE_CORRESPOND,
}
impl ABEOINTCLRR {
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
            ABEOINTCLRR::NO_IMPACT_ => false,
            ABEOINTCLRR::CLEAR_THE_CORRESPOND => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ABEOINTCLRR {
        match value {
            false => ABEOINTCLRR::NO_IMPACT_,
            true => ABEOINTCLRR::CLEAR_THE_CORRESPOND,
        }
    }
    #[doc = "Checks if the value of the field is `NO_IMPACT_`"]
    #[inline]
    pub fn is_no_impact_(&self) -> bool {
        *self == ABEOINTCLRR::NO_IMPACT_
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_CORRESPOND`"]
    #[inline]
    pub fn is_clear_the_correspond(&self) -> bool {
        *self == ABEOINTCLRR::CLEAR_THE_CORRESPOND
    }
}
#[doc = "Possible values of the field `ABTOINTCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABTOINTCLRR {
    #[doc = "No impact."]
    NO_IMPACT_,
    #[doc = "Clear the corresponding interrupt in the IIR."]
    CLEAR_THE_CORRESPOND,
}
impl ABTOINTCLRR {
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
            ABTOINTCLRR::NO_IMPACT_ => false,
            ABTOINTCLRR::CLEAR_THE_CORRESPOND => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ABTOINTCLRR {
        match value {
            false => ABTOINTCLRR::NO_IMPACT_,
            true => ABTOINTCLRR::CLEAR_THE_CORRESPOND,
        }
    }
    #[doc = "Checks if the value of the field is `NO_IMPACT_`"]
    #[inline]
    pub fn is_no_impact_(&self) -> bool {
        *self == ABTOINTCLRR::NO_IMPACT_
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_CORRESPOND`"]
    #[inline]
    pub fn is_clear_the_correspond(&self) -> bool {
        *self == ABTOINTCLRR::CLEAR_THE_CORRESPOND
    }
}
#[doc = "Values that can be written to the field `START`"]
pub enum STARTW {
    #[doc = "Auto-baud stop (auto-baud is not running)."]
    AUTO_BAUD_STOP_AUTO,
    #[doc = "Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion."]
    AUTO_BAUD_START_AUT,
}
impl STARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STARTW::AUTO_BAUD_STOP_AUTO => false,
            STARTW::AUTO_BAUD_START_AUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Auto-baud stop (auto-baud is not running)."]
    #[inline]
    pub fn auto_baud_stop_auto(self) -> &'a mut W {
        self.variant(STARTW::AUTO_BAUD_STOP_AUTO)
    }
    #[doc = "Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion."]
    #[inline]
    pub fn auto_baud_start_aut(self) -> &'a mut W {
        self.variant(STARTW::AUTO_BAUD_START_AUT)
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
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Mode 0."]
    MODE_0_,
    #[doc = "Mode 1."]
    MODE_1_,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MODEW::MODE_0_ => false,
            MODEW::MODE_1_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Mode 0."]
    #[inline]
    pub fn mode_0_(self) -> &'a mut W {
        self.variant(MODEW::MODE_0_)
    }
    #[doc = "Mode 1."]
    #[inline]
    pub fn mode_1_(self) -> &'a mut W {
        self.variant(MODEW::MODE_1_)
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
#[doc = "Values that can be written to the field `AUTORESTART`"]
pub enum AUTORESTARTW {
    #[doc = "No restart."]
    NO_RESTART_,
    #[doc = "Restart in case of time-out (counter restarts at next UARTn Rx falling edge)"]
    RESTART_IN_CASE_OF_T,
}
impl AUTORESTARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUTORESTARTW::NO_RESTART_ => false,
            AUTORESTARTW::RESTART_IN_CASE_OF_T => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUTORESTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTORESTARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUTORESTARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No restart."]
    #[inline]
    pub fn no_restart_(self) -> &'a mut W {
        self.variant(AUTORESTARTW::NO_RESTART_)
    }
    #[doc = "Restart in case of time-out (counter restarts at next UARTn Rx falling edge)"]
    #[inline]
    pub fn restart_in_case_of_t(self) -> &'a mut W {
        self.variant(AUTORESTARTW::RESTART_IN_CASE_OF_T)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ABEOINTCLR`"]
pub enum ABEOINTCLRW {
    #[doc = "No impact."]
    NO_IMPACT_,
    #[doc = "Clear the corresponding interrupt in the IIR."]
    CLEAR_THE_CORRESPOND,
}
impl ABEOINTCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ABEOINTCLRW::NO_IMPACT_ => false,
            ABEOINTCLRW::CLEAR_THE_CORRESPOND => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ABEOINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _ABEOINTCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ABEOINTCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No impact."]
    #[inline]
    pub fn no_impact_(self) -> &'a mut W {
        self.variant(ABEOINTCLRW::NO_IMPACT_)
    }
    #[doc = "Clear the corresponding interrupt in the IIR."]
    #[inline]
    pub fn clear_the_correspond(self) -> &'a mut W {
        self.variant(ABEOINTCLRW::CLEAR_THE_CORRESPOND)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ABTOINTCLR`"]
pub enum ABTOINTCLRW {
    #[doc = "No impact."]
    NO_IMPACT_,
    #[doc = "Clear the corresponding interrupt in the IIR."]
    CLEAR_THE_CORRESPOND,
}
impl ABTOINTCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ABTOINTCLRW::NO_IMPACT_ => false,
            ABTOINTCLRW::CLEAR_THE_CORRESPOND => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ABTOINTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _ABTOINTCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ABTOINTCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No impact."]
    #[inline]
    pub fn no_impact_(self) -> &'a mut W {
        self.variant(ABTOINTCLRW::NO_IMPACT_)
    }
    #[doc = "Clear the corresponding interrupt in the IIR."]
    #[inline]
    pub fn clear_the_correspond(self) -> &'a mut W {
        self.variant(ABTOINTCLRW::CLEAR_THE_CORRESPOND)
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
        const OFFSET: u8 = 9;
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
    #[doc = "Bit 0 - Start bit. This bit is automatically cleared after auto-baud completion."]
    #[inline]
    pub fn start(&self) -> STARTR {
        STARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Auto-baud mode select bit."]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Restart bit."]
    #[inline]
    pub fn autorestart(&self) -> AUTORESTARTR {
        AUTORESTARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - End of auto-baud interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact."]
    #[inline]
    pub fn abeointclr(&self) -> ABEOINTCLRR {
        ABEOINTCLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Auto-baud time-out interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact."]
    #[inline]
    pub fn abtointclr(&self) -> ABTOINTCLRR {
        ABTOINTCLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bit 0 - Start bit. This bit is automatically cleared after auto-baud completion."]
    #[inline]
    pub fn start(&mut self) -> _STARTW {
        _STARTW { w: self }
    }
    #[doc = "Bit 1 - Auto-baud mode select bit."]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 2 - Restart bit."]
    #[inline]
    pub fn autorestart(&mut self) -> _AUTORESTARTW {
        _AUTORESTARTW { w: self }
    }
    #[doc = "Bit 8 - End of auto-baud interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact."]
    #[inline]
    pub fn abeointclr(&mut self) -> _ABEOINTCLRW {
        _ABEOINTCLRW { w: self }
    }
    #[doc = "Bit 9 - Auto-baud time-out interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact."]
    #[inline]
    pub fn abtointclr(&mut self) -> _ABTOINTCLRW {
        _ABTOINTCLRW { w: self }
    }
}
