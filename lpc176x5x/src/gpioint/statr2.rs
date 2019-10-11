#[doc = "Reader of register STATR2"]
pub type R = crate::R<u32, super::STATR2>;
#[doc = "Reader of field `P2_0REI`"]
pub type P2_0REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_1REI`"]
pub type P2_1REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_2REI`"]
pub type P2_2REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_3REI`"]
pub type P2_3REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_4REI`"]
pub type P2_4REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_5REI`"]
pub type P2_5REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_6REI`"]
pub type P2_6REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_7REI`"]
pub type P2_7REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_8REI`"]
pub type P2_8REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_9REI`"]
pub type P2_9REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_10REI`"]
pub type P2_10REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_11REI`"]
pub type P2_11REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_12REI`"]
pub type P2_12REI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2_13REI`"]
pub type P2_13REI_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Status of Rising Edge Interrupt for P2\\[0\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_0rei(&self) -> P2_0REI_R {
        P2_0REI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Status of Rising Edge Interrupt for P2\\[1\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_1rei(&self) -> P2_1REI_R {
        P2_1REI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status of Rising Edge Interrupt for P2\\[2\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_2rei(&self) -> P2_2REI_R {
        P2_2REI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Status of Rising Edge Interrupt for P2\\[3\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_3rei(&self) -> P2_3REI_R {
        P2_3REI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Status of Rising Edge Interrupt for P2\\[4\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_4rei(&self) -> P2_4REI_R {
        P2_4REI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Status of Rising Edge Interrupt for P2\\[5\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_5rei(&self) -> P2_5REI_R {
        P2_5REI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Status of Rising Edge Interrupt for P2\\[6\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_6rei(&self) -> P2_6REI_R {
        P2_6REI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Status of Rising Edge Interrupt for P2\\[7\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_7rei(&self) -> P2_7REI_R {
        P2_7REI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Status of Rising Edge Interrupt for P2\\[8\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_8rei(&self) -> P2_8REI_R {
        P2_8REI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Status of Rising Edge Interrupt for P2\\[9\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_9rei(&self) -> P2_9REI_R {
        P2_9REI_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Status of Rising Edge Interrupt for P2\\[10\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_10rei(&self) -> P2_10REI_R {
        P2_10REI_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Status of Rising Edge Interrupt for P2\\[11\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_11rei(&self) -> P2_11REI_R {
        P2_11REI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Status of Rising Edge Interrupt for P2\\[12\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_12rei(&self) -> P2_12REI_R {
        P2_12REI_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Status of Rising Edge Interrupt for P2\\[13\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_13rei(&self) -> P2_13REI_R {
        P2_13REI_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
