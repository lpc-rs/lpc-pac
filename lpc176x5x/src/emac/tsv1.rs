#[doc = "Reader of register TSV1"]
pub type R = crate::R<u32, super::TSV1>;
#[doc = "Reader of field `TBC`"]
pub type TBC_R = crate::R<u16, u16>;
#[doc = "Reader of field `TCC`"]
pub type TCC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Transmit byte count. The total number of bytes in the frame, not counting the collided bytes."]
    #[inline(always)]
    pub fn tbc(&self) -> TBC_R {
        TBC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Transmit collision count. Number of collisions the current packet incurred during transmission attempts. The maximum number of collisions (16) cannot be represented."]
    #[inline(always)]
    pub fn tcc(&self) -> TCC_R {
        TCC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
