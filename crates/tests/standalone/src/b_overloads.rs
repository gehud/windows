#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(C)]
pub struct AE_RESACCESS {
    pub ae_ra_compname: u32,
    pub ae_ra_username: u32,
    pub ae_ra_resname: u32,
    pub ae_ra_operation: u32,
    pub ae_ra_returncode: u32,
    pub ae_ra_restype: u32,
    pub ae_ra_fileid: u32,
}
impl Copy for AE_RESACCESS {}
impl Clone for AE_RESACCESS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const AE_RESACCESS: u32 = 7u32;
