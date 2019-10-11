#[doc = "Reader of register RXSR"]
pub type R = crate::R<u32, super::RXSR>;
#[doc = "Reader of field `RS1`"]
pub type RS1_R = crate::R<bool, bool>;
#[doc = "Reader of field `RS2`"]
pub type RS2_R = crate::R<bool, bool>;
#[doc = "Reader of field `RB1`"]
pub type RB1_R = crate::R<bool, bool>;
#[doc = "Reader of field `RB2`"]
pub type RB2_R = crate::R<bool, bool>;
#[doc = "Reader of field `DOS1`"]
pub type DOS1_R = crate::R<bool, bool>;
#[doc = "Reader of field `DOS2`"]
pub type DOS2_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - When 1, CAN1 is receiving a message (same as RS in CAN1GSR)."]
    #[inline(always)]
    pub fn rs1(&self) -> RS1_R {
        RS1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1, CAN2 is receiving a message (same as RS in CAN2GSR)."]
    #[inline(always)]
    pub fn rs2(&self) -> RS2_R {
        RS2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - When 1, a received message is available in the CAN1 controller (same as RBS in CAN1GSR)."]
    #[inline(always)]
    pub fn rb1(&self) -> RB1_R {
        RB1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - When 1, a received message is available in the CAN2 controller (same as RBS in CAN2GSR)."]
    #[inline(always)]
    pub fn rb2(&self) -> RB2_R {
        RB2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - When 1, a message was lost because the preceding message to CAN1 controller was not read out quickly enough (same as DOS in CAN1GSR)."]
    #[inline(always)]
    pub fn dos1(&self) -> DOS1_R {
        DOS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - When 1, a message was lost because the preceding message to CAN2 controller was not read out quickly enough (same as DOS in CAN2GSR)."]
    #[inline(always)]
    pub fn dos2(&self) -> DOS2_R {
        DOS2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
