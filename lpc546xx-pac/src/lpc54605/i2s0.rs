///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_div: [u8; 0x0c6c],
    _reserved1: [u8; 0x0194],
    ///0xe00 - FIFO configuration and enable register.
    pub fifocfg: crate::Reg<fifocfg::FIFOCFG_SPEC>,
    ///0xe04 - FIFO status register.
    pub fifostat: crate::Reg<fifostat::FIFOSTAT_SPEC>,
    ///0xe08 - FIFO trigger settings for interrupt and DMA request.
    pub fifotrig: crate::Reg<fifotrig::FIFOTRIG_SPEC>,
    _reserved4: [u8; 0x04],
    ///0xe10 - FIFO interrupt enable set (enable) and read register.
    pub fifointenset: crate::Reg<fifointenset::FIFOINTENSET_SPEC>,
    ///0xe14 - FIFO interrupt enable clear (disable) and read register.
    pub fifointenclr: crate::Reg<fifointenclr::FIFOINTENCLR_SPEC>,
    ///0xe18 - FIFO interrupt status register.
    pub fifointstat: crate::Reg<fifointstat::FIFOINTSTAT_SPEC>,
    _reserved7: [u8; 0x04],
    ///0xe20 - FIFO write data.
    pub fifowr: crate::Reg<fifowr::FIFOWR_SPEC>,
    ///0xe24 - FIFO write data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA.
    pub fifowr48h: crate::Reg<fifowr48h::FIFOWR48H_SPEC>,
    _reserved9: [u8; 0x08],
    ///0xe30 - FIFO read data.
    pub fiford: crate::Reg<fiford::FIFORD_SPEC>,
    ///0xe34 - FIFO read data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA.
    pub fiford48h: crate::Reg<fiford48h::FIFORD48H_SPEC>,
    _reserved11: [u8; 0x08],
    ///0xe40 - FIFO data read with no FIFO pop.
    pub fifordnopop: crate::Reg<fifordnopop::FIFORDNOPOP_SPEC>,
    ///0xe44 - FIFO data read for upper data bits with no FIFO pop. May only be used if the I2S is configured for 2x 24-bit data and not using DMA.
    pub fiford48hnopop: crate::Reg<fiford48hnopop::FIFORD48HNOPOP_SPEC>,
    _reserved13: [u8; 0x0fb4],
    ///0x1dfc - I2S Module identification
    pub id: crate::Reg<id::ID_SPEC>,
}
impl RegisterBlock {
    ///0x00..0xc2c - no description available
    #[inline(always)]
    pub fn secchannel0(&self) -> &SECCHANNEL {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const SECCHANNEL) }
    }
    ///0x20..0xc4c - no description available
    #[inline(always)]
    pub fn secchannel1(&self) -> &SECCHANNEL {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const SECCHANNEL) }
    }
    ///0x40..0xc6c - no description available
    #[inline(always)]
    pub fn secchannel2(&self) -> &SECCHANNEL {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const SECCHANNEL) }
    }
    ///0xc00 - Configuration register 1 for the primary channel pair.
    #[inline(always)]
    pub fn cfg1(&self) -> &crate::Reg<cfg1::CFG1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3072usize)
                as *const crate::Reg<cfg1::CFG1_SPEC>)
        }
    }
    ///0xc04 - Configuration register 2 for the primary channel pair.
    #[inline(always)]
    pub fn cfg2(&self) -> &crate::Reg<cfg2::CFG2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3076usize)
                as *const crate::Reg<cfg2::CFG2_SPEC>)
        }
    }
    ///0xc08 - Status register for the primary channel pair.
    #[inline(always)]
    pub fn stat(&self) -> &crate::Reg<stat::STAT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3080usize)
                as *const crate::Reg<stat::STAT_SPEC>)
        }
    }
    ///0xc1c - Clock divider, used by all channel pairs.
    #[inline(always)]
    pub fn div(&self) -> &crate::Reg<div::DIV_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3100usize)
                as *const crate::Reg<div::DIV_SPEC>)
        }
    }
}
///Register block
#[repr(C)]
pub struct SECCHANNEL {
    _reserved0: [u8; 0x0c20],
    ///0xc20 - Configuration register 1 for channel pair
    pub pcfg1: crate::Reg<self::secchannel::pcfg1::PCFG1_SPEC>,
    ///0xc24 - Configuration register 2 for channel pair
    pub pcfg2: crate::Reg<self::secchannel::pcfg2::PCFG2_SPEC>,
    ///0xc28 - Status register for channel pair
    pub pstat: crate::Reg<self::secchannel::pstat::PSTAT_SPEC>,
}
///Register block
///no description available
pub mod secchannel;
///CFG1 register accessor: an alias for `Reg<CFG1_SPEC>`
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
///Configuration register 1 for the primary channel pair.
pub mod cfg1;
///CFG2 register accessor: an alias for `Reg<CFG2_SPEC>`
pub type CFG2 = crate::Reg<cfg2::CFG2_SPEC>;
///Configuration register 2 for the primary channel pair.
pub mod cfg2;
///STAT register accessor: an alias for `Reg<STAT_SPEC>`
pub type STAT = crate::Reg<stat::STAT_SPEC>;
///Status register for the primary channel pair.
pub mod stat;
///DIV register accessor: an alias for `Reg<DIV_SPEC>`
pub type DIV = crate::Reg<div::DIV_SPEC>;
///Clock divider, used by all channel pairs.
pub mod div;
///FIFOCFG register accessor: an alias for `Reg<FIFOCFG_SPEC>`
pub type FIFOCFG = crate::Reg<fifocfg::FIFOCFG_SPEC>;
///FIFO configuration and enable register.
pub mod fifocfg;
///FIFOSTAT register accessor: an alias for `Reg<FIFOSTAT_SPEC>`
pub type FIFOSTAT = crate::Reg<fifostat::FIFOSTAT_SPEC>;
///FIFO status register.
pub mod fifostat;
///FIFOTRIG register accessor: an alias for `Reg<FIFOTRIG_SPEC>`
pub type FIFOTRIG = crate::Reg<fifotrig::FIFOTRIG_SPEC>;
///FIFO trigger settings for interrupt and DMA request.
pub mod fifotrig;
///FIFOINTENSET register accessor: an alias for `Reg<FIFOINTENSET_SPEC>`
pub type FIFOINTENSET = crate::Reg<fifointenset::FIFOINTENSET_SPEC>;
///FIFO interrupt enable set (enable) and read register.
pub mod fifointenset;
///FIFOINTENCLR register accessor: an alias for `Reg<FIFOINTENCLR_SPEC>`
pub type FIFOINTENCLR = crate::Reg<fifointenclr::FIFOINTENCLR_SPEC>;
///FIFO interrupt enable clear (disable) and read register.
pub mod fifointenclr;
///FIFOINTSTAT register accessor: an alias for `Reg<FIFOINTSTAT_SPEC>`
pub type FIFOINTSTAT = crate::Reg<fifointstat::FIFOINTSTAT_SPEC>;
///FIFO interrupt status register.
pub mod fifointstat;
///FIFOWR register accessor: an alias for `Reg<FIFOWR_SPEC>`
pub type FIFOWR = crate::Reg<fifowr::FIFOWR_SPEC>;
///FIFO write data.
pub mod fifowr;
///FIFOWR48H register accessor: an alias for `Reg<FIFOWR48H_SPEC>`
pub type FIFOWR48H = crate::Reg<fifowr48h::FIFOWR48H_SPEC>;
///FIFO write data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA.
pub mod fifowr48h;
///FIFORD register accessor: an alias for `Reg<FIFORD_SPEC>`
pub type FIFORD = crate::Reg<fiford::FIFORD_SPEC>;
///FIFO read data.
pub mod fiford;
///FIFORD48H register accessor: an alias for `Reg<FIFORD48H_SPEC>`
pub type FIFORD48H = crate::Reg<fiford48h::FIFORD48H_SPEC>;
///FIFO read data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA.
pub mod fiford48h;
///FIFORDNOPOP register accessor: an alias for `Reg<FIFORDNOPOP_SPEC>`
pub type FIFORDNOPOP = crate::Reg<fifordnopop::FIFORDNOPOP_SPEC>;
///FIFO data read with no FIFO pop.
pub mod fifordnopop;
///FIFORD48HNOPOP register accessor: an alias for `Reg<FIFORD48HNOPOP_SPEC>`
pub type FIFORD48HNOPOP = crate::Reg<fiford48hnopop::FIFORD48HNOPOP_SPEC>;
///FIFO data read for upper data bits with no FIFO pop. May only be used if the I2S is configured for 2x 24-bit data and not using DMA.
pub mod fiford48hnopop;
///ID register accessor: an alias for `Reg<ID_SPEC>`
pub type ID = crate::Reg<id::ID_SPEC>;
///I2S Module identification
pub mod id;
