use super::FFIValue;
use libc::size_t;
use libloading::Library;
use std::collections::HashMap;

pub struct PackageLoader {
    packages: HashMap<String, Library>,
}

impl PackageLoader {
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
        }
    }

    pub fn load_package(
        &mut self,
        package_name: String,
        package_path: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let lib = libloading::Library::new(package_path)?;
            self.packages.insert(package_name, lib);
            Ok(())
        }
    }

    pub fn call_fn(
        &self,
        package_name: String,
        fn_name: String,
        args: &mut Vec<FFIValue>,
    ) -> Result<FFIValue, Box<dyn std::error::Error>> {
        unsafe {
            let func: libloading::Symbol<unsafe extern "C" fn(*mut FFIValue, size_t) -> FFIValue> =
                self.packages
                    .get(&package_name)
                    .unwrap()
                    .get(fn_name.as_bytes())?;
            Ok(func(args.as_mut_slice().as_mut_ptr(), args.len()))
        }
    }
}
