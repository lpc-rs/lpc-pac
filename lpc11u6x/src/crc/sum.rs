#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SUM {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type CRC_SUM_R = crate::FR<u32, u32>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - The most recent CRC sum can be read through this register with selected bit order and 1's complement post-processes."]
    #[inline(always)]
    pub fn crc_sum(&self) -> CRC_SUM_R {
        CRC_SUM_R::new((self.bits() & 0xffff_ffff) as u32)
    }
}
