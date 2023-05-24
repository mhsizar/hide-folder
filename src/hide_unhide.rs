use std::{path::Path, io::Error};
use std::os::windows::{fs::MetadataExt, ffi::OsStrExt};
use winapi::um::winnt::FILE_ATTRIBUTE_HIDDEN;

pub fn hide_directory(directory: &Path) -> Result<(), Error> {
    let metadata = directory.metadata()?; // Get the the metadata of the directory provided
    
    // Set the permission of the directory, returns error if directory doesn't exist——useful to evaluate the user input
    let mut permissions = metadata.permissions();
    permissions.set_readonly(true);
    std::fs::set_permissions(directory, permissions)?;
    
    // Check the file attributes from the metadata of the directory provided and see if it's already hidden
    let is_hidden = metadata.file_attributes() & FILE_ATTRIBUTE_HIDDEN != 0;

    if is_hidden {
        println!("Directory is already hidden!");
    } else {
        let file_attributes = metadata.file_attributes() | FILE_ATTRIBUTE_HIDDEN; // set attribute to FILE_ATTRIBUTE_HIDDEN if it's not already
    
        // Update the directory attribute to hide it (unsafe behaviour)
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
    let metadata = directory.metadata()?; // Get the the metadata of the directory provided
    
    // Set the permission of the directory, returns error if directory doesn't exist——useful to evaluate the user input
    let mut permissions = metadata.permissions();
    permissions.set_readonly(false);
    std::fs::set_permissions(directory, permissions)?;

    // Get the file attributes from the metadata of the directory provided and see if it's already hidden
    let is_hidden = metadata.file_attributes() & FILE_ATTRIBUTE_HIDDEN != 0;

    if !is_hidden {
        println!("Directory is already unhidden!");
    } else {
        let file_attributes = metadata.file_attributes() & !FILE_ATTRIBUTE_HIDDEN; // set attribute to not FILE_ATTRIBUTE_HIDDEN to unhide it
    
        // Update the directory attribute to unhide it (unsafe behaviour)
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
