#[doc = "Register `RG_INTF` reader"]
pub type R = crate::R<RgIntfSpec>;
#[doc = "Field `TOIF0` reader - Trigger overrun event flag of request generator channel 0"]
pub type Toif0R = crate::BitReader;
#[doc = "Field `TOIF1` reader - Trigger overrun event flag of request generator channel 1"]
pub type Toif1R = crate::BitReader;
#[doc = "Field `TOIF2` reader - Trigger overrun event flag of request generator channel 2"]
pub type Toif2R = crate::BitReader;
#[doc = "Field `TOIF3` reader - Trigger overrun event flag of request generator channel 3"]
pub type Toif3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Trigger overrun event flag of request generator channel 0"]
    #[inline(always)]
    pub fn toif0(&self) -> Toif0R {
        Toif0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Trigger overrun event flag of request generator channel 1"]
    #[inline(always)]
    pub fn toif1(&self) -> Toif1R {
        Toif1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trigger overrun event flag of request generator channel 2"]
    #[inline(always)]
    pub fn toif2(&self) -> Toif2R {
        Toif2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Trigger overrun event flag of request generator channel 3"]
    #[inline(always)]
    pub fn toif3(&self) -> Toif3R {
        Toif3R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Request generator interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`rg_intf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RgIntfSpec;
impl crate::RegisterSpec for RgIntfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rg_intf::R`](R) reader structure"]
impl crate::Readable for RgIntfSpec {}
#[doc = "`reset()` method sets RG_INTF to value 0"]
impl crate::Resettable for RgIntfSpec {}
