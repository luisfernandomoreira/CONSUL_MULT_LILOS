#[doc = "Register `CFG2` reader"]
pub type R = crate::R<Cfg2Spec>;
#[doc = "Register `CFG2` writer"]
pub type W = crate::W<Cfg2Spec>;
#[doc = "Field `LXTALCSS_IE` reader - LXTAL css interrupt enable"]
pub type LxtalcssIeR = crate::BitReader;
#[doc = "Field `LXTALCSS_IE` writer - LXTAL css interrupt enable"]
pub type LxtalcssIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HXTALCSS_IE` reader - HXTAL css interrupt enable"]
pub type HxtalcssIeR = crate::BitReader;
#[doc = "Field `HXTALCSS_IE` writer - HXTAL css interrupt enable"]
pub type HxtalcssIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCMEIE` reader - Multi-bits (two bits) non-correction error NMI interrupt enable (for GD32C231xx / GD32C221xx series)"]
pub type EccmeieR = crate::BitReader;
#[doc = "Field `ECCMEIE` writer - Multi-bits (two bits) non-correction error NMI interrupt enable (for GD32C231xx / GD32C221xx series)"]
pub type EccmeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCSEIE` reader - Single bit correction error interrupt enable(for GD32C231xx / GD32C221xx series)"]
pub type EccseieR = crate::BitReader;
#[doc = "Field `ECCSEIE` writer - Single bit correction error interrupt enable(for GD32C231xx / GD32C221xx series)"]
pub type EccseieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCSERRBITS` reader - Indicates the error bit(for GD32C231xx / GD32C221xx series)"]
pub type EccserrbitsR = crate::FieldReader;
#[doc = "Field `ECCEADDR` reader - Indicates the last ECC event on SRAM occurred. (for GD32C231xx / GD32C221xx series)"]
pub type EcceaddrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 1 - LXTAL css interrupt enable"]
    #[inline(always)]
    pub fn lxtalcss_ie(&self) -> LxtalcssIeR {
        LxtalcssIeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HXTAL css interrupt enable"]
    #[inline(always)]
    pub fn hxtalcss_ie(&self) -> HxtalcssIeR {
        HxtalcssIeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Multi-bits (two bits) non-correction error NMI interrupt enable (for GD32C231xx / GD32C221xx series)"]
    #[inline(always)]
    pub fn eccmeie(&self) -> EccmeieR {
        EccmeieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Single bit correction error interrupt enable(for GD32C231xx / GD32C221xx series)"]
    #[inline(always)]
    pub fn eccseie(&self) -> EccseieR {
        EccseieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 10:15 - Indicates the error bit(for GD32C231xx / GD32C221xx series)"]
    #[inline(always)]
    pub fn eccserrbits(&self) -> EccserrbitsR {
        EccserrbitsR::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 20:31 - Indicates the last ECC event on SRAM occurred. (for GD32C231xx / GD32C221xx series)"]
    #[inline(always)]
    pub fn ecceaddr(&self) -> EcceaddrR {
        EcceaddrR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 1 - LXTAL css interrupt enable"]
    #[inline(always)]
    pub fn lxtalcss_ie(&mut self) -> LxtalcssIeW<'_, Cfg2Spec> {
        LxtalcssIeW::new(self, 1)
    }
    #[doc = "Bit 2 - HXTAL css interrupt enable"]
    #[inline(always)]
    pub fn hxtalcss_ie(&mut self) -> HxtalcssIeW<'_, Cfg2Spec> {
        HxtalcssIeW::new(self, 2)
    }
    #[doc = "Bit 3 - Multi-bits (two bits) non-correction error NMI interrupt enable (for GD32C231xx / GD32C221xx series)"]
    #[inline(always)]
    pub fn eccmeie(&mut self) -> EccmeieW<'_, Cfg2Spec> {
        EccmeieW::new(self, 3)
    }
    #[doc = "Bit 4 - Single bit correction error interrupt enable(for GD32C231xx / GD32C221xx series)"]
    #[inline(always)]
    pub fn eccseie(&mut self) -> EccseieW<'_, Cfg2Spec> {
        EccseieW::new(self, 4)
    }
}
#[doc = "SYSCFG configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg2Spec;
impl crate::RegisterSpec for Cfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg2::R`](R) reader structure"]
impl crate::Readable for Cfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg2::W`](W) writer structure"]
impl crate::Writable for Cfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG2 to value 0x06"]
impl crate::Resettable for Cfg2Spec {
    const RESET_VALUE: u32 = 0x06;
}
