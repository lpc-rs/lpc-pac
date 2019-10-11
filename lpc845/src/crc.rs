#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC mode register"]
    pub mode: MODE,
    #[doc = "0x04 - CRC seed register"]
    pub seed: SEED,
    _reserved_2_sum: [u8; 4usize],
}
impl RegisterBlock {
    #[doc = "0x08 - CRC data register"]
    #[inline(always)]
    pub fn wr_data(&self) -> &WR_DATA {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const WR_DATA) }
    }
    #[doc = "0x08 - CRC data register"]
    #[inline(always)]
    pub fn wr_data_mut(&self) -> &mut WR_DATA {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut WR_DATA) }
    }
    #[doc = "0x08 - CRC checksum register"]
    #[inline(always)]
    pub fn sum(&self) -> &SUM {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const SUM) }
    }
    #[doc = "0x08 - CRC checksum register"]
    #[inline(always)]
    pub fn sum_mut(&self) -> &mut SUM {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut SUM) }
    }
}
#[doc = "CRC mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mode](mode) module"]
pub type MODE = crate::Reg<u32, _MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE;
#[doc = "`read()` method returns [mode::R](mode::R) reader structure"]
impl crate::Readable for MODE {}
#[doc = "`write(|w| ..)` method takes [mode::W](mode::W) writer structure"]
impl crate::Writable for MODE {}
#[doc = "CRC mode register"]
pub mod mode;
#[doc = "CRC seed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [seed](seed) module"]
pub type SEED = crate::Reg<u32, _SEED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEED;
#[doc = "`read()` method returns [seed::R](seed::R) reader structure"]
impl crate::Readable for SEED {}
#[doc = "`write(|w| ..)` method takes [seed::W](seed::W) writer structure"]
impl crate::Writable for SEED {}
#[doc = "CRC seed register"]
pub mod seed;
#[doc = "CRC checksum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sum](sum) module"]
pub type SUM = crate::Reg<u32, _SUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUM;
#[doc = "`read()` method returns [sum::R](sum::R) reader structure"]
impl crate::Readable for SUM {}
#[doc = "CRC checksum register"]
pub mod sum;
#[doc = "CRC data register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wr_data](wr_data) module"]
pub type WR_DATA = crate::Reg<u32, _WR_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WR_DATA;
#[doc = "`write(|w| ..)` method takes [wr_data::W](wr_data::W) writer structure"]
impl crate::Writable for WR_DATA {}
#[doc = "CRC data register"]
pub mod wr_data;
