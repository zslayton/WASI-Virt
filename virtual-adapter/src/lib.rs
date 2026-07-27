#![no_main]

mod config;
mod env;
mod io;

pub(crate) struct VirtAdapter;

pub(crate) mod bindings {
    #[cfg(feature = "wasi-0_2_x")]
    wit_bindgen::generate!({
        path: "../wit/0_2_x",
        world: "virtual-adapter",
        generate_all,
        merge_structurally_equal_types: true,
    });

    #[cfg(not(feature = "wasi-0_2_x"))]
    compile_error!("a feature specifying the WASI version must be enabled");

    use super::VirtAdapter;
    export!(VirtAdapter);
}
