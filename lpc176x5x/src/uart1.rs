#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "DLAB =1. Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. DLAB =0. Transmit Holding Register. The next character to be transmitted is written here. DLAB =0 Receiver Buffer Register. Contains the next received character to be read."]
    pub dll: DLL_UNION,
    #[doc = "DLAB =0. Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential UART1 interrupts. DLAB =1. Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider."]
    pub dlm: DLM_UNION,
    #[doc = "FIFO Control Register. Controls UART1 FIFO usage and modes. Interrupt ID Register. Identifies which interrupt(s) are pending."]
    pub fcr: FCR_UNION,
    #[doc = "0x0c - Line Control Register. Contains controls for frame formatting and break generation."]
    pub lcr: LCR,
    #[doc = "0x10 - Modem Control Register. Contains controls for flow control handshaking and loopback mode."]
    pub mcr: MCR,
    #[doc = "0x14 - Line Status Register. Contains flags for transmit and receive status, including line errors."]
    pub lsr: LSR,
    #[doc = "0x18 - Modem Status Register. Contains handshake signal status flags."]
    pub msr: MSR,
    #[doc = "0x1c - Scratch Pad Register. 8-bit temporary storage for software."]
    pub scr: SCR,
    #[doc = "0x20 - Auto-baud Control Register. Contains controls for the auto-baud feature."]
    pub acr: ACR,
    _reserved9: [u8; 4usize],
    #[doc = "0x28 - Fractional Divider Register. Generates a clock input for the baud rate divider."]
    pub fdr: FDR,
    _reserved10: [u8; 4usize],
    #[doc = "0x30 - Transmit Enable Register. Turns off UART transmitter for use with software flow control."]
    pub ter: TER,
    _reserved11: [u8; 24usize],
    #[doc = "0x4c - RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes."]
    pub rs485ctrl: RS485CTRL,
    #[doc = "0x50 - RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode."]
    pub rs485adrmatch: RS485ADRMATCH,
    #[doc = "0x54 - RS-485/EIA-485 direction control delay."]
    pub rs485dly: RS485DLY,
}
#[doc = "DLAB =1. Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. DLAB =0. Transmit Holding Register. The next character to be transmitted is written here. DLAB =0 Receiver Buffer Register. Contains the next received character to be read."]
#[repr(C)]
pub union DLL_UNION {
    #[doc = "0x00 - DLAB =1. Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider."]
    pub dll: DLL,
    #[doc = "0x00 - DLAB =0. Transmit Holding Register. The next character to be transmitted is written here."]
    pub thr: THR,
    #[doc = "0x00 - DLAB =0 Receiver Buffer Register. Contains the next received character to be read."]
    pub rbr: RBR,
}
#[doc = "DLAB =0. Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential UART1 interrupts. DLAB =1. Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider."]
#[repr(C)]
pub union DLM_UNION {
    #[doc = "0x04 - DLAB =0. Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential UART1 interrupts."]
    pub ier: IER,
    #[doc = "0x04 - DLAB =1. Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider."]
    pub dlm: DLM,
}
#[doc = "FIFO Control Register. Controls UART1 FIFO usage and modes. Interrupt ID Register. Identifies which interrupt(s) are pending."]
#[repr(C)]
pub union FCR_UNION {
    #[doc = "0x08 - FIFO Control Register. Controls UART1 FIFO usage and modes."]
    pub fcr: FCR,
    #[doc = "0x08 - Interrupt ID Register. Identifies which interrupt(s) are pending."]
    pub iir: IIR,
}
#[doc = "DLAB =0 Receiver Buffer Register. Contains the next received character to be read."]
pub struct RBR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DLAB =0 Receiver Buffer Register. Contains the next received character to be read."]
pub mod rbr;
#[doc = "DLAB =0. Transmit Holding Register. The next character to be transmitted is written here."]
pub struct THR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DLAB =0. Transmit Holding Register. The next character to be transmitted is written here."]
pub mod thr;
#[doc = "DLAB =1. Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider."]
pub struct DLL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DLAB =1. Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider."]
pub mod dll;
#[doc = "DLAB =1. Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider."]
pub struct DLM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DLAB =1. Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider."]
pub mod dlm;
#[doc = "DLAB =0. Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential UART1 interrupts."]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DLAB =0. Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential UART1 interrupts."]
pub mod ier;
#[doc = "Interrupt ID Register. Identifies which interrupt(s) are pending."]
pub struct IIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt ID Register. Identifies which interrupt(s) are pending."]
pub mod iir;
#[doc = "FIFO Control Register. Controls UART1 FIFO usage and modes."]
pub struct FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Control Register. Controls UART1 FIFO usage and modes."]
pub mod fcr;
#[doc = "Line Control Register. Contains controls for frame formatting and break generation."]
pub struct LCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Line Control Register. Contains controls for frame formatting and break generation."]
pub mod lcr;
#[doc = "Modem Control Register. Contains controls for flow control handshaking and loopback mode."]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Modem Control Register. Contains controls for flow control handshaking and loopback mode."]
pub mod mcr;
#[doc = "Line Status Register. Contains flags for transmit and receive status, including line errors."]
pub struct LSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Line Status Register. Contains flags for transmit and receive status, including line errors."]
pub mod lsr;
#[doc = "Modem Status Register. Contains handshake signal status flags."]
pub struct MSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Modem Status Register. Contains handshake signal status flags."]
pub mod msr;
#[doc = "Scratch Pad Register. 8-bit temporary storage for software."]
pub struct SCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scratch Pad Register. 8-bit temporary storage for software."]
pub mod scr;
#[doc = "Auto-baud Control Register. Contains controls for the auto-baud feature."]
pub struct ACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Auto-baud Control Register. Contains controls for the auto-baud feature."]
pub mod acr;
#[doc = "Fractional Divider Register. Generates a clock input for the baud rate divider."]
pub struct FDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fractional Divider Register. Generates a clock input for the baud rate divider."]
pub mod fdr;
#[doc = "Transmit Enable Register. Turns off UART transmitter for use with software flow control."]
pub struct TER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Enable Register. Turns off UART transmitter for use with software flow control."]
pub mod ter;
#[doc = "RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes."]
pub struct RS485CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes."]
pub mod rs485ctrl;
#[doc = "RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode."]
pub struct RS485ADRMATCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode."]
pub mod rs485adrmatch;
#[doc = "RS-485/EIA-485 direction control delay."]
pub struct RS485DLY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RS-485/EIA-485 direction control delay."]
pub mod rs485dly;
