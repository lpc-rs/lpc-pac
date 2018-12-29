#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCON {
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
#[doc = "Possible values of the field `PM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMR {
    #[doc = "Default. The part is in active or sleep mode."]
    DEFAULT,
    #[doc = "ARM WFI will enter Deep-sleep mode."]
    DEEPSLEEP,
    #[doc = "ARM WFI will enter Power-down mode."]
    POWERDOWN,
    #[doc = "ARM WFI will enter Deep-power down mode (ARM Cortex-M0 core powered-down)."]
    DEEPPOWERDOWN,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PMR::DEFAULT => 0,
            PMR::DEEPSLEEP => 1,
            PMR::POWERDOWN => 2,
            PMR::DEEPPOWERDOWN => 3,
            PMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PMR {
        match value {
            0 => PMR::DEFAULT,
            1 => PMR::DEEPSLEEP,
            2 => PMR::POWERDOWN,
            3 => PMR::DEEPPOWERDOWN,
            i => PMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline]
    pub fn is_default(&self) -> bool {
        *self == PMR::DEFAULT
    }
    #[doc = "Checks if the value of the field is `DEEPSLEEP`"]
    #[inline]
    pub fn is_deepsleep(&self) -> bool {
        *self == PMR::DEEPSLEEP
    }
    #[doc = "Checks if the value of the field is `POWERDOWN`"]
    #[inline]
    pub fn is_powerdown(&self) -> bool {
        *self == PMR::POWERDOWN
    }
    #[doc = "Checks if the value of the field is `DEEPPOWERDOWN`"]
    #[inline]
    pub fn is_deeppowerdown(&self) -> bool {
        *self == PMR::DEEPPOWERDOWN
    }
}
#[doc = r" Value of the field"]
pub struct NODPDR {
    bits: bool,
}
impl NODPDR {
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
#[doc = "Possible values of the field `SLEEPFLAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPFLAGR {
    #[doc = "Read: No power-down mode entered. LPC11U1x is in Active mode. Write: No effect."]
    NOPOWERDOWN,
    #[doc = "Read: Sleep/Deep-sleep or Deep power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0."]
    POWERDOWN,
}
impl SLEEPFLAGR {
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
            SLEEPFLAGR::NOPOWERDOWN => false,
            SLEEPFLAGR::POWERDOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLEEPFLAGR {
        match value {
            false => SLEEPFLAGR::NOPOWERDOWN,
            true => SLEEPFLAGR::POWERDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NOPOWERDOWN`"]
    #[inline]
    pub fn is_nopowerdown(&self) -> bool {
        *self == SLEEPFLAGR::NOPOWERDOWN
    }
    #[doc = "Checks if the value of the field is `POWERDOWN`"]
    #[inline]
    pub fn is_powerdown(&self) -> bool {
        *self == SLEEPFLAGR::POWERDOWN
    }
}
#[doc = "Possible values of the field `DPDFLAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPDFLAGR {
    #[doc = "Read: Deep power-down mode  not entered. Write: No effect."]
    DPNOTENTERED,
    #[doc = "Read: Deep power-down mode entered. Write: Clear the Deep power-down flag."]
    DPENTERED,
}
impl DPDFLAGR {
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
            DPDFLAGR::DPNOTENTERED => false,
            DPDFLAGR::DPENTERED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPDFLAGR {
        match value {
            false => DPDFLAGR::DPNOTENTERED,
            true => DPDFLAGR::DPENTERED,
        }
    }
    #[doc = "Checks if the value of the field is `DPNOTENTERED`"]
    #[inline]
    pub fn is_dpnotentered(&self) -> bool {
        *self == DPDFLAGR::DPNOTENTERED
    }
    #[doc = "Checks if the value of the field is `DPENTERED`"]
    #[inline]
    pub fn is_dpentered(&self) -> bool {
        *self == DPDFLAGR::DPENTERED
    }
}
#[doc = "Values that can be written to the field `PM`"]
pub enum PMW {
    #[doc = "Default. The part is in active or sleep mode."]
    DEFAULT,
    #[doc = "ARM WFI will enter Deep-sleep mode."]
    DEEPSLEEP,
    #[doc = "ARM WFI will enter Power-down mode."]
    POWERDOWN,
    #[doc = "ARM WFI will enter Deep-power down mode (ARM Cortex-M0 core powered-down)."]
    DEEPPOWERDOWN,
}
impl PMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PMW::DEFAULT => 0,
            PMW::DEEPSLEEP => 1,
            PMW::POWERDOWN => 2,
            PMW::DEEPPOWERDOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PMW<'a> {
    w: &'a mut W,
}
impl<'a> _PMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Default. The part is in active or sleep mode."]
    #[inline]
    pub fn default(self) -> &'a mut W {
        self.variant(PMW::DEFAULT)
    }
    #[doc = "ARM WFI will enter Deep-sleep mode."]
    #[inline]
    pub fn deepsleep(self) -> &'a mut W {
        self.variant(PMW::DEEPSLEEP)
    }
    #[doc = "ARM WFI will enter Power-down mode."]
    #[inline]
    pub fn powerdown(self) -> &'a mut W {
        self.variant(PMW::POWERDOWN)
    }
    #[doc = "ARM WFI will enter Deep-power down mode (ARM Cortex-M0 core powered-down)."]
    #[inline]
    pub fn deeppowerdown(self) -> &'a mut W {
        self.variant(PMW::DEEPPOWERDOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NODPDW<'a> {
    w: &'a mut W,
}
impl<'a> _NODPDW<'a> {
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
#[doc = "Values that can be written to the field `SLEEPFLAG`"]
pub enum SLEEPFLAGW {
    #[doc = "Read: No power-down mode entered. LPC11U1x is in Active mode. Write: No effect."]
    NOPOWERDOWN,
    #[doc = "Read: Sleep/Deep-sleep or Deep power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0."]
    POWERDOWN,
}
impl SLEEPFLAGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLEEPFLAGW::NOPOWERDOWN => false,
            SLEEPFLAGW::POWERDOWN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLEEPFLAGW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEEPFLAGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLEEPFLAGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read: No power-down mode entered. LPC11U1x is in Active mode. Write: No effect."]
    #[inline]
    pub fn nopowerdown(self) -> &'a mut W {
        self.variant(SLEEPFLAGW::NOPOWERDOWN)
    }
    #[doc = "Read: Sleep/Deep-sleep or Deep power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0."]
    #[inline]
    pub fn powerdown(self) -> &'a mut W {
        self.variant(SLEEPFLAGW::POWERDOWN)
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
#[doc = "Values that can be written to the field `DPDFLAG`"]
pub enum DPDFLAGW {
    #[doc = "Read: Deep power-down mode  not entered. Write: No effect."]
    DPNOTENTERED,
    #[doc = "Read: Deep power-down mode entered. Write: Clear the Deep power-down flag."]
    DPENTERED,
}
impl DPDFLAGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DPDFLAGW::DPNOTENTERED => false,
            DPDFLAGW::DPENTERED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DPDFLAGW<'a> {
    w: &'a mut W,
}
impl<'a> _DPDFLAGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DPDFLAGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read: Deep power-down mode not entered. Write: No effect."]
    #[inline]
    pub fn dpnotentered(self) -> &'a mut W {
        self.variant(DPDFLAGW::DPNOTENTERED)
    }
    #[doc = "Read: Deep power-down mode entered. Write: Clear the Deep power-down flag."]
    #[inline]
    pub fn dpentered(self) -> &'a mut W {
        self.variant(DPDFLAGW::DPENTERED)
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
        const OFFSET: u8 = 11;
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
    #[doc = "Bits 0:2 - Power mode"]
    #[inline]
    pub fn pm(&self) -> PMR {
        PMR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - A 1 in this bit prevents entry to Deep power-down mode when 0x3 is written to the PM field above, the SLEEPDEEP bit is set, and a WFI is executed. Execution continues after the WFI if this bit is 1. This bit is cleared only by power-on reset, so writing a one to this bit locks the part in a mode in which Deep power-down mode is blocked."]
    #[inline]
    pub fn nodpd(&self) -> NODPDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NODPDR { bits }
    }
    #[doc = "Bit 8 - Sleep mode flag"]
    #[inline]
    pub fn sleepflag(&self) -> SLEEPFLAGR {
        SLEEPFLAGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Deep power-down flag"]
    #[inline]
    pub fn dpdflag(&self) -> DPDFLAGR {
        DPDFLAGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
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
    #[doc = "Bits 0:2 - Power mode"]
    #[inline]
    pub fn pm(&mut self) -> _PMW {
        _PMW { w: self }
    }
    #[doc = "Bit 3 - A 1 in this bit prevents entry to Deep power-down mode when 0x3 is written to the PM field above, the SLEEPDEEP bit is set, and a WFI is executed. Execution continues after the WFI if this bit is 1. This bit is cleared only by power-on reset, so writing a one to this bit locks the part in a mode in which Deep power-down mode is blocked."]
    #[inline]
    pub fn nodpd(&mut self) -> _NODPDW {
        _NODPDW { w: self }
    }
    #[doc = "Bit 8 - Sleep mode flag"]
    #[inline]
    pub fn sleepflag(&mut self) -> _SLEEPFLAGW {
        _SLEEPFLAGW { w: self }
    }
    #[doc = "Bit 11 - Deep power-down flag"]
    #[inline]
    pub fn dpdflag(&mut self) -> _DPDFLAGW {
        _DPDFLAGW { w: self }
    }
}
