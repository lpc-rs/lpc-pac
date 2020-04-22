#[doc = "Reader of register SYNCCTRL"]
pub type R = crate::R<u32, super::SYNCCTRL>;
#[doc = "Writer for register SYNCCTRL"]
pub type W = crate::W<u32, super::SYNCCTRL>;
#[doc = "Register SYNCCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SYNCCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enables synchronous mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNC`"]
pub type SYNC_R = crate::R<bool, SYNC_A>;
impl SYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_A {
        match self.bits {
            false => SYNC_A::DISABLED,
            true => SYNC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYNC_A::ENABLED
    }
}
#[doc = "Write proxy for field `SYNC`"]
pub struct SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYNC_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYNC_A::ENABLED)
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
#[doc = "Clock source select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSRC_A {
    #[doc = "0: Synchronous slave mode (SCLK in)"]
    SYNCHRONOUS_SLAVE_MO = 0,
    #[doc = "1: Synchronous master mode (SCLK out)"]
    SYNCHRONOUS_MASTER_M = 1,
}
impl From<CSRC_A> for bool {
    #[inline(always)]
    fn from(variant: CSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSRC`"]
pub type CSRC_R = crate::R<bool, CSRC_A>;
impl CSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSRC_A {
        match self.bits {
            false => CSRC_A::SYNCHRONOUS_SLAVE_MO,
            true => CSRC_A::SYNCHRONOUS_MASTER_M,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS_SLAVE_MO`"]
    #[inline(always)]
    pub fn is_synchronous_slave_mo(&self) -> bool {
        *self == CSRC_A::SYNCHRONOUS_SLAVE_MO
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS_MASTER_M`"]
    #[inline(always)]
    pub fn is_synchronous_master_m(&self) -> bool {
        *self == CSRC_A::SYNCHRONOUS_MASTER_M
    }
}
#[doc = "Write proxy for field `CSRC`"]
pub struct CSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Synchronous slave mode (SCLK in)"]
    #[inline(always)]
    pub fn synchronous_slave_mo(self) -> &'a mut W {
        self.variant(CSRC_A::SYNCHRONOUS_SLAVE_MO)
    }
    #[doc = "Synchronous master mode (SCLK out)"]
    #[inline(always)]
    pub fn synchronous_master_m(self) -> &'a mut W {
        self.variant(CSRC_A::SYNCHRONOUS_MASTER_M)
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
#[doc = "Falling edge sampling.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FES_A {
    #[doc = "0: RxD is sampled on the rising edge of SCLK "]
    RISING = 0,
    #[doc = "1: RxD is sampled on the falling edge of SCLK"]
    FALLING = 1,
}
impl From<FES_A> for bool {
    #[inline(always)]
    fn from(variant: FES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FES`"]
pub type FES_R = crate::R<bool, FES_A>;
impl FES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FES_A {
        match self.bits {
            false => FES_A::RISING,
            true => FES_A::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == FES_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == FES_A::FALLING
    }
}
#[doc = "Write proxy for field `FES`"]
pub struct FES_W<'a> {
    w: &'a mut W,
}
impl<'a> FES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FES_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RxD is sampled on the rising edge of SCLK"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(FES_A::RISING)
    }
    #[doc = "RxD is sampled on the falling edge of SCLK"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(FES_A::FALLING)
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
#[doc = "Transmit synchronization bypass in synchronous slave mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSBYPASS_A {
    #[doc = "0: The input clock is synchronized prior to being used in clock edge detection logic"]
    SYNC = 0,
    #[doc = "1: The input clock is not synchronized prior to being used in clock edge detection logic. This allows for a high er input clock rate at the expense of potential metastability."]
    NOSYNC = 1,
}
impl From<TSBYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: TSBYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TSBYPASS`"]
pub type TSBYPASS_R = crate::R<bool, TSBYPASS_A>;
impl TSBYPASS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSBYPASS_A {
        match self.bits {
            false => TSBYPASS_A::SYNC,
            true => TSBYPASS_A::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == TSBYPASS_A::SYNC
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        *self == TSBYPASS_A::NOSYNC
    }
}
#[doc = "Write proxy for field `TSBYPASS`"]
pub struct TSBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> TSBYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSBYPASS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The input clock is synchronized prior to being used in clock edge detection logic"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(TSBYPASS_A::SYNC)
    }
    #[doc = "The input clock is not synchronized prior to being used in clock edge detection logic. This allows for a high er input clock rate at the expense of potential metastability."]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TSBYPASS_A::NOSYNC)
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
#[doc = "Continuous master clock enable (used only when CSRC is 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSCEN_A {
    #[doc = "0: SCLK cycles only when characters are being sent on TxD"]
    SCLK_CYCLES_ONLY_WHE = 0,
    #[doc = "1: SCLK runs continuously (characters can be received on RxD independently from transmission on TxD)"]
    SCLK_RUNS_CONTINUOUS = 1,
}
impl From<CSCEN_A> for bool {
    #[inline(always)]
    fn from(variant: CSCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSCEN`"]
