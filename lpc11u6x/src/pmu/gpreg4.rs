#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPREG4 {
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
#[doc = "Possible values of the field `WAKEUPHYS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUPHYSR {
    #[doc = "Disable Hysteresis for WAKUP pin disabled."]
    DISABLE_HYSTERESIS_F,
    #[doc = "Enable. Hysteresis for WAKEUP pin enabled."]
    ENABLE,
}
impl crate::ToBits<bool> for WAKEUPHYSR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WAKEUPHYSR::DISABLE_HYSTERESIS_F => false,
            WAKEUPHYSR::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WAKEUPHYS_R = crate::FR<bool, WAKEUPHYSR>;
impl WAKEUPHYS_R {
    #[doc = "Checks if the value of the field is `DISABLE_HYSTERESIS_F`"]
    #[inline(always)]
    pub fn is_disable_hysteresis_f(&self) -> bool {
        *self == WAKEUPHYSR::DISABLE_HYSTERESIS_F
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WAKEUPHYSR::ENABLE
    }
}
#[doc = "Values that can be written to the field `WAKEUPHYS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUPHYSW {
    #[doc = "Disable Hysteresis for WAKUP pin disabled."]
    DISABLE_HYSTERESIS_F,
    #[doc = "Enable. Hysteresis for WAKEUP pin enabled."]
    ENABLE,
}
impl WAKEUPHYSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEUPHYSW::DISABLE_HYSTERESIS_F => false,
            WAKEUPHYSW::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WAKEUPHYSW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUPHYSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEUPHYSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Hysteresis for WAKUP pin disabled."]
    #[inline(always)]
    pub fn disable_hysteresis_f(self) -> &'a mut W {
        self.variant(WAKEUPHYSW::DISABLE_HYSTERESIS_F)
    }
    #[doc = "Enable. Hysteresis for WAKEUP pin enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WAKEUPHYSW::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Possible values of the field `WAKEPAD_DISABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEPAD_DISABLER {
    #[doc = "Enable. The wake-up function is enabled on pin PIO0_16."]
    ENABLE,
    #[doc = "Disable. Setting this bit disables the wake-up function on pin PIO0_16."]
    DISABLE,
}
impl crate::ToBits<bool> for WAKEPAD_DISABLER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WAKEPAD_DISABLER::ENABLE => false,
            WAKEPAD_DISABLER::DISABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WAKEPAD_DISABLE_R = crate::FR<bool, WAKEPAD_DISABLER>;
impl WAKEPAD_DISABLE_R {
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WAKEPAD_DISABLER::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WAKEPAD_DISABLER::DISABLE
    }
}
#[doc = "Values that can be written to the field `WAKEPAD_DISABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEPAD_DISABLEW {
    #[doc = "Enable. The wake-up function is enabled on pin PIO0_16."]
    ENABLE,
    #[doc = "Disable. Setting this bit disables the wake-up function on pin PIO0_16."]
    DISABLE,
}
impl WAKEPAD_DISABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEPAD_DISABLEW::ENABLE => false,
            WAKEPAD_DISABLEW::DISABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WAKEPAD_DISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEPAD_DISABLEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEPAD_DISABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable. The wake-up function is enabled on pin PIO0_16."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WAKEPAD_DISABLEW::ENABLE)
    }
    #[doc = "Disable. Setting this bit disables the wake-up function on pin PIO0_16."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WAKEPAD_DISABLEW::DISABLE)
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
#[doc = r"Reader of the field"]
pub type GPDATA_R = crate::FR<u32, u32>;
#[doc = r"Proxy"]
pub struct _GPDATAW<'a> {
    w: &'a mut W,
}
impl<'a> _GPDATAW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 12)) | (((value as u32) & 0x000f_ffff) << 12);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 10 - WAKEUP pin hysteresis enable"]
    #[inline(always)]
    pub fn wakeuphys(&self) -> WAKEUPHYS_R {
        WAKEUPHYS_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - WAKEUP pin disable. Setting this bit disables the wake-up pin, so it can be used for other purposes. Never set this bit if you intend to use a pin to wake up the part from Deep power-down mode. You can only disable the wake-up pin if the RTC wake-up timer is enabled and configured. Setting this bit is not necessary if Deep power-down mode is not used."]
    #[inline(always)]
    pub fn wakepad_disable(&self) -> WAKEPAD_DISABLE_R {
        WAKEPAD_DISABLE_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:31 - Data retained during Deep power-down mode."]
    #[inline(always)]
    pub fn gpdata(&self) -> GPDATA_R {
        GPDATA_R::new(((self.bits() >> 12) & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 10 - WAKEUP pin hysteresis enable"]
    #[inline(always)]
    pub fn wakeuphys(&mut self) -> _WAKEUPHYSW {
        _WAKEUPHYSW { w: self }
    }
    #[doc = "Bit 11 - WAKEUP pin disable. Setting this bit disables the wake-up pin, so it can be used for other purposes. Never set this bit if you intend to use a pin to wake up the part from Deep power-down mode. You can only disable the wake-up pin if the RTC wake-up timer is enabled and configured. Setting this bit is not necessary if Deep power-down mode is not used."]
    #[inline(always)]
    pub fn wakepad_disable(&mut self) -> _WAKEPAD_DISABLEW {
        _WAKEPAD_DISABLEW { w: self }
    }
    #[doc = "Bits 12:31 - Data retained during Deep power-down mode."]
    #[inline(always)]
    pub fn gpdata(&mut self) -> _GPDATAW {
        _GPDATAW { w: self }
    }
}
