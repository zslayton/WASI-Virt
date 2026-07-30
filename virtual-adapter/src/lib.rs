#![no_main]

mod config;
mod env;
mod io;

pub(crate) struct VirtAdapter;

pub(crate) mod bindings {
    #[cfg(feature = "wasi-p2")]
    wit_bindgen::generate!({
        path: "../wit/p2",
        world: "virtual-adapter",
        generate_all,
        merge_structurally_equal_types: true,
    });

    #[cfg(not(feature = "wasi-p2"))]
    compile_error!(
        "a feature specifying the WASI version must be enabled (for example: 'wasi-p2')"
    );

    use super::VirtAdapter;
    export!(VirtAdapter);
}
