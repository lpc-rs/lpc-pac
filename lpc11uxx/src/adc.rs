#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - A/D Control Register. The CR register must be written to select the operating mode before A/D conversion can occur."]
    pub cr: CR,
    #[doc = "0x04 - A/D Global Data Register. Contains the result of the most recent A/D conversion."]
    pub gdr: GDR,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - A/D Interrupt Enable Register. This register contains enable bits that allow the DONE flag of each A/D channel to be included or excluded from contributing to the generation of an A/D interrupt."]
    pub inten: INTEN,
    #[doc = "0x10 - A/D Channel Data Register. This register contains the result of the most recent conversion completed on channel N"]
    pub dr: [DR; 8],
    #[doc = "0x30 - A/D Status Register. This register contains DONE and OVERRUN flags for all of the A/D channels, as well as the A/D interrupt flag."]
    pub stat: STAT,
}
#[doc = "A/D Control Register. The CR register must be written to select the operating mode before A/D conversion can occur.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "A/D Control Register. The CR register must be written to select the operating mode before A/D conversion can occur."]
pub mod cr;
#[doc = "A/D Global Data Register. Contains the result of the most recent A/D conversion.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gdr](gdr) module"]
pub type GDR = crate::Reg<u32, _GDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GDR;
#[doc = "`read()` method returns [gdr::R](gdr::R) reader structure"]
impl crate::Readable for GDR {}
#[doc = "`write(|w| ..)` method takes [gdr::W](gdr::W) writer structure"]
impl crate::Writable for GDR {}
#[doc = "A/D Global Data Register. Contains the result of the most recent A/D conversion."]
pub mod gdr;
#[doc = "A/D Interrupt Enable Register. This register contains enable bits that allow the DONE flag of each A/D channel to be included or excluded from contributing to the generation of an A/D interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "A/D Interrupt Enable Register. This register contains enable bits that allow the DONE flag of each A/D channel to be included or excluded from contributing to the generation of an A/D interrupt."]
pub mod inten;
#[doc = "A/D Channel Data Register. This register contains the result of the most recent conversion completed on channel N\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "A/D Channel Data Register. This register contains the result of the most recent conversion completed on channel N"]
pub mod dr;
#[doc = "A/D Status Register. This register contains DONE and OVERRUN flags for all of the A/D channels, as well as the A/D interrupt flag.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "A/D Status Register. This register contains DONE and OVERRUN flags for all of the A/D channels, as well as the A/D interrupt flag."]
pub mod stat;
