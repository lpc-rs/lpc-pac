#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RXDAT {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type RXDAT_R = crate::FR<u16, u16>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:8 - The USART Receiver Data register contains the next received character. The number of bits that are relevant depends on the USART configuration settings."]
    #[inline(always)]
    pub fn rxdat(&self) -> RXDAT_R {
        RXDAT_R::new((self.bits() & 0x01ff) as u16)
    }
}
