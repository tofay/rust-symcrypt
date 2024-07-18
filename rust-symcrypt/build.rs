fn main() {
    // If the SymCrypt version is 103.4 or later, add the symcrypt1034 cfg.
    if symcrypt_sys::SYMCRYPT_CODE_VERSION_API == 103
        && symcrypt_sys::SYMCRYPT_CODE_VERSION_MINOR >= 4
    {
        println!("cargo:rustc-cfg=symcrypt1034");
    }
}
