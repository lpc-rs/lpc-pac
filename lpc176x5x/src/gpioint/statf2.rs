#[doc = "Reader of register STATF2"]
pub type R = crate::R<u32, super::STATF2>;
#[doc = "Reader of field `P2_0FEI`"]
pub type P2_0FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_1FEI`"]
pub type P2_1FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_2FEI`"]
pub type P2_2FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_3FEI`"]
pub type P2_3FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_4FEI`"]
pub type P2_4FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_5FEI`"]
pub type P2_5FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_6FEI`"]
pub type P2_6FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_7FEI`"]
pub type P2_7FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_8FEI`"]
pub type P2_8FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_9FEI`"]
pub type P2_9FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_10FEI`"]
pub type P2_10FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_11FEI`"]
pub type P2_11FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_12FEI`"]
pub type P2_12FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_13FEI`"]
pub type P2_13FEI_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Status of Falling Edge Interrupt for P2\\[0\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_0fei(&self) -> P2_0FEI_R {
        P2_0FEI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Status of Falling Edge Interrupt for P2\\[1\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_1fei(&self) -> P2_1FEI_R {
        P2_1FEI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status of Falling Edge Interrupt for P2\\[2\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_2fei(&self) -> P2_2FEI_R {
        P2_2FEI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Status of Falling Edge Interrupt for P2\\[3\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_3fei(&self) -> P2_3FEI_R {
        P2_3FEI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Status of Falling Edge Interrupt for P2\\[4\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_4fei(&self) -> P2_4FEI_R {
        P2_4FEI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Status of Falling Edge Interrupt for P2\\[5\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_5fei(&self) -> P2_5FEI_R {
        P2_5FEI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Status of Falling Edge Interrupt for P2\\[6\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_6fei(&self) -> P2_6FEI_R {
        P2_6FEI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Status of Falling Edge Interrupt for P2\\[7\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_7fei(&self) -> P2_7FEI_R {
        P2_7FEI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Status of Falling Edge Interrupt for P2\\[8\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_8fei(&self) -> P2_8FEI_R {
        P2_8FEI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Status of Falling Edge Interrupt for P2\\[9\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_9fei(&self) -> P2_9FEI_R {
        P2_9FEI_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Status of Falling Edge Interrupt for P2\\[10\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_10fei(&self) -> P2_10FEI_R {
        P2_10FEI_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Status of Falling Edge Interrupt for P2\\[11\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_11fei(&self) -> P2_11FEI_R {
        P2_11FEI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Status of Falling Edge Interrupt for P2\\[12\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_12fei(&self) -> P2_12FEI_R {
        P2_12FEI_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Status of Falling Edge Interrupt for P2\\[13\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_13fei(&self) -> P2_13FEI_R {
        P2_13FEI_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
