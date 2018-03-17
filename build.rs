fn main() {
    println!("cargo:rustc-link-lib=pypy3-c");
    println!("cargo:rustc-link-search=native=/Users/omerba/anaconda/lib");
    println!("cargo:include=/Users/omerba/anaconda/include")
}
