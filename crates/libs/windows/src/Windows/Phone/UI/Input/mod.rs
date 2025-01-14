#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackPressedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BackPressedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl BackPressedEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHandled)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for BackPressedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackPressedEventArgs>();
}
unsafe impl windows_core::Interface for BackPressedEventArgs {
    type Vtable = <IBackPressedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBackPressedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BackPressedEventArgs {
    const NAME: &'static str = "Windows.Phone.UI.Input.BackPressedEventArgs";
}
unsafe impl Send for BackPressedEventArgs {}
unsafe impl Sync for BackPressedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CameraEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CameraEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CameraEventArgs {}
impl windows_core::RuntimeType for CameraEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICameraEventArgs>();
}
unsafe impl windows_core::Interface for CameraEventArgs {
    type Vtable = <ICameraEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICameraEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CameraEventArgs {
    const NAME: &'static str = "Windows.Phone.UI.Input.CameraEventArgs";
}
unsafe impl Send for CameraEventArgs {}
unsafe impl Sync for CameraEventArgs {}
pub struct HardwareButtons;
impl HardwareButtons {
    pub fn BackPressed<P0>(handler: Option<P0>) -> windows_core::Result<i64>
    where
        P0: FnMut(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<BackPressedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        Self::IHardwareButtonsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackPressed)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&handler.map(|closure| super::super::super::Foundation::EventHandler::<BackPressedEventArgs>::new(closure))), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveBackPressed(token: i64) -> windows_core::Result<()> {
        Self::IHardwareButtonsStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveBackPressed)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn CameraHalfPressed<P0>(handler: Option<P0>) -> windows_core::Result<i64>
    where
        P0: FnMut(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<CameraEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        Self::IHardwareButtonsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CameraHalfPressed)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&handler.map(|closure| super::super::super::Foundation::EventHandler::<CameraEventArgs>::new(closure))), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveCameraHalfPressed(token: i64) -> windows_core::Result<()> {
        Self::IHardwareButtonsStatics2(|this| unsafe { (windows_core::Interface::vtable(this).RemoveCameraHalfPressed)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn CameraPressed<P0>(handler: Option<P0>) -> windows_core::Result<i64>
    where
        P0: FnMut(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<CameraEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        Self::IHardwareButtonsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CameraPressed)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&handler.map(|closure| super::super::super::Foundation::EventHandler::<CameraEventArgs>::new(closure))), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveCameraPressed(token: i64) -> windows_core::Result<()> {
        Self::IHardwareButtonsStatics2(|this| unsafe { (windows_core::Interface::vtable(this).RemoveCameraPressed)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn CameraReleased<P0>(handler: Option<P0>) -> windows_core::Result<i64>
    where
        P0: FnMut(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<CameraEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        Self::IHardwareButtonsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CameraReleased)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&handler.map(|closure| super::super::super::Foundation::EventHandler::<CameraEventArgs>::new(closure))), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveCameraReleased(token: i64) -> windows_core::Result<()> {
        Self::IHardwareButtonsStatics2(|this| unsafe { (windows_core::Interface::vtable(this).RemoveCameraReleased)(windows_core::Interface::as_raw(this), token).ok() })
    }
    fn IHardwareButtonsStatics<R, F: FnOnce(&IHardwareButtonsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HardwareButtons, IHardwareButtonsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IHardwareButtonsStatics2<R, F: FnOnce(&IHardwareButtonsStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HardwareButtons, IHardwareButtonsStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for HardwareButtons {
    const NAME: &'static str = "Windows.Phone.UI.Input.HardwareButtons";
}
windows_core::imp::define_interface!(IBackPressedEventArgs, IBackPressedEventArgs_Vtbl, 0xf6f555ff_64ec_42a2_b93b_2fbc0c36a121);
impl windows_core::RuntimeType for IBackPressedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackPressedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICameraEventArgs, ICameraEventArgs_Vtbl, 0xb4063bda_201f_473d_bc69_e9e4ac57c9d0);
impl windows_core::RuntimeType for ICameraEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICameraEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IHardwareButtonsStatics, IHardwareButtonsStatics_Vtbl, 0x594b8780_da66_4fd8_a776_7506bd0cbfa7);
impl windows_core::RuntimeType for IHardwareButtonsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHardwareButtonsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BackPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveBackPressed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHardwareButtonsStatics2, IHardwareButtonsStatics2_Vtbl, 0x39c6c274_993f_40dd_854c_831a8934b92e);
impl windows_core::RuntimeType for IHardwareButtonsStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHardwareButtonsStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CameraHalfPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveCameraHalfPressed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub CameraPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveCameraPressed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub CameraReleased: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveCameraReleased: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
