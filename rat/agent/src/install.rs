use crate::config;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

pub fn install() -> Result<PathBuf, crate::Error> {
    let install_dir = config::get_agent_directory()?;
    let install_target = config::get_agent_install_target()?;

    if !install_target.exists() {
        println!("Installing into {}", install_dir.display());
        let current_exe = env::current_exe()?;

        fs::create_dir_all(&install_dir)?;

        fs::copy(current_exe, &install_target)?;

        // here, we could have fetched the bundle from a central server
        let bundle = PathBuf::from("bundle.zip");
        if bundle.exists() {
            println!(
                "bundle.zip found, extracting it to {}",
                install_dir.display()
            );

            extract_bundle(&install_dir, &bundle)?;
        } else {
            println!("bundle.zip NOT found");
        }
    }

    Ok(install_dir)
}

fn extract_bundle(install_dir: &Path, bundle: &Path) -> Result<(), crate::Error> {
    let zip_file = fs::File::open(bundle)?;
    let mut zip_archive = zip::ZipArchive::new(zip_file)?;

    for i in 0..zip_archive.len() {
        let mut archive_file = zip_archive.by_index(i)?;
        let dist_filename = match archive_file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        let dist_path = install_dir.join(dist_filename);

        if archive_file.is_dir() {
            fs::create_dir_all(&dist_path)?;
            continue;
        }

        if let Some(parent) = dist_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut dist_file = fs::File::create(&dist_path)?;
        io::copy(&mut archive_file, &mut dist_file)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = archive_file.unix_mode() {
                fs::set_permissions(&dist_path, fs::Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(())
}
