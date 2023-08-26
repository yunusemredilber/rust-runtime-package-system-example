mod package_loader;
use package_loader::*;

#[repr(C)]
pub enum FFIValue {
    Number(f64),
    Boolean(bool),
    Character(char),
    Void,
}

fn main() {
    let mut package_loader = PackageLoader::new();
    match package_loader.load_package(
        String::from("libcore_package"),
        String::from("./core-package/target/debug/libcore_package.dylib"),
    ) {
        Ok(_) => {}
        Err(error) => {
            panic!("{}", error);
        }
    };

    let add_numbers_result = package_loader.call_fn(
        String::from("libcore_package"),
        String::from("print"),
        &mut vec![
            FFIValue::Character('h'),
            FFIValue::Character('e'),
            FFIValue::Character('y'),
        ],
    );

    assert!(matches!(add_numbers_result, Ok(FFIValue::Void)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    static mut PACKAGE_LOADER: OnceLock<PackageLoader> = OnceLock::new();

    fn get_package_loader() -> &'static PackageLoader {
        unsafe {
            PACKAGE_LOADER.get_or_init(|| {
                let mut package_loader = PackageLoader::new();
                match package_loader.load_package(
                    String::from("libcore_package"),
                    String::from("./core-package/target/debug/libcore_package.dylib"),
                ) {
                    Ok(_) => {}
                    Err(error) => {
                        panic!("{}", error);
                    }
                };
                package_loader
            })
        }
    }

    #[test]
    fn add_numbers_works() {
        let package_loader = get_package_loader();

        let add_numbers_result = package_loader.call_fn(
            String::from("libcore_package"),
            String::from("add_numbers"),
            &mut vec![FFIValue::Number(1.0), FFIValue::Number(2.0)],
        );

        assert!(matches!(add_numbers_result, Ok(FFIValue::Number(3.0))));
    }
}
