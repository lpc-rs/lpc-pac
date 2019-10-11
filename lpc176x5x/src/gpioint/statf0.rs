#[doc = "Reader of register STATF0"]
pub type R = crate::R<u32, super::STATF0>;
#[doc = "Reader of field `P0_0FEI`"]
pub type P0_0FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_1FEI`"]
pub type P0_1FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_2FEI`"]
pub type P0_2FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_3FEI`"]
pub type P0_3FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_4FEI`"]
pub type P0_4FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_5FEI`"]
pub type P0_5FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_6FEI`"]
pub type P0_6FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_7FEI`"]
pub type P0_7FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_8FEI`"]
pub type P0_8FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_9FEI`"]
pub type P0_9FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_10FEI`"]
pub type P0_10FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_11FEI`"]
pub type P0_11FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_12FEI`"]
pub type P0_12FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_13FEI`"]
pub type P0_13FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_14FEI`"]
pub type P0_14FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_15FEI`"]
pub type P0_15FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_16FEI`"]
pub type P0_16FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_17FEI`"]
pub type P0_17FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_18FEI`"]
pub type P0_18FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_19FEI`"]
pub type P0_19FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_20FEI`"]
pub type P0_20FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_21FEI`"]
pub type P0_21FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_22FEI`"]
pub type P0_22FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_23FEI`"]
pub type P0_23FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_24FEI`"]
pub type P0_24FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_25FEI`"]
pub type P0_25FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_26FEI`"]
pub type P0_26FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_27FEI`"]
pub type P0_27FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_28FEI`"]
pub type P0_28FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_29FEI`"]
pub type P0_29FEI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0_30FEI`"]
pub type P0_30FEI_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Status of Falling Edge Interrupt for P0\\[0\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_0fei(&self) -> P0_0FEI_R {
        P0_0FEI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Status of Falling Edge Interrupt for P0\\[1\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_1fei(&self) -> P0_1FEI_R {
        P0_1FEI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status of Falling Edge Interrupt for P0\\[2\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_2fei(&self) -> P0_2FEI_R {
        P0_2FEI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Status of Falling Edge Interrupt for P0\\[3\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_3fei(&self) -> P0_3FEI_R {
        P0_3FEI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Status of Falling Edge Interrupt for P0\\[4\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_4fei(&self) -> P0_4FEI_R {
        P0_4FEI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Status of Falling Edge Interrupt for P0\\[5\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_5fei(&self) -> P0_5FEI_R {
        P0_5FEI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Status of Falling Edge Interrupt for P0\\[6\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_6fei(&self) -> P0_6FEI_R {
        P0_6FEI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Status of Falling Edge Interrupt for P0\\[7\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_7fei(&self) -> P0_7FEI_R {
        P0_7FEI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Status of Falling Edge Interrupt for P0\\[8\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_8fei(&self) -> P0_8FEI_R {
        P0_8FEI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Status of Falling Edge Interrupt for P0\\[9\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_9fei(&self) -> P0_9FEI_R {
        P0_9FEI_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Status of Falling Edge Interrupt for P0\\[10\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_10fei(&self) -> P0_10FEI_R {
        P0_10FEI_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Status of Falling Edge Interrupt for P0\\[11\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_11fei(&self) -> P0_11FEI_R {
        P0_11FEI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Status of Falling Edge Interrupt for P0\\[12\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_12fei(&self) -> P0_12FEI_R {
        P0_12FEI_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Status of Falling Edge Interrupt for P0\\[13\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_13fei(&self) -> P0_13FEI_R {
        P0_13FEI_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Status of Falling Edge Interrupt for P0\\[14\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_14fei(&self) -> P0_14FEI_R {
        P0_14FEI_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Status of Falling Edge Interrupt for P0\\[15\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_15fei(&self) -> P0_15FEI_R {
        P0_15FEI_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Status of Falling Edge Interrupt for P0\\[16\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_16fei(&self) -> P0_16FEI_R {
        P0_16FEI_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Status of Falling Edge Interrupt for P0\\[17\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_17fei(&self) -> P0_17FEI_R {
        P0_17FEI_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Status of Falling Edge Interrupt for P0\\[18\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_18fei(&self) -> P0_18FEI_R {
        P0_18FEI_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Status of Falling Edge Interrupt for P0\\[19\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_19fei(&self) -> P0_19FEI_R {
        P0_19FEI_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Status of Falling Edge Interrupt for P0\\[20\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_20fei(&self) -> P0_20FEI_R {
        P0_20FEI_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Status of Falling Edge Interrupt for P0\\[21\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_21fei(&self) -> P0_21FEI_R {
        P0_21FEI_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Status of Falling Edge Interrupt for P0\\[22\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_22fei(&self) -> P0_22FEI_R {
        P0_22FEI_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Status of Falling Edge Interrupt for P0\\[23\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_23fei(&self) -> P0_23FEI_R {
        P0_23FEI_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Status of Falling Edge Interrupt for P0\\[24\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_24fei(&self) -> P0_24FEI_R {
        P0_24FEI_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Status of Falling Edge Interrupt for P0\\[25\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_25fei(&self) -> P0_25FEI_R {
        P0_25FEI_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Status of Falling Edge Interrupt for P0\\[26\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_26fei(&self) -> P0_26FEI_R {
        P0_26FEI_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Status of Falling Edge Interrupt for P0\\[27\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_27fei(&self) -> P0_27FEI_R {
        P0_27FEI_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Status of Falling Edge Interrupt for P0\\[28\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_28fei(&self) -> P0_28FEI_R {
        P0_28FEI_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Status of Falling Edge Interrupt for P0\\[29\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_29fei(&self) -> P0_29FEI_R {
        P0_29FEI_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Status of Falling Edge Interrupt for P0\\[30\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_30fei(&self) -> P0_30FEI_R {
        P0_30FEI_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
