#[doc = r" Register block"]
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
#[doc = "A/D Control Register. The CR register must be written to select the operating mode before A/D conversion can occur."]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A/D Control Register. The CR register must be written to select the operating mode before A/D conversion can occur."]
pub mod cr;
#[doc = "A/D Global Data Register. Contains the result of the most recent A/D conversion."]
pub struct GDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A/D Global Data Register. Contains the result of the most recent A/D conversion."]
pub mod gdr;
#[doc = "A/D Interrupt Enable Register. This register contains enable bits that allow the DONE flag of each A/D channel to be included or excluded from contributing to the generation of an A/D interrupt."]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A/D Interrupt Enable Register. This register contains enable bits that allow the DONE flag of each A/D channel to be included or excluded from contributing to the generation of an A/D interrupt."]
pub mod inten;
#[doc = "A/D Channel Data Register. This register contains the result of the most recent conversion completed on channel N"]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A/D Channel Data Register. This register contains the result of the most recent conversion completed on channel N"]
pub mod dr;
#[doc = "A/D Status Register. This register contains DONE and OVERRUN flags for all of the A/D channels, as well as the A/D interrupt flag."]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A/D Status Register. This register contains DONE and OVERRUN flags for all of the A/D channels, as well as the A/D interrupt flag."]
pub mod stat;
