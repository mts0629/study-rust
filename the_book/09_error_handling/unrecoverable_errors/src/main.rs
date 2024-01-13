fn main() {
    // Panic macro
    // panic!("crash and burn");

    let v = vec![1, 2, 3];
    v[99]; // Beyond the range, panic

    // Backtrace with `$ RUST_BACKTRACE=1 cargo run`:
    // 
    // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:6:5
    // stack backtrace:
    //    0: rust_begin_unwind
    //              at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/std/src/panicking.rs:498:5
    //    1: core::panicking::panic_fmt
    //              at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/core/src/panicking.rs:107:14
    //    2: core::panicking::panic_bounds_check
    //              at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/core/src/panicking.rs:75:5
    //    3: <usize as core::slice::index::SliceIndex<[T]>>::index
    //              at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/core/src/slice/index.rs:184:10
    //    4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
    //              at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/core/src/slice/index.rs:15:9
    //    5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
    //              at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/alloc/src/vec/mod.rs:2528:9
    //    6: unrecoverable_errors::main
    //              at ./src/main.rs:6:5
    //    7: core::ops::function::FnOnce::call_once
    //              at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/core/src/ops/function.rs:227:5
    // note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
}
