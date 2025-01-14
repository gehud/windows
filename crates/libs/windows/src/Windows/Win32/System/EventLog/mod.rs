#[inline]
pub unsafe fn BackupEventLogA<P1>(heventlog: super::super::Foundation::HANDLE, lpbackupfilename: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn BackupEventLogA(heventlog : super::super::Foundation:: HANDLE, lpbackupfilename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { BackupEventLogA(heventlog, lpbackupfilename.param().abi()).ok() }
}
#[inline]
pub unsafe fn BackupEventLogW<P1>(heventlog: super::super::Foundation::HANDLE, lpbackupfilename: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn BackupEventLogW(heventlog : super::super::Foundation:: HANDLE, lpbackupfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { BackupEventLogW(heventlog, lpbackupfilename.param().abi()).ok() }
}
#[inline]
pub unsafe fn ClearEventLogA<P1>(heventlog: super::super::Foundation::HANDLE, lpbackupfilename: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn ClearEventLogA(heventlog : super::super::Foundation:: HANDLE, lpbackupfilename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { ClearEventLogA(heventlog, lpbackupfilename.param().abi()).ok() }
}
#[inline]
pub unsafe fn ClearEventLogW<P1>(heventlog: super::super::Foundation::HANDLE, lpbackupfilename: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn ClearEventLogW(heventlog : super::super::Foundation:: HANDLE, lpbackupfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { ClearEventLogW(heventlog, lpbackupfilename.param().abi()).ok() }
}
#[inline]
pub unsafe fn CloseEventLog(heventlog: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn CloseEventLog(heventlog : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { CloseEventLog(heventlog).ok() }
}
#[inline]
pub unsafe fn DeregisterEventSource(heventlog: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn DeregisterEventSource(heventlog : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { DeregisterEventSource(heventlog).ok() }
}
#[inline]
pub unsafe fn EvtArchiveExportedLog<P1>(session: Option<EVT_HANDLE>, logfilepath: P1, locale: u32, flags: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wevtapi.dll" "system" fn EvtArchiveExportedLog(session : EVT_HANDLE, logfilepath : windows_core::PCWSTR, locale : u32, flags : u32) -> windows_core::BOOL);
    unsafe { EvtArchiveExportedLog(session.unwrap_or(core::mem::zeroed()) as _, logfilepath.param().abi(), locale, flags).ok() }
}
#[inline]
pub unsafe fn EvtCancel(object: Option<EVT_HANDLE>) -> windows_core::Result<()> {
    windows_targets::link!("wevtapi.dll" "system" fn EvtCancel(object : EVT_HANDLE) -> windows_core::BOOL);
    unsafe { EvtCancel(object.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn EvtClearLog<P1, P2>(session: Option<EVT_HANDLE>, channelpath: P1, targetfilepath: P2, flags: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wevtapi.dll" "system" fn EvtClearLog(session : EVT_HANDLE, channelpath : windows_core::PCWSTR, targetfilepath : windows_core::PCWSTR, flags : u32) -> windows_core::BOOL);
    unsafe { EvtClearLog(session.unwrap_or(core::mem::zeroed()) as _, channelpath.param().abi(), targetfilepath.param().abi(), flags).ok() }
}
#[inline]
pub unsafe fn EvtClose(object: EVT_HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("wevtapi.dll" "system" fn EvtClose(object : EVT_HANDLE) -> windows_core::BOOL);
    unsafe { EvtClose(object).ok() }
}
#[inline]
pub unsafe fn EvtCreateBookmark<P0>(bookmarkxml: P0) -> windows_core::Result<EVT_HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wevtapi.dll" "system" fn EvtCreateBookmark(bookmarkxml : windows_core::PCWSTR) -> EVT_HANDLE);
    let result__ = unsafe { EvtCreateBookmark(bookmarkxml.param().abi()) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn EvtCreateRenderContext(valuepaths: Option<&[windows_core::PCWSTR]>, flags: u32) -> windows_core::Result<EVT_HANDLE> {
    windows_targets::link!("wevtapi.dll" "system" fn EvtCreateRenderContext(valuepathscount : u32, valuepaths : *const windows_core::PCWSTR, flags : u32) -> EVT_HANDLE);
    let result__ = unsafe { EvtCreateRenderContext(valuepaths.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(valuepaths.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), flags) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn EvtExportLog<P1, P2, P3>(session: Option<EVT_HANDLE>, path: P1, query: P2, targetfilepath: P3, flags: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wevtapi.dll" "system" fn EvtExportLog(session : EVT_HANDLE, path : windows_core::PCWSTR, query : windows_core::PCWSTR, targetfilepath : windows_core::PCWSTR, flags : u32) -> windows_core::BOOL);
    unsafe { EvtExportLog(session.unwrap_or(core::mem::zeroed()) as _, path.param().abi(), query.param().abi(), targetfilepath.param().abi(), flags).ok() }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn EvtFormatMessage(publishermetadata: Option<EVT_HANDLE>, event: Option<EVT_HANDLE>, messageid: u32, values: Option<&[EVT_VARIANT]>, flags: u32, buffer: Option<&mut [u16]>, bufferused: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("wevtapi.dll" "system" fn EvtFormatMessage(publishermetadata : EVT_HANDLE, event : EVT_HANDLE, messageid : u32, valuecount : u32, values : *const EVT_VARIANT, flags : u32, buffersize : u32, buffer : windows_core::PWSTR, bufferused : *mut u32) -> windows_core::BOOL);
    unsafe { EvtFormatMessage(publishermetadata.unwrap_or(core::mem::zeroed()) as _, event.unwrap_or(core::mem::zeroed()) as _, messageid, values.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(values.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), flags, buffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(buffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), bufferused as _).ok() }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn EvtGetChannelConfigProperty(channelconfig: EVT_HANDLE, propertyid: EVT_CHANNEL_CONFIG_PROPERTY_ID, flags: u32, propertyvaluebuffersize: u32, propertyvaluebuffer: Option<*mut EVT_VARIANT>, propertyvaluebufferused: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("wevtapi.dll" "system" fn EvtGetChannelConfigProperty(channelconfig : EVT_HANDLE, propertyid : EVT_CHANNEL_CONFIG_PROPERTY_ID, flags : u32, propertyvaluebuffersize : u32, propertyvaluebuffer : *mut EVT_VARIANT, propertyvaluebufferused : *mut u32) -> windows_core::BOOL);
    unsafe { EvtGetChannelConfigProperty(channelconfig, propertyid, flags, propertyvaluebuffersize, propertyvaluebuffer.unwrap_or(core::mem::zeroed()) as _, propertyvaluebufferused as _).ok() }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn EvtGetEventInfo(event: EVT_HANDLE, propertyid: EVT_EVENT_PROPERTY_ID, propertyvaluebuffersize: u32, propertyvaluebuffer: Option<*mut EVT_VARIANT>, propertyvaluebufferused: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("wevtapi.dll" "system" fn EvtGetEventInfo(event : EVT_HANDLE, propertyid : EVT_EVENT_PROPERTY_ID, propertyvaluebuffersize : u32, propertyvaluebuffer : *mut EVT_VARIANT, propertyvaluebufferused : *mut u32) -> windows_core::BOOL);
    unsafe { EvtGetEventInfo(event, propertyid, propertyvaluebuffersize, propertyvaluebuffer.unwrap_or(core::mem::zeroed()) as _, propertyvaluebufferused as _).ok() }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn EvtGetEventMetadataProperty(eventmetadata: EVT_HANDLE, propertyid: EVT_EVENT_METADATA_PROPERTY_ID, flags: u32, eventmetadatapropertybuffersize: u32, eventmetadatapropertybuffer: Option<*mut EVT_VARIANT>, eventmetadatapropertybufferused: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("wevtapi.dll" "system" fn EvtGetEventMetadataProperty(eventmetadata : EVT_HANDLE, propertyid : EVT_EVENT_METADATA_PROPERTY_ID, flags : u32, eventmetadatapropertybuffersize : u32, eventmetadatapropertybuffer : *mut EVT_VARIANT, eventmetadatapropertybufferused : *mut u32) -> windows_core::BOOL);
    unsafe { EvtGetEventMetadataProperty(eventmetadata, propertyid, flags, eventmetadatapropertybuffersize, eventmetadatapropertybuffer.unwrap_or(core::mem::zeroed()) as _, eventmetadatapropertybufferused as _).ok() }
}
#[inline]
pub unsafe fn EvtGetExtendedStatus(buffer: Option<&mut [u16]>, bufferused: *mut u32) -> u32 {
    windows_targets::link!("wevtapi.dll" "system" fn EvtGetExtendedStatus(buffersize : u32, buffer : windows_core::PWSTR, bufferused : *mut u32) -> u32);
    unsafe { EvtGetExtendedStatus(buffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(buffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), bufferused as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn EvtGetLogInfo(log: EVT_HANDLE, propertyid: EVT_LOG_PROPERTY_ID, propertyvaluebuffersize: u32, propertyvaluebuffer: Option<*mut EVT_VARIANT>, propertyvaluebufferused: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("wevtapi.dll" "system" fn EvtGetLogInfo(log : EVT_HANDLE, propertyid : EVT_LOG_PROPERTY_ID, propertyvaluebuffersize : u32, propertyvaluebuffer : *mut EVT_VARIANT, propertyvaluebufferused : *mut u32) -> windows_core::BOOL);
    unsafe { EvtGetLogInfo(log, propertyid, propertyvaluebuffersize, propertyvaluebuffer.unwrap_or(core::mem::zeroed()) as _, propertyvaluebufferused as _).ok() }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn EvtGetObjectArrayProperty(objectarray: isize, propertyid: u32, arrayindex: u32, flags: u32, propertyvaluebuffersize: u32, propertyvaluebuffer: Option<*mut EVT_VARIANT>, propertyvaluebufferused: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("wevtapi.dll" "system" fn EvtGetObjectArrayProperty(objectarray : isize, propertyid : u32, arrayindex : u32, flags : u32, propertyvaluebuffersize : u32, propertyvaluebuffer : *mut EVT_VARIANT, propertyvaluebufferused : *mut u32) -> windows_core::BOOL);
    unsafe { EvtGetObjectArrayProperty(objectarray, propertyid, arrayindex, flags, propertyvaluebuffersize, propertyvaluebuffer.unwrap_or(core::mem::zeroed()) as _, propertyvaluebufferused as _).ok() }
}
#[inline]
pub unsafe fn EvtGetObjectArraySize(objectarray: isize, objectarraysize: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("wevtapi.dll" "system" fn EvtGetObjectArraySize(objectarray : isize, objectarraysize : *mut u32) -> windows_core::BOOL);
    unsafe { EvtGetObjectArraySize(objectarray, objectarraysize as _).ok() }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn EvtGetPublisherMetadataProperty(publishermetadata: EVT_HANDLE, propertyid: EVT_PUBLISHER_METADATA_PROPERTY_ID, flags: u32, publishermetadatapropertybuffersize: u32, publishermetadatapropertybuffer: Option<*mut EVT_VARIANT>, publishermetadatapropertybufferused: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("wevtapi.dll" "system" fn EvtGetPublisherMetadataProperty(publishermetadata : EVT_HANDLE, propertyid : EVT_PUBLISHER_METADATA_PROPERTY_ID, flags : u32, publishermetadatapropertybuffersize : u32, publishermetadatapropertybuffer : *mut EVT_VARIANT, publishermetadatapropertybufferused : *mut u32) -> windows_core::BOOL);
    unsafe { EvtGetPublisherMetadataProperty(publishermetadata, propertyid, flags, publishermetadatapropertybuffersize, publishermetadatapropertybuffer.unwrap_or(core::mem::zeroed()) as _, publishermetadatapropertybufferused as _).ok() }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn EvtGetQueryInfo(queryorsubscription: EVT_HANDLE, propertyid: EVT_QUERY_PROPERTY_ID, propertyvaluebuffersize: u32, propertyvaluebuffer: Option<*mut EVT_VARIANT>, propertyvaluebufferused: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("wevtapi.dll" "system" fn EvtGetQueryInfo(queryorsubscription : EVT_HANDLE, propertyid : EVT_QUERY_PROPERTY_ID, propertyvaluebuffersize : u32, propertyvaluebuffer : *mut EVT_VARIANT, propertyvaluebufferused : *mut u32) -> windows_core::BOOL);
    unsafe { EvtGetQueryInfo(queryorsubscription, propertyid, propertyvaluebuffersize, propertyvaluebuffer.unwrap_or(core::mem::zeroed()) as _, propertyvaluebufferused as _).ok() }
}
#[inline]
pub unsafe fn EvtNext(resultset: EVT_HANDLE, events: &mut [isize], timeout: u32, flags: u32, returned: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("wevtapi.dll" "system" fn EvtNext(resultset : EVT_HANDLE, eventssize : u32, events : *mut isize, timeout : u32, flags : u32, returned : *mut u32) -> windows_core::BOOL);
    unsafe { EvtNext(resultset, events.len().try_into().unwrap(), core::mem::transmute(events.as_ptr()), timeout, flags, returned as _).ok() }
}
#[inline]
pub unsafe fn EvtNextChannelPath(channelenum: EVT_HANDLE, channelpathbuffer: Option<&mut [u16]>, channelpathbufferused: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("wevtapi.dll" "system" fn EvtNextChannelPath(channelenum : EVT_HANDLE, channelpathbuffersize : u32, channelpathbuffer : windows_core::PWSTR, channelpathbufferused : *mut u32) -> windows_core::BOOL);
    unsafe { EvtNextChannelPath(channelenum, channelpathbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(channelpathbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), channelpathbufferused as _).ok() }
}
#[inline]
pub unsafe fn EvtNextEventMetadata(eventmetadataenum: EVT_HANDLE, flags: u32) -> windows_core::Result<EVT_HANDLE> {
    windows_targets::link!("wevtapi.dll" "system" fn EvtNextEventMetadata(eventmetadataenum : EVT_HANDLE, flags : u32) -> EVT_HANDLE);
    let result__ = unsafe { EvtNextEventMetadata(eventmetadataenum, flags) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn EvtNextPublisherId(publisherenum: EVT_HANDLE, publisheridbuffer: Option<&mut [u16]>, publisheridbufferused: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("wevtapi.dll" "system" fn EvtNextPublisherId(publisherenum : EVT_HANDLE, publisheridbuffersize : u32, publisheridbuffer : windows_core::PWSTR, publisheridbufferused : *mut u32) -> windows_core::BOOL);
    unsafe { EvtNextPublisherId(publisherenum, publisheridbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(publisheridbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), publisheridbufferused as _).ok() }
}
#[inline]
pub unsafe fn EvtOpenChannelConfig<P1>(session: Option<EVT_HANDLE>, channelpath: P1, flags: u32) -> windows_core::Result<EVT_HANDLE>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wevtapi.dll" "system" fn EvtOpenChannelConfig(session : EVT_HANDLE, channelpath : windows_core::PCWSTR, flags : u32) -> EVT_HANDLE);
    let result__ = unsafe { EvtOpenChannelConfig(session.unwrap_or(core::mem::zeroed()) as _, channelpath.param().abi(), flags) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn EvtOpenChannelEnum(session: Option<EVT_HANDLE>, flags: u32) -> windows_core::Result<EVT_HANDLE> {
    windows_targets::link!("wevtapi.dll" "system" fn EvtOpenChannelEnum(session : EVT_HANDLE, flags : u32) -> EVT_HANDLE);
    let result__ = unsafe { EvtOpenChannelEnum(session.unwrap_or(core::mem::zeroed()) as _, flags) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn EvtOpenEventMetadataEnum(publishermetadata: EVT_HANDLE, flags: u32) -> windows_core::Result<EVT_HANDLE> {
    windows_targets::link!("wevtapi.dll" "system" fn EvtOpenEventMetadataEnum(publishermetadata : EVT_HANDLE, flags : u32) -> EVT_HANDLE);
    let result__ = unsafe { EvtOpenEventMetadataEnum(publishermetadata, flags) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn EvtOpenLog<P1>(session: Option<EVT_HANDLE>, path: P1, flags: u32) -> windows_core::Result<EVT_HANDLE>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wevtapi.dll" "system" fn EvtOpenLog(session : EVT_HANDLE, path : windows_core::PCWSTR, flags : u32) -> EVT_HANDLE);
    let result__ = unsafe { EvtOpenLog(session.unwrap_or(core::mem::zeroed()) as _, path.param().abi(), flags) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn EvtOpenPublisherEnum(session: Option<EVT_HANDLE>, flags: u32) -> windows_core::Result<EVT_HANDLE> {
    windows_targets::link!("wevtapi.dll" "system" fn EvtOpenPublisherEnum(session : EVT_HANDLE, flags : u32) -> EVT_HANDLE);
    let result__ = unsafe { EvtOpenPublisherEnum(session.unwrap_or(core::mem::zeroed()) as _, flags) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn EvtOpenPublisherMetadata<P1, P2>(session: Option<EVT_HANDLE>, publisherid: P1, logfilepath: P2, locale: u32, flags: u32) -> windows_core::Result<EVT_HANDLE>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wevtapi.dll" "system" fn EvtOpenPublisherMetadata(session : EVT_HANDLE, publisherid : windows_core::PCWSTR, logfilepath : windows_core::PCWSTR, locale : u32, flags : u32) -> EVT_HANDLE);
    let result__ = unsafe { EvtOpenPublisherMetadata(session.unwrap_or(core::mem::zeroed()) as _, publisherid.param().abi(), logfilepath.param().abi(), locale, flags) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn EvtOpenSession(loginclass: EVT_LOGIN_CLASS, login: *const core::ffi::c_void, timeout: Option<u32>, flags: Option<u32>) -> windows_core::Result<EVT_HANDLE> {
    windows_targets::link!("wevtapi.dll" "system" fn EvtOpenSession(loginclass : EVT_LOGIN_CLASS, login : *const core::ffi::c_void, timeout : u32, flags : u32) -> EVT_HANDLE);
    let result__ = unsafe { EvtOpenSession(loginclass, login, timeout.unwrap_or(core::mem::zeroed()) as _, flags.unwrap_or(core::mem::zeroed()) as _) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn EvtQuery<P1, P2>(session: Option<EVT_HANDLE>, path: P1, query: P2, flags: u32) -> windows_core::Result<EVT_HANDLE>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wevtapi.dll" "system" fn EvtQuery(session : EVT_HANDLE, path : windows_core::PCWSTR, query : windows_core::PCWSTR, flags : u32) -> EVT_HANDLE);
    let result__ = unsafe { EvtQuery(session.unwrap_or(core::mem::zeroed()) as _, path.param().abi(), query.param().abi(), flags) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn EvtRender(context: Option<EVT_HANDLE>, fragment: EVT_HANDLE, flags: u32, buffersize: u32, buffer: Option<*mut core::ffi::c_void>, bufferused: *mut u32, propertycount: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("wevtapi.dll" "system" fn EvtRender(context : EVT_HANDLE, fragment : EVT_HANDLE, flags : u32, buffersize : u32, buffer : *mut core::ffi::c_void, bufferused : *mut u32, propertycount : *mut u32) -> windows_core::BOOL);
    unsafe { EvtRender(context.unwrap_or(core::mem::zeroed()) as _, fragment, flags, buffersize, buffer.unwrap_or(core::mem::zeroed()) as _, bufferused as _, propertycount as _).ok() }
}
#[inline]
pub unsafe fn EvtSaveChannelConfig(channelconfig: EVT_HANDLE, flags: u32) -> windows_core::Result<()> {
    windows_targets::link!("wevtapi.dll" "system" fn EvtSaveChannelConfig(channelconfig : EVT_HANDLE, flags : u32) -> windows_core::BOOL);
    unsafe { EvtSaveChannelConfig(channelconfig, flags).ok() }
}
#[inline]
pub unsafe fn EvtSeek(resultset: EVT_HANDLE, position: i64, bookmark: Option<EVT_HANDLE>, timeout: Option<u32>, flags: u32) -> windows_core::Result<()> {
    windows_targets::link!("wevtapi.dll" "system" fn EvtSeek(resultset : EVT_HANDLE, position : i64, bookmark : EVT_HANDLE, timeout : u32, flags : u32) -> windows_core::BOOL);
    unsafe { EvtSeek(resultset, position, bookmark.unwrap_or(core::mem::zeroed()) as _, timeout.unwrap_or(core::mem::zeroed()) as _, flags).ok() }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn EvtSetChannelConfigProperty(channelconfig: EVT_HANDLE, propertyid: EVT_CHANNEL_CONFIG_PROPERTY_ID, flags: u32, propertyvalue: *const EVT_VARIANT) -> windows_core::Result<()> {
    windows_targets::link!("wevtapi.dll" "system" fn EvtSetChannelConfigProperty(channelconfig : EVT_HANDLE, propertyid : EVT_CHANNEL_CONFIG_PROPERTY_ID, flags : u32, propertyvalue : *const EVT_VARIANT) -> windows_core::BOOL);
    unsafe { EvtSetChannelConfigProperty(channelconfig, propertyid, flags, propertyvalue).ok() }
}
#[inline]
pub unsafe fn EvtSubscribe<P2, P3>(session: Option<EVT_HANDLE>, signalevent: Option<super::super::Foundation::HANDLE>, channelpath: P2, query: P3, bookmark: Option<EVT_HANDLE>, context: Option<*const core::ffi::c_void>, callback: Option<EVT_SUBSCRIBE_CALLBACK>, flags: u32) -> windows_core::Result<EVT_HANDLE>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wevtapi.dll" "system" fn EvtSubscribe(session : EVT_HANDLE, signalevent : super::super::Foundation:: HANDLE, channelpath : windows_core::PCWSTR, query : windows_core::PCWSTR, bookmark : EVT_HANDLE, context : *const core::ffi::c_void, callback : EVT_SUBSCRIBE_CALLBACK, flags : u32) -> EVT_HANDLE);
    let result__ = unsafe { EvtSubscribe(session.unwrap_or(core::mem::zeroed()) as _, signalevent.unwrap_or(core::mem::zeroed()) as _, channelpath.param().abi(), query.param().abi(), bookmark.unwrap_or(core::mem::zeroed()) as _, context.unwrap_or(core::mem::zeroed()) as _, callback.unwrap_or(core::mem::zeroed()) as _, flags) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn EvtUpdateBookmark(bookmark: EVT_HANDLE, event: EVT_HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("wevtapi.dll" "system" fn EvtUpdateBookmark(bookmark : EVT_HANDLE, event : EVT_HANDLE) -> windows_core::BOOL);
    unsafe { EvtUpdateBookmark(bookmark, event).ok() }
}
#[inline]
pub unsafe fn GetEventLogInformation(heventlog: super::super::Foundation::HANDLE, dwinfolevel: u32, lpbuffer: *mut core::ffi::c_void, cbbufsize: u32, pcbbytesneeded: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn GetEventLogInformation(heventlog : super::super::Foundation:: HANDLE, dwinfolevel : u32, lpbuffer : *mut core::ffi::c_void, cbbufsize : u32, pcbbytesneeded : *mut u32) -> windows_core::BOOL);
    unsafe { GetEventLogInformation(heventlog, dwinfolevel, lpbuffer as _, cbbufsize, pcbbytesneeded as _).ok() }
}
#[inline]
pub unsafe fn GetNumberOfEventLogRecords(heventlog: super::super::Foundation::HANDLE, numberofrecords: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn GetNumberOfEventLogRecords(heventlog : super::super::Foundation:: HANDLE, numberofrecords : *mut u32) -> windows_core::BOOL);
    unsafe { GetNumberOfEventLogRecords(heventlog, numberofrecords as _).ok() }
}
#[inline]
pub unsafe fn GetOldestEventLogRecord(heventlog: super::super::Foundation::HANDLE, oldestrecord: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn GetOldestEventLogRecord(heventlog : super::super::Foundation:: HANDLE, oldestrecord : *mut u32) -> windows_core::BOOL);
    unsafe { GetOldestEventLogRecord(heventlog, oldestrecord as _).ok() }
}
#[inline]
pub unsafe fn NotifyChangeEventLog(heventlog: super::super::Foundation::HANDLE, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn NotifyChangeEventLog(heventlog : super::super::Foundation:: HANDLE, hevent : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { NotifyChangeEventLog(heventlog, hevent).ok() }
}
#[inline]
pub unsafe fn OpenBackupEventLogA<P0, P1>(lpuncservername: P0, lpfilename: P1) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn OpenBackupEventLogA(lpuncservername : windows_core::PCSTR, lpfilename : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { OpenBackupEventLogA(lpuncservername.param().abi(), lpfilename.param().abi()) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn OpenBackupEventLogW<P0, P1>(lpuncservername: P0, lpfilename: P1) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn OpenBackupEventLogW(lpuncservername : windows_core::PCWSTR, lpfilename : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { OpenBackupEventLogW(lpuncservername.param().abi(), lpfilename.param().abi()) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn OpenEventLogA<P0, P1>(lpuncservername: P0, lpsourcename: P1) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn OpenEventLogA(lpuncservername : windows_core::PCSTR, lpsourcename : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { OpenEventLogA(lpuncservername.param().abi(), lpsourcename.param().abi()) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn OpenEventLogW<P0, P1>(lpuncservername: P0, lpsourcename: P1) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn OpenEventLogW(lpuncservername : windows_core::PCWSTR, lpsourcename : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { OpenEventLogW(lpuncservername.param().abi(), lpsourcename.param().abi()) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn ReadEventLogA(heventlog: super::super::Foundation::HANDLE, dwreadflags: READ_EVENT_LOG_READ_FLAGS, dwrecordoffset: u32, lpbuffer: *mut core::ffi::c_void, nnumberofbytestoread: u32, pnbytesread: *mut u32, pnminnumberofbytesneeded: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn ReadEventLogA(heventlog : super::super::Foundation:: HANDLE, dwreadflags : READ_EVENT_LOG_READ_FLAGS, dwrecordoffset : u32, lpbuffer : *mut core::ffi::c_void, nnumberofbytestoread : u32, pnbytesread : *mut u32, pnminnumberofbytesneeded : *mut u32) -> windows_core::BOOL);
    unsafe { ReadEventLogA(heventlog, dwreadflags, dwrecordoffset, lpbuffer as _, nnumberofbytestoread, pnbytesread as _, pnminnumberofbytesneeded as _).ok() }
}
#[inline]
pub unsafe fn ReadEventLogW(heventlog: super::super::Foundation::HANDLE, dwreadflags: READ_EVENT_LOG_READ_FLAGS, dwrecordoffset: u32, lpbuffer: *mut core::ffi::c_void, nnumberofbytestoread: u32, pnbytesread: *mut u32, pnminnumberofbytesneeded: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn ReadEventLogW(heventlog : super::super::Foundation:: HANDLE, dwreadflags : READ_EVENT_LOG_READ_FLAGS, dwrecordoffset : u32, lpbuffer : *mut core::ffi::c_void, nnumberofbytestoread : u32, pnbytesread : *mut u32, pnminnumberofbytesneeded : *mut u32) -> windows_core::BOOL);
    unsafe { ReadEventLogW(heventlog, dwreadflags, dwrecordoffset, lpbuffer as _, nnumberofbytestoread, pnbytesread as _, pnminnumberofbytesneeded as _).ok() }
}
#[inline]
pub unsafe fn RegisterEventSourceA<P0, P1>(lpuncservername: P0, lpsourcename: P1) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegisterEventSourceA(lpuncservername : windows_core::PCSTR, lpsourcename : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { RegisterEventSourceA(lpuncservername.param().abi(), lpsourcename.param().abi()) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn RegisterEventSourceW<P0, P1>(lpuncservername: P0, lpsourcename: P1) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn RegisterEventSourceW(lpuncservername : windows_core::PCWSTR, lpsourcename : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { RegisterEventSourceW(lpuncservername.param().abi(), lpsourcename.param().abi()) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ReportEventA(heventlog: super::super::Foundation::HANDLE, wtype: REPORT_EVENT_TYPE, wcategory: u16, dweventid: u32, lpusersid: Option<super::super::Security::PSID>, dwdatasize: u32, lpstrings: Option<&[windows_core::PCSTR]>, lprawdata: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn ReportEventA(heventlog : super::super::Foundation:: HANDLE, wtype : REPORT_EVENT_TYPE, wcategory : u16, dweventid : u32, lpusersid : super::super::Security:: PSID, wnumstrings : u16, dwdatasize : u32, lpstrings : *const windows_core::PCSTR, lprawdata : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ReportEventA(heventlog, wtype, wcategory, dweventid, lpusersid.unwrap_or(core::mem::zeroed()) as _, lpstrings.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dwdatasize, core::mem::transmute(lpstrings.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lprawdata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ReportEventW(heventlog: super::super::Foundation::HANDLE, wtype: REPORT_EVENT_TYPE, wcategory: u16, dweventid: u32, lpusersid: Option<super::super::Security::PSID>, dwdatasize: u32, lpstrings: Option<&[windows_core::PCWSTR]>, lprawdata: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn ReportEventW(heventlog : super::super::Foundation:: HANDLE, wtype : REPORT_EVENT_TYPE, wcategory : u16, dweventid : u32, lpusersid : super::super::Security:: PSID, wnumstrings : u16, dwdatasize : u32, lpstrings : *const windows_core::PCWSTR, lprawdata : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ReportEventW(heventlog, wtype, wcategory, dweventid, lpusersid.unwrap_or(core::mem::zeroed()) as _, lpstrings.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dwdatasize, core::mem::transmute(lpstrings.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lprawdata.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EVENTLOGRECORD {
    pub Length: u32,
    pub Reserved: u32,
    pub RecordNumber: u32,
    pub TimeGenerated: u32,
    pub TimeWritten: u32,
    pub EventID: u32,
    pub EventType: REPORT_EVENT_TYPE,
    pub NumStrings: u16,
    pub EventCategory: u16,
    pub ReservedFlags: u16,
    pub ClosingRecordNumber: u32,
    pub StringOffset: u32,
    pub UserSidLength: u32,
    pub UserSidOffset: u32,
    pub DataLength: u32,
    pub DataOffset: u32,
}
impl Default for EVENTLOGRECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EVENTLOG_AUDIT_FAILURE: REPORT_EVENT_TYPE = REPORT_EVENT_TYPE(16u16);
pub const EVENTLOG_AUDIT_SUCCESS: REPORT_EVENT_TYPE = REPORT_EVENT_TYPE(8u16);
pub const EVENTLOG_ERROR_TYPE: REPORT_EVENT_TYPE = REPORT_EVENT_TYPE(1u16);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EVENTLOG_FULL_INFORMATION {
    pub dwFull: u32,
}
impl Default for EVENTLOG_FULL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EVENTLOG_INFORMATION_TYPE: REPORT_EVENT_TYPE = REPORT_EVENT_TYPE(4u16);
pub const EVENTLOG_SEEK_READ: READ_EVENT_LOG_READ_FLAGS = READ_EVENT_LOG_READ_FLAGS(2u32);
pub const EVENTLOG_SEQUENTIAL_READ: READ_EVENT_LOG_READ_FLAGS = READ_EVENT_LOG_READ_FLAGS(1u32);
pub const EVENTLOG_SUCCESS: REPORT_EVENT_TYPE = REPORT_EVENT_TYPE(0u16);
pub const EVENTLOG_WARNING_TYPE: REPORT_EVENT_TYPE = REPORT_EVENT_TYPE(2u16);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EVENTSFORLOGFILE {
    pub ulSize: u32,
    pub szLogicalLogFile: [u16; 256],
    pub ulNumRecords: u32,
    pub pEventLogRecords: [EVENTLOGRECORD; 1],
}
impl Default for EVENTSFORLOGFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EVT_ALL_ACCESS: u32 = 7u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVT_CHANNEL_CLOCK_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVT_CHANNEL_CONFIG_PROPERTY_ID(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVT_CHANNEL_ISOLATION_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVT_CHANNEL_REFERENCE_FLAGS(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVT_CHANNEL_SID_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVT_CHANNEL_TYPE(pub i32);
pub const EVT_CLEAR_ACCESS: u32 = 4u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVT_EVENT_METADATA_PROPERTY_ID(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVT_EVENT_PROPERTY_ID(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVT_EXPORTLOG_FLAGS(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVT_FORMAT_MESSAGE_FLAGS(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct EVT_HANDLE(pub isize);
impl EVT_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl windows_core::Free for EVT_HANDLE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_targets::link!("wevtapi.dll" "system" fn EvtClose(object : isize) -> i32);
            unsafe {
                EvtClose(self.0);
            }
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVT_LOGIN_CLASS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVT_LOG_PROPERTY_ID(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVT_OPEN_LOG_FLAGS(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVT_PUBLISHER_METADATA_PROPERTY_ID(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVT_QUERY_FLAGS(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVT_QUERY_PROPERTY_ID(pub i32);
pub const EVT_READ_ACCESS: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVT_RENDER_CONTEXT_FLAGS(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVT_RENDER_FLAGS(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EVT_RPC_LOGIN {
    pub Server: windows_core::PWSTR,
    pub User: windows_core::PWSTR,
    pub Domain: windows_core::PWSTR,
    pub Password: windows_core::PWSTR,
    pub Flags: u32,
}
impl Default for EVT_RPC_LOGIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVT_RPC_LOGIN_FLAGS(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVT_SEEK_FLAGS(pub u32);
pub type EVT_SUBSCRIBE_CALLBACK = Option<unsafe extern "system" fn(action: EVT_SUBSCRIBE_NOTIFY_ACTION, usercontext: *const core::ffi::c_void, event: EVT_HANDLE) -> u32>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVT_SUBSCRIBE_FLAGS(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVT_SUBSCRIBE_NOTIFY_ACTION(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVT_SYSTEM_PROPERTY_ID(pub i32);
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub struct EVT_VARIANT {
    pub Anonymous: EVT_VARIANT_0,
    pub Count: u32,
    pub Type: u32,
}
#[cfg(feature = "Win32_Security")]
impl Default for EVT_VARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub union EVT_VARIANT_0 {
    pub BooleanVal: windows_core::BOOL,
    pub SByteVal: i8,
    pub Int16Val: i16,
    pub Int32Val: i32,
    pub Int64Val: i64,
    pub ByteVal: u8,
    pub UInt16Val: u16,
    pub UInt32Val: u32,
    pub UInt64Val: u64,
    pub SingleVal: f32,
    pub DoubleVal: f64,
    pub FileTimeVal: u64,
    pub SysTimeVal: *mut super::super::Foundation::SYSTEMTIME,
    pub GuidVal: *mut windows_core::GUID,
    pub StringVal: windows_core::PCWSTR,
    pub AnsiStringVal: windows_core::PCSTR,
    pub BinaryVal: *mut u8,
    pub SidVal: super::super::Security::PSID,
    pub SizeTVal: usize,
    pub BooleanArr: *mut windows_core::BOOL,
    pub SByteArr: *mut i8,
    pub Int16Arr: *mut i16,
    pub Int32Arr: *mut i32,
    pub Int64Arr: *mut i64,
    pub ByteArr: *mut u8,
    pub UInt16Arr: *mut u16,
    pub UInt32Arr: *mut u32,
    pub UInt64Arr: *mut u64,
    pub SingleArr: *mut f32,
    pub DoubleArr: *mut f64,
    pub FileTimeArr: *mut super::super::Foundation::FILETIME,
    pub SysTimeArr: *mut super::super::Foundation::SYSTEMTIME,
    pub GuidArr: *mut windows_core::GUID,
    pub StringArr: *mut windows_core::PWSTR,
    pub AnsiStringArr: *mut windows_core::PSTR,
    pub SidArr: *mut super::super::Security::PSID,
    pub SizeTArr: *mut usize,
    pub EvtHandleVal: EVT_HANDLE,
    pub XmlVal: windows_core::PCWSTR,
    pub XmlValArr: *const windows_core::PCWSTR,
}
#[cfg(feature = "Win32_Security")]
impl Default for EVT_VARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVT_VARIANT_TYPE(pub i32);
pub const EVT_VARIANT_TYPE_ARRAY: u32 = 128u32;
pub const EVT_VARIANT_TYPE_MASK: u32 = 127u32;
pub const EVT_WRITE_ACCESS: u32 = 2u32;
pub const EventMetadataEventChannel: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(2i32);
pub const EventMetadataEventID: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(0i32);
pub const EventMetadataEventKeyword: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(6i32);
pub const EventMetadataEventLevel: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(3i32);
pub const EventMetadataEventMessageID: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(7i32);
pub const EventMetadataEventOpcode: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(4i32);
pub const EventMetadataEventTask: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(5i32);
pub const EventMetadataEventTemplate: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(8i32);
pub const EventMetadataEventVersion: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(1i32);
pub const EvtChannelClockTypeQPC: EVT_CHANNEL_CLOCK_TYPE = EVT_CHANNEL_CLOCK_TYPE(1i32);
pub const EvtChannelClockTypeSystemTime: EVT_CHANNEL_CLOCK_TYPE = EVT_CHANNEL_CLOCK_TYPE(0i32);
pub const EvtChannelConfigAccess: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(5i32);
pub const EvtChannelConfigClassicEventlog: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(4i32);
pub const EvtChannelConfigEnabled: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(0i32);
pub const EvtChannelConfigIsolation: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(1i32);
pub const EvtChannelConfigOwningPublisher: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(3i32);
pub const EvtChannelConfigPropertyIdEND: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(21i32);
pub const EvtChannelConfigType: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(2i32);
pub const EvtChannelIsolationTypeApplication: EVT_CHANNEL_ISOLATION_TYPE = EVT_CHANNEL_ISOLATION_TYPE(0i32);
pub const EvtChannelIsolationTypeCustom: EVT_CHANNEL_ISOLATION_TYPE = EVT_CHANNEL_ISOLATION_TYPE(2i32);
pub const EvtChannelIsolationTypeSystem: EVT_CHANNEL_ISOLATION_TYPE = EVT_CHANNEL_ISOLATION_TYPE(1i32);
pub const EvtChannelLoggingConfigAutoBackup: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(7i32);
pub const EvtChannelLoggingConfigLogFilePath: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(9i32);
pub const EvtChannelLoggingConfigMaxSize: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(8i32);
pub const EvtChannelLoggingConfigRetention: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(6i32);
pub const EvtChannelPublisherList: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(19i32);
pub const EvtChannelPublishingConfigBufferSize: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(13i32);
pub const EvtChannelPublishingConfigClockType: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(17i32);
pub const EvtChannelPublishingConfigControlGuid: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(12i32);
pub const EvtChannelPublishingConfigFileMax: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(20i32);
pub const EvtChannelPublishingConfigKeywords: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(11i32);
pub const EvtChannelPublishingConfigLatency: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(16i32);
pub const EvtChannelPublishingConfigLevel: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(10i32);
pub const EvtChannelPublishingConfigMaxBuffers: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(15i32);
pub const EvtChannelPublishingConfigMinBuffers: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(14i32);
pub const EvtChannelPublishingConfigSidType: EVT_CHANNEL_CONFIG_PROPERTY_ID = EVT_CHANNEL_CONFIG_PROPERTY_ID(18i32);
pub const EvtChannelReferenceImported: EVT_CHANNEL_REFERENCE_FLAGS = EVT_CHANNEL_REFERENCE_FLAGS(1u32);
pub const EvtChannelSidTypeNone: EVT_CHANNEL_SID_TYPE = EVT_CHANNEL_SID_TYPE(0i32);
pub const EvtChannelSidTypePublishing: EVT_CHANNEL_SID_TYPE = EVT_CHANNEL_SID_TYPE(1i32);
pub const EvtChannelTypeAdmin: EVT_CHANNEL_TYPE = EVT_CHANNEL_TYPE(0i32);
pub const EvtChannelTypeAnalytic: EVT_CHANNEL_TYPE = EVT_CHANNEL_TYPE(2i32);
pub const EvtChannelTypeDebug: EVT_CHANNEL_TYPE = EVT_CHANNEL_TYPE(3i32);
pub const EvtChannelTypeOperational: EVT_CHANNEL_TYPE = EVT_CHANNEL_TYPE(1i32);
pub const EvtEventMetadataPropertyIdEND: EVT_EVENT_METADATA_PROPERTY_ID = EVT_EVENT_METADATA_PROPERTY_ID(9i32);
pub const EvtEventPath: EVT_EVENT_PROPERTY_ID = EVT_EVENT_PROPERTY_ID(1i32);
pub const EvtEventPropertyIdEND: EVT_EVENT_PROPERTY_ID = EVT_EVENT_PROPERTY_ID(2i32);
pub const EvtEventQueryIDs: EVT_EVENT_PROPERTY_ID = EVT_EVENT_PROPERTY_ID(0i32);
pub const EvtExportLogChannelPath: EVT_EXPORTLOG_FLAGS = EVT_EXPORTLOG_FLAGS(1u32);
pub const EvtExportLogFilePath: EVT_EXPORTLOG_FLAGS = EVT_EXPORTLOG_FLAGS(2u32);
pub const EvtExportLogOverwrite: EVT_EXPORTLOG_FLAGS = EVT_EXPORTLOG_FLAGS(8192u32);
pub const EvtExportLogTolerateQueryErrors: EVT_EXPORTLOG_FLAGS = EVT_EXPORTLOG_FLAGS(4096u32);
pub const EvtFormatMessageChannel: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(6u32);
pub const EvtFormatMessageEvent: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(1u32);
pub const EvtFormatMessageId: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(8u32);
pub const EvtFormatMessageKeyword: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(5u32);
pub const EvtFormatMessageLevel: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(2u32);
pub const EvtFormatMessageOpcode: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(4u32);
pub const EvtFormatMessageProvider: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(7u32);
pub const EvtFormatMessageTask: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(3u32);
pub const EvtFormatMessageXml: EVT_FORMAT_MESSAGE_FLAGS = EVT_FORMAT_MESSAGE_FLAGS(9u32);
pub const EvtLogAttributes: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(4i32);
pub const EvtLogCreationTime: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(0i32);
pub const EvtLogFileSize: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(3i32);
pub const EvtLogFull: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(7i32);
pub const EvtLogLastAccessTime: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(1i32);
pub const EvtLogLastWriteTime: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(2i32);
pub const EvtLogNumberOfLogRecords: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(5i32);
pub const EvtLogOldestRecordNumber: EVT_LOG_PROPERTY_ID = EVT_LOG_PROPERTY_ID(6i32);
pub const EvtOpenChannelPath: EVT_OPEN_LOG_FLAGS = EVT_OPEN_LOG_FLAGS(1u32);
pub const EvtOpenFilePath: EVT_OPEN_LOG_FLAGS = EVT_OPEN_LOG_FLAGS(2u32);
pub const EvtPublisherMetadataChannelReferenceFlags: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(10i32);
pub const EvtPublisherMetadataChannelReferenceID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(9i32);
pub const EvtPublisherMetadataChannelReferenceIndex: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(8i32);
pub const EvtPublisherMetadataChannelReferenceMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(11i32);
pub const EvtPublisherMetadataChannelReferencePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(7i32);
pub const EvtPublisherMetadataChannelReferences: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(6i32);
pub const EvtPublisherMetadataHelpLink: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(4i32);
pub const EvtPublisherMetadataKeywordMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(28i32);
pub const EvtPublisherMetadataKeywordName: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(26i32);
pub const EvtPublisherMetadataKeywordValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(27i32);
pub const EvtPublisherMetadataKeywords: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(25i32);
pub const EvtPublisherMetadataLevelMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(15i32);
pub const EvtPublisherMetadataLevelName: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(13i32);
pub const EvtPublisherMetadataLevelValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(14i32);
pub const EvtPublisherMetadataLevels: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(12i32);
pub const EvtPublisherMetadataMessageFilePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(3i32);
pub const EvtPublisherMetadataOpcodeMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(24i32);
pub const EvtPublisherMetadataOpcodeName: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(22i32);
pub const EvtPublisherMetadataOpcodeValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(23i32);
pub const EvtPublisherMetadataOpcodes: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(21i32);
pub const EvtPublisherMetadataParameterFilePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(2i32);
pub const EvtPublisherMetadataPropertyIdEND: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(29i32);
pub const EvtPublisherMetadataPublisherGuid: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(0i32);
pub const EvtPublisherMetadataPublisherMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(5i32);
pub const EvtPublisherMetadataResourceFilePath: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(1i32);
pub const EvtPublisherMetadataTaskEventGuid: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(18i32);
pub const EvtPublisherMetadataTaskMessageID: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(20i32);
pub const EvtPublisherMetadataTaskName: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(17i32);
pub const EvtPublisherMetadataTaskValue: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(19i32);
pub const EvtPublisherMetadataTasks: EVT_PUBLISHER_METADATA_PROPERTY_ID = EVT_PUBLISHER_METADATA_PROPERTY_ID(16i32);
pub const EvtQueryChannelPath: EVT_QUERY_FLAGS = EVT_QUERY_FLAGS(1u32);
pub const EvtQueryFilePath: EVT_QUERY_FLAGS = EVT_QUERY_FLAGS(2u32);
pub const EvtQueryForwardDirection: EVT_QUERY_FLAGS = EVT_QUERY_FLAGS(256u32);
pub const EvtQueryNames: EVT_QUERY_PROPERTY_ID = EVT_QUERY_PROPERTY_ID(0i32);
pub const EvtQueryPropertyIdEND: EVT_QUERY_PROPERTY_ID = EVT_QUERY_PROPERTY_ID(2i32);
pub const EvtQueryReverseDirection: EVT_QUERY_FLAGS = EVT_QUERY_FLAGS(512u32);
pub const EvtQueryStatuses: EVT_QUERY_PROPERTY_ID = EVT_QUERY_PROPERTY_ID(1i32);
pub const EvtQueryTolerateQueryErrors: EVT_QUERY_FLAGS = EVT_QUERY_FLAGS(4096u32);
pub const EvtRenderBookmark: EVT_RENDER_FLAGS = EVT_RENDER_FLAGS(2u32);
pub const EvtRenderContextSystem: EVT_RENDER_CONTEXT_FLAGS = EVT_RENDER_CONTEXT_FLAGS(1u32);
pub const EvtRenderContextUser: EVT_RENDER_CONTEXT_FLAGS = EVT_RENDER_CONTEXT_FLAGS(2u32);
pub const EvtRenderContextValues: EVT_RENDER_CONTEXT_FLAGS = EVT_RENDER_CONTEXT_FLAGS(0u32);
pub const EvtRenderEventValues: EVT_RENDER_FLAGS = EVT_RENDER_FLAGS(0u32);
pub const EvtRenderEventXml: EVT_RENDER_FLAGS = EVT_RENDER_FLAGS(1u32);
pub const EvtRpcLogin: EVT_LOGIN_CLASS = EVT_LOGIN_CLASS(1i32);
pub const EvtRpcLoginAuthDefault: EVT_RPC_LOGIN_FLAGS = EVT_RPC_LOGIN_FLAGS(0u32);
pub const EvtRpcLoginAuthKerberos: EVT_RPC_LOGIN_FLAGS = EVT_RPC_LOGIN_FLAGS(2u32);
pub const EvtRpcLoginAuthNTLM: EVT_RPC_LOGIN_FLAGS = EVT_RPC_LOGIN_FLAGS(3u32);
pub const EvtRpcLoginAuthNegotiate: EVT_RPC_LOGIN_FLAGS = EVT_RPC_LOGIN_FLAGS(1u32);
pub const EvtSeekOriginMask: EVT_SEEK_FLAGS = EVT_SEEK_FLAGS(7u32);
pub const EvtSeekRelativeToBookmark: EVT_SEEK_FLAGS = EVT_SEEK_FLAGS(4u32);
pub const EvtSeekRelativeToCurrent: EVT_SEEK_FLAGS = EVT_SEEK_FLAGS(3u32);
pub const EvtSeekRelativeToFirst: EVT_SEEK_FLAGS = EVT_SEEK_FLAGS(1u32);
pub const EvtSeekRelativeToLast: EVT_SEEK_FLAGS = EVT_SEEK_FLAGS(2u32);
pub const EvtSeekStrict: EVT_SEEK_FLAGS = EVT_SEEK_FLAGS(65536u32);
pub const EvtSubscribeActionDeliver: EVT_SUBSCRIBE_NOTIFY_ACTION = EVT_SUBSCRIBE_NOTIFY_ACTION(1i32);
pub const EvtSubscribeActionError: EVT_SUBSCRIBE_NOTIFY_ACTION = EVT_SUBSCRIBE_NOTIFY_ACTION(0i32);
pub const EvtSubscribeOriginMask: EVT_SUBSCRIBE_FLAGS = EVT_SUBSCRIBE_FLAGS(3u32);
pub const EvtSubscribeStartAfterBookmark: EVT_SUBSCRIBE_FLAGS = EVT_SUBSCRIBE_FLAGS(3u32);
pub const EvtSubscribeStartAtOldestRecord: EVT_SUBSCRIBE_FLAGS = EVT_SUBSCRIBE_FLAGS(2u32);
pub const EvtSubscribeStrict: EVT_SUBSCRIBE_FLAGS = EVT_SUBSCRIBE_FLAGS(65536u32);
pub const EvtSubscribeToFutureEvents: EVT_SUBSCRIBE_FLAGS = EVT_SUBSCRIBE_FLAGS(1u32);
pub const EvtSubscribeTolerateQueryErrors: EVT_SUBSCRIBE_FLAGS = EVT_SUBSCRIBE_FLAGS(4096u32);
pub const EvtSystemActivityID: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(10i32);
pub const EvtSystemChannel: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(14i32);
pub const EvtSystemComputer: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(15i32);
pub const EvtSystemEventID: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(2i32);
pub const EvtSystemEventRecordId: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(9i32);
pub const EvtSystemKeywords: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(7i32);
pub const EvtSystemLevel: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(4i32);
pub const EvtSystemOpcode: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(6i32);
pub const EvtSystemProcessID: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(12i32);
pub const EvtSystemPropertyIdEND: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(18i32);
pub const EvtSystemProviderGuid: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(1i32);
pub const EvtSystemProviderName: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(0i32);
pub const EvtSystemQualifiers: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(3i32);
pub const EvtSystemRelatedActivityID: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(11i32);
pub const EvtSystemTask: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(5i32);
pub const EvtSystemThreadID: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(13i32);
pub const EvtSystemTimeCreated: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(8i32);
pub const EvtSystemUserID: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(16i32);
pub const EvtSystemVersion: EVT_SYSTEM_PROPERTY_ID = EVT_SYSTEM_PROPERTY_ID(17i32);
pub const EvtVarTypeAnsiString: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(2i32);
pub const EvtVarTypeBinary: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(14i32);
pub const EvtVarTypeBoolean: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(13i32);
pub const EvtVarTypeByte: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(4i32);
pub const EvtVarTypeDouble: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(12i32);
pub const EvtVarTypeEvtHandle: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(32i32);
pub const EvtVarTypeEvtXml: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(35i32);
pub const EvtVarTypeFileTime: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(17i32);
pub const EvtVarTypeGuid: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(15i32);
pub const EvtVarTypeHexInt32: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(20i32);
pub const EvtVarTypeHexInt64: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(21i32);
pub const EvtVarTypeInt16: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(5i32);
pub const EvtVarTypeInt32: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(7i32);
pub const EvtVarTypeInt64: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(9i32);
pub const EvtVarTypeNull: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(0i32);
pub const EvtVarTypeSByte: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(3i32);
pub const EvtVarTypeSid: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(19i32);
pub const EvtVarTypeSingle: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(11i32);
pub const EvtVarTypeSizeT: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(16i32);
pub const EvtVarTypeString: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(1i32);
pub const EvtVarTypeSysTime: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(18i32);
pub const EvtVarTypeUInt16: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(6i32);
pub const EvtVarTypeUInt32: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(8i32);
pub const EvtVarTypeUInt64: EVT_VARIANT_TYPE = EVT_VARIANT_TYPE(10i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct READ_EVENT_LOG_READ_FLAGS(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REPORT_EVENT_TYPE(pub u16);
