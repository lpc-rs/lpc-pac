#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ICR {
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
#[doc = "Possible values of the field `IRDAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRDAENR {
    #[doc = "IrDA mode is disabled, USARTn acts as a standard USART."]
    IRDA_MODE_IS_DISABLE,
    #[doc = "IrDA mode is enabled."]
    IRDA_MODE_IS_ENABLED,
}
impl crate::ToBits<bool> for IRDAENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            IRDAENR::IRDA_MODE_IS_DISABLE => false,
            IRDAENR::IRDA_MODE_IS_ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type IRDAEN_R = crate::FR<bool, IRDAENR>;
impl IRDAEN_R {
    #[doc = "Checks if the value of the field is `IRDA_MODE_IS_DISABLE`"]
    #[inline(always)]
    pub fn is_irda_mode_is_disable(&self) -> bool {
        *self == IRDAENR::IRDA_MODE_IS_DISABLE
    }
    #[doc = "Checks if the value of the field is `IRDA_MODE_IS_ENABLED`"]
    #[inline(always)]
    pub fn is_irda_mode_is_enabled(&self) -> bool {
        *self == IRDAENR::IRDA_MODE_IS_ENABLED
    }
}
#[doc = "Values that can be written to the field `IRDAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRDAENW {
    #[doc = "IrDA mode is disabled, USARTn acts as a standard USART."]
    IRDA_MODE_IS_DISABLE,
    #[doc = "IrDA mode is enabled."]
    IRDA_MODE_IS_ENABLED,
}
impl IRDAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            IRDAENW::IRDA_MODE_IS_DISABLE => false,
            IRDAENW::IRDA_MODE_IS_ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _IRDAENW<'a> {
    w: &'a mut W,
}
impl<'a> _IRDAENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRDAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "IrDA mode is disabled, USARTn acts as a standard USART."]
    #[inline(always)]
    pub fn irda_mode_is_disable(self) -> &'a mut W {
        self.variant(IRDAENW::IRDA_MODE_IS_DISABLE)
    }
    #[doc = "IrDA mode is enabled."]
    #[inline(always)]
    pub fn irda_mode_is_enabled(self) -> &'a mut W {
        self.variant(IRDAENW::IRDA_MODE_IS_ENABLED)
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
#[doc = "Possible values of the field `IRDAINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRDAINVR {
    #[doc = "The serial input is not inverted."]
    INVERTED,
    #[doc = "The serial input is inverted. This has no effect on the serial output."]
    NOT_INVERTED,
}
impl crate::ToBits<bool> for IRDAINVR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            IRDAINVR::INVERTED => false,
            IRDAINVR::NOT_INVERTED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type IRDAINV_R = crate::FR<bool, IRDAINVR>;
impl IRDAINV_R {
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == IRDAINVR::INVERTED
    }
    #[doc = "Checks if the value of the field is `NOT_INVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == IRDAINVR::NOT_INVERTED
    }
}
#[doc = "Values that can be written to the field `IRDAINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRDAINVW {
    #[doc = "The serial input is not inverted."]
    INVERTED,
    #[doc = "The serial input is inverted. This has no effect on the serial output."]
    NOT_INVERTED,
}
impl IRDAINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            IRDAINVW::INVERTED => false,
            IRDAINVW::NOT_INVERTED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _IRDAINVW<'a> {
    w: &'a mut W,
}
impl<'a> _IRDAINVW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRDAINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The serial input is not inverted."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(IRDAINVW::INVERTED)
    }
    #[doc = "The serial input is inverted. This has no effect on the serial output."]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(IRDAINVW::NOT_INVERTED)
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
#[doc = "Possible values of the field `FIXPULSEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIXPULSEENR {
    #[doc = "IrDA fixed pulse width mode disabled."]
    DISABLED,
    #[doc = "IrDA fixed pulse width mode enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for FIXPULSEENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            FIXPULSEENR::DISABLED => false,
            FIXPULSEENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FIXPULSEEN_R = crate::FR<bool, FIXPULSEENR>;
