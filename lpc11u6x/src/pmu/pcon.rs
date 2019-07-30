#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCON {
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
}
impl crate::ToBits<u8> for PMR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PMR::DEFAULT => 0,
            PMR::DEEP_SLEEP => 1,
            PMR::POWER_DOWN => 2,
            PMR::DEEP_POWER_DOWN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PM_R = crate::FR<u8, PMR>;
impl PM_R {
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == PMR::DEFAULT
    }
    #[doc = "Checks if the value of the field is `DEEP_SLEEP`"]
    #[inline(always)]
    pub fn is_deep_sleep(&self) -> bool {
        *self == PMR::DEEP_SLEEP
    }
    #[doc = "Checks if the value of the field is `POWER_DOWN`"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == PMR::POWER_DOWN
    }
    #[doc = "Checks if the value of the field is `DEEP_POWER_DOWN`"]
    #[inline(always)]
    pub fn is_deep_power_down(&self) -> bool {
        *self == PMR::DEEP_POWER_DOWN
    }
}
#[doc = "Values that can be written to the field `PM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PMW::DEFAULT => 0,
            PMW::DEEP_SLEEP => 1,
            PMW::POWER_DOWN => 2,
            PMW::DEEP_POWER_DOWN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PMW<'a> {
    w: &'a mut W,
}
impl<'a> _PMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Default. The part is in active or sleep mode."]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(PMW::DEFAULT)
    }
    #[doc = "Deep-sleep. ARM WFI will enter Deep-sleep mode."]
    #[inline(always)]
    pub fn deep_sleep(self) -> &'a mut W {
        self.variant(PMW::DEEP_SLEEP)
    }
    #[doc = "Power-down. ARM WFI will enter Power-down mode."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(PMW::POWER_DOWN)
    }
    #[doc = "Deep power-down. ARM WFI will enter Deep-power down mode (ARM Cortex-M0+ core powered-down)."]
    #[inline(always)]
    pub fn deep_power_down(self) -> &'a mut W {
        self.variant(PMW::DEEP_POWER_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type NODPD_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _NODPDW<'a> {
    w: &'a mut W,
}
impl<'a> _NODPDW<'a> {
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
#[doc = "Possible values of the field `SLEEPFLAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPFLAGR {
    #[doc = "Active mode. Read: No power-down mode entered. Part is in Active mode. Write: No effect."]
    ACTIVE_MODE,
    #[doc = "Low power mode. Read: Sleep/Deep-sleep or Deep power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0."]
    LOW_POWER_MODE,
}
impl crate::ToBits<bool> for SLEEPFLAGR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SLEEPFLAGR::ACTIVE_MODE => false,
            SLEEPFLAGR::LOW_POWER_MODE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SLEEPFLAG_R = crate::FR<bool, SLEEPFLAGR>;
impl SLEEPFLAG_R {
    #[doc = "Checks if the value of the field is `ACTIVE_MODE`"]
    #[inline(always)]
    pub fn is_active_mode(&self) -> bool {
        *self == SLEEPFLAGR::ACTIVE_MODE
    }
    #[doc = "Checks if the value of the field is `LOW_POWER_MODE`"]
    #[inline(always)]
    pub fn is_low_power_mode(&self) -> bool {
        *self == SLEEPFLAGR::LOW_POWER_MODE
    }
}
#[doc = "Values that can be written to the field `SLEEPFLAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPFLAGW {
    #[doc = "Active mode. Read: No power-down mode entered. Part is in Active mode. Write: No effect."]
    ACTIVE_MODE,
    #[doc = "Low power mode. Read: Sleep/Deep-sleep or Deep power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0."]
    LOW_POWER_MODE,
}
impl SLEEPFLAGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SLEEPFLAGW::ACTIVE_MODE => false,
            SLEEPFLAGW::LOW_POWER_MODE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SLEEPFLAGW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEEPFLAGW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEPFLAGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active mode. Read: No power-down mode entered. Part is in Active mode. Write: No effect."]
    #[inline(always)]
    pub fn active_mode(self) -> &'a mut W {
        self.variant(SLEEPFLAGW::ACTIVE_MODE)
    }
    #[doc = "Low power mode. Read: Sleep/Deep-sleep or Deep power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0."]
    #[inline(always)]
    pub fn low_power_mode(self) -> &'a mut W {
        self.variant(SLEEPFLAGW::LOW_POWER_MODE)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
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
impl crate::ToBits<bool> for DPDFLAGR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DPDFLAGR::NOT_DEEP_POWER_DOWN => false,
            DPDFLAGR::DEEP_POWER_DOWN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DPDFLAG_R = crate::FR<bool, DPDFLAGR>;
