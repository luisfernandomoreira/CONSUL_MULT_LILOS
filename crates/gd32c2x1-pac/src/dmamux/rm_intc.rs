#[doc = "Register `RM_INTC` writer"]
pub type W = crate::W<RmIntcSpec>;
#[doc = "Field `SOIFC0` writer - Clear bit for synchronization overrun event flag of request multiplexer channel 0"]
pub type Soifc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOIFC1` writer - Clear bit for synchronization overrun event flag of request multiplexer channel 1"]
pub type Soifc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOIFC2` writer - Clear bit for synchronization overrun event flag of request multiplexer channel 2"]
pub type Soifc2W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear bit for synchronization overrun event flag of request multiplexer channel 0"]
    #[inline(always)]
    pub fn soifc0(&mut self) -> Soifc0W<'_, RmIntcSpec> {
        Soifc0W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear bit for synchronization overrun event flag of request multiplexer channel 1"]
    #[inline(always)]
    pub fn soifc1(&mut self) -> Soifc1W<'_, RmIntcSpec> {
        Soifc1W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear bit for synchronization overrun event flag of request multiplexer channel 2"]
    #[inline(always)]
    pub fn soifc2(&mut self) -> Soifc2W<'_, RmIntcSpec> {
        Soifc2W::new(self, 2)
    }
}
#[doc = "Request multiplexer channel interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rm_intc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RmIntcSpec;
impl crate::RegisterSpec for RmIntcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rm_intc::W`](W) writer structure"]
impl crate::Writable for RmIntcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RM_INTC to value 0"]
impl crate::Resettable for RmIntcSpec {}