impl FIXPULSEEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FIXPULSEENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FIXPULSEENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `FIXPULSEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIXPULSEENW {
    #[doc = "IrDA fixed pulse width mode disabled."]
    DISABLED,
    #[doc = "IrDA fixed pulse width mode enabled."]
    ENABLED,
}
impl FIXPULSEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            FIXPULSEENW::DISABLED => false,
            FIXPULSEENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FIXPULSEENW<'a> {
    w: &'a mut W,
}
impl<'a> _FIXPULSEENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIXPULSEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "IrDA fixed pulse width mode disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FIXPULSEENW::DISABLED)
    }
    #[doc = "IrDA fixed pulse width mode enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FIXPULSEENW::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `PULSEDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PULSEDIVR {
    #[doc = "3 / (16 x baud rate)"]
    _3_DIV_16_X_BAUD_RATE,
    #[doc = "2 x TPCLK"]
    _2_X_TPCLK,
    #[doc = "4 x TPCLK"]
    _4_X_TPCLK,
    #[doc = "8 x TPCLK"]
    _8_X_TPCLK,
    #[doc = "16 x TPCLK"]
    _16_X_TPCLK,
    #[doc = "32 x TPCLK"]
    _32_X_TPCLK,
    #[doc = "64 x TPCLK"]
    _64_X_TPCLK,
    #[doc = "128 x TPCLK"]
    _128_X_TPCLK,
}
impl crate::ToBits<u8> for PULSEDIVR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PULSEDIVR::_3_DIV_16_X_BAUD_RATE => 0,
            PULSEDIVR::_2_X_TPCLK => 1,
            PULSEDIVR::_4_X_TPCLK => 2,
            PULSEDIVR::_8_X_TPCLK => 3,
            PULSEDIVR::_16_X_TPCLK => 4,
            PULSEDIVR::_32_X_TPCLK => 5,
            PULSEDIVR::_64_X_TPCLK => 6,
            PULSEDIVR::_128_X_TPCLK => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PULSEDIV_R = crate::FR<u8, PULSEDIVR>;
