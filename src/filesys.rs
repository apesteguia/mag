use std::{
    fs::{self, File, Metadata},
    os::unix::fs::FileExt,
    path::{Path, PathBuf},
};

const FILE_READ_BYTES: usize = 256;

#[derive(Debug, Clone)]
pub struct MagItem {
    pub path: PathBuf,
    pub metadata: Option<Metadata>,
}

impl MagItem {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref().to_owned();
        let metadata = match fs::metadata(&path) {
            Ok(meta) => Some(meta),
            Err(_) => None,
        };
        Self { path, metadata }
    }

    pub fn from<P: AsRef<Path>>(path: P, metadata: Option<Metadata>) -> Self {
        let path = path.as_ref().to_owned();

        Self { path, metadata }
    }
}

#[derive(Debug, Clone)]
pub struct MagFile {
    pub data: MagItem,
}

impl MagFile {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref().to_owned();
        let data = MagItem::new(&path);

        Self { data }
    }

    pub fn file_contents(&self) -> String {
        let mut buf = [0u8; FILE_READ_BYTES];
        let file = match File::open(&self.data.path) {
            Ok(f) => f,
            Err(e) => return format!("{} {}", "can't open file".to_string(), e),
        };

        match file.read_at(&mut buf, 0) {
            Ok(_) => String::from_utf8_lossy(&buf).to_string(),
            Err(e) => return format!("{} {}", "can't open file".to_string(), e),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MagFolder {
    pub data: MagItem,
    pub items: Vec<MagEntry>,
}

impl MagFolder {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref().to_owned();
        let data = MagItem::new(&path);
        let items = Vec::new();

        Self { data, items }
    }

    pub fn get_entries(&mut self) {
        self.items.clear();
        let entries = match fs::read_dir(&self.data.path) {
            Ok(entries) => entries,
            Err(_) => return,
        };

        for entry in entries {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => return,
            };
            let path = entry.path();

            let metadata = match fs::metadata(entry.path()) {
                Ok(m) => Some(m),
                Err(_) => None,
            };

            if let Some(metadata) = metadata {
                if metadata.is_file() {
                    self.items.push(MagEntry::File(MagFile::new(&path)));
                } else {
                    self.items.push(MagEntry::Dir(MagFolder::new(&path)));
                }
            }
        }
        self.sort_entries();
    }

    pub fn get_entries_return(&mut self) -> Option<Self> {
        self.items.clear();
        let entries = match fs::read_dir(&self.data.path) {
            Ok(entries) => entries,
            Err(_) => return None,
        };

        for entry in entries {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => return None,
            };
            let path = entry.path();

            let metadata = match fs::metadata(entry.path()) {
                Ok(m) => Some(m),
                Err(_) => None,
            };

            if let Some(metadata) = metadata {
                if metadata.is_file() {
                    self.items.push(MagEntry::File(MagFile::new(&path)));
                } else {
                    self.items.push(MagEntry::Dir(MagFolder::new(&path)));
                }
            }
        }
        self.sort_entries();
        Some(self.clone())
    }

    pub fn sort_entries(&mut self) {
        self.items.sort_by(|a, b| {
            let order_variant = a.variant_order().cmp(&b.variant_order());

            if order_variant == std::cmp::Ordering::Equal {
                a.path().cmp(b.path())
            } else {
                order_variant
            }
        });
    }

    pub fn return_entries(&self) -> Option<Vec<MagEntry>> {
        let mut v: Vec<MagEntry> = Vec::new();

        let entries = match fs::read_dir(&self.data.path) {
            Ok(entries) => entries,
            Err(_) => return None,
        };

        for entry in entries {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => return None,
            };
            let path = entry.path();

            let metadata = match fs::metadata(entry.path()) {
                Ok(m) => Some(m),
                Err(_) => None,
            };

            if let Some(metadata) = metadata {
                if metadata.is_file() {
                    v.push(MagEntry::File(MagFile::new(&path)));
                } else {
                    v.push(MagEntry::Dir(MagFolder::new(&path)));
                }
            }
        }

        Some(v)
    }
}

#[derive(Debug, Clone)]
pub enum MagEntry {
    Dir(MagFolder),
    File(MagFile),
}

impl MagEntry {
    pub fn get_folder(&self) -> Option<MagFolder> {
        match self {
            MagEntry::Dir(d) => Some(d.clone()),
            MagEntry::File(_) => None,
        }
    }
    pub fn get_file(&self) -> Option<MagFile> {
        match self {
            MagEntry::File(d) => Some(d.clone()),
            MagEntry::Dir(_) => None,
        }
    }

    pub fn get_path(&self) -> &PathBuf {
        match self {
            MagEntry::File(f) => &f.data.path,
            MagEntry::Dir(d) => &d.data.path,
        }
    }

    pub fn get_folder_path(&self, idx: usize) -> Option<PathBuf> {
        match self {
            MagEntry::Dir(d) => Some(d.items[idx].get_path().to_path_buf()),
            MagEntry::File(_) => None,
        }
    }

    pub fn variant_order(&self) -> i32 {
        match self {
            MagEntry::Dir(_) => 0,  // Los directorios primero
            MagEntry::File(_) => 1, // Los archivos después
        }
    }

    pub fn path(&self) -> &Path {
        match self {
            MagEntry::Dir(folder) => &folder.data.path,
            MagEntry::File(file) => &file.data.path,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file() {
        let f = MagFile::new("/home/mikel/Escritorio/rust/mag/Cargo.toml");
        let s = f.file_contents();
        println!("{s}");
        assert!(s.len() > 0);
    }
}
