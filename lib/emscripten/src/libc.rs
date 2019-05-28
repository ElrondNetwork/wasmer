extern crate libc;
extern crate wasmer_runtime_core;

use std::convert::TryInto;

use wasmer_runtime_core::vm::Ctx;

pub fn current_sigrtmax(_ctx: &mut Ctx) -> i32 {
    debug!("emscripten::current_sigrtmax");
    0
}

pub fn current_sigrtmin(_ctx: &mut Ctx) -> i32 {
    debug!("emscripten::current_sigrtmin");
    0
}

pub fn endpwent(_ctx: &mut Ctx) {
    debug!("emscripten::endpwent");
}

pub fn execv(_ctx: &mut Ctx, _a: i32, _b: i32) -> i32 {
    debug!("emscripten::execv");
    0
}

pub fn fexecve(_ctx: &mut Ctx, _a: i32, _b: i32, _c: i32) -> i32 {
    debug!("emscripten::fexecve");
    0
}

pub fn fpathconf(_ctx: &mut Ctx, _a: i32, _b: i32) -> i32 {
    debug!("emscripten::fpathconf");
    0
}

pub fn getitimer(_ctx: &mut Ctx, _a: i32, _b: i32) -> i32 {
    debug!("emscripten::getitimer");
    0
}

pub fn getpwent(_ctx: &mut Ctx) -> i32 {
    debug!("emscripten::getpwent");
    0
}

pub fn killpg(_ctx: &mut Ctx, _a: i32, _b: i32) -> i32 {
    debug!("emscripten::killpg");
    0
}

pub fn pathconf(ctx: &mut Ctx, path_ptr: i32, name: i32) -> i32 {
    let path = emscripten_memory_pointer!(ctx.memory(0), path_ptr) as *const i8;

    unsafe {
        libc::pathconf(path, name).try_into().unwrap()
    }
}

pub fn setpwent(_ctx: &mut Ctx) {
    debug!("emscripten::setpwent");
}

pub fn sigismember(_ctx: &mut Ctx, _a: i32, _b: i32) -> i32 {
    debug!("emscripten::sigismember");
    0
}

pub fn sigpending(_ctx: &mut Ctx, _a: i32) -> i32 {
    debug!("emscripten::sigpending");
    0
}
