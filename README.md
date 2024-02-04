# Miminal rustc ICE reproduction

## How to reproduce
Just build `embassy-nrf`
```bash
cd embassy-nrf/
cargo build
```

## Stack error
```
✦ ❯ cargo build
   Compiling jewel v0.1.0 (/home/guiss/projetos/ble/lifetime_different_crates_ice/jewel-rs)
   Compiling embassy-nrf v0.1.0 (/home/guiss/projetos/ble/lifetime_different_crates_ice/embassy-nrf)
error[E0726]: implicit elided lifetime not allowed here
 --> src/lib.rs:9:6
  |
9 | impl BleRadio for Radio {
  |      ^^^^^^^^ expected lifetime parameter
  |
help: indicate the anonymous lifetime
  |
9 | impl BleRadio<'_> for Radio {
  |              ++++

thread 'rustc' panicked at compiler/rustc_middle/src/ty/context.rs:1196:53:
DefId::expect_local: `DefId(4:13 ~ jewel[24ae]::BleRadio::transmit::{opaque#0}::'a)` isn't local
stack backtrace:
   0:     0x7bf728a94816 - std::backtrace_rs::backtrace::libunwind::trace::he9b5424ca56bbca0
                               at /rustc/88189a71e4e4376eea82ac61db6a539612eb200a/library/std/src/../../backtrace/src/backtrace/libunwind.rs:104:5
   1:     0x7bf728a94816 - std::backtrace_rs::backtrace::trace_unsynchronized::h3564b84f57da2e1b
                               at /rustc/88189a71e4e4376eea82ac61db6a539612eb200a/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7bf728a94816 - std::sys_common::backtrace::_print_fmt::h44ca1bab4cfe911b
                               at /rustc/88189a71e4e4376eea82ac61db6a539612eb200a/library/std/src/sys_common/backtrace.rs:68:5
   3:     0x7bf728a94816 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf57bf3a288fcf4bc
                               at /rustc/88189a71e4e4376eea82ac61db6a539612eb200a/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7bf728ae6e10 - core::fmt::rt::Argument::fmt::h819418805491a499
                               at /rustc/88189a71e4e4376eea82ac61db6a539612eb200a/library/core/src/fmt/rt.rs:142:9
   5:     0x7bf728ae6e10 - core::fmt::write::hdf7249a8a29010f3
                               at /rustc/88189a71e4e4376eea82ac61db6a539612eb200a/library/core/src/fmt/mod.rs:1120:17
   6:     0x7bf728a8814f - std::io::Write::write_fmt::h2bd186ffd57674cf
                               at /rustc/88189a71e4e4376eea82ac61db6a539612eb200a/library/std/src/io/mod.rs:1810:15
   7:     0x7bf728a945f4 - std::sys_common::backtrace::_print::h3fd3fec21323f445
                               at /rustc/88189a71e4e4376eea82ac61db6a539612eb200a/library/std/src/sys_common/backtrace.rs:47:5
   8:     0x7bf728a945f4 - std::sys_common::backtrace::print::h63d79712d19c9376
                               at /rustc/88189a71e4e4376eea82ac61db6a539612eb200a/library/std/src/sys_common/backtrace.rs:34:9
   9:     0x7bf728a97387 - std::panicking::default_hook::{{closure}}::h5221b22f986bd5a4
  10:     0x7bf728a970e9 - std::panicking::default_hook::h092cd25740c5f920
                               at /rustc/88189a71e4e4376eea82ac61db6a539612eb200a/library/std/src/panicking.rs:292:9
  11:     0x7bf72576dcdc - std[20dd01746d97080d]::panicking::update_hook::<alloc[e20806006b8c186d]::boxed::Box<rustc_driver_impl[5959fcfa8d1d1ca4]::install_ice_hook::{closure#0}>>::{closure#0}
  12:     0x7bf728a97ad6 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h978c6082b45e43ce
                               at /rustc/88189a71e4e4376eea82ac61db6a539612eb200a/library/alloc/src/boxed.rs:2030:9
  13:     0x7bf728a97ad6 - std::panicking::rust_panic_with_hook::h4c5290d4ef19b28b
                               at /rustc/88189a71e4e4376eea82ac61db6a539612eb200a/library/std/src/panicking.rs:785:13
  14:     0x7bf728a97822 - std::panicking::begin_panic_handler::{{closure}}::h9a6cdc050fc1c54a
                               at /rustc/88189a71e4e4376eea82ac61db6a539612eb200a/library/std/src/panicking.rs:659:13
  15:     0x7bf728a94d16 - std::sys_common::backtrace::__rust_end_short_backtrace::ha4d1156e558e4eed
                               at /rustc/88189a71e4e4376eea82ac61db6a539612eb200a/library/std/src/sys_common/backtrace.rs:171:18
  16:     0x7bf728a97574 - rust_begin_unwind
                               at /rustc/88189a71e4e4376eea82ac61db6a539612eb200a/library/std/src/panicking.rs:647:5
  17:     0x7bf728ae3515 - core::panicking::panic_fmt::h5f26cb9a8ff06e61
                               at /rustc/88189a71e4e4376eea82ac61db6a539612eb200a/library/core/src/panicking.rs:72:14
  18:     0x7bf725be731d - <rustc_middle[a7a29c4e21e9b57]::ty::context::TyCtxt>::is_suitable_region
  19:     0x7bf725a1b544 - <rustc_infer[10d8ac8d71669794]::infer::error_reporting::nice_region_error::NiceRegionError>::try_report
  20:     0x7bf725a1a4c1 - <rustc_infer[10d8ac8d71669794]::infer::error_reporting::TypeErrCtxt>::try_report_nice_region_error
  21:     0x7bf725a22ddd - <rustc_infer[10d8ac8d71669794]::infer::error_reporting::TypeErrCtxt>::report_region_errors
  22:     0x7bf726eee2db - <rustc_trait_selection[e1a86cdab96b6151]::traits::engine::ObligationCtxt>::resolve_regions_and_report_errors
  23:     0x7bf72754e167 - rustc_hir_analysis[78382cd4123c02ec]::check::check::check_item_type
  24:     0x7bf726e94590 - rustc_hir_analysis[78382cd4123c02ec]::check::wfcheck::check_well_formed
  25:     0x7bf726e93559 - rustc_query_impl[c67c2b45fd6d76a]::plumbing::__rust_begin_short_backtrace::<rustc_query_impl[c67c2b45fd6d76a]::query_impl::check_well_formed::dynamic_query::{closure#2}::{closure#0}, rustc_middle[a7a29c4e21e9b57]::query::erase::Erased<[u8; 1usize]>>
  26:     0x7bf726e8f001 - rustc_query_system[796ead6f43aea3ee]::query::plumbing::try_execute_query::<rustc_query_impl[c67c2b45fd6d76a]::DynamicConfig<rustc_query_system[796ead6f43aea3ee]::query::caches::VecCache<rustc_hir[1a82c26e5ac3d8a5]::hir_id::OwnerId, rustc_middle[a7a29c4e21e9b57]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[c67c2b45fd6d76a]::plumbing::QueryCtxt, true>
  27:     0x7bf726e8eaff - rustc_query_impl[c67c2b45fd6d76a]::query_impl::check_well_formed::get_query_incr::__rust_end_short_backtrace
  28:     0x7bf726e9044c - rustc_hir_analysis[78382cd4123c02ec]::check::wfcheck::check_mod_type_wf
  29:     0x7bf726e9037d - rustc_query_impl[c67c2b45fd6d76a]::plumbing::__rust_begin_short_backtrace::<rustc_query_impl[c67c2b45fd6d76a]::query_impl::check_mod_type_wf::dynamic_query::{closure#2}::{closure#0}, rustc_middle[a7a29c4e21e9b57]::query::erase::Erased<[u8; 1usize]>>
  30:     0x7bf72777e731 - rustc_query_system[796ead6f43aea3ee]::query::plumbing::try_execute_query::<rustc_query_impl[c67c2b45fd6d76a]::DynamicConfig<rustc_query_system[796ead6f43aea3ee]::query::caches::DefaultCache<rustc_span[22bc0c0910288393]::def_id::LocalModDefId, rustc_middle[a7a29c4e21e9b57]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[c67c2b45fd6d76a]::plumbing::QueryCtxt, true>
  31:     0x7bf7275f2f02 - rustc_query_impl[c67c2b45fd6d76a]::query_impl::check_mod_type_wf::get_query_incr::__rust_end_short_backtrace
  32:     0x7bf726c7c445 - rustc_middle[a7a29c4e21e9b57]::query::plumbing::query_ensure_error_guaranteed::<rustc_query_system[796ead6f43aea3ee]::query::caches::DefaultCache<rustc_span[22bc0c0910288393]::def_id::LocalModDefId, rustc_middle[a7a29c4e21e9b57]::query::erase::Erased<[u8; 1usize]>>, ()>
  33:     0x7bf726c7cd8b - rustc_hir_analysis[78382cd4123c02ec]::check_crate
  34:     0x7bf72742b052 - rustc_interface[a4c2db5fc6bd6a93]::passes::analysis
  35:     0x7bf72742ac9f - rustc_query_impl[c67c2b45fd6d76a]::plumbing::__rust_begin_short_backtrace::<rustc_query_impl[c67c2b45fd6d76a]::query_impl::analysis::dynamic_query::{closure#2}::{closure#0}, rustc_middle[a7a29c4e21e9b57]::query::erase::Erased<[u8; 1usize]>>
  36:     0x7bf72741f3e7 - rustc_query_system[796ead6f43aea3ee]::query::plumbing::try_execute_query::<rustc_query_impl[c67c2b45fd6d76a]::DynamicConfig<rustc_query_system[796ead6f43aea3ee]::query::caches::SingleCache<rustc_middle[a7a29c4e21e9b57]::query::erase::Erased<[u8; 1usize]>>, false, false, false>, rustc_query_impl[c67c2b45fd6d76a]::plumbing::QueryCtxt, true>
  37:     0x7bf72741efe0 - rustc_query_impl[c67c2b45fd6d76a]::query_impl::analysis::get_query_incr::__rust_end_short_backtrace
  38:     0x7bf72791aeb3 - rustc_interface[a4c2db5fc6bd6a93]::interface::run_compiler::<core[7fbf663f9cc01fa9]::result::Result<(), rustc_span[22bc0c0910288393]::ErrorGuaranteed>, rustc_driver_impl[5959fcfa8d1d1ca4]::run_compiler::{closure#0}>::{closure#0}
  39:     0x7bf7279fcdde - std[20dd01746d97080d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a4c2db5fc6bd6a93]::util::run_in_thread_with_globals<rustc_interface[a4c2db5fc6bd6a93]::util::run_in_thread_pool_with_globals<rustc_interface[a4c2db5fc6bd6a93]::interface::run_compiler<core[7fbf663f9cc01fa9]::result::Result<(), rustc_span[22bc0c0910288393]::ErrorGuaranteed>, rustc_driver_impl[5959fcfa8d1d1ca4]::run_compiler::{closure#0}>::{closure#0}, core[7fbf663f9cc01fa9]::result::Result<(), rustc_span[22bc0c0910288393]::ErrorGuaranteed>>::{closure#0}, core[7fbf663f9cc01fa9]::result::Result<(), rustc_span[22bc0c0910288393]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7fbf663f9cc01fa9]::result::Result<(), rustc_span[22bc0c0910288393]::ErrorGuaranteed>>
  40:     0x7bf7279fcc0d - <<std[20dd01746d97080d]::thread::Builder>::spawn_unchecked_<rustc_interface[a4c2db5fc6bd6a93]::util::run_in_thread_with_globals<rustc_interface[a4c2db5fc6bd6a93]::util::run_in_thread_pool_with_globals<rustc_interface[a4c2db5fc6bd6a93]::interface::run_compiler<core[7fbf663f9cc01fa9]::result::Result<(), rustc_span[22bc0c0910288393]::ErrorGuaranteed>, rustc_driver_impl[5959fcfa8d1d1ca4]::run_compiler::{closure#0}>::{closure#0}, core[7fbf663f9cc01fa9]::result::Result<(), rustc_span[22bc0c0910288393]::ErrorGuaranteed>>::{closure#0}, core[7fbf663f9cc01fa9]::result::Result<(), rustc_span[22bc0c0910288393]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7fbf663f9cc01fa9]::result::Result<(), rustc_span[22bc0c0910288393]::ErrorGuaranteed>>::{closure#1} as core[7fbf663f9cc01fa9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7bf728aa13c5 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h476ab21e86af8c38
                               at /rustc/88189a71e4e4376eea82ac61db6a539612eb200a/library/alloc/src/boxed.rs:2016:9
  42:     0x7bf728aa13c5 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h03a8c656e3f61339
                               at /rustc/88189a71e4e4376eea82ac61db6a539612eb200a/library/alloc/src/boxed.rs:2016:9
  43:     0x7bf728aa13c5 - std::sys::pal::unix::thread::Thread::new::thread_start::h967fe69886e84602
                               at /rustc/88189a71e4e4376eea82ac61db6a539612eb200a/library/std/src/sys/pal/unix/thread.rs:108:17
  44:     0x7bf7228aa9eb - <unknown>
  45:     0x7bf72292e7cc - <unknown>
  46:                0x0 - <unknown>

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: please attach the file at `/home/guiss/projetos/ble/lifetime_different_crates_ice/embassy-nrf/rustc-ice-2024-02-04T18_15_45-59050.txt` to your bug report

note: compiler flags: --crate-type lib -C embed-bitcode=no -C debuginfo=2 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [check_well_formed] checking that `<impl at src/lib.rs:9:1: 9:24>` is well-formed
#1 [check_mod_type_wf] checking that types are well-formed in top-level module
#2 [analysis] running analysis passes on this crate
end of query stack
For more information about this error, try `rustc --explain E0726`.
error: could not compile `embassy-nrf` (lib) due to 1 previous error
```
