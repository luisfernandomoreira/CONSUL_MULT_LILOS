#[doc = "Register `RG_INTC` reader"]
pub type R = crate::R<RgIntcSpec>;
#[doc = "Field `TOIFC0` reader - Clear bit for trigger overrun event flag of request generator channel 0"]
pub type Toifc0R = crate::BitReader;
#[doc = "Field `TOIFC1` reader - Clear bit for trigger overrun event flag of request generator channel 1"]
pub type Toifc1R = crate::BitReader;
#[doc = "Field `TOIFC2` reader - Clear bit for trigger overrun event flag of request generator channel 2"]
pub type Toifc2R = crate::BitReader;
#[doc = "Field `TOIFC3` reader - Clear bit for trigger overrun event flag of request generator channel 3"]
pub type Toifc3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Clear bit for trigger overrun event flag of request generator channel 0"]
    #[inline(always)]
    pub fn toifc0(&self) -> Toifc0R {
        Toifc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear bit for trigger overrun event flag of request generator channel 1"]
    #[inline(always)]
    pub fn toifc1(&self) -> Toifc1R {
        Toifc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear bit for trigger overrun event flag of request generator channel 2"]
    #[inline(always)]
    pub fn toifc2(&self) -> Toifc2R {
        Toifc2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear bit for trigger overrun event flag of request generator channel 3"]
    #[inline(always)]
    pub fn toifc3(&self) -> Toifc3R {
        Toifc3R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Rquest generator interrupt flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`rg_intc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RgIntcSpec;
impl crate::RegisterSpec for RgIntcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rg_intc::R`](R) reader structure"]
impl crate::Readable for RgIntcSpec {}
#[doc = "`reset()` method sets RG_INTC to value 0"]
impl crate::Resettable for RgIntcSpec {}
