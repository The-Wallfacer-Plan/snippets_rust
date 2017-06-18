extern crate llvm_sys as llvm;
extern crate clang;

mod llvm_jit;
mod llvm_nop;
mod clang_struct;

fn main() {
    clang_struct::struct_clang();
    llvm_jit::jit();
    llvm_nop::nop();
}