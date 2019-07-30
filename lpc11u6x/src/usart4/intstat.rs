#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::INTSTAT {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type RXRDY_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TXRDY_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TXIDLE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DELTACTS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TXDISINT_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVERRUNINT_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DELTARXBRK_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type START_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type FRAMERRINT_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PARITYERRINT_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RXNOISEINT_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type ABERR_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Receiver Ready flag."]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmitter Ready flag."]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmitter idle status."]
    #[inline(always)]
    pub fn txidle(&self) -> TXIDLE_R {
        TXIDLE_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit is set when a change in the state of the CTS input is detected."]
    #[inline(always)]
    pub fn deltacts(&self) -> DELTACTS_R {
        DELTACTS_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmitter Disabled Interrupt flag."]
    #[inline(always)]
    pub fn txdisint(&self) -> TXDISINT_R {
        TXDISINT_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Overrun Error interrupt flag."]
    #[inline(always)]
    pub fn overrunint(&self) -> OVERRUNINT_R {
        OVERRUNINT_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This bit is set when a change in the state of receiver break detection occurs."]
    #[inline(always)]
    pub fn deltarxbrk(&self) -> DELTARXBRK_R {
        DELTARXBRK_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This bit is set when a start is detected on the receiver input."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Framing Error interrupt flag."]
    #[inline(always)]
    pub fn framerrint(&self) -> FRAMERRINT_R {
        FRAMERRINT_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Parity Error interrupt flag."]
    #[inline(always)]
    pub fn parityerrint(&self) -> PARITYERRINT_R {
        PARITYERRINT_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Received Noise interrupt flag."]
    #[inline(always)]
    pub fn rxnoiseint(&self) -> RXNOISEINT_R {
        RXNOISEINT_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Auto-baud Error flag."]
    #[inline(always)]
    pub fn aberr(&self) -> ABERR_R {
        ABERR_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
}
