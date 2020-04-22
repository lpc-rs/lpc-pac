#[doc = "Reader of register RS485CTRL"]
pub type R = crate::R<u32, super::RS485CTRL>;
#[doc = "Writer for register RS485CTRL"]
pub type W = crate::W<u32, super::RS485CTRL>;
#[doc = "Register RS485CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::RS485CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "NMM enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMMEN_A {
    #[doc = "0: RS-485/EIA-485 Normal Multidrop Mode (NMM) is disabled."]
    DISABLED = 0,
    #[doc = "1: RS-485/EIA-485 Normal Multidrop Mode (NMM) is enabled. In this mode, an address is detected when a received byte causes the USART to set the parity error and generate an interrupt."]
    ENABLED = 1,
}
impl From<NMMEN_A> for bool {
    #[inline(always)]
    fn from(variant: NMMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NMMEN`"]
pub type NMMEN_R = crate::R<bool, NMMEN_A>;
impl NMMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMMEN_A {
        match self.bits {
            false => NMMEN_A::DISABLED,
            true => NMMEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NMMEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NMMEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `NMMEN`"]
pub struct NMMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NMMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NMMEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NMMEN_A::DISABLED)
    }
    #[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is enabled. In this mode, an address is detected when a received byte causes the USART to set the parity error and generate an interrupt."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NMMEN_A::ENABLED)
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
#[doc = "Receiver enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDIS_A {
    #[doc = "0: The receiver is enabled."]
    ENABLED = 0,
    #[doc = "1: The receiver is disabled."]
    DISABLED = 1,
}
impl From<RXDIS_A> for bool {
    #[inline(always)]
    fn from(variant: RXDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXDIS`"]
pub type RXDIS_R = crate::R<bool, RXDIS_A>;
impl RXDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDIS_A {
        match self.bits {
            false => RXDIS_A::ENABLED,
            true => RXDIS_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXDIS_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXDIS_A::DISABLED
    }
}
#[doc = "Write proxy for field `RXDIS`"]
pub struct RXDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The receiver is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXDIS_A::ENABLED)
    }
    #[doc = "The receiver is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXDIS_A::DISABLED)
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
#[doc = "AAD enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AADEN_A {
    #[doc = "0: Auto Address Detect (AAD) is disabled."]
    DISABLED = 0,
    #[doc = "1: Auto Address Detect (AAD) is enabled."]
    ENABLED = 1,
}
impl From<AADEN_A> for bool {
    #[inline(always)]
    fn from(variant: AADEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AADEN`"]
pub type AADEN_R = crate::R<bool, AADEN_A>;
impl AADEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AADEN_A {
        match self.bits {
            false => AADEN_A::DISABLED,
            true => AADEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AADEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AADEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `AADEN`"]
pub struct AADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AADEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AADEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Auto Address Detect (AAD) is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AADEN_A::DISABLED)
    }
    #[doc = "Auto Address Detect (AAD) is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AADEN_A::ENABLED)
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
#[doc = "Select direction control pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_A {
    #[doc = "0: If direction control is enabled (bit DCTRL = 1), pin RTS is used for direction control."]
    RTS = 0,
    #[doc = "1: If direction control is enabled (bit DCTRL = 1), pin DTR is used for direction control."]
    DTR = 1,
}
impl From<SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<bool, SEL_A>;
impl SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            false => SEL_A::RTS,
            true => SEL_A::DTR,
        }
    }
    #[doc = "Checks if the value of the field is `RTS`"]
    #[inline(always)]
    pub fn is_rts(&self) -> bool {
        *self == SEL_A::RTS
    }
    #[doc = "Checks if the value of the field is `DTR`"]
    #[inline(always)]
    pub fn is_dtr(&self) -> bool {
        *self == SEL_A::DTR
    }
}
#[doc = "Write proxy for field `SEL`"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "If direction control is enabled (bit DCTRL = 1), pin RTS is used for direction control."]
    #[inline(always)]
    pub fn rts(self) -> &'a mut W {
        self.variant(SEL_A::RTS)
    }
    #[doc = "If direction control is enabled (bit DCTRL = 1), pin DTR is used for direction control."]
    #[inline(always)]
    pub fn dtr(self) -> &'a mut W {
        self.variant(SEL_A::DTR)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Auto direction control enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCTRL_A {
    #[doc = "0: Disable Auto Direction Control."]
    DISABLE_AUTO_DIRECTI = 0,
    #[doc = "1: Enable Auto Direction Control."]
    ENABLE_AUTO_DIRECTIO = 1,
}
impl From<DCTRL_A> for bool {
    #[inline(always)]
    fn from(variant: DCTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCTRL`"]
pub type DCTRL_R = crate::R<bool, DCTRL_A>;
impl DCTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCTRL_A {
        match self.bits {
            false => DCTRL_A::DISABLE_AUTO_DIRECTI,
            true => DCTRL_A::ENABLE_AUTO_DIRECTIO,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_AUTO_DIRECTI`"]
    #[inline(always)]
    pub fn is_disable_auto_directi(&self) -> bool {
        *self == DCTRL_A::DISABLE_AUTO_DIRECTI
    }
    #[doc = "Checks if the value of the field is `ENABLE_AUTO_DIRECTIO`"]
    #[inline(always)]
    pub fn is_enable_auto_directio(&self) -> bool {
        *self == DCTRL_A::ENABLE_AUTO_DIRECTIO
    }
}
#[doc = "Write proxy for field `DCTRL`"]
pub struct DCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Auto Direction Control."]
    #[inline(always)]
    pub fn disable_auto_directi(self) -> &'a mut W {
        self.variant(DCTRL_A::DISABLE_AUTO_DIRECTI)
    }
    #[doc = "Enable Auto Direction Control."]
    #[inline(always)]
    pub fn enable_auto_directio(self) -> &'a mut W {
        self.variant(DCTRL_A::ENABLE_AUTO_DIRECTIO)
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
#[doc = "Polarity control. This bit reverses the polarity of the direction control signal on the RTS (or DTR) pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OINV_A {
    #[doc = "0: The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    LOW = 0,
    #[doc = "1: The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    HIGH = 1,
}
impl From<OINV_A> for bool {
    #[inline(always)]
    fn from(variant: OINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OINV`"]
pub type OINV_R = crate::R<bool, OINV_A>;
impl OINV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OINV_A {
        match self.bits {
            false => OINV_A::LOW,
            true => OINV_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OINV_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OINV_A::HIGH
    }
}
#[doc = "Write proxy for field `OINV`"]
pub struct OINV_W<'a> {
    w: &'a mut W,
}
impl<'a> OINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OINV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OINV_A::LOW)
    }
    #[doc = "The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OINV_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - NMM enable."]
    #[inline(always)]
    pub fn nmmen(&self) -> NMMEN_R {
        NMMEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receiver enable."]
    #[inline(always)]
    pub fn rxdis(&self) -> RXDIS_R {
        RXDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AAD enable."]
    #[inline(always)]
    pub fn aaden(&self) -> AADEN_R {
        AADEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Select direction control pin"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Auto direction control enable."]
    #[inline(always)]
    pub fn dctrl(&self) -> DCTRL_R {
        DCTRL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Polarity control. This bit reverses the polarity of the direction control signal on the RTS (or DTR) pin."]
    #[inline(always)]
    pub fn oinv(&self) -> OINV_R {
        OINV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NMM enable."]
    #[inline(always)]
    pub fn nmmen(&mut self) -> NMMEN_W {
        NMMEN_W { w: self }
    }
    #[doc = "Bit 1 - Receiver enable."]
    #[inline(always)]
    pub fn rxdis(&mut self) -> RXDIS_W {
        RXDIS_W { w: self }
    }
    #[doc = "Bit 2 - AAD enable."]
    #[inline(always)]
    pub fn aaden(&mut self) -> AADEN_W {
        AADEN_W { w: self }
    }
    #[doc = "Bit 3 - Select direction control pin"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Bit 4 - Auto direction control enable."]
    #[inline(always)]
    pub fn dctrl(&mut self) -> DCTRL_W {
        DCTRL_W { w: self }
    }
    #[doc = "Bit 5 - Polarity control. This bit reverses the polarity of the direction control signal on the RTS (or DTR) pin."]
    #[inline(always)]
    pub fn oinv(&mut self) -> OINV_W {
        OINV_W { w: self }
    }
}
