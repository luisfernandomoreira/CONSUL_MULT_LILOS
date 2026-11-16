#[doc = "Register `RM_INTF` reader"]
pub type R = crate::R<RmIntfSpec>;
#[doc = "Field `SOIF0` reader - Synchronization overrun event flag of request multiplexer channel 0"]
pub type Soif0R = crate::BitReader;
#[doc = "Field `SOIF1` reader - Synchronization overrun event flag of request multiplexer channel 1"]
pub type Soif1R = crate::BitReader;
#[doc = "Field `SOIF2` reader - Synchronization overrun event flag of request multiplexer channel 2"]
pub type Soif2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Synchronization overrun event flag of request multiplexer channel 0"]
    #[inline(always)]
    pub fn soif0(&self) -> Soif0R {
        Soif0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Synchronization overrun event flag of request multiplexer channel 1"]
    #[inline(always)]
    pub fn soif1(&self) -> Soif1R {
        Soif1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization overrun event flag of request multiplexer channel 2"]
    #[inline(always)]
    pub fn soif2(&self) -> Soif2R {
        Soif2R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Request multiplexer channel interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`rm_intf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RmIntfSpec;
impl crate::RegisterSpec for RmIntfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rm_intf::R`](R) reader structure"]
impl crate::Readable for RmIntfSpec {}
#[doc = "`reset()` method sets RM_INTF to value 0"]
impl crate::Resettable for RmIntfSpec {}
