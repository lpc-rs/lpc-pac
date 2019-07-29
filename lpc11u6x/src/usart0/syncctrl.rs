#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYNCCTRL {
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
#[doc = "Possible values of the field `SYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl SYNCR {
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
            SYNCR::DISABLED => false,
            SYNCR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNCR {
        match value {
            false => SYNCR::DISABLED,
            true => SYNCR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SYNCR::ENABLED
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
impl CSRCR {
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
            CSRCR::SYNCHRONOUS_SLAVE_MO => false,
            CSRCR::SYNCHRONOUS_MASTER_M => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSRCR {
        match value {
            false => CSRCR::SYNCHRONOUS_SLAVE_MO,
            true => CSRCR::SYNCHRONOUS_MASTER_M,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS_SLAVE_MO`"]
    #[inline]
    pub fn is_synchronous_slave_mo(&self) -> bool {
        *self == CSRCR::SYNCHRONOUS_SLAVE_MO
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS_MASTER_M`"]
    #[inline]
    pub fn is_synchronous_master_m(&self) -> bool {
        *self == CSRCR::SYNCHRONOUS_MASTER_M
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
impl FESR {
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
            FESR::RISING => false,
            FESR::FALLING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FESR {
        match value {
            false => FESR::RISING,
            true => FESR::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == FESR::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == FESR::FALLING
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
impl TSBYPASSR {
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
            TSBYPASSR::SYNC => false,
            TSBYPASSR::NOSYNC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSBYPASSR {
        match value {
            false => TSBYPASSR::SYNC,
            true => TSBYPASSR::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline]
    pub fn is_sync(&self) -> bool {
        *self == TSBYPASSR::SYNC
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline]
    pub fn is_nosync(&self) -> bool {
        *self == TSBYPASSR::NOSYNC
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
impl CSCENR {
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
            CSCENR::SCLK_CYCLES_ONLY_WHE => false,
            CSCENR::SCLK_RUNS_CONTINUOUS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSCENR {
        match value {
            false => CSCENR::SCLK_CYCLES_ONLY_WHE,
            true => CSCENR::SCLK_RUNS_CONTINUOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SCLK_CYCLES_ONLY_WHE`"]
    #[inline]
    pub fn is_sclk_cycles_only_whe(&self) -> bool {
        *self == CSCENR::SCLK_CYCLES_ONLY_WHE
    }
    #[doc = "Checks if the value of the field is `SCLK_RUNS_CONTINUOUS`"]
    #[inline]
    pub fn is_sclk_runs_continuous(&self) -> bool {
        *self == CSCENR::SCLK_RUNS_CONTINUOUS
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
impl SSDISR {
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
            SSDISR::SEND_START_AND_STOP_ => false,
            SSDISR::DO_NOT_SEND_STARTSTOP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSDISR {
        match value {
            false => SSDISR::SEND_START_AND_STOP_,
            true => SSDISR::DO_NOT_SEND_STARTSTOP,
        }
    }
    #[doc = "Checks if the value of the field is `SEND_START_AND_STOP_`"]
    #[inline]
    pub fn is_send_start_and_stop_(&self) -> bool {
        *self == SSDISR::SEND_START_AND_STOP_
    }
    #[doc = "Checks if the value of the field is `DO_NOT_SEND_STARTSTOP`"]
    #[inline]
    pub fn is_do_not_send_startstop(&self) -> bool {
        *self == SSDISR::DO_NOT_SEND_STARTSTOP
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
impl CCCLRR {
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
            CCCLRR::CSCEN_IS_UNDER_SOFTW => false,
            CCCLRR::HARDWARE_CLEARS_CSCE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCCLRR {
        match value {
            false => CCCLRR::CSCEN_IS_UNDER_SOFTW,
            true => CCCLRR::HARDWARE_CLEARS_CSCE,
        }
    }
    #[doc = "Checks if the value of the field is `CSCEN_IS_UNDER_SOFTW`"]
    #[inline]
    pub fn is_cscen_is_under_softw(&self) -> bool {
        *self == CCCLRR::CSCEN_IS_UNDER_SOFTW
    }
    #[doc = "Checks if the value of the field is `HARDWARE_CLEARS_CSCE`"]
    #[inline]
    pub fn is_hardware_clears_csce(&self) -> bool {
        *self == CCCLRR::HARDWARE_CLEARS_CSCE
    }
}
#[doc = "Values that can be written to the field `SYNC`"]
pub enum SYNCW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl SYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYNCW::DISABLED => false,
            SYNCW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYNCW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYNCW::ENABLED)
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
#[doc = "Values that can be written to the field `CSRC`"]
pub enum CSRCW {
    #[doc = "Synchronous slave mode (SCLK in)"]
    SYNCHRONOUS_SLAVE_MO,
    #[doc = "Synchronous master mode (SCLK out)"]
    SYNCHRONOUS_MASTER_M,
}
impl CSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSRCW::SYNCHRONOUS_SLAVE_MO => false,
            CSRCW::SYNCHRONOUS_MASTER_M => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Synchronous slave mode (SCLK in)"]
    #[inline]
    pub fn synchronous_slave_mo(self) -> &'a mut W {
        self.variant(CSRCW::SYNCHRONOUS_SLAVE_MO)
    }
    #[doc = "Synchronous master mode (SCLK out)"]
    #[inline]
    pub fn synchronous_master_m(self) -> &'a mut W {
        self.variant(CSRCW::SYNCHRONOUS_MASTER_M)
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
#[doc = "Values that can be written to the field `FES`"]
pub enum FESW {
    #[doc = "RxD is sampled on the rising edge of SCLK "]
    RISING,
    #[doc = "RxD is sampled on the falling edge of SCLK"]
    FALLING,
}
impl FESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FESW::RISING => false,
            FESW::FALLING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FESW<'a> {
    w: &'a mut W,
}
impl<'a> _FESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RxD is sampled on the rising edge of SCLK"]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(FESW::RISING)
    }
    #[doc = "RxD is sampled on the falling edge of SCLK"]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(FESW::FALLING)
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
#[doc = "Values that can be written to the field `TSBYPASS`"]
pub enum TSBYPASSW {
    #[doc = "The input clock is synchronized prior to being used in clock edge detection logic"]
    SYNC,
    #[doc = "The input clock is not synchronized prior to being used in clock edge detection logic. This allows for a high er input clock rate at the expense of potential metastability."]
    NOSYNC,
}
impl TSBYPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSBYPASSW::SYNC => false,
            TSBYPASSW::NOSYNC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSBYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _TSBYPASSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSBYPASSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The input clock is synchronized prior to being used in clock edge detection logic"]
    #[inline]
    pub fn sync(self) -> &'a mut W {
        self.variant(TSBYPASSW::SYNC)
    }
    #[doc = "The input clock is not synchronized prior to being used in clock edge detection logic. This allows for a high er input clock rate at the expense of potential metastability."]
    #[inline]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TSBYPASSW::NOSYNC)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CSCEN`"]
pub enum CSCENW {
    #[doc = "SCLK cycles only when characters are being sent on TxD"]
    SCLK_CYCLES_ONLY_WHE,
    #[doc = "SCLK runs continuously (characters can be received on RxD independently from transmission on TxD)"]
    SCLK_RUNS_CONTINUOUS,
}
impl CSCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSCENW::SCLK_CYCLES_ONLY_WHE => false,
            CSCENW::SCLK_RUNS_CONTINUOUS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSCENW<'a> {
    w: &'a mut W,
}
impl<'a> _CSCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SCLK cycles only when characters are being sent on TxD"]
    #[inline]
    pub fn sclk_cycles_only_whe(self) -> &'a mut W {
        self.variant(CSCENW::SCLK_CYCLES_ONLY_WHE)
    }
    #[doc = "SCLK runs continuously (characters can be received on RxD independently from transmission on TxD)"]
    #[inline]
    pub fn sclk_runs_continuous(self) -> &'a mut W {
        self.variant(CSCENW::SCLK_RUNS_CONTINUOUS)
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
#[doc = "Values that can be written to the field `SSDIS`"]
pub enum SSDISW {
    #[doc = "Send start and stop bits as in other modes."]
    SEND_START_AND_STOP_,
    #[doc = "Do not send start/stop bits."]
    DO_NOT_SEND_STARTSTOP,
}
impl SSDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSDISW::SEND_START_AND_STOP_ => false,
            SSDISW::DO_NOT_SEND_STARTSTOP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSDISW<'a> {
    w: &'a mut W,
}
impl<'a> _SSDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Send start and stop bits as in other modes."]
    #[inline]
    pub fn send_start_and_stop_(self) -> &'a mut W {
        self.variant(SSDISW::SEND_START_AND_STOP_)
    }
    #[doc = "Do not send start/stop bits."]
    #[inline]
    pub fn do_not_send_startstop(self) -> &'a mut W {
        self.variant(SSDISW::DO_NOT_SEND_STARTSTOP)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CCCLR`"]
pub enum CCCLRW {
    #[doc = "CSCEN is under software control."]
    CSCEN_IS_UNDER_SOFTW,
    #[doc = "Hardware clears CSCEN after each character is received."]
    HARDWARE_CLEARS_CSCE,
}
impl CCCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCCLRW::CSCEN_IS_UNDER_SOFTW => false,
            CCCLRW::HARDWARE_CLEARS_CSCE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CCCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CSCEN is under software control."]
    #[inline]
    pub fn cscen_is_under_softw(self) -> &'a mut W {
        self.variant(CCCLRW::CSCEN_IS_UNDER_SOFTW)
    }
    #[doc = "Hardware clears CSCEN after each character is received."]
    #[inline]
    pub fn hardware_clears_csce(self) -> &'a mut W {
        self.variant(CCCLRW::HARDWARE_CLEARS_CSCE)
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
        const OFFSET: u8 = 6;
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
    #[doc = "Bit 0 - Enables synchronous mode."]
    #[inline]
    pub fn sync(&self) -> SYNCR {
        SYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Clock source select."]
    #[inline]
    pub fn csrc(&self) -> CSRCR {
        CSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Falling edge sampling."]
    #[inline]
    pub fn fes(&self) -> FESR {
        FESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Transmit synchronization bypass in synchronous slave mode."]
    #[inline]
    pub fn tsbypass(&self) -> TSBYPASSR {
        TSBYPASSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Continuous master clock enable (used only when CSRC is 1)"]
    #[inline]
    pub fn cscen(&self) -> CSCENR {
        CSCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Start/stop bits"]
    #[inline]
    pub fn ssdis(&self) -> SSDISR {
        SSDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Continuous clock clear"]
    #[inline]
    pub fn ccclr(&self) -> CCCLRR {
        CCCLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
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
    #[doc = "Bit 0 - Enables synchronous mode."]
    #[inline]
    pub fn sync(&mut self) -> _SYNCW {
        _SYNCW { w: self }
    }
    #[doc = "Bit 1 - Clock source select."]
    #[inline]
    pub fn csrc(&mut self) -> _CSRCW {
        _CSRCW { w: self }
    }
    #[doc = "Bit 2 - Falling edge sampling."]
    #[inline]
    pub fn fes(&mut self) -> _FESW {
        _FESW { w: self }
    }
    #[doc = "Bit 3 - Transmit synchronization bypass in synchronous slave mode."]
    #[inline]
    pub fn tsbypass(&mut self) -> _TSBYPASSW {
        _TSBYPASSW { w: self }
    }
    #[doc = "Bit 4 - Continuous master clock enable (used only when CSRC is 1)"]
    #[inline]
    pub fn cscen(&mut self) -> _CSCENW {
        _CSCENW { w: self }
    }
    #[doc = "Bit 5 - Start/stop bits"]
    #[inline]
    pub fn ssdis(&mut self) -> _SSDISW {
        _SSDISW { w: self }
    }
    #[doc = "Bit 6 - Continuous clock clear"]
    #[inline]
    pub fn ccclr(&mut self) -> _CCCLRW {
        _CCCLRW { w: self }
    }
}
