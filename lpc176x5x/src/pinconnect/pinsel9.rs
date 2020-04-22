#[doc = "Reader of register PINSEL9"]
pub type R = crate::R<u32, super::PINSEL9>;
#[doc = "Writer for register PINSEL9"]
pub type W = crate::W<u32, super::PINSEL9>;
#[doc = "Register PINSEL9 `reset()`'s with value 0"]
impl crate::ResetValue for super::PINSEL9 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pin function select P4.28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P4_28_A {
    #[doc = "0: GPIO P4.28"]
    GPIO_P4 = 0,
    #[doc = "1: RX_MCLK"]
    RX_MCLK = 1,
    #[doc = "2: MAT2.0"]
    MAT2 = 2,
    #[doc = "3: TXD3"]
    TXD3 = 3,
}
impl From<P4_28_A> for u8 {
    #[inline(always)]
    fn from(variant: P4_28_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P4_28`"]
pub type P4_28_R = crate::R<u8, P4_28_A>;
impl P4_28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P4_28_A {
        match self.bits {
            0 => P4_28_A::GPIO_P4,
            1 => P4_28_A::RX_MCLK,
            2 => P4_28_A::MAT2,
            3 => P4_28_A::TXD3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P4`"]
    #[inline(always)]
    pub fn is_gpio_p4(&self) -> bool {
        *self == P4_28_A::GPIO_P4
    }
    #[doc = "Checks if the value of the field is `RX_MCLK`"]
    #[inline(always)]
    pub fn is_rx_mclk(&self) -> bool {
        *self == P4_28_A::RX_MCLK
    }
    #[doc = "Checks if the value of the field is `MAT2`"]
    #[inline(always)]
    pub fn is_mat2(&self) -> bool {
        *self == P4_28_A::MAT2
    }
    #[doc = "Checks if the value of the field is `TXD3`"]
    #[inline(always)]
    pub fn is_txd3(&self) -> bool {
        *self == P4_28_A::TXD3
    }
}
#[doc = "Write proxy for field `P4_28`"]
pub struct P4_28_W<'a> {
    w: &'a mut W,
}
impl<'a> P4_28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P4_28_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P4.28"]
    #[inline(always)]
    pub fn gpio_p4(self) -> &'a mut W {
        self.variant(P4_28_A::GPIO_P4)
    }
    #[doc = "RX_MCLK"]
    #[inline(always)]
    pub fn rx_mclk(self) -> &'a mut W {
        self.variant(P4_28_A::RX_MCLK)
    }
    #[doc = "MAT2.0"]
    #[inline(always)]
    pub fn mat2(self) -> &'a mut W {
        self.variant(P4_28_A::MAT2)
    }
    #[doc = "TXD3"]
    #[inline(always)]
    pub fn txd3(self) -> &'a mut W {
        self.variant(P4_28_A::TXD3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Pin function select P4.29.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P4_29_A {
    #[doc = "0: GPIO P4.29"]
    GPIO_P4 = 0,
    #[doc = "1: TX_MCLK"]
    TX_MCLK = 1,
    #[doc = "2: MAT2.1"]
    MAT2 = 2,
    #[doc = "3: RXD3"]
    RXD3 = 3,
}
impl From<P4_29_A> for u8 {
    #[inline(always)]
    fn from(variant: P4_29_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P4_29`"]
pub type P4_29_R = crate::R<u8, P4_29_A>;
impl P4_29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P4_29_A {
        match self.bits {
            0 => P4_29_A::GPIO_P4,
            1 => P4_29_A::TX_MCLK,
            2 => P4_29_A::MAT2,
            3 => P4_29_A::RXD3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_P4`"]
    #[inline(always)]
    pub fn is_gpio_p4(&self) -> bool {
        *self == P4_29_A::GPIO_P4
    }
    #[doc = "Checks if the value of the field is `TX_MCLK`"]
    #[inline(always)]
    pub fn is_tx_mclk(&self) -> bool {
        *self == P4_29_A::TX_MCLK
    }
    #[doc = "Checks if the value of the field is `MAT2`"]
    #[inline(always)]
    pub fn is_mat2(&self) -> bool {
        *self == P4_29_A::MAT2
    }
    #[doc = "Checks if the value of the field is `RXD3`"]
    #[inline(always)]
    pub fn is_rxd3(&self) -> bool {
        *self == P4_29_A::RXD3
    }
}
#[doc = "Write proxy for field `P4_29`"]
pub struct P4_29_W<'a> {
    w: &'a mut W,
}
impl<'a> P4_29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P4_29_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO P4.29"]
    #[inline(always)]
    pub fn gpio_p4(self) -> &'a mut W {
        self.variant(P4_29_A::GPIO_P4)
    }
    #[doc = "TX_MCLK"]
    #[inline(always)]
    pub fn tx_mclk(self) -> &'a mut W {
        self.variant(P4_29_A::TX_MCLK)
    }
    #[doc = "MAT2.1"]
    #[inline(always)]
    pub fn mat2(self) -> &'a mut W {
        self.variant(P4_29_A::MAT2)
    }
    #[doc = "RXD3"]
    #[inline(always)]
    pub fn rxd3(self) -> &'a mut W {
        self.variant(P4_29_A::RXD3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:25 - Pin function select P4.28."]
    #[inline(always)]
    pub fn p4_28(&self) -> P4_28_R {
        P4_28_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Pin function select P4.29."]
    #[inline(always)]
    pub fn p4_29(&self) -> P4_29_R {
        P4_29_R::new(((self.bits >> 26) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 24:25 - Pin function select P4.28."]
    #[inline(always)]
    pub fn p4_28(&mut self) -> P4_28_W {
        P4_28_W { w: self }
    }
    #[doc = "Bits 26:27 - Pin function select P4.29."]
    #[inline(always)]
    pub fn p4_29(&mut self) -> P4_29_W {
        P4_29_W { w: self }
    }
}
