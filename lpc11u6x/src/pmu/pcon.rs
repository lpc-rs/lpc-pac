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
    #[doc = "Deep-sleep. ARM WFI will enter Deep-sleep mode."]
    DEEP_SLEEP,
    #[doc = "Power-down. ARM WFI will enter Power-down mode."]
    POWER_DOWN,
    #[doc = "Deep power-down. ARM WFI will enter Deep-power down mode (ARM Cortex-M0+ core powered-down)."]
    DEEP_POWER_DOWN,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PMR::DEFAULT => 0,
            PMR::DEEP_SLEEP => 1,
            PMR::POWER_DOWN => 2,
            PMR::DEEP_POWER_DOWN => 3,
            PMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PMR {
        match value {
            0 => PMR::DEFAULT,
            1 => PMR::DEEP_SLEEP,
            2 => PMR::POWER_DOWN,
            3 => PMR::DEEP_POWER_DOWN,
            i => PMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline]
    pub fn is_default(&self) -> bool {
        *self == PMR::DEFAULT
    }
    #[doc = "Checks if the value of the field is `DEEP_SLEEP`"]
    #[inline]
    pub fn is_deep_sleep(&self) -> bool {
        *self == PMR::DEEP_SLEEP
    }
    #[doc = "Checks if the value of the field is `POWER_DOWN`"]
    #[inline]
    pub fn is_power_down(&self) -> bool {
        *self == PMR::POWER_DOWN
    }
    #[doc = "Checks if the value of the field is `DEEP_POWER_DOWN`"]
    #[inline]
    pub fn is_deep_power_down(&self) -> bool {
        *self == PMR::DEEP_POWER_DOWN
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
    #[doc = "Active mode. Read: No power-down mode entered. Part is in Active mode. Write: No effect."]
    ACTIVE_MODE,
    #[doc = "Low power mode. Read: Sleep/Deep-sleep or Deep power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0."]
    LOW_POWER_MODE,
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
            SLEEPFLAGR::ACTIVE_MODE => false,
            SLEEPFLAGR::LOW_POWER_MODE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLEEPFLAGR {
        match value {
            false => SLEEPFLAGR::ACTIVE_MODE,
            true => SLEEPFLAGR::LOW_POWER_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_MODE`"]
    #[inline]
    pub fn is_active_mode(&self) -> bool {
        *self == SLEEPFLAGR::ACTIVE_MODE
    }
    #[doc = "Checks if the value of the field is `LOW_POWER_MODE`"]
    #[inline]
    pub fn is_low_power_mode(&self) -> bool {
        *self == SLEEPFLAGR::LOW_POWER_MODE
    }
}
#[doc = "Possible values of the field `DPDFLAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPDFLAGR {
    #[doc = "Not Deep power-down. Read: Deep power-down mode not entered. Write: No effect."]
    NOT_DEEP_POWER_DOWN,
    #[doc = "Deep power-down. Read: Deep power-down mode entered. Write: Clear the Deep power-down flag."]
    DEEP_POWER_DOWN,
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
            DPDFLAGR::NOT_DEEP_POWER_DOWN => false,
            DPDFLAGR::DEEP_POWER_DOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPDFLAGR {
        match value {
            false => DPDFLAGR::NOT_DEEP_POWER_DOWN,
            true => DPDFLAGR::DEEP_POWER_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DEEP_POWER_DOWN`"]
    #[inline]
    pub fn is_not_deep_power_down(&self) -> bool {
        *self == DPDFLAGR::NOT_DEEP_POWER_DOWN
    }
    #[doc = "Checks if the value of the field is `DEEP_POWER_DOWN`"]
    #[inline]
    pub fn is_deep_power_down(&self) -> bool {
        *self == DPDFLAGR::DEEP_POWER_DOWN
    }
}
#[doc = "Values that can be written to the field `PM`"]
pub enum PMW {
    #[doc = "Default. The part is in active or sleep mode."]
    DEFAULT,
    #[doc = "Deep-sleep. ARM WFI will enter Deep-sleep mode."]
    DEEP_SLEEP,
    #[doc = "Power-down. ARM WFI will enter Power-down mode."]
    POWER_DOWN,
    #[doc = "Deep power-down. ARM WFI will enter Deep-power down mode (ARM Cortex-M0+ core powered-down)."]
    DEEP_POWER_DOWN,
}
impl PMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PMW::DEFAULT => 0,
            PMW::DEEP_SLEEP => 1,
            PMW::POWER_DOWN => 2,
            PMW::DEEP_POWER_DOWN => 3,
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
    #[doc = "Deep-sleep. ARM WFI will enter Deep-sleep mode."]
    #[inline]
    pub fn deep_sleep(self) -> &'a mut W {
        self.variant(PMW::DEEP_SLEEP)
    }
    #[doc = "Power-down. ARM WFI will enter Power-down mode."]
    #[inline]
    pub fn power_down(self) -> &'a mut W {
        self.variant(PMW::POWER_DOWN)
    }
    #[doc = "Deep power-down. ARM WFI will enter Deep-power down mode (ARM Cortex-M0+ core powered-down)."]
    #[inline]
    pub fn deep_power_down(self) -> &'a mut W {
        self.variant(PMW::DEEP_POWER_DOWN)
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
    #[doc = "Active mode. Read: No power-down mode entered. Part is in Active mode. Write: No effect."]
    ACTIVE_MODE,
    #[doc = "Low power mode. Read: Sleep/Deep-sleep or Deep power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0."]
    LOW_POWER_MODE,
}
impl SLEEPFLAGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLEEPFLAGW::ACTIVE_MODE => false,
            SLEEPFLAGW::LOW_POWER_MODE => true,
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
    #[doc = "Active mode. Read: No power-down mode entered. Part is in Active mode. Write: No effect."]
    #[inline]
    pub fn active_mode(self) -> &'a mut W {
        self.variant(SLEEPFLAGW::ACTIVE_MODE)
    }
    #[doc = "Low power mode. Read: Sleep/Deep-sleep or Deep power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0."]
    #[inline]
    pub fn low_power_mode(self) -> &'a mut W {
        self.variant(SLEEPFLAGW::LOW_POWER_MODE)
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
    #[doc = "Not Deep power-down. Read: Deep power-down mode not entered. Write: No effect."]
    NOT_DEEP_POWER_DOWN,
    #[doc = "Deep power-down. Read: Deep power-down mode entered. Write: Clear the Deep power-down flag."]
    DEEP_POWER_DOWN,
}
impl DPDFLAGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DPDFLAGW::NOT_DEEP_POWER_DOWN => false,
            DPDFLAGW::DEEP_POWER_DOWN => true,
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
    #[doc = "Not Deep power-down. Read: Deep power-down mode not entered. Write: No effect."]
    #[inline]
    pub fn not_deep_power_down(self) -> &'a mut W {
        self.variant(DPDFLAGW::NOT_DEEP_POWER_DOWN)
    }
    #[doc = "Deep power-down. Read: Deep power-down mode entered. Write: Clear the Deep power-down flag."]
    #[inline]
    pub fn deep_power_down(self) -> &'a mut W {
        self.variant(DPDFLAGW::DEEP_POWER_DOWN)
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
    #[doc = "Bit 3 - A 1 in this bit prevents entry to Deep power-down mode when 0x3 is written to the PM field above, the SLEEPDEEP bit is set, and a WFI is executed. This bit is cleared only by power-on reset, so writing a one to this bit locks the part in a mode in which Deep power-down mode is blocked."]
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
    #[doc = "Bit 3 - A 1 in this bit prevents entry to Deep power-down mode when 0x3 is written to the PM field above, the SLEEPDEEP bit is set, and a WFI is executed. This bit is cleared only by power-on reset, so writing a one to this bit locks the part in a mode in which Deep power-down mode is blocked."]
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
