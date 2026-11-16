#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cmp0_cs: Cmp0Cs,
    cmp1_cs: Cmp1Cs,
}
impl RegisterBlock {
    #[doc = "0x00 - CMP0 Control / Status register"]
    #[inline(always)]
    pub const fn cmp0_cs(&self) -> &Cmp0Cs {
        &self.cmp0_cs
    }
    #[doc = "0x04 - CMP1 Control / Status register"]
    #[inline(always)]
    pub const fn cmp1_cs(&self) -> &Cmp1Cs {
        &self.cmp1_cs
    }
}
#[doc = "CMP0_CS (rw) register accessor: CMP0 Control / Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp0_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp0_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp0_cs`] module"]
#[doc(alias = "CMP0_CS")]
pub type Cmp0Cs = crate::Reg<cmp0_cs::Cmp0CsSpec>;
#[doc = "CMP0 Control / Status register"]
pub mod cmp0_cs;
#[doc = "CMP1_CS (rw) register accessor: CMP1 Control / Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp1_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1_cs`] module"]
#[doc(alias = "CMP1_CS")]
pub type Cmp1Cs = crate::Reg<cmp1_cs::Cmp1CsSpec>;
#[doc = "CMP1 Control / Status register"]
pub mod cmp1_cs;
