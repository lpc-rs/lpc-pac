#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::INPUT {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type AIN0_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type AIN1_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type AIN2_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type AIN3_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SIN0_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SIN1_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SIN2_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SIN3_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Real-time status of input 0."]
    #[inline(always)]
    pub fn ain0(&self) -> AIN0_R {
        AIN0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Real-time status of input 1."]
    #[inline(always)]
    pub fn ain1(&self) -> AIN1_R {
        AIN1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Real-time status of input 2."]
    #[inline(always)]
    pub fn ain2(&self) -> AIN2_R {
        AIN2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Real-time status of input 3."]
    #[inline(always)]
    pub fn ain3(&self) -> AIN3_R {
        AIN3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Input 0 state synchronized to the SCT clock."]
    #[inline(always)]
    pub fn sin0(&self) -> SIN0_R {
        SIN0_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Input 1 state synchronized to the SCT clock."]
    #[inline(always)]
    pub fn sin1(&self) -> SIN1_R {
        SIN1_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Input 2 state synchronized to the SCT clock."]
    #[inline(always)]
    pub fn sin2(&self) -> SIN2_R {
        SIN2_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Input 3 state synchronized to the SCT clock."]
    #[inline(always)]
    pub fn sin3(&self) -> SIN3_R {
        SIN3_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
}
