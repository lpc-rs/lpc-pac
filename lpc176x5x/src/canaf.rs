#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Acceptance Filter Register"]
    pub afmr: AFMR,
    #[doc = "0x04 - Standard Frame Individual Start Address Register"]
    pub sff_sa: SFF_SA,
    #[doc = "0x08 - Standard Frame Group Start Address Register"]
    pub sff_grp_sa: SFF_GRP_SA,
    #[doc = "0x0c - Extended Frame Start Address Register"]
    pub eff_sa: EFF_SA,
    #[doc = "0x10 - Extended Frame Group Start Address Register"]
    pub eff_grp_sa: EFF_GRP_SA,
    #[doc = "0x14 - End of AF Tables register"]
    pub endoftable: ENDOFTABLE,
    #[doc = "0x18 - LUT Error Address register"]
    pub luterrad: LUTERRAD,
    #[doc = "0x1c - LUT Error Register"]
    pub luterr: LUTERR,
    #[doc = "0x20 - FullCAN interrupt enable register"]
    pub fcanie: FCANIE,
    #[doc = "0x24 - FullCAN interrupt and capture register0"]
    pub fcanic0: FCANIC0,
    #[doc = "0x28 - FullCAN interrupt and capture register1"]
    pub fcanic1: FCANIC1,
}
#[doc = "Acceptance Filter Register"]
pub struct AFMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Acceptance Filter Register"]
pub mod afmr;
#[doc = "Standard Frame Individual Start Address Register"]
pub struct SFF_SA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Standard Frame Individual Start Address Register"]
pub mod sff_sa;
#[doc = "Standard Frame Group Start Address Register"]
pub struct SFF_GRP_SA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Standard Frame Group Start Address Register"]
pub mod sff_grp_sa;
#[doc = "Extended Frame Start Address Register"]
pub struct EFF_SA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Extended Frame Start Address Register"]
pub mod eff_sa;
#[doc = "Extended Frame Group Start Address Register"]
pub struct EFF_GRP_SA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Extended Frame Group Start Address Register"]
pub mod eff_grp_sa;
#[doc = "End of AF Tables register"]
pub struct ENDOFTABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "End of AF Tables register"]
pub mod endoftable;
#[doc = "LUT Error Address register"]
pub struct LUTERRAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LUT Error Address register"]
pub mod luterrad;
#[doc = "LUT Error Register"]
pub struct LUTERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LUT Error Register"]
pub mod luterr;
#[doc = "FullCAN interrupt enable register"]
pub struct FCANIE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FullCAN interrupt enable register"]
pub mod fcanie;
#[doc = "FullCAN interrupt and capture register0"]
pub struct FCANIC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FullCAN interrupt and capture register0"]
pub mod fcanic0;
#[doc = "FullCAN interrupt and capture register1"]
pub struct FCANIC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FullCAN interrupt and capture register1"]
pub mod fcanic1;
