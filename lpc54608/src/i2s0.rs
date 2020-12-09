#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_div: [u8; 3180usize],
    _reserved1: [u8; 404usize],
    #[doc = "0xe00 - FIFO configuration and enable register."]
    pub fifocfg: FIFOCFG,
    #[doc = "0xe04 - FIFO status register."]
    pub fifostat: FIFOSTAT,
    #[doc = "0xe08 - FIFO trigger settings for interrupt and DMA request."]
    pub fifotrig: FIFOTRIG,
    _reserved4: [u8; 4usize],
    #[doc = "0xe10 - FIFO interrupt enable set (enable) and read register."]
    pub fifointenset: FIFOINTENSET,
    #[doc = "0xe14 - FIFO interrupt enable clear (disable) and read register."]
    pub fifointenclr: FIFOINTENCLR,
    #[doc = "0xe18 - FIFO interrupt status register."]
    pub fifointstat: FIFOINTSTAT,
    _reserved7: [u8; 4usize],
    #[doc = "0xe20 - FIFO write data."]
    pub fifowr: FIFOWR,
    #[doc = "0xe24 - FIFO write data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
    pub fifowr48h: FIFOWR48H,
    _reserved9: [u8; 8usize],
    #[doc = "0xe30 - FIFO read data."]
    pub fiford: FIFORD,
    #[doc = "0xe34 - FIFO read data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
    pub fiford48h: FIFORD48H,
    _reserved11: [u8; 8usize],
    #[doc = "0xe40 - FIFO data read with no FIFO pop."]
    pub fifordnopop: FIFORDNOPOP,
    #[doc = "0xe44 - FIFO data read for upper data bits with no FIFO pop. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
    pub fiford48hnopop: FIFORD48HNOPOP,
    _reserved13: [u8; 4020usize],
    #[doc = "0x1dfc - I2S Module identification"]
    pub id: ID,
}
impl RegisterBlock {
    #[doc = "0x00 - no description available"]
    #[inline(always)]
    pub fn secchannel0(&self) -> &SECCHANNEL {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const SECCHANNEL) }
    }
    #[doc = "0x00 - no description available"]
    #[inline(always)]
    pub fn secchannel0_mut(&self) -> &mut SECCHANNEL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut SECCHANNEL) }
    }
    #[doc = "0x20 - no description available"]
    #[inline(always)]
    pub fn secchannel1(&self) -> &SECCHANNEL {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const SECCHANNEL) }
    }
    #[doc = "0x20 - no description available"]
    #[inline(always)]
    pub fn secchannel1_mut(&self) -> &mut SECCHANNEL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(32usize) as *mut SECCHANNEL) }
    }
    #[doc = "0x40 - no description available"]
    #[inline(always)]
    pub fn secchannel2(&self) -> &SECCHANNEL {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const SECCHANNEL) }
    }
    #[doc = "0x40 - no description available"]
    #[inline(always)]
    pub fn secchannel2_mut(&self) -> &mut SECCHANNEL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut SECCHANNEL) }
    }
    #[doc = "0xc00 - Configuration register 1 for the primary channel pair."]
    #[inline(always)]
    pub fn cfg1(&self) -> &CFG1 {
        unsafe { &*(((self as *const Self) as *const u8).add(3072usize) as *const CFG1) }
    }
    #[doc = "0xc00 - Configuration register 1 for the primary channel pair."]
    #[inline(always)]
    pub fn cfg1_mut(&self) -> &mut CFG1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3072usize) as *mut CFG1) }
    }
    #[doc = "0xc04 - Configuration register 2 for the primary channel pair."]
    #[inline(always)]
    pub fn cfg2(&self) -> &CFG2 {
        unsafe { &*(((self as *const Self) as *const u8).add(3076usize) as *const CFG2) }
    }
    #[doc = "0xc04 - Configuration register 2 for the primary channel pair."]
    #[inline(always)]
    pub fn cfg2_mut(&self) -> &mut CFG2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3076usize) as *mut CFG2) }
    }
    #[doc = "0xc08 - Status register for the primary channel pair."]
    #[inline(always)]
    pub fn stat(&self) -> &STAT {
        unsafe { &*(((self as *const Self) as *const u8).add(3080usize) as *const STAT) }
    }
    #[doc = "0xc08 - Status register for the primary channel pair."]
    #[inline(always)]
    pub fn stat_mut(&self) -> &mut STAT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3080usize) as *mut STAT) }
    }
    #[doc = "0xc1c - Clock divider, used by all channel pairs."]
    #[inline(always)]
    pub fn div(&self) -> &DIV {
        unsafe { &*(((self as *const Self) as *const u8).add(3100usize) as *const DIV) }
    }
    #[doc = "0xc1c - Clock divider, used by all channel pairs."]
    #[inline(always)]
    pub fn div_mut(&self) -> &mut DIV {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3100usize) as *mut DIV) }
    }
}
#[doc = r"Register block"]
#[repr(C)]
pub struct SECCHANNEL {
    _reserved0: [u8; 3104usize],
    #[doc = "0xc20 - Configuration register 1 for channel pair"]
    pub pcfg1: self::secchannel::PCFG1,
    #[doc = "0xc24 - Configuration register 2 for channel pair"]
    pub pcfg2: self::secchannel::PCFG2,
    #[doc = "0xc28 - Status register for channel pair"]
    pub pstat: self::secchannel::PSTAT,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod secchannel;
#[doc = "Configuration register 1 for the primary channel pair.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg1](cfg1) module"]
pub type CFG1 = crate::Reg<u32, _CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG1;
#[doc = "`read()` method returns [cfg1::R](cfg1::R) reader structure"]
impl crate::Readable for CFG1 {}
#[doc = "`write(|w| ..)` method takes [cfg1::W](cfg1::W) writer structure"]
impl crate::Writable for CFG1 {}
#[doc = "Configuration register 1 for the primary channel pair."]
pub mod cfg1;
#[doc = "Configuration register 2 for the primary channel pair.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg2](cfg2) module"]
pub type CFG2 = crate::Reg<u32, _CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG2;
#[doc = "`read()` method returns [cfg2::R](cfg2::R) reader structure"]
impl crate::Readable for CFG2 {}
#[doc = "`write(|w| ..)` method takes [cfg2::W](cfg2::W) writer structure"]
impl crate::Writable for CFG2 {}
#[doc = "Configuration register 2 for the primary channel pair."]
pub mod cfg2;
#[doc = "Status register for the primary channel pair.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "Status register for the primary channel pair."]
pub mod stat;
#[doc = "Clock divider, used by all channel pairs.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div](div) module"]
pub type DIV = crate::Reg<u32, _DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIV;
#[doc = "`read()` method returns [div::R](div::R) reader structure"]
impl crate::Readable for DIV {}
#[doc = "`write(|w| ..)` method takes [div::W](div::W) writer structure"]
impl crate::Writable for DIV {}
#[doc = "Clock divider, used by all channel pairs."]
pub mod div;
#[doc = "FIFO configuration and enable register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifocfg](fifocfg) module"]
pub type FIFOCFG = crate::Reg<u32, _FIFOCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOCFG;
#[doc = "`read()` method returns [fifocfg::R](fifocfg::R) reader structure"]
impl crate::Readable for FIFOCFG {}
#[doc = "`write(|w| ..)` method takes [fifocfg::W](fifocfg::W) writer structure"]
impl crate::Writable for FIFOCFG {}
#[doc = "FIFO configuration and enable register."]
pub mod fifocfg;
#[doc = "FIFO status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifostat](fifostat) module"]
pub type FIFOSTAT = crate::Reg<u32, _FIFOSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOSTAT;
#[doc = "`read()` method returns [fifostat::R](fifostat::R) reader structure"]
impl crate::Readable for FIFOSTAT {}
#[doc = "`write(|w| ..)` method takes [fifostat::W](fifostat::W) writer structure"]
impl crate::Writable for FIFOSTAT {}
#[doc = "FIFO status register."]
pub mod fifostat;
#[doc = "FIFO trigger settings for interrupt and DMA request.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifotrig](fifotrig) module"]
pub type FIFOTRIG = crate::Reg<u32, _FIFOTRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOTRIG;
#[doc = "`read()` method returns [fifotrig::R](fifotrig::R) reader structure"]
impl crate::Readable for FIFOTRIG {}
#[doc = "`write(|w| ..)` method takes [fifotrig::W](fifotrig::W) writer structure"]
impl crate::Writable for FIFOTRIG {}
#[doc = "FIFO trigger settings for interrupt and DMA request."]
pub mod fifotrig;
#[doc = "FIFO interrupt enable set (enable) and read register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifointenset](fifointenset) module"]
pub type FIFOINTENSET = crate::Reg<u32, _FIFOINTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOINTENSET;
#[doc = "`read()` method returns [fifointenset::R](fifointenset::R) reader structure"]
impl crate::Readable for FIFOINTENSET {}
#[doc = "`write(|w| ..)` method takes [fifointenset::W](fifointenset::W) writer structure"]
impl crate::Writable for FIFOINTENSET {}
#[doc = "FIFO interrupt enable set (enable) and read register."]
pub mod fifointenset;
#[doc = "FIFO interrupt enable clear (disable) and read register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifointenclr](fifointenclr) module"]
pub type FIFOINTENCLR = crate::Reg<u32, _FIFOINTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOINTENCLR;
#[doc = "`read()` method returns [fifointenclr::R](fifointenclr::R) reader structure"]
impl crate::Readable for FIFOINTENCLR {}
#[doc = "`write(|w| ..)` method takes [fifointenclr::W](fifointenclr::W) writer structure"]
impl crate::Writable for FIFOINTENCLR {}
#[doc = "FIFO interrupt enable clear (disable) and read register."]
pub mod fifointenclr;
#[doc = "FIFO interrupt status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifointstat](fifointstat) module"]
pub type FIFOINTSTAT = crate::Reg<u32, _FIFOINTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOINTSTAT;
#[doc = "`read()` method returns [fifointstat::R](fifointstat::R) reader structure"]
impl crate::Readable for FIFOINTSTAT {}
#[doc = "FIFO interrupt status register."]
pub mod fifointstat;
#[doc = "FIFO write data.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifowr](fifowr) module"]
pub type FIFOWR = crate::Reg<u32, _FIFOWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOWR;
#[doc = "`write(|w| ..)` method takes [fifowr::W](fifowr::W) writer structure"]
impl crate::Writable for FIFOWR {}
#[doc = "FIFO write data."]
pub mod fifowr;
#[doc = "FIFO write data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifowr48h](fifowr48h) module"]
pub type FIFOWR48H = crate::Reg<u32, _FIFOWR48H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOWR48H;
#[doc = "`write(|w| ..)` method takes [fifowr48h::W](fifowr48h::W) writer structure"]
impl crate::Writable for FIFOWR48H {}
#[doc = "FIFO write data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
pub mod fifowr48h;
#[doc = "FIFO read data.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fiford](fiford) module"]
pub type FIFORD = crate::Reg<u32, _FIFORD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFORD;
#[doc = "`read()` method returns [fiford::R](fiford::R) reader structure"]
impl crate::Readable for FIFORD {}
#[doc = "FIFO read data."]
pub mod fiford;
#[doc = "FIFO read data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fiford48h](fiford48h) module"]
pub type FIFORD48H = crate::Reg<u32, _FIFORD48H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFORD48H;
#[doc = "`read()` method returns [fiford48h::R](fiford48h::R) reader structure"]
impl crate::Readable for FIFORD48H {}
#[doc = "FIFO read data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
pub mod fiford48h;
#[doc = "FIFO data read with no FIFO pop.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifordnopop](fifordnopop) module"]
pub type FIFORDNOPOP = crate::Reg<u32, _FIFORDNOPOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFORDNOPOP;
#[doc = "`read()` method returns [fifordnopop::R](fifordnopop::R) reader structure"]
impl crate::Readable for FIFORDNOPOP {}
#[doc = "FIFO data read with no FIFO pop."]
pub mod fifordnopop;
#[doc = "FIFO data read for upper data bits with no FIFO pop. May only be used if the I2S is configured for 2x 24-bit data and not using DMA.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fiford48hnopop](fiford48hnopop) module"]
pub type FIFORD48HNOPOP = crate::Reg<u32, _FIFORD48HNOPOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFORD48HNOPOP;
#[doc = "`read()` method returns [fiford48hnopop::R](fiford48hnopop::R) reader structure"]
impl crate::Readable for FIFORD48HNOPOP {}
#[doc = "FIFO data read for upper data bits with no FIFO pop. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
pub mod fiford48hnopop;
#[doc = "I2S Module identification\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id](id) module"]
pub type ID = crate::Reg<u32, _ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID;
#[doc = "`read()` method returns [id::R](id::R) reader structure"]
impl crate::Readable for ID {}
#[doc = "I2S Module identification"]
pub mod id;