impl DPDFLAG_R {
    #[doc = "Checks if the value of the field is `NOT_DEEP_POWER_DOWN`"]
    #[inline(always)]
    pub fn is_not_deep_power_down(&self) -> bool {
        *self == DPDFLAGR::NOT_DEEP_POWER_DOWN
    }
    #[doc = "Checks if the value of the field is `DEEP_POWER_DOWN`"]
    #[inline(always)]
    pub fn is_deep_power_down(&self) -> bool {
        *self == DPDFLAGR::DEEP_POWER_DOWN
    }
}
#[doc = "Values that can be written to the field `DPDFLAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPDFLAGW {
    #[doc = "Not Deep power-down. Read: Deep power-down mode not entered. Write: No effect."]
    NOT_DEEP_POWER_DOWN,
    #[doc = "Deep power-down. Read: Deep power-down mode entered. Write: Clear the Deep power-down flag."]
    DEEP_POWER_DOWN,
}
impl DPDFLAGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DPDFLAGW::NOT_DEEP_POWER_DOWN => false,
            DPDFLAGW::DEEP_POWER_DOWN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DPDFLAGW<'a> {
    w: &'a mut W,
}
impl<'a> _DPDFLAGW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPDFLAGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not Deep power-down. Read: Deep power-down mode not entered. Write: No effect."]
    #[inline(always)]
    pub fn not_deep_power_down(self) -> &'a mut W {
        self.variant(DPDFLAGW::NOT_DEEP_POWER_DOWN)
    }
    #[doc = "Deep power-down. Read: Deep power-down mode entered. Write: Clear the Deep power-down flag."]
    #[inline(always)]
    pub fn deep_power_down(self) -> &'a mut W {
        self.variant(DPDFLAGW::DEEP_POWER_DOWN)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Power mode"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new((self.bits() & 0x07) as u8)
    }
    #[doc = "Bit 3 - A 1 in this bit prevents entry to Deep power-down mode when 0x3 is written to the PM field above, the SLEEPDEEP bit is set, and a WFI is executed. This bit is cleared only by power-on reset, so writing a one to this bit locks the part in a mode in which Deep power-down mode is blocked."]
    #[inline(always)]
    pub fn nodpd(&self) -> NODPD_R {
        NODPD_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Sleep mode flag"]
    #[inline(always)]
    pub fn sleepflag(&self) -> SLEEPFLAG_R {
        SLEEPFLAG_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Deep power-down flag"]
    #[inline(always)]
    pub fn dpdflag(&self) -> DPDFLAG_R {
        DPDFLAG_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Power mode"]
    #[inline(always)]
    pub fn pm(&mut self) -> _PMW {
        _PMW { w: self }
    }
    #[doc = "Bit 3 - A 1 in this bit prevents entry to Deep power-down mode when 0x3 is written to the PM field above, the SLEEPDEEP bit is set, and a WFI is executed. This bit is cleared only by power-on reset, so writing a one to this bit locks the part in a mode in which Deep power-down mode is blocked."]
    #[inline(always)]
    pub fn nodpd(&mut self) -> _NODPDW {
        _NODPDW { w: self }
    }
    #[doc = "Bit 8 - Sleep mode flag"]
    #[inline(always)]
    pub fn sleepflag(&mut self) -> _SLEEPFLAGW {
        _SLEEPFLAGW { w: self }
    }
    #[doc = "Bit 11 - Deep power-down flag"]
    #[inline(always)]
    pub fn dpdflag(&mut self) -> _DPDFLAGW {
        _DPDFLAGW { w: self }
    }
}
