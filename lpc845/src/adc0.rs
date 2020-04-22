#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Control register. Contains the clock divide value, resolution selection, sampling time selection, and mode controls."]
    pub ctrl: CTRL,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - ADC Conversion Sequence-n control register: Controls triggering and channel selection for conversion sequence-n. Also specifies interrupt mode for sequence-n."]
    pub seq_ctrla: SEQ_CTRL,
    #[doc = "0x0c - ADC Conversion Sequence-n control register: Controls triggering and channel selection for conversion sequence-n. Also specifies interrupt mode for sequence-n."]
    pub seq_ctrlb: SEQ_CTRL,
    #[doc = "0x10 - ADC Sequence-n Global Data register. This register contains the result of the most recent ADC conversion performed under sequence-n."]
    pub seq_gdata: SEQ_GDAT,
    #[doc = "0x14 - ADC Sequence-n Global Data register. This register contains the result of the most recent ADC conversion performed under sequence-n."]
    pub seq_gdatb: SEQ_GDAT,
    _reserved5: [u8; 8usize],
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
#[doc = "ADC Control register. Contains the clock divide value, resolution selection, sampling time selection, and mode controls.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "ADC Control register. Contains the clock divide value, resolution selection, sampling time selection, and mode controls."]
pub mod ctrl;
#[doc = "ADC Conversion Sequence-n control register: Controls triggering and channel selection for conversion sequence-n. Also specifies interrupt mode for sequence-n.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_ctrl](seq_ctrl) module"]
pub type SEQ_CTRL = crate::Reg<u32, _SEQ_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ_CTRL;
#[doc = "`read()` method returns [seq_ctrl::R](seq_ctrl::R) reader structure"]
impl crate::Readable for SEQ_CTRL {}
#[doc = "`write(|w| ..)` method takes [seq_ctrl::W](seq_ctrl::W) writer structure"]
impl crate::Writable for SEQ_CTRL {}
#[doc = "ADC Conversion Sequence-n control register: Controls triggering and channel selection for conversion sequence-n. Also specifies interrupt mode for sequence-n."]
pub mod seq_ctrl;
#[doc = "ADC Sequence-n Global Data register. This register contains the result of the most recent ADC conversion performed under sequence-n.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_gdat](seq_gdat) module"]
pub type SEQ_GDAT = crate::Reg<u32, _SEQ_GDAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ_GDAT;
#[doc = "`read()` method returns [seq_gdat::R](seq_gdat::R) reader structure"]
impl crate::Readable for SEQ_GDAT {}
#[doc = "ADC Sequence-n Global Data register. This register contains the result of the most recent ADC conversion performed under sequence-n."]
pub mod seq_gdat;
#[doc = "ADC Channel N Data register. This register contains the result of the most recent conversion completed on channel N.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dat](dat) module"]
pub type DAT = crate::Reg<u32, _DAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAT;
#[doc = "`read()` method returns [dat::R](dat::R) reader structure"]
impl crate::Readable for DAT {}
#[doc = "ADC Channel N Data register. This register contains the result of the most recent conversion completed on channel N."]
pub mod dat;
#[doc = "ADC Low Compare Threshold register 0: Contains the lower threshold level for automatic threshold comparison for any channels linked to threshold pair 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thr0_low](thr0_low) module"]
pub type THR0_LOW = crate::Reg<u32, _THR0_LOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _THR0_LOW;
#[doc = "`read()` method returns [thr0_low::R](thr0_low::R) reader structure"]
impl crate::Readable for THR0_LOW {}
#[doc = "`write(|w| ..)` method takes [thr0_low::W](thr0_low::W) writer structure"]
impl crate::Writable for THR0_LOW {}
#[doc = "ADC Low Compare Threshold register 0: Contains the lower threshold level for automatic threshold comparison for any channels linked to threshold pair 0."]
pub mod thr0_low;
#[doc = "ADC Low Compare Threshold register 1: Contains the lower threshold level for automatic threshold comparison for any channels linked to threshold pair 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thr1_low](thr1_low) module"]
pub type THR1_LOW = crate::Reg<u32, _THR1_LOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _THR1_LOW;
#[doc = "`read()` method returns [thr1_low::R](thr1_low::R) reader structure"]
impl crate::Readable for THR1_LOW {}
#[doc = "`write(|w| ..)` method takes [thr1_low::W](thr1_low::W) writer structure"]
impl crate::Writable for THR1_LOW {}
#[doc = "ADC Low Compare Threshold register 1: Contains the lower threshold level for automatic threshold comparison for any channels linked to threshold pair 1."]
pub mod thr1_low;
#[doc = "ADC High Compare Threshold register 0: Contains the upper threshold level for automatic threshold comparison for any channels linked to threshold pair 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thr0_high](thr0_high) module"]
pub type THR0_HIGH = crate::Reg<u32, _THR0_HIGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _THR0_HIGH;
#[doc = "`read()` method returns [thr0_high::R](thr0_high::R) reader structure"]
impl crate::Readable for THR0_HIGH {}
#[doc = "`write(|w| ..)` method takes [thr0_high::W](thr0_high::W) writer structure"]
impl crate::Writable for THR0_HIGH {}
#[doc = "ADC High Compare Threshold register 0: Contains the upper threshold level for automatic threshold comparison for any channels linked to threshold pair 0."]
pub mod thr0_high;
#[doc = "ADC High Compare Threshold register 1: Contains the upper threshold level for automatic threshold comparison for any channels linked to threshold pair 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thr1_high](thr1_high) module"]
pub type THR1_HIGH = crate::Reg<u32, _THR1_HIGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _THR1_HIGH;
#[doc = "`read()` method returns [thr1_high::R](thr1_high::R) reader structure"]
impl crate::Readable for THR1_HIGH {}
#[doc = "`write(|w| ..)` method takes [thr1_high::W](thr1_high::W) writer structure"]
impl crate::Writable for THR1_HIGH {}
#[doc = "ADC High Compare Threshold register 1: Contains the upper threshold level for automatic threshold comparison for any channels linked to threshold pair 1."]
pub mod thr1_high;
#[doc = "ADC Channel-Threshold Select register. Specifies which set of threshold compare registers are to be used for each channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chan_thrsel](chan_thrsel) module"]
pub type CHAN_THRSEL = crate::Reg<u32, _CHAN_THRSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHAN_THRSEL;
#[doc = "`read()` method returns [chan_thrsel::R](chan_thrsel::R) reader structure"]
impl crate::Readable for CHAN_THRSEL {}
#[doc = "`write(|w| ..)` method takes [chan_thrsel::W](chan_thrsel::W) writer structure"]
impl crate::Writable for CHAN_THRSEL {}
#[doc = "ADC Channel-Threshold Select register. Specifies which set of threshold compare registers are to be used for each channel"]
pub mod chan_thrsel;
#[doc = "ADC Interrupt Enable register. This register contains enable bits that enable the sequence-A, sequence-B, threshold compare and data overrun interrupts to be generated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "ADC Interrupt Enable register. This register contains enable bits that enable the sequence-A, sequence-B, threshold compare and data overrun interrupts to be generated."]
pub mod inten;
#[doc = "ADC Flags register. Contains the four interrupt/DMA trigger flags and the individual component overrun and threshold-compare flags. (The overrun bits replicate information stored in the result registers).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flags](flags) module"]
pub type FLAGS = crate::Reg<u32, _FLAGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLAGS;
#[doc = "`read()` method returns [flags::R](flags::R) reader structure"]
impl crate::Readable for FLAGS {}
#[doc = "`write(|w| ..)` method takes [flags::W](flags::W) writer structure"]
impl crate::Writable for FLAGS {}
#[doc = "ADC Flags register. Contains the four interrupt/DMA trigger flags and the individual component overrun and threshold-compare flags. (The overrun bits replicate information stored in the result registers)."]
pub mod flags;
#[doc = "ADC Startup register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trm](trm) module"]
pub type TRM = crate::Reg<u32, _TRM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRM;
#[doc = "`read()` method returns [trm::R](trm::R) reader structure"]
impl crate::Readable for TRM {}
#[doc = "`write(|w| ..)` method takes [trm::W](trm::W) writer structure"]
impl crate::Writable for TRM {}
#[doc = "ADC Startup register."]
pub mod trm;
