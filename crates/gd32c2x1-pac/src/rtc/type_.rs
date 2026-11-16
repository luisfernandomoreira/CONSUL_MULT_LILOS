#[doc = "Register `TYPE` reader"]
pub type R = crate::R<TypeSpec>;
#[doc = "Register `TYPE` writer"]
pub type W = crate::W<TypeSpec>;
#[doc = "Field `DISPU` reader - RTC_ALRM output pull up disable bit"]
pub type DispuR = crate::BitReader;
#[doc = "Field `DISPU` writer - RTC_ALRM output pull up disable bit"]
pub type DispuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRMOUTTYPE` reader - RTC_ALARM Output Type"]
pub type AlrmouttypeR = crate::BitReader;
#[doc = "Field `ALRMOUTTYPE` writer - RTC_ALARM Output Type"]
pub type AlrmouttypeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 15 - RTC_ALRM output pull up disable bit"]
    #[inline(always)]
    pub fn dispu(&self) -> DispuR {
        DispuR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - RTC_ALARM Output Type"]
    #[inline(always)]
    pub fn alrmouttype(&self) -> AlrmouttypeR {
        AlrmouttypeR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - RTC_ALRM output pull up disable bit"]
    #[inline(always)]
    pub fn dispu(&mut self) -> DispuW<'_, TypeSpec> {
        DispuW::new(self, 15)
    }
    #[doc = "Bit 18 - RTC_ALARM Output Type"]
    #[inline(always)]
    pub fn alrmouttype(&mut self) -> AlrmouttypeW<'_, TypeSpec> {
        AlrmouttypeW::new(self, 18)
    }
}
#[doc = "Type register\n\nYou can [`read`](crate::Reg::read) this register and get [`type_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`type_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TypeSpec;
impl crate::RegisterSpec for TypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`type_::R`](R) reader structure"]
impl crate::Readable for TypeSpec {}
#[doc = "`write(|w| ..)` method takes [`type_::W`](W) writer structure"]
impl crate::Writable for TypeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TYPE to value 0"]
impl crate::Resettable for TypeSpec {}
