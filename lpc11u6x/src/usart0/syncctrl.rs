#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYNCCTRL {
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
#[doc = "Possible values of the field `SYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl crate::ToBits<bool> for SYNCR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SYNCR::DISABLED => false,
            SYNCR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SYNC_R = crate::FR<bool, SYNCR>;
impl SYNC_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYNCR::ENABLED
    }
}
#[doc = "Values that can be written to the field `SYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl SYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SYNCW::DISABLED => false,
            SYNCW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYNCW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYNCW::ENABLED)
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
#[doc = "Possible values of the field `CSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSRCR {
    #[doc = "Synchronous slave mode (SCLK in)"]
    SYNCHRONOUS_SLAVE_MO,
    #[doc = "Synchronous master mode (SCLK out)"]
    SYNCHRONOUS_MASTER_M,
}
impl crate::ToBits<bool> for CSRCR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CSRCR::SYNCHRONOUS_SLAVE_MO => false,
            CSRCR::SYNCHRONOUS_MASTER_M => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CSRC_R = crate::FR<bool, CSRCR>;
impl CSRC_R {
    #[doc = "Checks if the value of the field is `SYNCHRONOUS_SLAVE_MO`"]
    #[inline(always)]
    pub fn is_synchronous_slave_mo(&self) -> bool {
        *self == CSRCR::SYNCHRONOUS_SLAVE_MO
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS_MASTER_M`"]
    #[inline(always)]
    pub fn is_synchronous_master_m(&self) -> bool {
        *self == CSRCR::SYNCHRONOUS_MASTER_M
    }
}
#[doc = "Values that can be written to the field `CSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSRCW {
    #[doc = "Synchronous slave mode (SCLK in)"]
    SYNCHRONOUS_SLAVE_MO,
    #[doc = "Synchronous master mode (SCLK out)"]
    SYNCHRONOUS_MASTER_M,
}
impl CSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CSRCW::SYNCHRONOUS_SLAVE_MO => false,
            CSRCW::SYNCHRONOUS_MASTER_M => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CSRCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Synchronous slave mode (SCLK in)"]
    #[inline(always)]
    pub fn synchronous_slave_mo(self) -> &'a mut W {
        self.variant(CSRCW::SYNCHRONOUS_SLAVE_MO)
    }
    #[doc = "Synchronous master mode (SCLK out)"]
    #[inline(always)]
    pub fn synchronous_master_m(self) -> &'a mut W {
        self.variant(CSRCW::SYNCHRONOUS_MASTER_M)
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
#[doc = "Possible values of the field `FES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FESR {
    #[doc = "RxD is sampled on the rising edge of SCLK "]
    RISING,
    #[doc = "RxD is sampled on the falling edge of SCLK"]
    FALLING,
}
impl crate::ToBits<bool> for FESR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            FESR::RISING => false,
            FESR::FALLING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FES_R = crate::FR<bool, FESR>;
impl FES_R {
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == FESR::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == FESR::FALLING
    }
}
#[doc = "Values that can be written to the field `FES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FESW {
    #[doc = "RxD is sampled on the rising edge of SCLK "]
    RISING,
    #[doc = "RxD is sampled on the falling edge of SCLK"]
    FALLING,
}
impl FESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            FESW::RISING => false,
            FESW::FALLING => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FESW<'a> {
    w: &'a mut W,
}
impl<'a> _FESW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RxD is sampled on the rising edge of SCLK"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(FESW::RISING)
    }
    #[doc = "RxD is sampled on the falling edge of SCLK"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(FESW::FALLING)
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
#[doc = "Possible values of the field `TSBYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSBYPASSR {
    #[doc = "The input clock is synchronized prior to being used in clock edge detection logic"]
    SYNC,
    #[doc = "The input clock is not synchronized prior to being used in clock edge detection logic. This allows for a high er input clock rate at the expense of potential metastability."]
    NOSYNC,
}
impl crate::ToBits<bool> for TSBYPASSR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TSBYPASSR::SYNC => false,
            TSBYPASSR::NOSYNC => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TSBYPASS_R = crate::FR<bool, TSBYPASSR>;
