use std::io::Write;

use eyre::Context;

use crate::folders::Folder;
use crate::prelude::*;
//
pub use crate::RUN_OPERATION; // Leave this as false to not accidentially rename a bunch of files lol
//
use crate::{stamp_converter, Seperators, AM_PM_SEP, DATE_SEP, DATE_TIME_SEP, TIME_SEP};

pub fn process_folder(folder: &Folder) {
    let std_out = std::io::stdout();
    let mut handle = std_out.lock();

    let seps = Seperators::new(DATE_SEP, DATE_TIME_SEP, TIME_SEP, AM_PM_SEP);

    for file in &folder.children_files {
        let old_path = file.path.clone();
        let old_name = old_path.file_name().unwrap().to_str().unwrap();

        let new_name = format!(
            "{}, {old_name}",
            convert_name(&old_path, old_name, seps.as_ref()).unwrap() // new_date_time
        );

        let new_path = old_path.with_file_name(new_name);

        let o_path = PathBuf::from(old_path);
        let n_path = PathBuf::from(new_path);

        writeln!(
            handle,
            "{:<20} -> {:<20}",
            o_path.to_str().unwrap(),
            n_path.to_str().unwrap()
        )
        .unwrap();

        if RUN_OPERATION {
            // if program stops/crashes here, this ensures we don't run the same files a second time and append the date again
            if n_path == o_path {
                continue;
            }
            // Rename the file
            std::fs::rename(&o_path, &n_path).unwrap();
        }
    }

    for child_folder in &folder.children {
        // Use recursion to process all subfolders
        process_folder(child_folder);
    }

    // These may actually be out of order due to the way stdout is buffered and the recursive nature of the function
    handle.flush().unwrap();
}

pub fn convert_name(
    old_path: &PathBuf,
    old_name: &str,
    // creation_time: &str,
    seps: &Seperators,
) -> Result<String> {
    let creation_time = get_creation_time(&old_path).unwrap();
    let new_date = stamp_converter::flip_date_format(&creation_time, seps)?;
    let new_name = format!("{new_date} {old_name}");
    Ok(new_name)
}

// Is it pretty? No. Does it work? Yes.
// Should I be using c/c++/c# to do this? Probably.
// Will I? Maybe.
pub fn get_creation_time(path: &PathBuf) -> Result<String> {
    let cmd = std::process::Command::new("powershell")
        .arg("-Command")
        .arg(format!(
            "$File = Get-Item \"{}\"; 
            $ShellApplication = New-Object -ComObject Shell.Application;
            $ShellFolder = $ShellApplication.Namespace($File.Directory.FullName);
            $ShellFile = $ShellFolder.ParseName($File.Name);
            $v = $ShellFolder.GetDetailsOf($ShellFile, 208);
            Write-Host $v",
            path.to_str().unwrap()
        ))
        .output()
        .wrap_err("Failed to execute powershell command")?;

    let output = String::from_utf8(cmd.stdout).wrap_err("Failed to convert output to string")?;
    let output = output.trim().to_string().replace("?", "");
    // Produces ----- `creation_time: "26/05/2022 12:40 AM"`

    // let as_systime = chrono::NaiveDateTime::parse_from_str(&output, "%d/%m/%Y %I:%M %p")
    //     .wrap_err("Failed to parse creation time")?;
    // println!("as_systime: {:?}", as_systime);

    Ok(output)
}
