#[cfg(feature = "Storage_Streams")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileInformation(windows_core::IUnknown);
#[cfg(feature = "Storage_Streams")]
windows_core::imp::interface_hierarchy!(FileInformation, windows_core::IUnknown, windows_core::IInspectable, IStorageItemInformation);
#[cfg(feature = "Storage_Streams")]
windows_core::imp::required_hierarchy!(FileInformation, super::Streams::IInputStreamReference, super::Streams::IRandomAccessStreamReference, super::IStorageFile, super::IStorageFile2, super::IStorageFilePropertiesWithAvailability, super::IStorageItem, super::IStorageItem2, super::IStorageItemProperties, super::IStorageItemPropertiesWithProvider);
#[cfg(feature = "Storage_Streams")]
impl FileInformation {
    pub fn OpenSequentialReadAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<super::Streams::IInputStream>> {
        let this = &windows_core::Interface::cast::<super::Streams::IInputStreamReference>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenSequentialReadAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OpenReadAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<super::Streams::IRandomAccessStreamWithContentType>> {
        let this = &windows_core::Interface::cast::<super::Streams::IRandomAccessStreamReference>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenReadAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FileType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileType)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ContentType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentType)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn OpenAsync(&self, accessmode: super::FileAccessMode) -> windows_core::Result<windows_future::IAsyncOperation<super::Streams::IRandomAccessStream>> {
        let this = &windows_core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenAsync)(windows_core::Interface::as_raw(this), accessmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OpenTransactedWriteAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageStreamTransaction>> {
        let this = &windows_core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenTransactedWriteAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CopyOverloadDefaultNameAndOptions<P0>(&self, destinationfolder: P0) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFile>>
    where
        P0: windows_core::Param<super::IStorageFolder>,
    {
        let this = &windows_core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CopyOverloadDefaultNameAndOptions)(windows_core::Interface::as_raw(this), destinationfolder.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CopyOverloadDefaultOptions<P0>(&self, destinationfolder: P0, desirednewname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFile>>
    where
        P0: windows_core::Param<super::IStorageFolder>,
    {
        let this = &windows_core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CopyOverloadDefaultOptions)(windows_core::Interface::as_raw(this), destinationfolder.param().abi(), core::mem::transmute_copy(desirednewname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CopyOverload<P0>(&self, destinationfolder: P0, desirednewname: &windows_core::HSTRING, option: super::NameCollisionOption) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFile>>
    where
        P0: windows_core::Param<super::IStorageFolder>,
    {
        let this = &windows_core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CopyOverload)(windows_core::Interface::as_raw(this), destinationfolder.param().abi(), core::mem::transmute_copy(desirednewname), option, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CopyAndReplaceAsync<P0>(&self, filetoreplace: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<super::IStorageFile>,
    {
        let this = &windows_core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CopyAndReplaceAsync)(windows_core::Interface::as_raw(this), filetoreplace.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MoveOverloadDefaultNameAndOptions<P0>(&self, destinationfolder: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<super::IStorageFolder>,
    {
        let this = &windows_core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveOverloadDefaultNameAndOptions)(windows_core::Interface::as_raw(this), destinationfolder.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MoveOverloadDefaultOptions<P0>(&self, destinationfolder: P0, desirednewname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<super::IStorageFolder>,
    {
        let this = &windows_core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveOverloadDefaultOptions)(windows_core::Interface::as_raw(this), destinationfolder.param().abi(), core::mem::transmute_copy(desirednewname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MoveOverload<P0>(&self, destinationfolder: P0, desirednewname: &windows_core::HSTRING, option: super::NameCollisionOption) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<super::IStorageFolder>,
    {
        let this = &windows_core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveOverload)(windows_core::Interface::as_raw(this), destinationfolder.param().abi(), core::mem::transmute_copy(desirednewname), option, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MoveAndReplaceAsync<P0>(&self, filetoreplace: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<super::IStorageFile>,
    {
        let this = &windows_core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveAndReplaceAsync)(windows_core::Interface::as_raw(this), filetoreplace.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OpenWithOptionsAsync(&self, accessmode: super::FileAccessMode, options: super::StorageOpenOptions) -> windows_core::Result<windows_future::IAsyncOperation<super::Streams::IRandomAccessStream>> {
        let this = &windows_core::Interface::cast::<super::IStorageFile2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenWithOptionsAsync)(windows_core::Interface::as_raw(this), accessmode, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OpenTransactedWriteWithOptionsAsync(&self, options: super::StorageOpenOptions) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageStreamTransaction>> {
        let this = &windows_core::Interface::cast::<super::IStorageFile2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenTransactedWriteWithOptionsAsync)(windows_core::Interface::as_raw(this), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsAvailable(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IStorageFilePropertiesWithAvailability>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAvailable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RenameAsyncOverloadDefaultOptions(&self, desiredname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenameAsyncOverloadDefaultOptions)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(desiredname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RenameAsync(&self, desiredname: &windows_core::HSTRING, option: super::NameCollisionOption) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenameAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(desiredname), option, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteAsyncOverloadDefaultOptions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteAsync(&self, option: super::StorageDeleteOption) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteAsync)(windows_core::Interface::as_raw(this), option, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn GetBasicPropertiesAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<super::FileProperties::BasicProperties>> {
        let this = &windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBasicPropertiesAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Path(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Path)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Attributes(&self) -> windows_core::Result<super::FileAttributes> {
        let this = &windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Attributes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DateCreated(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = &windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DateCreated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsOfType(&self, r#type: super::StorageItemTypes) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOfType)(windows_core::Interface::as_raw(this), r#type, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn GetParentAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFolder>> {
        let this = &windows_core::Interface::cast::<super::IStorageItem2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetParentAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsEqual<P0>(&self, item: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::IStorageItem>,
    {
        let this = &windows_core::Interface::cast::<super::IStorageItem2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEqual)(windows_core::Interface::as_raw(this), item.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn MusicProperties(&self) -> windows_core::Result<super::FileProperties::MusicProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MusicProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn VideoProperties(&self) -> windows_core::Result<super::FileProperties::VideoProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn ImageProperties(&self) -> windows_core::Result<super::FileProperties::ImageProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImageProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn DocumentProperties(&self) -> windows_core::Result<super::FileProperties::DocumentProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn BasicProperties(&self) -> windows_core::Result<super::FileProperties::BasicProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BasicProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn Thumbnail(&self) -> windows_core::Result<super::FileProperties::StorageItemThumbnail> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Thumbnail)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ThumbnailUpdated<P0>(&self, changedhandler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IStorageItemInformation, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ThumbnailUpdated)(windows_core::Interface::as_raw(this), changedhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveThumbnailUpdated(&self, eventcookie: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveThumbnailUpdated)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn PropertiesUpdated<P0>(&self, changedhandler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IStorageItemInformation, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PropertiesUpdated)(windows_core::Interface::as_raw(this), changedhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePropertiesUpdated(&self, eventcookie: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePropertiesUpdated)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: super::FileProperties::ThumbnailMode) -> windows_core::Result<windows_future::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetThumbnailAsyncOverloadDefaultSizeDefaultOptions)(windows_core::Interface::as_raw(this), mode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32) -> windows_core::Result<windows_future::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetThumbnailAsyncOverloadDefaultOptions)(windows_core::Interface::as_raw(this), mode, requestedsize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn GetThumbnailAsync(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32, options: super::FileProperties::ThumbnailOptions) -> windows_core::Result<windows_future::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetThumbnailAsync)(windows_core::Interface::as_raw(this), mode, requestedsize, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn DisplayType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayType)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FolderRelativeId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FolderRelativeId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn Properties(&self) -> windows_core::Result<super::FileProperties::StorageItemContentProperties> {
        let this = &windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Provider(&self) -> windows_core::Result<super::StorageProvider> {
        let this = &windows_core::Interface::cast::<super::IStorageItemPropertiesWithProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Provider)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeType for FileInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageItemInformation>();
}
#[cfg(feature = "Storage_Streams")]
unsafe impl windows_core::Interface for FileInformation {
    type Vtable = <IStorageItemInformation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageItemInformation as windows_core::Interface>::IID;
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for FileInformation {
    const NAME: &'static str = "Windows.Storage.BulkAccess.FileInformation";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileInformationFactory(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FileInformationFactory, windows_core::IUnknown, windows_core::IInspectable);
impl FileInformationFactory {
    pub fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<IStorageItemInformation>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemsAsync)(windows_core::Interface::as_raw(this), startindex, maxitemstoretrieve, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetItemsAsyncDefaultStartAndCount(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<IStorageItemInformation>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemsAsyncDefaultStartAndCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetFilesAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<FileInformation>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFilesAsync)(windows_core::Interface::as_raw(this), startindex, maxitemstoretrieve, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetFilesAsyncDefaultStartAndCount(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<FileInformation>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFilesAsyncDefaultStartAndCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn GetFoldersAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<FolderInformation>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFoldersAsync)(windows_core::Interface::as_raw(this), startindex, maxitemstoretrieve, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn GetFoldersAsyncDefaultStartAndCount(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<FolderInformation>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFoldersAsyncDefaultStartAndCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetVirtualizedItemsVector(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetVirtualizedItemsVector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetVirtualizedFilesVector(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetVirtualizedFilesVector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetVirtualizedFoldersVector(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetVirtualizedFoldersVector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub fn CreateWithMode<P0>(queryresult: P0, mode: super::FileProperties::ThumbnailMode) -> windows_core::Result<FileInformationFactory>
    where
        P0: windows_core::Param<super::Search::IStorageQueryResultBase>,
    {
        Self::IFileInformationFactoryFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithMode)(windows_core::Interface::as_raw(this), queryresult.param().abi(), mode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub fn CreateWithModeAndSize<P0>(queryresult: P0, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32) -> windows_core::Result<FileInformationFactory>
    where
        P0: windows_core::Param<super::Search::IStorageQueryResultBase>,
    {
        Self::IFileInformationFactoryFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithModeAndSize)(windows_core::Interface::as_raw(this), queryresult.param().abi(), mode, requestedthumbnailsize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub fn CreateWithModeAndSizeAndOptions<P0>(queryresult: P0, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions) -> windows_core::Result<FileInformationFactory>
    where
        P0: windows_core::Param<super::Search::IStorageQueryResultBase>,
    {
        Self::IFileInformationFactoryFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithModeAndSizeAndOptions)(windows_core::Interface::as_raw(this), queryresult.param().abi(), mode, requestedthumbnailsize, thumbnailoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub fn CreateWithModeAndSizeAndOptionsAndFlags<P0>(queryresult: P0, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions, delayload: bool) -> windows_core::Result<FileInformationFactory>
    where
        P0: windows_core::Param<super::Search::IStorageQueryResultBase>,
    {
        Self::IFileInformationFactoryFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithModeAndSizeAndOptionsAndFlags)(windows_core::Interface::as_raw(this), queryresult.param().abi(), mode, requestedthumbnailsize, thumbnailoptions, delayload, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IFileInformationFactoryFactory<R, F: FnOnce(&IFileInformationFactoryFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FileInformationFactory, IFileInformationFactoryFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FileInformationFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFileInformationFactory>();
}
unsafe impl windows_core::Interface for FileInformationFactory {
    type Vtable = <IFileInformationFactory as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFileInformationFactory as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FileInformationFactory {
    const NAME: &'static str = "Windows.Storage.BulkAccess.FileInformationFactory";
}
unsafe impl Send for FileInformationFactory {}
unsafe impl Sync for FileInformationFactory {}
#[cfg(feature = "Storage_Search")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FolderInformation(windows_core::IUnknown);
#[cfg(feature = "Storage_Search")]
windows_core::imp::interface_hierarchy!(FolderInformation, windows_core::IUnknown, windows_core::IInspectable, IStorageItemInformation);
#[cfg(feature = "Storage_Search")]
windows_core::imp::required_hierarchy!(FolderInformation, super::IStorageFolder, super::IStorageFolder2, super::Search::IStorageFolderQueryOperations, super::IStorageItem, super::IStorageItem2, super::IStorageItemProperties, super::IStorageItemPropertiesWithProvider);
#[cfg(feature = "Storage_Search")]
impl FolderInformation {
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFileAsyncOverloadDefaultOptions(&self, desiredname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFile>> {
        let this = &windows_core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFileAsyncOverloadDefaultOptions)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(desiredname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFileAsync(&self, desiredname: &windows_core::HSTRING, options: super::CreationCollisionOption) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFile>> {
        let this = &windows_core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFileAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(desiredname), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFolderAsyncOverloadDefaultOptions(&self, desiredname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFolder>> {
        let this = &windows_core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFolderAsyncOverloadDefaultOptions)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(desiredname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFolderAsync(&self, desiredname: &windows_core::HSTRING, options: super::CreationCollisionOption) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFolder>> {
        let this = &windows_core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFolderAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(desiredname), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetFileAsync(&self, name: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFile>> {
        let this = &windows_core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFileAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFolderAsync(&self, name: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFolder>> {
        let this = &windows_core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFolderAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetItemAsync(&self, name: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::IStorageItem>> {
        let this = &windows_core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetFilesAsyncOverloadDefaultOptionsStartAndCount(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::StorageFile>>> {
        let this = &windows_core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFilesAsyncOverloadDefaultOptionsStartAndCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::StorageFolder>>> {
        let this = &windows_core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFoldersAsyncOverloadDefaultOptionsStartAndCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetItemsAsyncOverloadDefaultStartAndCount(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::IStorageItem>>> {
        let this = &windows_core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemsAsyncOverloadDefaultStartAndCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetItemAsync(&self, name: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::IStorageItem>> {
        let this = &windows_core::Interface::cast::<super::IStorageFolder2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetItemAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetIndexedStateAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<super::Search::IndexedState>> {
        let this = &windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIndexedStateAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFileQueryOverloadDefault(&self) -> windows_core::Result<super::Search::StorageFileQueryResult> {
        let this = &windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFileQueryOverloadDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFileQuery(&self, query: super::Search::CommonFileQuery) -> windows_core::Result<super::Search::StorageFileQueryResult> {
        let this = &windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFileQuery)(windows_core::Interface::as_raw(this), query, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFileQueryWithOptions<P0>(&self, queryoptions: P0) -> windows_core::Result<super::Search::StorageFileQueryResult>
    where
        P0: windows_core::Param<super::Search::QueryOptions>,
    {
        let this = &windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFileQueryWithOptions)(windows_core::Interface::as_raw(this), queryoptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFolderQueryOverloadDefault(&self) -> windows_core::Result<super::Search::StorageFolderQueryResult> {
        let this = &windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFolderQueryOverloadDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFolderQuery(&self, query: super::Search::CommonFolderQuery) -> windows_core::Result<super::Search::StorageFolderQueryResult> {
        let this = &windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFolderQuery)(windows_core::Interface::as_raw(this), query, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFolderQueryWithOptions<P0>(&self, queryoptions: P0) -> windows_core::Result<super::Search::StorageFolderQueryResult>
    where
        P0: windows_core::Param<super::Search::QueryOptions>,
    {
        let this = &windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFolderQueryWithOptions)(windows_core::Interface::as_raw(this), queryoptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateItemQuery(&self) -> windows_core::Result<super::Search::StorageItemQueryResult> {
        let this = &windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateItemQuery)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateItemQueryWithOptions<P0>(&self, queryoptions: P0) -> windows_core::Result<super::Search::StorageItemQueryResult>
    where
        P0: windows_core::Param<super::Search::QueryOptions>,
    {
        let this = &windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateItemQueryWithOptions)(windows_core::Interface::as_raw(this), queryoptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetFilesAsync(&self, query: super::Search::CommonFileQuery, startindex: u32, maxitemstoretrieve: u32) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::StorageFile>>> {
        let this = &windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFilesAsync)(windows_core::Interface::as_raw(this), query, startindex, maxitemstoretrieve, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetFilesAsyncOverloadDefaultStartAndCount(&self, query: super::Search::CommonFileQuery) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::StorageFile>>> {
        let this = &windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFilesAsyncOverloadDefaultStartAndCount)(windows_core::Interface::as_raw(this), query, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFoldersAsync(&self, query: super::Search::CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::StorageFolder>>> {
        let this = &windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFoldersAsync)(windows_core::Interface::as_raw(this), query, startindex, maxitemstoretrieve, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFoldersAsyncOverloadDefaultStartAndCount(&self, query: super::Search::CommonFolderQuery) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::StorageFolder>>> {
        let this = &windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFoldersAsyncOverloadDefaultStartAndCount)(windows_core::Interface::as_raw(this), query, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::IStorageItem>>> {
        let this = &windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemsAsync)(windows_core::Interface::as_raw(this), startindex, maxitemstoretrieve, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AreQueryOptionsSupported<P0>(&self, queryoptions: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::Search::QueryOptions>,
    {
        let this = &windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AreQueryOptionsSupported)(windows_core::Interface::as_raw(this), queryoptions.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn IsCommonFolderQuerySupported(&self, query: super::Search::CommonFolderQuery) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCommonFolderQuerySupported)(windows_core::Interface::as_raw(this), query, &mut result__).map(|| result__)
        }
    }
    pub fn IsCommonFileQuerySupported(&self, query: super::Search::CommonFileQuery) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCommonFileQuerySupported)(windows_core::Interface::as_raw(this), query, &mut result__).map(|| result__)
        }
    }
    pub fn RenameAsyncOverloadDefaultOptions(&self, desiredname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenameAsyncOverloadDefaultOptions)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(desiredname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RenameAsync(&self, desiredname: &windows_core::HSTRING, option: super::NameCollisionOption) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenameAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(desiredname), option, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteAsyncOverloadDefaultOptions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteAsync(&self, option: super::StorageDeleteOption) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteAsync)(windows_core::Interface::as_raw(this), option, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn GetBasicPropertiesAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<super::FileProperties::BasicProperties>> {
        let this = &windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBasicPropertiesAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Path(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Path)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Attributes(&self) -> windows_core::Result<super::FileAttributes> {
        let this = &windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Attributes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DateCreated(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = &windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DateCreated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsOfType(&self, r#type: super::StorageItemTypes) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOfType)(windows_core::Interface::as_raw(this), r#type, &mut result__).map(|| result__)
        }
    }
    pub fn GetParentAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFolder>> {
        let this = &windows_core::Interface::cast::<super::IStorageItem2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetParentAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsEqual<P0>(&self, item: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::IStorageItem>,
    {
        let this = &windows_core::Interface::cast::<super::IStorageItem2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEqual)(windows_core::Interface::as_raw(this), item.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn MusicProperties(&self) -> windows_core::Result<super::FileProperties::MusicProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MusicProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn VideoProperties(&self) -> windows_core::Result<super::FileProperties::VideoProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn ImageProperties(&self) -> windows_core::Result<super::FileProperties::ImageProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImageProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn DocumentProperties(&self) -> windows_core::Result<super::FileProperties::DocumentProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn BasicProperties(&self) -> windows_core::Result<super::FileProperties::BasicProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BasicProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn Thumbnail(&self) -> windows_core::Result<super::FileProperties::StorageItemThumbnail> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Thumbnail)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ThumbnailUpdated<P0>(&self, changedhandler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IStorageItemInformation, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ThumbnailUpdated)(windows_core::Interface::as_raw(this), changedhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveThumbnailUpdated(&self, eventcookie: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveThumbnailUpdated)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn PropertiesUpdated<P0>(&self, changedhandler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IStorageItemInformation, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PropertiesUpdated)(windows_core::Interface::as_raw(this), changedhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePropertiesUpdated(&self, eventcookie: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePropertiesUpdated)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: super::FileProperties::ThumbnailMode) -> windows_core::Result<windows_future::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetThumbnailAsyncOverloadDefaultSizeDefaultOptions)(windows_core::Interface::as_raw(this), mode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32) -> windows_core::Result<windows_future::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetThumbnailAsyncOverloadDefaultOptions)(windows_core::Interface::as_raw(this), mode, requestedsize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32, options: super::FileProperties::ThumbnailOptions) -> windows_core::Result<windows_future::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetThumbnailAsync)(windows_core::Interface::as_raw(this), mode, requestedsize, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn DisplayType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayType)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FolderRelativeId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FolderRelativeId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn Properties(&self) -> windows_core::Result<super::FileProperties::StorageItemContentProperties> {
        let this = &windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Provider(&self) -> windows_core::Result<super::StorageProvider> {
        let this = &windows_core::Interface::cast::<super::IStorageItemPropertiesWithProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Provider)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Storage_Search")]
impl windows_core::RuntimeType for FolderInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageItemInformation>();
}
#[cfg(feature = "Storage_Search")]
unsafe impl windows_core::Interface for FolderInformation {
    type Vtable = <IStorageItemInformation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageItemInformation as windows_core::Interface>::IID;
}
#[cfg(feature = "Storage_Search")]
impl windows_core::RuntimeName for FolderInformation {
    const NAME: &'static str = "Windows.Storage.BulkAccess.FolderInformation";
}
windows_core::imp::define_interface!(IFileInformationFactory, IFileInformationFactory_Vtbl, 0x401d88be_960f_4d6d_a7d0_1a3861e76c83);
impl windows_core::RuntimeType for IFileInformationFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFileInformationFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetItemsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetItemsAsyncDefaultStartAndCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetFilesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetFilesAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetFilesAsyncDefaultStartAndCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetFilesAsyncDefaultStartAndCount: usize,
    #[cfg(feature = "Storage_Search")]
    pub GetFoldersAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    GetFoldersAsync: usize,
    #[cfg(feature = "Storage_Search")]
    pub GetFoldersAsyncDefaultStartAndCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    GetFoldersAsyncDefaultStartAndCount: usize,
    pub GetVirtualizedItemsVector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVirtualizedFilesVector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVirtualizedFoldersVector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFileInformationFactoryFactory, IFileInformationFactoryFactory_Vtbl, 0x84ea0e7d_e4a2_4f00_8afa_af5e0f826bd5);
impl windows_core::RuntimeType for IFileInformationFactoryFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFileInformationFactoryFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub CreateWithMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::FileProperties::ThumbnailMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Search")))]
    CreateWithMode: usize,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub CreateWithModeAndSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::FileProperties::ThumbnailMode, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Search")))]
    CreateWithModeAndSize: usize,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub CreateWithModeAndSizeAndOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::FileProperties::ThumbnailMode, u32, super::FileProperties::ThumbnailOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Search")))]
    CreateWithModeAndSizeAndOptions: usize,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub CreateWithModeAndSizeAndOptionsAndFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::FileProperties::ThumbnailMode, u32, super::FileProperties::ThumbnailOptions, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Search")))]
    CreateWithModeAndSizeAndOptionsAndFlags: usize,
}
windows_core::imp::define_interface!(IStorageItemInformation, IStorageItemInformation_Vtbl, 0x87a5cb8b_8972_4f40_8de0_d86fb179d8fa);
impl windows_core::RuntimeType for IStorageItemInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IStorageItemInformation, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageItemInformation {
    #[cfg(feature = "Storage_FileProperties")]
    pub fn MusicProperties(&self) -> windows_core::Result<super::FileProperties::MusicProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MusicProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn VideoProperties(&self) -> windows_core::Result<super::FileProperties::VideoProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn ImageProperties(&self) -> windows_core::Result<super::FileProperties::ImageProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImageProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn DocumentProperties(&self) -> windows_core::Result<super::FileProperties::DocumentProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn BasicProperties(&self) -> windows_core::Result<super::FileProperties::BasicProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BasicProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn Thumbnail(&self) -> windows_core::Result<super::FileProperties::StorageItemThumbnail> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Thumbnail)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ThumbnailUpdated<P0>(&self, changedhandler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IStorageItemInformation, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ThumbnailUpdated)(windows_core::Interface::as_raw(this), changedhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveThumbnailUpdated(&self, eventcookie: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveThumbnailUpdated)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn PropertiesUpdated<P0>(&self, changedhandler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IStorageItemInformation, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PropertiesUpdated)(windows_core::Interface::as_raw(this), changedhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePropertiesUpdated(&self, eventcookie: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePropertiesUpdated)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
}
#[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl windows_core::RuntimeName for IStorageItemInformation {
    const NAME: &'static str = "Windows.Storage.BulkAccess.IStorageItemInformation";
}
#[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
pub trait IStorageItemInformation_Impl: windows_core::IUnknownImpl {
    fn MusicProperties(&self) -> windows_core::Result<super::FileProperties::MusicProperties>;
    fn VideoProperties(&self) -> windows_core::Result<super::FileProperties::VideoProperties>;
    fn ImageProperties(&self) -> windows_core::Result<super::FileProperties::ImageProperties>;
    fn DocumentProperties(&self) -> windows_core::Result<super::FileProperties::DocumentProperties>;
    fn BasicProperties(&self) -> windows_core::Result<super::FileProperties::BasicProperties>;
    fn Thumbnail(&self) -> windows_core::Result<super::FileProperties::StorageItemThumbnail>;
    fn ThumbnailUpdated(&self, changedHandler: windows_core::Ref<'_, super::super::Foundation::TypedEventHandler<IStorageItemInformation, windows_core::IInspectable>>) -> windows_core::Result<i64>;
    fn RemoveThumbnailUpdated(&self, eventCookie: i64) -> windows_core::Result<()>;
    fn PropertiesUpdated(&self, changedHandler: windows_core::Ref<'_, super::super::Foundation::TypedEventHandler<IStorageItemInformation, windows_core::IInspectable>>) -> windows_core::Result<i64>;
    fn RemovePropertiesUpdated(&self, eventCookie: i64) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl IStorageItemInformation_Vtbl {
    pub const fn new<Identity: IStorageItemInformation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MusicProperties<Identity: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageItemInformation_Impl::MusicProperties(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VideoProperties<Identity: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageItemInformation_Impl::VideoProperties(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ImageProperties<Identity: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageItemInformation_Impl::ImageProperties(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DocumentProperties<Identity: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageItemInformation_Impl::DocumentProperties(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BasicProperties<Identity: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageItemInformation_Impl::BasicProperties(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Thumbnail<Identity: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageItemInformation_Impl::Thumbnail(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ThumbnailUpdated<Identity: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, changedhandler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageItemInformation_Impl::ThumbnailUpdated(this, core::mem::transmute_copy(&changedhandler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveThumbnailUpdated<Identity: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventcookie: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorageItemInformation_Impl::RemoveThumbnailUpdated(this, eventcookie).into()
            }
        }
        unsafe extern "system" fn PropertiesUpdated<Identity: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, changedhandler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageItemInformation_Impl::PropertiesUpdated(this, core::mem::transmute_copy(&changedhandler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePropertiesUpdated<Identity: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventcookie: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorageItemInformation_Impl::RemovePropertiesUpdated(this, eventcookie).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageItemInformation, OFFSET>(),
            MusicProperties: MusicProperties::<Identity, OFFSET>,
            VideoProperties: VideoProperties::<Identity, OFFSET>,
            ImageProperties: ImageProperties::<Identity, OFFSET>,
            DocumentProperties: DocumentProperties::<Identity, OFFSET>,
            BasicProperties: BasicProperties::<Identity, OFFSET>,
            Thumbnail: Thumbnail::<Identity, OFFSET>,
            ThumbnailUpdated: ThumbnailUpdated::<Identity, OFFSET>,
            RemoveThumbnailUpdated: RemoveThumbnailUpdated::<Identity, OFFSET>,
            PropertiesUpdated: PropertiesUpdated::<Identity, OFFSET>,
            RemovePropertiesUpdated: RemovePropertiesUpdated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageItemInformation as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IStorageItemInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_FileProperties")]
    pub MusicProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    MusicProperties: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub VideoProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    VideoProperties: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub ImageProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    ImageProperties: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub DocumentProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    DocumentProperties: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub BasicProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    BasicProperties: usize,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub Thumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Streams")))]
    Thumbnail: usize,
    pub ThumbnailUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveThumbnailUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PropertiesUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePropertiesUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
