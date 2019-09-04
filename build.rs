use std::{
    env,
    fs::{self, File},
    io::{prelude::*, Seek, Write},
    iter::Iterator,
    path::Path,
};

use walkdir::{DirEntry, WalkDir};
use zip::{result::ZipError, write::FileOptions};

fn main() {
    let profile = env::var("PROFILE").unwrap();
    println!("cargo:rustc-cfg=build={:?}", &profile);

    if profile == "debug" {
        return;
    }

    let src_dir = "resources";
    let dst_file = "resources.zip";

    let _ = fs::remove_file(Path::new(dst_file));

    let method = zip::CompressionMethod::Deflated;

    match compress(src_dir, dst_file, method) {
        Ok(_) => println!("done: {} written to {}", src_dir, dst_file),
        Err(e) => {
            println!("error: {:?}", e);
            std::process::exit(1);
        }
    }
}

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        if path.is_file() {
            println!("adding file {:?} as {:?} ...", path, name);
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if name.as_os_str().len() != 0 {
            println!("adding dir {:?} as {:?} ...", path, name);
            zip.add_directory_from_path(name, options)?;
        }
    }

    zip.finish()?;
    Ok(())
}

fn compress(
    src_dir: &str,
    dst_file: &str,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let path = Path::new(dst_file);
    let file = File::create(&path).unwrap();

    let walkdir = WalkDir::new(src_dir.to_string());
    let it = walkdir.into_iter();

    zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, file, method)?;

    Ok(())
}
