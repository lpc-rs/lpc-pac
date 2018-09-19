#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - A/D Control Register. Contains the clock divide value, enable bits for each sequence and the A/D power-down bit."]
    pub ctrl: CTRL,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - A/D Conversion Sequence-A control Register: Controls triggering and channel selection for conversion sequence-A. Also specifies interrupt mode for sequence-A."]
    pub seqa_ctrl: SEQA_CTRL,
    #[doc = "0x0c - A/D Conversion Sequence-B Control Register: Controls triggering and channel selection for conversion sequence-B. Also specifies interrupt mode for sequence-B."]
    pub seqb_ctrl: SEQB_CTRL,
    #[doc = "0x10 - A/D Sequence-A Global Data Register. This register contains the result of the most recent A/D conversion performed under sequence-A"]
    pub seqa_gdat: SEQA_GDAT,
    #[doc = "0x14 - A/D Sequence-B Global Data Register. This register contains the result of the most recent A/D conversion performed under sequence-B"]
    pub seqb_gdat: SEQB_GDAT,
    _reserved1: [u8; 8usize],
    #[doc = "0x20 - A/D Channel 0 Data Register. This register contains the result of the most recent conversion completed on channel 0."]
    pub dat: [DAT; 12],
    #[doc = "0x50 - A/D Low Compare Threshold Register 0 : Contains the lower threshold level for automatic threshold comparison for any channels linked to threshold pair 0."]
    pub thr0_low: THR0_LOW,
    #[doc = "0x54 - A/D Low Compare Threshold Register 1: Contains the lower threshold level for automatic threshold comparison for any channels linked to threshold pair 1."]
    pub thr1_low: THR1_LOW,
    #[doc = "0x58 - A/D High Compare Threshold Register 0: Contains the upper threshold level for automatic threshold comparison for any channels linked to threshold pair 0."]
    pub thr0_high: THR0_HIGH,
    #[doc = "0x5c - A/D High Compare Threshold Register 1: Contains the upper threshold level for automatic threshold comparison for any channels linked to threshold pair 1."]
    pub thr1_high: THR1_HIGH,
    #[doc = "0x60 - A/D Channel-Threshold Select Register. Specifies which set of threshold compare registers are to be used for each channel"]
    pub chan_thrsel: CHAN_THRSEL,
    #[doc = "0x64 - A/D Interrupt Enable Register. This register contains enable bits that enable the sequence-A, sequence-B, threshold compare and data overrun interrupts to be generated."]
    pub inten: INTEN,
    #[doc = "0x68 - A/D Flags Register. Contains the four interrupt request flags and the individual component overrun and threshold-compare flags. (The overrun bits replicate information stored in the result registers)."]
    pub flags: FLAGS,
    #[doc = "0x6c - ADC trim register."]
    pub trm: TRM,
}
#[doc = "A/D Control Register. Contains the clock divide value, enable bits for each sequence and the A/D power-down bit."]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A/D Control Register. Contains the clock divide value, enable bits for each sequence and the A/D power-down bit."]
pub mod ctrl;
#[doc = "A/D Conversion Sequence-A control Register: Controls triggering and channel selection for conversion sequence-A. Also specifies interrupt mode for sequence-A."]
pub struct SEQA_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A/D Conversion Sequence-A control Register: Controls triggering and channel selection for conversion sequence-A. Also specifies interrupt mode for sequence-A."]
pub mod seqa_ctrl;
#[doc = "A/D Conversion Sequence-B Control Register: Controls triggering and channel selection for conversion sequence-B. Also specifies interrupt mode for sequence-B."]
pub struct SEQB_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A/D Conversion Sequence-B Control Register: Controls triggering and channel selection for conversion sequence-B. Also specifies interrupt mode for sequence-B."]
pub mod seqb_ctrl;
#[doc = "A/D Sequence-A Global Data Register. This register contains the result of the most recent A/D conversion performed under sequence-A"]
pub struct SEQA_GDAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A/D Sequence-A Global Data Register. This register contains the result of the most recent A/D conversion performed under sequence-A"]
pub mod seqa_gdat;
#[doc = "A/D Sequence-B Global Data Register. This register contains the result of the most recent A/D conversion performed under sequence-B"]
pub struct SEQB_GDAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A/D Sequence-B Global Data Register. This register contains the result of the most recent A/D conversion performed under sequence-B"]
pub mod seqb_gdat;
#[doc = "A/D Channel 0 Data Register. This register contains the result of the most recent conversion completed on channel 0."]
pub struct DAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A/D Channel 0 Data Register. This register contains the result of the most recent conversion completed on channel 0."]
pub mod dat;
#[doc = "A/D Low Compare Threshold Register 0 : Contains the lower threshold level for automatic threshold comparison for any channels linked to threshold pair 0."]
pub struct THR0_LOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A/D Low Compare Threshold Register 0 : Contains the lower threshold level for automatic threshold comparison for any channels linked to threshold pair 0."]
pub mod thr0_low;
#[doc = "A/D Low Compare Threshold Register 1: Contains the lower threshold level for automatic threshold comparison for any channels linked to threshold pair 1."]
pub struct THR1_LOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A/D Low Compare Threshold Register 1: Contains the lower threshold level for automatic threshold comparison for any channels linked to threshold pair 1."]
pub mod thr1_low;
#[doc = "A/D High Compare Threshold Register 0: Contains the upper threshold level for automatic threshold comparison for any channels linked to threshold pair 0."]
pub struct THR0_HIGH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A/D High Compare Threshold Register 0: Contains the upper threshold level for automatic threshold comparison for any channels linked to threshold pair 0."]
pub mod thr0_high;
#[doc = "A/D High Compare Threshold Register 1: Contains the upper threshold level for automatic threshold comparison for any channels linked to threshold pair 1."]
pub struct THR1_HIGH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A/D High Compare Threshold Register 1: Contains the upper threshold level for automatic threshold comparison for any channels linked to threshold pair 1."]
pub mod thr1_high;
#[doc = "A/D Channel-Threshold Select Register. Specifies which set of threshold compare registers are to be used for each channel"]
pub struct CHAN_THRSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A/D Channel-Threshold Select Register. Specifies which set of threshold compare registers are to be used for each channel"]
pub mod chan_thrsel;
#[doc = "A/D Interrupt Enable Register. This register contains enable bits that enable the sequence-A, sequence-B, threshold compare and data overrun interrupts to be generated."]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A/D Interrupt Enable Register. This register contains enable bits that enable the sequence-A, sequence-B, threshold compare and data overrun interrupts to be generated."]
pub mod inten;
#[doc = "A/D Flags Register. Contains the four interrupt request flags and the individual component overrun and threshold-compare flags. (The overrun bits replicate information stored in the result registers)."]
pub struct FLAGS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A/D Flags Register. Contains the four interrupt request flags and the individual component overrun and threshold-compare flags. (The overrun bits replicate information stored in the result registers)."]
pub mod flags;
#[doc = "ADC trim register."]
pub struct TRM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC trim register."]
pub mod trm;
