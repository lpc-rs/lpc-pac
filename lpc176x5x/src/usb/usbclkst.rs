#[doc = "Reader of register USBCLKST"]
pub type R = crate::R<u32, super::USBCLKST>;
#[doc = "Reader of field `DEV_CLK_ON`"]
pub type DEV_CLK_ON_R = crate::R<bool, bool>;
#[doc = "Reader of field `PORTSEL_CLK_ON`"]
pub type PORTSEL_CLK_ON_R = crate::R<bool, bool>;
#[doc = "Reader of field `AHB_CLK_ON`"]
pub type AHB_CLK_ON_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - Device clock on. The usbclk input to the device controller is active ."]
    #[inline(always)]
    pub fn dev_clk_on(&self) -> DEV_CLK_ON_R {
        DEV_CLK_ON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port select register clock on."]
    #[inline(always)]
    pub fn portsel_clk_on(&self) -> PORTSEL_CLK_ON_R {
        PORTSEL_CLK_ON_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AHB clock on."]
    #[inline(always)]
    pub fn ahb_clk_on(&self) -> AHB_CLK_ON_R {
        AHB_CLK_ON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
