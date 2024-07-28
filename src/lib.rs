use std::sync::Arc;

use webrogue_runtime::WasiFactory;

extern "C" {
    fn wr_rs_sleep(ms: u32);
}

fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "full");

    let lifecycle = webrogue_runtime::Lifecycle::new();

    let mut wasi_factory = webrogue_wasi_sync::WasiFactory::new();
    wasi_factory.sleep = Some(webrogue_wasi_sync::Sleep {
        f: Arc::new(|duration| unsafe { wr_rs_sleep(duration.as_millis() as u32) }),
    });
    let mut wasi = wasi_factory.make();

    wasi_factory.add_actual_dir(&mut wasi, std::env::current_dir()?, "/");

    let reader = webrogue_runtime::wrapp::Reader::from_vec(
        include_bytes!("../external/webrogue_rs/example_apps/bin/simple.wrapp").to_vec(),
    )?;

    webrogue_std_stream_sdl::run_in_terminal(
        wasi,
        std::sync::Arc::new(move |wasi| {
            let backend = webrogue_backend_web::Backend::new();
            lifecycle.run(backend, wasi, reader.clone());
        }),
    );

    // webrogue_std_stream_os::bind_streams(&mut wasi);

    // std::io::stdout().write(b"test\n");

    // let backend = webrogue_backend_wasm3::Backend::new();
    // lifecycle.run(backend, wasi, reader)?;

    Ok(())
}

#[no_mangle]
extern "C" fn rust_main() {
    match main() {
        Err(e) => {
            panic!("{}", e.to_string())
        }
        Ok(_) => {}
    }
}
