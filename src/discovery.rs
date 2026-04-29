use crate::error::check;
use crate::ffi;
use crate::types::SenderInfo;

pub fn enumerate_senders() -> crate::error::Result<Vec<SenderInfo>> {
    let mut array = ffi::NozzleSenderInfoArray {
        items: std::ptr::null_mut(),
        count: 0,
    };

    let rc = unsafe { ffi::nozzle_enumerate_senders(&mut array) };
    if rc != 0 {
        let _ = check(rc as _);
        return Ok(Vec::new());
    }

    let items = if array.items.is_null() || array.count == 0 {
        Vec::new()
    } else {
        unsafe {
            std::slice::from_raw_parts(array.items, array.count as usize)
        }
        .iter()
        .map(|item| SenderInfo {
            name: crate::cstr_to_string(item.name),
            application_name: crate::cstr_to_string(item.application_name),
            id: crate::cstr_to_string(item.id),
            backend: crate::types::BackendType::from_raw(item.backend as _),
        })
        .collect()
    };

    unsafe { ffi::nozzle_free_sender_info_array(&mut array) };

    Ok(items)
}
