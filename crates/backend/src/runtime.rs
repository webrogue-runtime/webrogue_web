use std::sync::{Arc, Mutex};

use anyhow::Ok;

webrogue_backend_web_macro::make_web_link_functions!();

pub struct Runtime {}

impl Runtime {
    pub fn new() -> Self {
        Self {}
    }
}

extern "C" {
    fn wr_rs_em_js_initWasmModule(pointer: *const u8, size: u32);
    fn wr_rs_em_js_makeWorker(jsonPtr: *const u8);
    fn wr_rs_em_js_terminateWorker();
    fn wr_rs_em_js_execFunc(funcNamePtr: *const u8);
    fn wr_rs_em_js_isExecutionFinished() -> bool;
    fn wr_rs_em_js_continueFuncExecution();
    fn wr_rs_em_js_modErrorSize() -> u32;
    fn wr_rs_em_js_getModError(error: *mut u8);
    fn wr_rs_em_js_getImportedFuncId() -> u32;
    fn wr_rs_em_js_getArgUInt32(argNum: u32) -> u32;
    fn wr_rs_em_js_writeUInt32Result(result: u32);
    fn wr_rs_em_js_getArgUInt64(argNum: u32) -> u64;
    fn wr_rs_em_js_writeUInt64Result(result: u64);
}

fn exec_async_func(func_name: &str, context: Arc<Mutex<webrogue_runtime::Context>>) -> bool {
    unsafe {
        let funcs = get_func_vec(context);
        let mut func_name = func_name.as_bytes().to_vec();
        func_name.push(0);
        wr_rs_em_js_execFunc(func_name.as_ptr());
        while !wr_rs_em_js_isExecutionFinished() {
            let func_id = wr_rs_em_js_getImportedFuncId();
            funcs[func_id as usize]();
            // callImportedFunc(getImportedFuncId());
            // if (!procExit)
            wr_rs_em_js_continueFuncExecution();
        }
        let error_size = wr_rs_em_js_modErrorSize();
        if error_size != 0 {
            let mut error_text = vec![0u8; error_size as usize];
            wr_rs_em_js_getModError(error_text.as_mut_ptr());
            eprintln!("error: {}", String::from_utf8(error_text).unwrap());
            return false;
        }
        return true;
    }
}

impl webrogue_runtime::Runtime for Runtime {
    fn run(
        &self,
        wasi: webrogue_runtime::wasi_common::WasiCtx,
        bytecode: Vec<u8>,
    ) -> anyhow::Result<()> {
        unsafe {
            let mut worker_config = config_str().as_bytes().to_vec();
            worker_config.push(0);
            let context =
                webrogue_runtime::Context::new(Box::new(crate::memory::MemoryFactory {}), wasi);
            let context = Arc::new(Mutex::new(context));
            wr_rs_em_js_makeWorker(worker_config.as_ptr());
            wr_rs_em_js_initWasmModule(bytecode.as_ptr(), bytecode.len() as u32);

            exec_async_func("_start", context);
            wr_rs_em_js_terminateWorker();
        }
        Ok(())
    }
}
