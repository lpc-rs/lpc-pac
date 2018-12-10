#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG {
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
#[doc = "Possible values of the field `ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLER {
    #[doc = "Disabled. The USART is disabled and the internal state machine and counters are reset. While Enable = 0, all USART interrupts and DMA transfers are disabled. When Enable is set again, CFG and most other control bits remain unchanged. For instance, when re-enabled, the USART will immediately generate a TxRdy interrupt (if  enabled in the INTENSET register) or a DMA transfer request because the transmitter has been reset and is therefore available."]
    DISABLED,
    #[doc = "Enabled. The USART is enabled for operation."]
    ENABLED,
}
impl ENABLER {
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
            ENABLER::DISABLED => false,
            ENABLER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENABLER {
        match value {
            false => ENABLER::DISABLED,
            true => ENABLER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLER::ENABLED
    }
}
#[doc = "Possible values of the field `DATALEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATALENR {
    #[doc = "7 bit Data length."]
    _7_BIT_DATA_LENGTH,
    #[doc = "8 bit Data length."]
    _8_BIT_DATA_LENGTH,
    #[doc = "9 bit data length. The 9th bit is commonly used for addressing in multidrop mode. See the ADDRDET bit in the CTL register."]
    _9_BIT_DATA_LENGTH,
}
impl DATALENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DATALENR::_7_BIT_DATA_LENGTH => 0,
            DATALENR::_8_BIT_DATA_LENGTH => 1,
            DATALENR::_9_BIT_DATA_LENGTH => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DATALENR {
        match value {
            0 => DATALENR::_7_BIT_DATA_LENGTH,
            1 => DATALENR::_8_BIT_DATA_LENGTH,
            2 => DATALENR::_9_BIT_DATA_LENGTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_7_BIT_DATA_LENGTH`"]
    #[inline]
    pub fn is_7_bit_data_length(&self) -> bool {
        *self == DATALENR::_7_BIT_DATA_LENGTH
    }
    #[doc = "Checks if the value of the field is `_8_BIT_DATA_LENGTH`"]
    #[inline]
    pub fn is_8_bit_data_length(&self) -> bool {
        *self == DATALENR::_8_BIT_DATA_LENGTH
    }
    #[doc = "Checks if the value of the field is `_9_BIT_DATA_LENGTH`"]
    #[inline]
    pub fn is_9_bit_data_length(&self) -> bool {
        *self == DATALENR::_9_BIT_DATA_LENGTH
    }
}
#[doc = "Possible values of the field `PARITYSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITYSELR {
    #[doc = "No parity."]
    NO_PARITY,
    #[doc = "Even parity. Adds a bit to each character such that the number of 1s in a transmitted character is even, and the number of 1s in a received character is expected to be even."]
    EVEN_PARITY,
    #[doc = "Odd parity. Adds a bit to each character such that the number of 1s in a transmitted character is odd, and the number of 1s in a received character is expected to be odd."]
    ODD_PARITY,
}
impl PARITYSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PARITYSELR::NO_PARITY => 0,
            PARITYSELR::EVEN_PARITY => 2,
            PARITYSELR::ODD_PARITY => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PARITYSELR {
        match value {
            0 => PARITYSELR::NO_PARITY,
            2 => PARITYSELR::EVEN_PARITY,
            3 => PARITYSELR::ODD_PARITY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_PARITY`"]
    #[inline]
    pub fn is_no_parity(&self) -> bool {
        *self == PARITYSELR::NO_PARITY
    }
    #[doc = "Checks if the value of the field is `EVEN_PARITY`"]
    #[inline]
    pub fn is_even_parity(&self) -> bool {
        *self == PARITYSELR::EVEN_PARITY
    }
    #[doc = "Checks if the value of the field is `ODD_PARITY`"]
    #[inline]
    pub fn is_odd_parity(&self) -> bool {
        *self == PARITYSELR::ODD_PARITY
    }
}
#[doc = "Possible values of the field `STOPLEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPLENR {
    #[doc = "1 stop bit."]
    _1_STOP_BIT,
    #[doc = "2 stop bits. This setting should only be used for asynchronous communication."]
    _2_STOP_BITS,
}
impl STOPLENR {
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
            STOPLENR::_1_STOP_BIT => false,
            STOPLENR::_2_STOP_BITS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOPLENR {
        match value {
            false => STOPLENR::_1_STOP_BIT,
            true => STOPLENR::_2_STOP_BITS,
        }
    }
    #[doc = "Checks if the value of the field is `_1_STOP_BIT`"]
    #[inline]
    pub fn is_1_stop_bit(&self) -> bool {
        *self == STOPLENR::_1_STOP_BIT
    }
    #[doc = "Checks if the value of the field is `_2_STOP_BITS`"]
    #[inline]
    pub fn is_2_stop_bits(&self) -> bool {
        *self == STOPLENR::_2_STOP_BITS
    }
}
#[doc = "Possible values of the field `CTSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSENR {
    #[doc = "No flow control. The transmitter does not receive any automatic flow control signal."]
    NO_FLOW_CONTROL,
    #[doc = "Flow control enabled. The transmitter uses  the CTS input (or RTS output in loopback mode) for flow control purposes."]
    FLOW_CONTROL_ENABLED,
}
impl CTSENR {
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
            CTSENR::NO_FLOW_CONTROL => false,
            CTSENR::FLOW_CONTROL_ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTSENR {
        match value {
            false => CTSENR::NO_FLOW_CONTROL,
            true => CTSENR::FLOW_CONTROL_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLOW_CONTROL`"]
    #[inline]
    pub fn is_no_flow_control(&self) -> bool {
        *self == CTSENR::NO_FLOW_CONTROL
    }
    #[doc = "Checks if the value of the field is `FLOW_CONTROL_ENABLED`"]
    #[inline]
    pub fn is_flow_control_enabled(&self) -> bool {
        *self == CTSENR::FLOW_CONTROL_ENABLED
    }
}
#[doc = "Possible values of the field `SYNCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCENR {
    #[doc = "Asynchronous mode is selected."]
    ASYNCHRONOUS_MODE_IS,
    #[doc = "Synchronous mode is selected."]
    SYNCHRONOUS_MODE_IS,
}
impl SYNCENR {
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
            SYNCENR::ASYNCHRONOUS_MODE_IS => false,
            SYNCENR::SYNCHRONOUS_MODE_IS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNCENR {
        match value {
            false => SYNCENR::ASYNCHRONOUS_MODE_IS,
            true => SYNCENR::SYNCHRONOUS_MODE_IS,
        }
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS_MODE_IS`"]
    #[inline]
    pub fn is_asynchronous_mode_is(&self) -> bool {
        *self == SYNCENR::ASYNCHRONOUS_MODE_IS
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS_MODE_IS`"]
    #[inline]
    pub fn is_synchronous_mode_is(&self) -> bool {
        *self == SYNCENR::SYNCHRONOUS_MODE_IS
    }
}
#[doc = "Possible values of the field `CLKPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKPOLR {
    #[doc = "Falling edge. Un_RXD is sampled on the falling edge of SCLK."]
    FALLING_EDGE,
    #[doc = "Rising edge. Un_RXD is sampled on the rising edge of SCLK."]
    RISING_EDGE,
}
impl CLKPOLR {
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
            CLKPOLR::FALLING_EDGE => false,
            CLKPOLR::RISING_EDGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKPOLR {
        match value {
            false => CLKPOLR::FALLING_EDGE,
            true => CLKPOLR::RISING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == CLKPOLR::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == CLKPOLR::RISING_EDGE
    }
}
#[doc = "Possible values of the field `SYNCMST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCMSTR {
    #[doc = "Slave. When synchronous mode is enabled, the USART is a slave."]
    SLAVE,
    #[doc = "Master. When synchronous mode is enabled, the USART is a master."]
    MASTER,
}
impl SYNCMSTR {
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
            SYNCMSTR::SLAVE => false,
            SYNCMSTR::MASTER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNCMSTR {
        match value {
            false => SYNCMSTR::SLAVE,
            true => SYNCMSTR::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline]
    pub fn is_slave(&self) -> bool {
        *self == SYNCMSTR::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline]
    pub fn is_master(&self) -> bool {
        *self == SYNCMSTR::MASTER
    }
}
#[doc = "Possible values of the field `LOOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPR {
    #[doc = "Normal operation."]
    NORMAL_OPERATION,
    #[doc = "Loopback mode. This provides a mechanism to perform diagnostic loopback testing for USART data. Serial data from the transmitter (Un_TXD) is connected internally to serial input of the receive (Un_RXD). Un_TXD and Un_RTS activity will also appear on external pins if these functions are configured to appear on device pins. The receiver RTS signal is also looped back to CTS and performs flow control if enabled by CTSEN."]
    LOOPBACK_MODE,
}
impl LOOPR {
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
            LOOPR::NORMAL_OPERATION => false,
            LOOPR::LOOPBACK_MODE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOOPR {
        match value {
            false => LOOPR::NORMAL_OPERATION,
            true => LOOPR::LOOPBACK_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_OPERATION`"]
    #[inline]
    pub fn is_normal_operation(&self) -> bool {
        *self == LOOPR::NORMAL_OPERATION
    }
    #[doc = "Checks if the value of the field is `LOOPBACK_MODE`"]
    #[inline]
    pub fn is_loopback_mode(&self) -> bool {
        *self == LOOPR::LOOPBACK_MODE
    }
}
#[doc = "Possible values of the field `OETA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OETAR {
    #[doc = "Deasserted. If selected by OESEL, the Output Enable signal deasserted at the end of the last stop bit of a transmission."]
    DEASSERTED,
    #[doc = "Asserted. If selected by OESEL, the Output Enable signal remains asserted for 1 character time after then end the last stop bit of a transmission. OE will also remain asserted if another transmit begins before it is deasserted."]
    ASSERTED,
}
impl OETAR {
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
            OETAR::DEASSERTED => false,
            OETAR::ASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OETAR {
        match value {
            false => OETAR::DEASSERTED,
            true => OETAR::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `DEASSERTED`"]
    #[inline]
    pub fn is_deasserted(&self) -> bool {
        *self == OETAR::DEASSERTED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline]
    pub fn is_asserted(&self) -> bool {
        *self == OETAR::ASSERTED
    }
}
#[doc = "Possible values of the field `AUTOADDR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOADDRR {
    #[doc = "Disabled. When addressing is enabled by ADDRDET, address matching is done by software. This provides the possibility of versatile addressing (e.g. respond to more than one address)."]
    DISABLED,
    #[doc = "Enabled. When addressing is enabled by ADDRDET, address matching is done by hardware, using the value in the ADDR register as the address to match."]
    ENABLED,
}
impl AUTOADDRR {
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
            AUTOADDRR::DISABLED => false,
            AUTOADDRR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUTOADDRR {
        match value {
            false => AUTOADDRR::DISABLED,
            true => AUTOADDRR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == AUTOADDRR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == AUTOADDRR::ENABLED
    }
}
#[doc = "Possible values of the field `OESEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OESELR {
    #[doc = "Flow control. The RTS signal is used as the standard flow control function."]
    FLOW_CONTROL,
    #[doc = "Output enable. The RTS signal is taken over in order to provide an output enable signal to control an RS-485 transceiver."]
    OUTPUT_ENABLE,
}
impl OESELR {
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
            OESELR::FLOW_CONTROL => false,
            OESELR::OUTPUT_ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OESELR {
        match value {
            false => OESELR::FLOW_CONTROL,
            true => OESELR::OUTPUT_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `FLOW_CONTROL`"]
    #[inline]
    pub fn is_flow_control(&self) -> bool {
        *self == OESELR::FLOW_CONTROL
    }
    #[doc = "Checks if the value of the field is `OUTPUT_ENABLE`"]
    #[inline]
    pub fn is_output_enable(&self) -> bool {
        *self == OESELR::OUTPUT_ENABLE
    }
}
#[doc = "Possible values of the field `OEPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OEPOLR {
    #[doc = "Low. If selected by OESEL, the output enable is active low."]
    LOW,
    #[doc = "High. If selected by OESEL, the output enable is active high."]
    HIGH,
}
impl OEPOLR {
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
            OEPOLR::LOW => false,
            OEPOLR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OEPOLR {
        match value {
            false => OEPOLR::LOW,
            true => OEPOLR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == OEPOLR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == OEPOLR::HIGH
    }
}
#[doc = "Possible values of the field `RXPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPOLR {
    #[doc = "Not changed. The RX signal is used as it arrives from the pin. This means that the RX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1."]
    NOT_CHANGED,
    #[doc = "Inverted. The RX signal is inverted before being used by the UART. This means that the RX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0."]
    INVERTED,
}
impl RXPOLR {
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
            RXPOLR::NOT_CHANGED => false,
            RXPOLR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXPOLR {
        match value {
            false => RXPOLR::NOT_CHANGED,
            true => RXPOLR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_CHANGED`"]
    #[inline]
    pub fn is_not_changed(&self) -> bool {
        *self == RXPOLR::NOT_CHANGED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == RXPOLR::INVERTED
    }
}
#[doc = "Possible values of the field `TXPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPOLR {
    #[doc = "Not changed. The TX signal is sent out without change. This means that the TX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1."]
    NOT_CHANGED,
    #[doc = "Inverted. The TX signal is inverted by the UART before being sent out. This means that the TX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0."]
    INVERTED,
}
impl TXPOLR {
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
            TXPOLR::NOT_CHANGED => false,
            TXPOLR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXPOLR {
        match value {
            false => TXPOLR::NOT_CHANGED,
            true => TXPOLR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_CHANGED`"]
    #[inline]
    pub fn is_not_changed(&self) -> bool {
        *self == TXPOLR::NOT_CHANGED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == TXPOLR::INVERTED
    }
}
#[doc = "Values that can be written to the field `ENABLE`"]
pub enum ENABLEW {
    #[doc = "Disabled. The USART is disabled and the internal state machine and counters are reset. While Enable = 0, all USART interrupts and DMA transfers are disabled. When Enable is set again, CFG and most other control bits remain unchanged. For instance, when re-enabled, the USART will immediately generate a TxRdy interrupt (if  enabled in the INTENSET register) or a DMA transfer request because the transmitter has been reset and is therefore available."]
    DISABLED,
    #[doc = "Enabled. The USART is enabled for operation."]
    ENABLED,
}
impl ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENABLEW::DISABLED => false,
            ENABLEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The USART is disabled and the internal state machine and counters are reset. While Enable = 0, all USART interrupts and DMA transfers are disabled. When Enable is set again, CFG and most other control bits remain unchanged. For instance, when re-enabled, the USART will immediately generate a TxRdy interrupt (if enabled in the INTENSET register) or a DMA transfer request because the transmitter has been reset and is therefore available."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLEW::DISABLED)
    }
    #[doc = "Enabled. The USART is enabled for operation."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLEW::ENABLED)
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
#[doc = "Values that can be written to the field `DATALEN`"]
pub enum DATALENW {
    #[doc = "7 bit Data length."]
    _7_BIT_DATA_LENGTH,
    #[doc = "8 bit Data length."]
    _8_BIT_DATA_LENGTH,
    #[doc = "9 bit data length. The 9th bit is commonly used for addressing in multidrop mode. See the ADDRDET bit in the CTL register."]
    _9_BIT_DATA_LENGTH,
}
impl DATALENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DATALENW::_7_BIT_DATA_LENGTH => 0,
            DATALENW::_8_BIT_DATA_LENGTH => 1,
            DATALENW::_9_BIT_DATA_LENGTH => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATALENW<'a> {
    w: &'a mut W,
}
impl<'a> _DATALENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATALENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "7 bit Data length."]
    #[inline]
    pub fn _7_bit_data_length(self) -> &'a mut W {
        self.variant(DATALENW::_7_BIT_DATA_LENGTH)
    }
    #[doc = "8 bit Data length."]
    #[inline]
    pub fn _8_bit_data_length(self) -> &'a mut W {
        self.variant(DATALENW::_8_BIT_DATA_LENGTH)
    }
    #[doc = "9 bit data length. The 9th bit is commonly used for addressing in multidrop mode. See the ADDRDET bit in the CTL register."]
    #[inline]
    pub fn _9_bit_data_length(self) -> &'a mut W {
        self.variant(DATALENW::_9_BIT_DATA_LENGTH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PARITYSEL`"]
pub enum PARITYSELW {
    #[doc = "No parity."]
    NO_PARITY,
    #[doc = "Even parity. Adds a bit to each character such that the number of 1s in a transmitted character is even, and the number of 1s in a received character is expected to be even."]
    EVEN_PARITY,
    #[doc = "Odd parity. Adds a bit to each character such that the number of 1s in a transmitted character is odd, and the number of 1s in a received character is expected to be odd."]
    ODD_PARITY,
}
impl PARITYSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PARITYSELW::NO_PARITY => 0,
            PARITYSELW::EVEN_PARITY => 2,
            PARITYSELW::ODD_PARITY => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PARITYSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PARITYSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PARITYSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No parity."]
    #[inline]
    pub fn no_parity(self) -> &'a mut W {
        self.variant(PARITYSELW::NO_PARITY)
    }
    #[doc = "Even parity. Adds a bit to each character such that the number of 1s in a transmitted character is even, and the number of 1s in a received character is expected to be even."]
    #[inline]
    pub fn even_parity(self) -> &'a mut W {
        self.variant(PARITYSELW::EVEN_PARITY)
    }
    #[doc = "Odd parity. Adds a bit to each character such that the number of 1s in a transmitted character is odd, and the number of 1s in a received character is expected to be odd."]
    #[inline]
    pub fn odd_parity(self) -> &'a mut W {
        self.variant(PARITYSELW::ODD_PARITY)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STOPLEN`"]
pub enum STOPLENW {
    #[doc = "1 stop bit."]
    _1_STOP_BIT,
    #[doc = "2 stop bits. This setting should only be used for asynchronous communication."]
    _2_STOP_BITS,
}
impl STOPLENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOPLENW::_1_STOP_BIT => false,
            STOPLENW::_2_STOP_BITS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPLENW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPLENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPLENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "1 stop bit."]
    #[inline]
    pub fn _1_stop_bit(self) -> &'a mut W {
        self.variant(STOPLENW::_1_STOP_BIT)
    }
    #[doc = "2 stop bits. This setting should only be used for asynchronous communication."]
    #[inline]
    pub fn _2_stop_bits(self) -> &'a mut W {
        self.variant(STOPLENW::_2_STOP_BITS)
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
#[doc = "Values that can be written to the field `CTSEN`"]
pub enum CTSENW {
    #[doc = "No flow control. The transmitter does not receive any automatic flow control signal."]
    NO_FLOW_CONTROL,
    #[doc = "Flow control enabled. The transmitter uses  the CTS input (or RTS output in loopback mode) for flow control purposes."]
    FLOW_CONTROL_ENABLED,
}
impl CTSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTSENW::NO_FLOW_CONTROL => false,
            CTSENW::FLOW_CONTROL_ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTSENW<'a> {
    w: &'a mut W,
}
impl<'a> _CTSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No flow control. The transmitter does not receive any automatic flow control signal."]
    #[inline]
    pub fn no_flow_control(self) -> &'a mut W {
        self.variant(CTSENW::NO_FLOW_CONTROL)
    }
    #[doc = "Flow control enabled. The transmitter uses the CTS input (or RTS output in loopback mode) for flow control purposes."]
    #[inline]
    pub fn flow_control_enabled(self) -> &'a mut W {
        self.variant(CTSENW::FLOW_CONTROL_ENABLED)
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
#[doc = "Values that can be written to the field `SYNCEN`"]
pub enum SYNCENW {
    #[doc = "Asynchronous mode is selected."]
    ASYNCHRONOUS_MODE_IS,
    #[doc = "Synchronous mode is selected."]
    SYNCHRONOUS_MODE_IS,
}
impl SYNCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYNCENW::ASYNCHRONOUS_MODE_IS => false,
            SYNCENW::SYNCHRONOUS_MODE_IS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Asynchronous mode is selected."]
    #[inline]
    pub fn asynchronous_mode_is(self) -> &'a mut W {
        self.variant(SYNCENW::ASYNCHRONOUS_MODE_IS)
    }
    #[doc = "Synchronous mode is selected."]
    #[inline]
    pub fn synchronous_mode_is(self) -> &'a mut W {
        self.variant(SYNCENW::SYNCHRONOUS_MODE_IS)
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
#[doc = "Values that can be written to the field `CLKPOL`"]
pub enum CLKPOLW {
    #[doc = "Falling edge. Un_RXD is sampled on the falling edge of SCLK."]
    FALLING_EDGE,
    #[doc = "Rising edge. Un_RXD is sampled on the rising edge of SCLK."]
    RISING_EDGE,
}
impl CLKPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKPOLW::FALLING_EDGE => false,
            CLKPOLW::RISING_EDGE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Falling edge. Un_RXD is sampled on the falling edge of SCLK."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(CLKPOLW::FALLING_EDGE)
    }
    #[doc = "Rising edge. Un_RXD is sampled on the rising edge of SCLK."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(CLKPOLW::RISING_EDGE)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYNCMST`"]
pub enum SYNCMSTW {
    #[doc = "Slave. When synchronous mode is enabled, the USART is a slave."]
    SLAVE,
    #[doc = "Master. When synchronous mode is enabled, the USART is a master."]
    MASTER,
}
impl SYNCMSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYNCMSTW::SLAVE => false,
            SYNCMSTW::MASTER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCMSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCMSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCMSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slave. When synchronous mode is enabled, the USART is a slave."]
    #[inline]
    pub fn slave(self) -> &'a mut W {
        self.variant(SYNCMSTW::SLAVE)
    }
    #[doc = "Master. When synchronous mode is enabled, the USART is a master."]
    #[inline]
    pub fn master(self) -> &'a mut W {
        self.variant(SYNCMSTW::MASTER)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LOOP`"]
pub enum LOOPW {
    #[doc = "Normal operation."]
    NORMAL_OPERATION,
    #[doc = "Loopback mode. This provides a mechanism to perform diagnostic loopback testing for USART data. Serial data from the transmitter (Un_TXD) is connected internally to serial input of the receive (Un_RXD). Un_TXD and Un_RTS activity will also appear on external pins if these functions are configured to appear on device pins. The receiver RTS signal is also looped back to CTS and performs flow control if enabled by CTSEN."]
    LOOPBACK_MODE,
}
impl LOOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOOPW::NORMAL_OPERATION => false,
            LOOPW::LOOPBACK_MODE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOOPW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn normal_operation(self) -> &'a mut W {
        self.variant(LOOPW::NORMAL_OPERATION)
    }
    #[doc = "Loopback mode. This provides a mechanism to perform diagnostic loopback testing for USART data. Serial data from the transmitter (Un_TXD) is connected internally to serial input of the receive (Un_RXD). Un_TXD and Un_RTS activity will also appear on external pins if these functions are configured to appear on device pins. The receiver RTS signal is also looped back to CTS and performs flow control if enabled by CTSEN."]
    #[inline]
    pub fn loopback_mode(self) -> &'a mut W {
        self.variant(LOOPW::LOOPBACK_MODE)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OETA`"]
pub enum OETAW {
    #[doc = "Deasserted. If selected by OESEL, the Output Enable signal deasserted at the end of the last stop bit of a transmission."]
    DEASSERTED,
    #[doc = "Asserted. If selected by OESEL, the Output Enable signal remains asserted for 1 character time after then end the last stop bit of a transmission. OE will also remain asserted if another transmit begins before it is deasserted."]
    ASSERTED,
}
impl OETAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OETAW::DEASSERTED => false,
            OETAW::ASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OETAW<'a> {
    w: &'a mut W,
}
impl<'a> _OETAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OETAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Deasserted. If selected by OESEL, the Output Enable signal deasserted at the end of the last stop bit of a transmission."]
    #[inline]
    pub fn deasserted(self) -> &'a mut W {
        self.variant(OETAW::DEASSERTED)
    }
    #[doc = "Asserted. If selected by OESEL, the Output Enable signal remains asserted for 1 character time after then end the last stop bit of a transmission. OE will also remain asserted if another transmit begins before it is deasserted."]
    #[inline]
    pub fn asserted(self) -> &'a mut W {
        self.variant(OETAW::ASSERTED)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUTOADDR`"]
pub enum AUTOADDRW {
    #[doc = "Disabled. When addressing is enabled by ADDRDET, address matching is done by software. This provides the possibility of versatile addressing (e.g. respond to more than one address)."]
    DISABLED,
    #[doc = "Enabled. When addressing is enabled by ADDRDET, address matching is done by hardware, using the value in the ADDR register as the address to match."]
    ENABLED,
}
impl AUTOADDRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUTOADDRW::DISABLED => false,
            AUTOADDRW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUTOADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOADDRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUTOADDRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. When addressing is enabled by ADDRDET, address matching is done by software. This provides the possibility of versatile addressing (e.g. respond to more than one address)."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AUTOADDRW::DISABLED)
    }
    #[doc = "Enabled. When addressing is enabled by ADDRDET, address matching is done by hardware, using the value in the ADDR register as the address to match."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AUTOADDRW::ENABLED)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OESEL`"]
pub enum OESELW {
    #[doc = "Flow control. The RTS signal is used as the standard flow control function."]
    FLOW_CONTROL,
    #[doc = "Output enable. The RTS signal is taken over in order to provide an output enable signal to control an RS-485 transceiver."]
    OUTPUT_ENABLE,
}
impl OESELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OESELW::FLOW_CONTROL => false,
            OESELW::OUTPUT_ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OESELW<'a> {
    w: &'a mut W,
}
impl<'a> _OESELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OESELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Flow control. The RTS signal is used as the standard flow control function."]
    #[inline]
    pub fn flow_control(self) -> &'a mut W {
        self.variant(OESELW::FLOW_CONTROL)
    }
    #[doc = "Output enable. The RTS signal is taken over in order to provide an output enable signal to control an RS-485 transceiver."]
    #[inline]
    pub fn output_enable(self) -> &'a mut W {
        self.variant(OESELW::OUTPUT_ENABLE)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OEPOL`"]
pub enum OEPOLW {
    #[doc = "Low. If selected by OESEL, the output enable is active low."]
    LOW,
    #[doc = "High. If selected by OESEL, the output enable is active high."]
    HIGH,
}
impl OEPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OEPOLW::LOW => false,
            OEPOLW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OEPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _OEPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OEPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low. If selected by OESEL, the output enable is active low."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(OEPOLW::LOW)
    }
    #[doc = "High. If selected by OESEL, the output enable is active high."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(OEPOLW::HIGH)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXPOL`"]
pub enum RXPOLW {
    #[doc = "Not changed. The RX signal is used as it arrives from the pin. This means that the RX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1."]
    NOT_CHANGED,
    #[doc = "Inverted. The RX signal is inverted before being used by the UART. This means that the RX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0."]
    INVERTED,
}
impl RXPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXPOLW::NOT_CHANGED => false,
            RXPOLW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _RXPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not changed. The RX signal is used as it arrives from the pin. This means that the RX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1."]
    #[inline]
    pub fn not_changed(self) -> &'a mut W {
        self.variant(RXPOLW::NOT_CHANGED)
    }
    #[doc = "Inverted. The RX signal is inverted before being used by the UART. This means that the RX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(RXPOLW::INVERTED)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXPOL`"]
pub enum TXPOLW {
    #[doc = "Not changed. The TX signal is sent out without change. This means that the TX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1."]
    NOT_CHANGED,
    #[doc = "Inverted. The TX signal is inverted by the UART before being sent out. This means that the TX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0."]
    INVERTED,
}
impl TXPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXPOLW::NOT_CHANGED => false,
            TXPOLW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _TXPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not changed. The TX signal is sent out without change. This means that the TX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1."]
    #[inline]
    pub fn not_changed(self) -> &'a mut W {
        self.variant(TXPOLW::NOT_CHANGED)
    }
    #[doc = "Inverted. The TX signal is inverted by the UART before being sent out. This means that the TX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TXPOLW::INVERTED)
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
        const OFFSET: u8 = 23;
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
    #[doc = "Bit 0 - USART Enable."]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:3 - Selects the data size for the USART."]
    #[inline]
    pub fn datalen(&self) -> DATALENR {
        DATALENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Selects what type of parity is used by the USART."]
    #[inline]
    pub fn paritysel(&self) -> PARITYSELR {
        PARITYSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Number of stop bits appended to transmitted data. Only a single stop bit is required for received data."]
    #[inline]
    pub fn stoplen(&self) -> STOPLENR {
        STOPLENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - CTS Enable. Determines whether CTS is used for flow control. CTS can be from the input pin, or from the USART's own RTS if loopback mode is enabled."]
    #[inline]
    pub fn ctsen(&self) -> CTSENR {
        CTSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Selects synchronous or asynchronous operation."]
    #[inline]
    pub fn syncen(&self) -> SYNCENR {
        SYNCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Selects the clock polarity and sampling edge of received data in synchronous mode."]
    #[inline]
    pub fn clkpol(&self) -> CLKPOLR {
        CLKPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Synchronous mode Master select."]
    #[inline]
    pub fn syncmst(&self) -> SYNCMSTR {
        SYNCMSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Selects data loopback mode."]
    #[inline]
    pub fn loop_(&self) -> LOOPR {
        LOOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Output Enable Turnaround time enable for RS-485 operation."]
    #[inline]
    pub fn oeta(&self) -> OETAR {
        OETAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Automatic Address matching enable."]
    #[inline]
    pub fn autoaddr(&self) -> AUTOADDRR {
        AUTOADDRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Output Enable Select."]
    #[inline]
    pub fn oesel(&self) -> OESELR {
        OESELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Output Enable Polarity."]
    #[inline]
    pub fn oepol(&self) -> OEPOLR {
        OEPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Receive data polarity."]
    #[inline]
    pub fn rxpol(&self) -> RXPOLR {
        RXPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Transmit data polarity."]
    #[inline]
    pub fn txpol(&self) -> TXPOLR {
        TXPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
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
    #[doc = "Bit 0 - USART Enable."]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bits 2:3 - Selects the data size for the USART."]
    #[inline]
    pub fn datalen(&mut self) -> _DATALENW {
        _DATALENW { w: self }
    }
    #[doc = "Bits 4:5 - Selects what type of parity is used by the USART."]
    #[inline]
    pub fn paritysel(&mut self) -> _PARITYSELW {
        _PARITYSELW { w: self }
    }
    #[doc = "Bit 6 - Number of stop bits appended to transmitted data. Only a single stop bit is required for received data."]
    #[inline]
    pub fn stoplen(&mut self) -> _STOPLENW {
        _STOPLENW { w: self }
    }
    #[doc = "Bit 9 - CTS Enable. Determines whether CTS is used for flow control. CTS can be from the input pin, or from the USART's own RTS if loopback mode is enabled."]
    #[inline]
    pub fn ctsen(&mut self) -> _CTSENW {
        _CTSENW { w: self }
    }
    #[doc = "Bit 11 - Selects synchronous or asynchronous operation."]
    #[inline]
    pub fn syncen(&mut self) -> _SYNCENW {
        _SYNCENW { w: self }
    }
    #[doc = "Bit 12 - Selects the clock polarity and sampling edge of received data in synchronous mode."]
    #[inline]
    pub fn clkpol(&mut self) -> _CLKPOLW {
        _CLKPOLW { w: self }
    }
    #[doc = "Bit 14 - Synchronous mode Master select."]
    #[inline]
    pub fn syncmst(&mut self) -> _SYNCMSTW {
        _SYNCMSTW { w: self }
    }
    #[doc = "Bit 15 - Selects data loopback mode."]
    #[inline]
    pub fn loop_(&mut self) -> _LOOPW {
        _LOOPW { w: self }
    }
    #[doc = "Bit 18 - Output Enable Turnaround time enable for RS-485 operation."]
    #[inline]
    pub fn oeta(&mut self) -> _OETAW {
        _OETAW { w: self }
    }
    #[doc = "Bit 19 - Automatic Address matching enable."]
    #[inline]
    pub fn autoaddr(&mut self) -> _AUTOADDRW {
        _AUTOADDRW { w: self }
    }
    #[doc = "Bit 20 - Output Enable Select."]
    #[inline]
    pub fn oesel(&mut self) -> _OESELW {
        _OESELW { w: self }
    }
    #[doc = "Bit 21 - Output Enable Polarity."]
    #[inline]
    pub fn oepol(&mut self) -> _OEPOLW {
        _OEPOLW { w: self }
    }
    #[doc = "Bit 22 - Receive data polarity."]
    #[inline]
    pub fn rxpol(&mut self) -> _RXPOLW {
        _RXPOLW { w: self }
    }
    #[doc = "Bit 23 - Transmit data polarity."]
    #[inline]
    pub fn txpol(&mut self) -> _TXPOLW {
        _TXPOLW { w: self }
    }
}
