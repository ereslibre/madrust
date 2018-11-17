#![feature(box_syntax)]
#![feature(rustc_private)]

extern crate getopts;
extern crate rustc;
extern crate rustc_driver;
extern crate rustc_codegen_utils;
extern crate rustc_metadata;
extern crate syntax;

use rustc::hir::def_id::LOCAL_CRATE;
use rustc::session::Session;
use rustc_driver::{driver, Compilation, CompilerCalls};

struct RustTest {}

impl<'a> CompilerCalls<'a> for RustTest {
    fn build_controller(
        self: Box<Self>,
        _: &Session,
        _: &getopts::Matches,
    ) -> driver::CompileController<'a> {
        let mut control = driver::CompileController::basic();
        control.after_analysis.stop = Compilation::Stop;
        control.after_analysis.callback = box |state: &mut driver::CompileState| {
            for mir_key in state.tcx.unwrap().mir_keys(LOCAL_CRATE).iter() {
                let mir = state.tcx.unwrap().optimized_mir(*mir_key);
                println!();
                state.session.span_note_without_error(mir.span, "Printing MIR for this");
                state.session.note_without_error(&format!("{:?}", mir));
                println!();
            }
        };
        control
    }
}

fn main() {
    rustc_driver::run(|| {
        let args: Vec<_> = std::env::args().collect();
        rustc_driver::run_compiler(&args, box RustTest{}, None, None)
    });
}
