extern crate clang;

use clang::*;
use std::path::PathBuf;

pub fn gen_clang() -> Clang {
    // Acquire an instance of `Clang`
    let clang = Clang::new().unwrap();
    return clang;
}


pub fn check_structs<S: AsRef<str>>(clang: &Clang, f: impl Into<PathBuf>, args: &[S]) {

    // Create a new `Index`
    let index = Index::new(&clang, false, false);

    // Parse a source file into a translation unit
    let tu = index.parser(f).arguments(args).parse().unwrap();
    println!("{:?}", tu.get_memory_usage());
    println!("{:?}", tu.get_target());

    // Get the structs in this translation unit
    let structs = tu.get_entity()
        .get_children()
        .into_iter()
        .filter(|e| e.get_kind() == EntityKind::StructDecl)
        .collect::<Vec<_>>();

    // Print information about the structs
    for cst in structs {
        let type_ = cst.get_type().unwrap();
        let size = type_.get_sizeof().unwrap();
        println!("struct: {:?} (size: {} bytes)",
                 cst.get_name().unwrap(),
                 size);

        for field in cst.get_children() {
            let name = field.get_name().unwrap();
            let offset = type_.get_offsetof(&name).unwrap();
            println!("    field: {:?} (offset: {} bits)", name, offset);
        }
    }
}

pub fn func_name_info<S: AsRef<str>>(clang: &Clang, f: impl Into<PathBuf>, args: &[S]) {
    let index = Index::new(clang, false, false);
    let tu = index.parser(f).arguments(args).parse().unwrap();
    let funcs: Vec<_> = tu.get_entity()
        .get_children()
        .into_iter()
        .filter(|e| e.get_kind() == EntityKind::FunctionDecl)
        .collect();

    for cdecl in funcs {
        let ty = cdecl.get_type().unwrap();
        let defs = cdecl.get_definition();
        let cname = cdecl.get_name();
        if let Some(cf) = defs {
            eprintln!("DEF: {:?}", cname);
        } else {
            eprintln!("DECL: {:?}", cname);
        }
    }
}

fn main() {
    let clang = gen_clang();
    func_name_info(&clang, "res/structs.c", &["-c"]);
}