impl PULSEDIV_R {
    #[doc = "Checks if the value of the field is `_3_DIV_16_X_BAUD_RATE`"]
    #[inline(always)]
    pub fn is_3_div_16_x_baud_rate(&self) -> bool {
        *self == PULSEDIVR::_3_DIV_16_X_BAUD_RATE
    }
    #[doc = "Checks if the value of the field is `_2_X_TPCLK`"]
    #[inline(always)]
    pub fn is_2_x_tpclk(&self) -> bool {
        *self == PULSEDIVR::_2_X_TPCLK
    }
    #[doc = "Checks if the value of the field is `_4_X_TPCLK`"]
    #[inline(always)]
    pub fn is_4_x_tpclk(&self) -> bool {
        *self == PULSEDIVR::_4_X_TPCLK
    }
    #[doc = "Checks if the value of the field is `_8_X_TPCLK`"]
    #[inline(always)]
    pub fn is_8_x_tpclk(&self) -> bool {
        *self == PULSEDIVR::_8_X_TPCLK
    }
    #[doc = "Checks if the value of the field is `_16_X_TPCLK`"]
    #[inline(always)]
    pub fn is_16_x_tpclk(&self) -> bool {
        *self == PULSEDIVR::_16_X_TPCLK
    }
    #[doc = "Checks if the value of the field is `_32_X_TPCLK`"]
    #[inline(always)]
    pub fn is_32_x_tpclk(&self) -> bool {
        *self == PULSEDIVR::_32_X_TPCLK
    }
    #[doc = "Checks if the value of the field is `_64_X_TPCLK`"]
    #[inline(always)]
    pub fn is_64_x_tpclk(&self) -> bool {
        *self == PULSEDIVR::_64_X_TPCLK
    }
    #[doc = "Checks if the value of the field is `_128_X_TPCLK`"]
    #[inline(always)]
    pub fn is_128_x_tpclk(&self) -> bool {
        *self == PULSEDIVR::_128_X_TPCLK
    }
}
#[doc = "Values that can be written to the field `PULSEDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PULSEDIVW {
    #[doc = "3 / (16 x baud rate)"]
    _3_DIV_16_X_BAUD_RATE,
    #[doc = "2 x TPCLK"]
    _2_X_TPCLK,
    #[doc = "4 x TPCLK"]
    _4_X_TPCLK,
    #[doc = "8 x TPCLK"]
    _8_X_TPCLK,
    #[doc = "16 x TPCLK"]
    _16_X_TPCLK,
    #[doc = "32 x TPCLK"]
    _32_X_TPCLK,
    #[doc = "64 x TPCLK"]
    _64_X_TPCLK,
    #[doc = "128 x TPCLK"]
    _128_X_TPCLK,
}
impl PULSEDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PULSEDIVW::_3_DIV_16_X_BAUD_RATE => 0,
            PULSEDIVW::_2_X_TPCLK => 1,
            PULSEDIVW::_4_X_TPCLK => 2,
            PULSEDIVW::_8_X_TPCLK => 3,
            PULSEDIVW::_16_X_TPCLK => 4,
            PULSEDIVW::_32_X_TPCLK => 5,
            PULSEDIVW::_64_X_TPCLK => 6,
            PULSEDIVW::_128_X_TPCLK => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PULSEDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _PULSEDIVW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PULSEDIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "3 / (16 x baud rate)"]
    #[inline(always)]
    pub fn _3_div_16_x_baud_rate(self) -> &'a mut W {
        self.variant(PULSEDIVW::_3_DIV_16_X_BAUD_RATE)
    }
    #[doc = "2 x TPCLK"]
    #[inline(always)]
    pub fn _2_x_tpclk(self) -> &'a mut W {
        self.variant(PULSEDIVW::_2_X_TPCLK)
    }
    #[doc = "4 x TPCLK"]
    #[inline(always)]
    pub fn _4_x_tpclk(self) -> &'a mut W {
        self.variant(PULSEDIVW::_4_X_TPCLK)
    }
    #[doc = "8 x TPCLK"]
    #[inline(always)]
    pub fn _8_x_tpclk(self) -> &'a mut W {
        self.variant(PULSEDIVW::_8_X_TPCLK)
    }
    #[doc = "16 x TPCLK"]
    #[inline(always)]
    pub fn _16_x_tpclk(self) -> &'a mut W {
        self.variant(PULSEDIVW::_16_X_TPCLK)
    }
    #[doc = "32 x TPCLK"]
    #[inline(always)]
    pub fn _32_x_tpclk(self) -> &'a mut W {
        self.variant(PULSEDIVW::_32_X_TPCLK)
    }
    #[doc = "64 x TPCLK"]
    #[inline(always)]
    pub fn _64_x_tpclk(self) -> &'a mut W {
        self.variant(PULSEDIVW::_64_X_TPCLK)
    }
    #[doc = "128 x TPCLK"]
    #[inline(always)]
    pub fn _128_x_tpclk(self) -> &'a mut W {
        self.variant(PULSEDIVW::_128_X_TPCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - IrDA mode enable"]
    #[inline(always)]
    pub fn irdaen(&self) -> IRDAEN_R {
        IRDAEN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Serial input inverter"]
    #[inline(always)]
    pub fn irdainv(&self) -> IRDAINV_R {
        IRDAINV_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IrDA fixed pulse width mode."]
    #[inline(always)]
    pub fn fixpulseen(&self) -> FIXPULSEEN_R {
        FIXPULSEEN_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Configures the pulse width when FixPulseEn = 1."]
    #[inline(always)]
    pub fn pulsediv(&self) -> PULSEDIV_R {
        PULSEDIV_R::new(((self.bits() >> 3) & 0x07) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - IrDA mode enable"]
    #[inline(always)]
    pub fn irdaen(&mut self) -> _IRDAENW {
        _IRDAENW { w: self }
    }
    #[doc = "Bit 1 - Serial input inverter"]
    #[inline(always)]
    pub fn irdainv(&mut self) -> _IRDAINVW {
        _IRDAINVW { w: self }
    }
    #[doc = "Bit 2 - IrDA fixed pulse width mode."]
    #[inline(always)]
    pub fn fixpulseen(&mut self) -> _FIXPULSEENW {
        _FIXPULSEENW { w: self }
    }
    #[doc = "Bits 3:5 - Configures the pulse width when FixPulseEn = 1."]
    #[inline(always)]
    pub fn pulsediv(&mut self) -> _PULSEDIVW {
        _PULSEDIVW { w: self }
    }
}
