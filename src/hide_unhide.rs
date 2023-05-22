use std::{path::Path, io::Error};
use std::os::windows::{fs::MetadataExt, ffi::OsStrExt};
use winapi::um::winnt::FILE_ATTRIBUTE_HIDDEN;

pub fn hide_directory(directory: &Path) -> Result<(), Error> {
    let metadata = directory.metadata()?;
    let mut permissions = metadata.permissions();
    permissions.set_readonly(true);
    std::fs::set_permissions(directory, permissions)?;
    
    let is_hidden = metadata.file_attributes() & FILE_ATTRIBUTE_HIDDEN != 0;

    if is_hidden {
        println!("Directory is already hidden!");
    } else {
        let file_attributes = metadata.file_attributes() | FILE_ATTRIBUTE_HIDDEN;
        unsafe {
        winapi::um::fileapi::SetFileAttributesW(directory
            .as_os_str()
            .encode_wide()
            .chain(Some(0))
            .collect::<Vec<_>>()
            .as_ptr(), file_attributes);
    }
    println!("Directory hidden successfully!");
    }

    Ok(())
}

pub fn unhide_directory(directory: &Path) -> Result<(), Error> {
    let metadata = directory.metadata()?;
    let mut permissions = metadata.permissions();
    permissions.set_readonly(false);
    std::fs::set_permissions(directory, permissions)?;

    let is_hidden = metadata.file_attributes() & FILE_ATTRIBUTE_HIDDEN != 0;

    if !is_hidden {
        println!("Directory is already unhidden!");
    } else {
        let file_attributes = metadata.file_attributes() & !FILE_ATTRIBUTE_HIDDEN;
        unsafe {
            winapi::um::fileapi::SetFileAttributesW(directory
                .as_os_str()
                .encode_wide()
                .chain(Some(0))
                .collect::<Vec<_>>()
                .as_ptr(), file_attributes);
        }
        println!("Directory unhidden successfully!");
    }

    Ok(())
}