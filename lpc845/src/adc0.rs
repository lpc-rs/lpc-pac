#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Control register. Contains the clock divide value, resolution selection, sampling time selection, and mode controls."]
    pub ctrl: CTRL,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - ADC Conversion Sequence-n control register: Controls triggering and channel selection for conversion sequence-n. Also specifies interrupt mode for sequence-n."]
    pub seq_ctrla: SEQ_CTRL,
    #[doc = "0x0c - ADC Conversion Sequence-n control register: Controls triggering and channel selection for conversion sequence-n. Also specifies interrupt mode for sequence-n."]
    pub seq_ctrlb: SEQ_CTRL,
    #[doc = "0x10 - ADC Sequence-n Global Data register. This register contains the result of the most recent ADC conversion performed under sequence-n."]
    pub seq_gdata: SEQ_GDAT,
    #[doc = "0x14 - ADC Sequence-n Global Data register. This register contains the result of the most recent ADC conversion performed under sequence-n."]
    pub seq_gdatb: SEQ_GDAT,
    _reserved1: [u8; 8usize],
    #[doc = "0x20 - ADC Channel N Data register. This register contains the result of the most recent conversion completed on channel N."]
    pub dat: [DAT; 12],
    #[doc = "0x50 - ADC Low Compare Threshold register 0: Contains the lower threshold level for automatic threshold comparison for any channels linked to threshold pair 0."]
    pub thr0_low: THR0_LOW,
    #[doc = "0x54 - ADC Low Compare Threshold register 1: Contains the lower threshold level for automatic threshold comparison for any channels linked to threshold pair 1."]
    pub thr1_low: THR1_LOW,
    #[doc = "0x58 - ADC High Compare Threshold register 0: Contains the upper threshold level for automatic threshold comparison for any channels linked to threshold pair 0."]
    pub thr0_high: THR0_HIGH,
    #[doc = "0x5c - ADC High Compare Threshold register 1: Contains the upper threshold level for automatic threshold comparison for any channels linked to threshold pair 1."]
    pub thr1_high: THR1_HIGH,
    #[doc = "0x60 - ADC Channel-Threshold Select register. Specifies which set of threshold compare registers are to be used for each channel"]
    pub chan_thrsel: CHAN_THRSEL,
    #[doc = "0x64 - ADC Interrupt Enable register. This register contains enable bits that enable the sequence-A, sequence-B, threshold compare and data overrun interrupts to be generated."]
    pub inten: INTEN,
    #[doc = "0x68 - ADC Flags register. Contains the four interrupt/DMA trigger flags and the individual component overrun and threshold-compare flags. (The overrun bits replicate information stored in the result registers)."]
    pub flags: FLAGS,
    #[doc = "0x6c - ADC Startup register."]
    pub trm: TRM,
}
#[doc = "ADC Control register. Contains the clock divide value, resolution selection, sampling time selection, and mode controls."]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Control register. Contains the clock divide value, resolution selection, sampling time selection, and mode controls."]
pub mod ctrl;
#[doc = "ADC Conversion Sequence-n control register: Controls triggering and channel selection for conversion sequence-n. Also specifies interrupt mode for sequence-n."]
pub struct SEQ_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Conversion Sequence-n control register: Controls triggering and channel selection for conversion sequence-n. Also specifies interrupt mode for sequence-n."]
pub mod seq_ctrl;
#[doc = "ADC Sequence-n Global Data register. This register contains the result of the most recent ADC conversion performed under sequence-n."]
pub struct SEQ_GDAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sequence-n Global Data register. This register contains the result of the most recent ADC conversion performed under sequence-n."]
pub mod seq_gdat;
#[doc = "ADC Channel N Data register. This register contains the result of the most recent conversion completed on channel N."]
pub struct DAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Channel N Data register. This register contains the result of the most recent conversion completed on channel N."]
pub mod dat;
#[doc = "ADC Low Compare Threshold register 0: Contains the lower threshold level for automatic threshold comparison for any channels linked to threshold pair 0."]
pub struct THR0_LOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Low Compare Threshold register 0: Contains the lower threshold level for automatic threshold comparison for any channels linked to threshold pair 0."]
pub mod thr0_low;
#[doc = "ADC Low Compare Threshold register 1: Contains the lower threshold level for automatic threshold comparison for any channels linked to threshold pair 1."]
pub struct THR1_LOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Low Compare Threshold register 1: Contains the lower threshold level for automatic threshold comparison for any channels linked to threshold pair 1."]
pub mod thr1_low;
#[doc = "ADC High Compare Threshold register 0: Contains the upper threshold level for automatic threshold comparison for any channels linked to threshold pair 0."]
pub struct THR0_HIGH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC High Compare Threshold register 0: Contains the upper threshold level for automatic threshold comparison for any channels linked to threshold pair 0."]
pub mod thr0_high;
#[doc = "ADC High Compare Threshold register 1: Contains the upper threshold level for automatic threshold comparison for any channels linked to threshold pair 1."]
pub struct THR1_HIGH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC High Compare Threshold register 1: Contains the upper threshold level for automatic threshold comparison for any channels linked to threshold pair 1."]
pub mod thr1_high;
#[doc = "ADC Channel-Threshold Select register. Specifies which set of threshold compare registers are to be used for each channel"]
pub struct CHAN_THRSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Channel-Threshold Select register. Specifies which set of threshold compare registers are to be used for each channel"]
pub mod chan_thrsel;
#[doc = "ADC Interrupt Enable register. This register contains enable bits that enable the sequence-A, sequence-B, threshold compare and data overrun interrupts to be generated."]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Interrupt Enable register. This register contains enable bits that enable the sequence-A, sequence-B, threshold compare and data overrun interrupts to be generated."]
pub mod inten;
#[doc = "ADC Flags register. Contains the four interrupt/DMA trigger flags and the individual component overrun and threshold-compare flags. (The overrun bits replicate information stored in the result registers)."]
pub struct FLAGS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Flags register. Contains the four interrupt/DMA trigger flags and the individual component overrun and threshold-compare flags. (The overrun bits replicate information stored in the result registers)."]
pub mod flags;
#[doc = "ADC Startup register."]
pub struct TRM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Startup register."]
pub mod trm;