impl TSBYPASS_R {
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == TSBYPASSR::SYNC
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        *self == TSBYPASSR::NOSYNC
    }
}
#[doc = "Values that can be written to the field `TSBYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSBYPASSW {
    #[doc = "The input clock is synchronized prior to being used in clock edge detection logic"]
    SYNC,
    #[doc = "The input clock is not synchronized prior to being used in clock edge detection logic. This allows for a high er input clock rate at the expense of potential metastability."]
    NOSYNC,
}
impl TSBYPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TSBYPASSW::SYNC => false,
            TSBYPASSW::NOSYNC => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TSBYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _TSBYPASSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSBYPASSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The input clock is synchronized prior to being used in clock edge detection logic"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(TSBYPASSW::SYNC)
    }
    #[doc = "The input clock is not synchronized prior to being used in clock edge detection logic. This allows for a high er input clock rate at the expense of potential metastability."]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TSBYPASSW::NOSYNC)
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
#[doc = "Possible values of the field `CSCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSCENR {
    #[doc = "SCLK cycles only when characters are being sent on TxD"]
    SCLK_CYCLES_ONLY_WHE,
    #[doc = "SCLK runs continuously (characters can be received on RxD independently from transmission on TxD)"]
    SCLK_RUNS_CONTINUOUS,
}
impl crate::ToBits<bool> for CSCENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CSCENR::SCLK_CYCLES_ONLY_WHE => false,
            CSCENR::SCLK_RUNS_CONTINUOUS => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CSCEN_R = crate::FR<bool, CSCENR>;
impl CSCEN_R {
    #[doc = "Checks if the value of the field is `SCLK_CYCLES_ONLY_WHE`"]
    #[inline(always)]
    pub fn is_sclk_cycles_only_whe(&self) -> bool {
        *self == CSCENR::SCLK_CYCLES_ONLY_WHE
    }
    #[doc = "Checks if the value of the field is `SCLK_RUNS_CONTINUOUS`"]
    #[inline(always)]
    pub fn is_sclk_runs_continuous(&self) -> bool {
        *self == CSCENR::SCLK_RUNS_CONTINUOUS
    }
}
#[doc = "Values that can be written to the field `CSCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSCENW {
    #[doc = "SCLK cycles only when characters are being sent on TxD"]
    SCLK_CYCLES_ONLY_WHE,
    #[doc = "SCLK runs continuously (characters can be received on RxD independently from transmission on TxD)"]
    SCLK_RUNS_CONTINUOUS,
}
impl CSCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CSCENW::SCLK_CYCLES_ONLY_WHE => false,
            CSCENW::SCLK_RUNS_CONTINUOUS => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CSCENW<'a> {
    w: &'a mut W,
}
impl<'a> _CSCENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SCLK cycles only when characters are being sent on TxD"]
    #[inline(always)]
    pub fn sclk_cycles_only_whe(self) -> &'a mut W {
        self.variant(CSCENW::SCLK_CYCLES_ONLY_WHE)
    }
    #[doc = "SCLK runs continuously (characters can be received on RxD independently from transmission on TxD)"]
    #[inline(always)]
    pub fn sclk_runs_continuous(self) -> &'a mut W {
        self.variant(CSCENW::SCLK_RUNS_CONTINUOUS)
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
#[doc = "Possible values of the field `SSDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSDISR {
    #[doc = "Send start and stop bits as in other modes."]
    SEND_START_AND_STOP_,
    #[doc = "Do not send start/stop bits."]
    DO_NOT_SEND_STARTSTOP,
}
impl crate::ToBits<bool> for SSDISR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SSDISR::SEND_START_AND_STOP_ => false,
            SSDISR::DO_NOT_SEND_STARTSTOP => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SSDIS_R = crate::FR<bool, SSDISR>;
impl SSDIS_R {
    #[doc = "Checks if the value of the field is `SEND_START_AND_STOP_`"]
    #[inline(always)]
    pub fn is_send_start_and_stop_(&self) -> bool {
        *self == SSDISR::SEND_START_AND_STOP_
    }
    #[doc = "Checks if the value of the field is `DO_NOT_SEND_STARTSTOP`"]
    #[inline(always)]
    pub fn is_do_not_send_startstop(&self) -> bool {
        *self == SSDISR::DO_NOT_SEND_STARTSTOP
    }
}
#[doc = "Values that can be written to the field `SSDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSDISW {
    #[doc = "Send start and stop bits as in other modes."]
    SEND_START_AND_STOP_,
    #[doc = "Do not send start/stop bits."]
    DO_NOT_SEND_STARTSTOP,
}
impl SSDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SSDISW::SEND_START_AND_STOP_ => false,
            SSDISW::DO_NOT_SEND_STARTSTOP => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SSDISW<'a> {
    w: &'a mut W,
}
impl<'a> _SSDISW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Send start and stop bits as in other modes."]
    #[inline(always)]
    pub fn send_start_and_stop_(self) -> &'a mut W {
        self.variant(SSDISW::SEND_START_AND_STOP_)
    }
    #[doc = "Do not send start/stop bits."]
    #[inline(always)]
    pub fn do_not_send_startstop(self) -> &'a mut W {
        self.variant(SSDISW::DO_NOT_SEND_STARTSTOP)
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
#[doc = "Possible values of the field `CCCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCCLRR {
    #[doc = "CSCEN is under software control."]
    CSCEN_IS_UNDER_SOFTW,
    #[doc = "Hardware clears CSCEN after each character is received."]
    HARDWARE_CLEARS_CSCE,
}
impl crate::ToBits<bool> for CCCLRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CCCLRR::CSCEN_IS_UNDER_SOFTW => false,
            CCCLRR::HARDWARE_CLEARS_CSCE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CCCLR_R = crate::FR<bool, CCCLRR>;
