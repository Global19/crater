#[macro_use]
mod driver;

minicrater! {
    single_thread_small {
        ex: "small",
        crate_select: "demo",
        ..Default::default()
    },

    single_thread_full {
        ex: "full",
        crate_select: "local",
        ..Default::default()
    },

    single_thread_blacklist {
        ex: "blacklist",
        crate_select: "demo",
        ..Default::default()
    },

    single_thread_ignore_blacklist {
        ex: "ignore-blacklist",
        crate_select: "demo",
        ignore_blacklist: true,
        ..Default::default()
    },

    multi_thread_full {
        ex: "full",
        crate_select: "local",
        multithread: true,
        ..Default::default()
    },

    clippy_small {
        ex: "clippy",
        crate_select: "demo",
        mode: "clippy",
        toolchains: &["stable", "stable+rustflags=-Dclippy::all"],
        ..Default::default()
    },
}