pub type CSCEN_R = crate::R<bool, CSCEN_A>;
impl CSCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSCEN_A {
        match self.bits {
            false => CSCEN_A::SCLK_CYCLES_ONLY_WHE,
            true => CSCEN_A::SCLK_RUNS_CONTINUOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SCLK_CYCLES_ONLY_WHE`"]
    #[inline(always)]
    pub fn is_sclk_cycles_only_whe(&self) -> bool {
        *self == CSCEN_A::SCLK_CYCLES_ONLY_WHE
    }
    #[doc = "Checks if the value of the field is `SCLK_RUNS_CONTINUOUS`"]
    #[inline(always)]
    pub fn is_sclk_runs_continuous(&self) -> bool {
        *self == CSCEN_A::SCLK_RUNS_CONTINUOUS
    }
}
#[doc = "Write proxy for field `CSCEN`"]
pub struct CSCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CSCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SCLK cycles only when characters are being sent on TxD"]
    #[inline(always)]
    pub fn sclk_cycles_only_whe(self) -> &'a mut W {
        self.variant(CSCEN_A::SCLK_CYCLES_ONLY_WHE)
    }
    #[doc = "SCLK runs continuously (characters can be received on RxD independently from transmission on TxD)"]
    #[inline(always)]
    pub fn sclk_runs_continuous(self) -> &'a mut W {
        self.variant(CSCEN_A::SCLK_RUNS_CONTINUOUS)
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
#[doc = "Start/stop bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSDIS_A {
    #[doc = "0: Send start and stop bits as in other modes."]
    SEND_START_STOP = 0,
    #[doc = "1: Do not send start/stop bits."]
    DONT_SEND_START_STOP = 1,
}
impl From<SSDIS_A> for bool {
    #[inline(always)]
    fn from(variant: SSDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSDIS`"]
pub type SSDIS_R = crate::R<bool, SSDIS_A>;
impl SSDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSDIS_A {
        match self.bits {
            false => SSDIS_A::SEND_START_STOP,
            true => SSDIS_A::DONT_SEND_START_STOP,
        }
    }
    #[doc = "Checks if the value of the field is `SEND_START_STOP`"]
    #[inline(always)]
    pub fn is_send_start_stop(&self) -> bool {
        *self == SSDIS_A::SEND_START_STOP
    }
    #[doc = "Checks if the value of the field is `DONT_SEND_START_STOP`"]
    #[inline(always)]
    pub fn is_dont_send_start_stop(&self) -> bool {
        *self == SSDIS_A::DONT_SEND_START_STOP
    }
}
#[doc = "Write proxy for field `SSDIS`"]
pub struct SSDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SSDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Send start and stop bits as in other modes."]
    #[inline(always)]
    pub fn send_start_stop(self) -> &'a mut W {
        self.variant(SSDIS_A::SEND_START_STOP)
    }
    #[doc = "Do not send start/stop bits."]
    #[inline(always)]
    pub fn dont_send_start_stop(self) -> &'a mut W {
        self.variant(SSDIS_A::DONT_SEND_START_STOP)
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
#[doc = "Continuous clock clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCCLR_A {
    #[doc = "0: CSCEN is under software control."]
    SOFTWARE = 0,
    #[doc = "1: Hardware clears CSCEN after each character is received."]
    HARDWARE = 1,
}
impl From<CCCLR_A> for bool {
    #[inline(always)]
    fn from(variant: CCCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCCLR`"]
pub type CCCLR_R = crate::R<bool, CCCLR_A>;
impl CCCLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCCLR_A {
        match self.bits {
            false => CCCLR_A::SOFTWARE,
            true => CCCLR_A::HARDWARE,
        }
    }
    #[doc = "Checks if the value of the field is `SOFTWARE`"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == CCCLR_A::SOFTWARE
    }
    #[doc = "Checks if the value of the field is `HARDWARE`"]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == CCCLR_A::HARDWARE
    }
}
#[doc = "Write proxy for field `CCCLR`"]
pub struct CCCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CCCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCCLR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CSCEN is under software control."]
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(CCCLR_A::SOFTWARE)
    }
    #[doc = "Hardware clears CSCEN after each character is received."]
    #[inline(always)]
    pub fn hardware(self) -> &'a mut W {
        self.variant(CCCLR_A::HARDWARE)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enables synchronous mode."]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock source select."]
    #[inline(always)]
    pub fn csrc(&self) -> CSRC_R {
        CSRC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Falling edge sampling."]
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit synchronization bypass in synchronous slave mode."]
    #[inline(always)]
    pub fn tsbypass(&self) -> TSBYPASS_R {
        TSBYPASS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Continuous master clock enable (used only when CSRC is 1)"]
    #[inline(always)]
    pub fn cscen(&self) -> CSCEN_R {
        CSCEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Start/stop bits"]
    #[inline(always)]
    pub fn ssdis(&self) -> SSDIS_R {
        SSDIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Continuous clock clear"]
    #[inline(always)]
    pub fn ccclr(&self) -> CCCLR_R {
        CCCLR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables synchronous mode."]
    #[inline(always)]
    pub fn sync(&mut self) -> SYNC_W {
        SYNC_W { w: self }
    }
    #[doc = "Bit 1 - Clock source select."]
    #[inline(always)]
    pub fn csrc(&mut self) -> CSRC_W {
        CSRC_W { w: self }
    }
    #[doc = "Bit 2 - Falling edge sampling."]
    #[inline(always)]
    pub fn fes(&mut self) -> FES_W {
        FES_W { w: self }
    }
    #[doc = "Bit 3 - Transmit synchronization bypass in synchronous slave mode."]
    #[inline(always)]
    pub fn tsbypass(&mut self) -> TSBYPASS_W {
        TSBYPASS_W { w: self }
    }
    #[doc = "Bit 4 - Continuous master clock enable (used only when CSRC is 1)"]
    #[inline(always)]
    pub fn cscen(&mut self) -> CSCEN_W {
        CSCEN_W { w: self }
    }
    #[doc = "Bit 5 - Start/stop bits"]
    #[inline(always)]
    pub fn ssdis(&mut self) -> SSDIS_W {
        SSDIS_W { w: self }
    }
    #[doc = "Bit 6 - Continuous clock clear"]
    #[inline(always)]
    pub fn ccclr(&mut self) -> CCCLR_W {
        CCCLR_W { w: self }
    }
}
