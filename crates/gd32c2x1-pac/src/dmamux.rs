#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rm_ch0cfg: RmCh0cfg,
    rm_ch1cfg: RmCh1cfg,
    rm_ch2cfg: RmCh2cfg,
    _reserved3: [u8; 0x74],
    rm_intf: RmIntf,
    rm_intc: RmIntc,
    _reserved5: [u8; 0x78],
    rg_ch0cfg: RgCh0cfg,
    rg_ch1cfg: RgCh1cfg,
    rg_ch2cfg: RgCh2cfg,
    rg_ch3cfg: RgCh3cfg,
    _reserved9: [u8; 0x30],
    rg_intf: RgIntf,
    rg_intc: RgIntc,
}
impl RegisterBlock {
    #[doc = "0x00 - Request multiplexer channel x configuration register"]
    #[inline(always)]
    pub const fn rm_ch0cfg(&self) -> &RmCh0cfg {
        &self.rm_ch0cfg
    }
    #[doc = "0x04 - Request multiplexer channel x configuration register"]
    #[inline(always)]
    pub const fn rm_ch1cfg(&self) -> &RmCh1cfg {
        &self.rm_ch1cfg
    }
    #[doc = "0x08 - Request multiplexer channel x configuration register"]
    #[inline(always)]
    pub const fn rm_ch2cfg(&self) -> &RmCh2cfg {
        &self.rm_ch2cfg
    }
    #[doc = "0x80 - Request multiplexer channel interrupt flag register"]
    #[inline(always)]
    pub const fn rm_intf(&self) -> &RmIntf {
        &self.rm_intf
    }
    #[doc = "0x84 - Request multiplexer channel interrupt flag clear register"]
    #[inline(always)]
    pub const fn rm_intc(&self) -> &RmIntc {
        &self.rm_intc
    }
    #[doc = "0x100 - Request generator channel x configuration register"]
    #[inline(always)]
    pub const fn rg_ch0cfg(&self) -> &RgCh0cfg {
        &self.rg_ch0cfg
    }
    #[doc = "0x104 - Request generator channel 1 configuration register"]
    #[inline(always)]
    pub const fn rg_ch1cfg(&self) -> &RgCh1cfg {
        &self.rg_ch1cfg
    }
    #[doc = "0x108 - Request generator channel 2 configuration register"]
    #[inline(always)]
    pub const fn rg_ch2cfg(&self) -> &RgCh2cfg {
        &self.rg_ch2cfg
    }
    #[doc = "0x10c - Request generator channel x configuration register"]
    #[inline(always)]
    pub const fn rg_ch3cfg(&self) -> &RgCh3cfg {
        &self.rg_ch3cfg
    }
    #[doc = "0x140 - Request generator interrupt flag register"]
    #[inline(always)]
    pub const fn rg_intf(&self) -> &RgIntf {
        &self.rg_intf
    }
    #[doc = "0x144 - Rquest generator interrupt flag clear register"]
    #[inline(always)]
    pub const fn rg_intc(&self) -> &RgIntc {
        &self.rg_intc
    }
}
#[doc = "RM_CH0CFG (rw) register accessor: Request multiplexer channel x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rm_ch0cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rm_ch0cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rm_ch0cfg`] module"]
#[doc(alias = "RM_CH0CFG")]
pub type RmCh0cfg = crate::Reg<rm_ch0cfg::RmCh0cfgSpec>;
#[doc = "Request multiplexer channel x configuration register"]
pub mod rm_ch0cfg;
#[doc = "RM_CH1CFG (rw) register accessor: Request multiplexer channel x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rm_ch1cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rm_ch1cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rm_ch1cfg`] module"]
#[doc(alias = "RM_CH1CFG")]
pub type RmCh1cfg = crate::Reg<rm_ch1cfg::RmCh1cfgSpec>;
#[doc = "Request multiplexer channel x configuration register"]
pub mod rm_ch1cfg;
#[doc = "RM_CH2CFG (rw) register accessor: Request multiplexer channel x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rm_ch2cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rm_ch2cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rm_ch2cfg`] module"]
#[doc(alias = "RM_CH2CFG")]
pub type RmCh2cfg = crate::Reg<rm_ch2cfg::RmCh2cfgSpec>;
#[doc = "Request multiplexer channel x configuration register"]
pub mod rm_ch2cfg;
#[doc = "RM_INTF (r) register accessor: Request multiplexer channel interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`rm_intf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rm_intf`] module"]
#[doc(alias = "RM_INTF")]
pub type RmIntf = crate::Reg<rm_intf::RmIntfSpec>;
#[doc = "Request multiplexer channel interrupt flag register"]
pub mod rm_intf;
#[doc = "RM_INTC (w) register accessor: Request multiplexer channel interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rm_intc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rm_intc`] module"]
#[doc(alias = "RM_INTC")]
pub type RmIntc = crate::Reg<rm_intc::RmIntcSpec>;
#[doc = "Request multiplexer channel interrupt flag clear register"]
pub mod rm_intc;
#[doc = "RG_CH0CFG (rw) register accessor: Request generator channel x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rg_ch0cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg_ch0cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg_ch0cfg`] module"]
#[doc(alias = "RG_CH0CFG")]
pub type RgCh0cfg = crate::Reg<rg_ch0cfg::RgCh0cfgSpec>;
#[doc = "Request generator channel x configuration register"]
pub mod rg_ch0cfg;
#[doc = "RG_CH1CFG (rw) register accessor: Request generator channel 1 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rg_ch1cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg_ch1cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg_ch1cfg`] module"]
#[doc(alias = "RG_CH1CFG")]
pub type RgCh1cfg = crate::Reg<rg_ch1cfg::RgCh1cfgSpec>;
#[doc = "Request generator channel 1 configuration register"]
pub mod rg_ch1cfg;
#[doc = "RG_CH2CFG (rw) register accessor: Request generator channel 2 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rg_ch2cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg_ch2cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg_ch2cfg`] module"]
#[doc(alias = "RG_CH2CFG")]
pub type RgCh2cfg = crate::Reg<rg_ch2cfg::RgCh2cfgSpec>;
#[doc = "Request generator channel 2 configuration register"]
pub mod rg_ch2cfg;
#[doc = "RG_CH3CFG (rw) register accessor: Request generator channel x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rg_ch3cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg_ch3cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg_ch3cfg`] module"]
#[doc(alias = "RG_CH3CFG")]
pub type RgCh3cfg = crate::Reg<rg_ch3cfg::RgCh3cfgSpec>;
#[doc = "Request generator channel x configuration register"]
pub mod rg_ch3cfg;
#[doc = "RG_INTF (r) register accessor: Request generator interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`rg_intf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg_intf`] module"]
#[doc(alias = "RG_INTF")]
pub type RgIntf = crate::Reg<rg_intf::RgIntfSpec>;
#[doc = "Request generator interrupt flag register"]
pub mod rg_intf;
#[doc = "RG_INTC (r) register accessor: Rquest generator interrupt flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`rg_intc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg_intc`] module"]
#[doc(alias = "RG_INTC")]
pub type RgIntc = crate::Reg<rg_intc::RgIntcSpec>;
#[doc = "Rquest generator interrupt flag clear register"]
pub mod rg_intc;