impl CCCLR_R {
    #[doc = "Checks if the value of the field is `CSCEN_IS_UNDER_SOFTW`"]
    #[inline(always)]
    pub fn is_cscen_is_under_softw(&self) -> bool {
        *self == CCCLRR::CSCEN_IS_UNDER_SOFTW
    }
    #[doc = "Checks if the value of the field is `HARDWARE_CLEARS_CSCE`"]
    #[inline(always)]
    pub fn is_hardware_clears_csce(&self) -> bool {
        *self == CCCLRR::HARDWARE_CLEARS_CSCE
    }
}
#[doc = "Values that can be written to the field `CCCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCCLRW {
    #[doc = "CSCEN is under software control."]
    CSCEN_IS_UNDER_SOFTW,
    #[doc = "Hardware clears CSCEN after each character is received."]
    HARDWARE_CLEARS_CSCE,
}
impl CCCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CCCLRW::CSCEN_IS_UNDER_SOFTW => false,
            CCCLRW::HARDWARE_CLEARS_CSCE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CCCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CCCLRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CSCEN is under software control."]
    #[inline(always)]
    pub fn cscen_is_under_softw(self) -> &'a mut W {
        self.variant(CCCLRW::CSCEN_IS_UNDER_SOFTW)
    }
    #[doc = "Hardware clears CSCEN after each character is received."]
    #[inline(always)]
    pub fn hardware_clears_csce(self) -> &'a mut W {
        self.variant(CCCLRW::HARDWARE_CLEARS_CSCE)
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
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enables synchronous mode."]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock source select."]
    #[inline(always)]
    pub fn csrc(&self) -> CSRC_R {
        CSRC_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Falling edge sampling."]
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit synchronization bypass in synchronous slave mode."]
    #[inline(always)]
    pub fn tsbypass(&self) -> TSBYPASS_R {
        TSBYPASS_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Continuous master clock enable (used only when CSRC is 1)"]
    #[inline(always)]
    pub fn cscen(&self) -> CSCEN_R {
        CSCEN_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Start/stop bits"]
    #[inline(always)]
    pub fn ssdis(&self) -> SSDIS_R {
        SSDIS_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Continuous clock clear"]
    #[inline(always)]
    pub fn ccclr(&self) -> CCCLR_R {
        CCCLR_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enables synchronous mode."]
    #[inline(always)]
    pub fn sync(&mut self) -> _SYNCW {
        _SYNCW { w: self }
    }
    #[doc = "Bit 1 - Clock source select."]
    #[inline(always)]
    pub fn csrc(&mut self) -> _CSRCW {
        _CSRCW { w: self }
    }
    #[doc = "Bit 2 - Falling edge sampling."]
    #[inline(always)]
    pub fn fes(&mut self) -> _FESW {
        _FESW { w: self }
    }
    #[doc = "Bit 3 - Transmit synchronization bypass in synchronous slave mode."]
    #[inline(always)]
    pub fn tsbypass(&mut self) -> _TSBYPASSW {
        _TSBYPASSW { w: self }
    }
    #[doc = "Bit 4 - Continuous master clock enable (used only when CSRC is 1)"]
    #[inline(always)]
    pub fn cscen(&mut self) -> _CSCENW {
        _CSCENW { w: self }
    }
    #[doc = "Bit 5 - Start/stop bits"]
    #[inline(always)]
    pub fn ssdis(&mut self) -> _SSDISW {
        _SSDISW { w: self }
    }
    #[doc = "Bit 6 - Continuous clock clear"]
    #[inline(always)]
    pub fn ccclr(&mut self) -> _CCCLRW {
        _CCCLRW { w: self }
    }
}
