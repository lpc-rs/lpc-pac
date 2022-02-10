///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - CRC mode register
    pub mode: crate::Reg<mode::MODE_SPEC>,
    ///0x04 - CRC seed register
    pub seed: crate::Reg<seed::SEED_SPEC>,
    _reserved_2_sum_wr_data: [u8; 0x04],
}
impl RegisterBlock {
    ///0x08 - CRC data register
    #[inline(always)]
    pub fn sum_wr_data_wr_data(
        &self,
    ) -> &crate::Reg<sum_wr_data_wr_data::SUM_WR_DATA_WR_DATA_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize)
                as *const crate::Reg<sum_wr_data_wr_data::SUM_WR_DATA_WR_DATA_SPEC>)
        }
    }
    ///0x08 - CRC checksum register
    #[inline(always)]
    pub fn sum_wr_data_sum(&self) -> &crate::Reg<sum_wr_data_sum::SUM_WR_DATA_SUM_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize)
                as *const crate::Reg<sum_wr_data_sum::SUM_WR_DATA_SUM_SPEC>)
        }
    }
}
///MODE register accessor: an alias for `Reg<MODE_SPEC>`
pub type MODE = crate::Reg<mode::MODE_SPEC>;
///CRC mode register
pub mod mode;
///SEED register accessor: an alias for `Reg<SEED_SPEC>`
pub type SEED = crate::Reg<seed::SEED_SPEC>;
///CRC seed register
pub mod seed;
///SUM_WR_DATA_SUM register accessor: an alias for `Reg<SUM_WR_DATA_SUM_SPEC>`
pub type SUM_WR_DATA_SUM = crate::Reg<sum_wr_data_sum::SUM_WR_DATA_SUM_SPEC>;
///CRC checksum register
pub mod sum_wr_data_sum;
///SUM_WR_DATA_WR_DATA register accessor: an alias for `Reg<SUM_WR_DATA_WR_DATA_SPEC>`
pub type SUM_WR_DATA_WR_DATA = crate::Reg<sum_wr_data_wr_data::SUM_WR_DATA_WR_DATA_SPEC>;
///CRC data register
pub mod sum_wr_data_wr_data;
