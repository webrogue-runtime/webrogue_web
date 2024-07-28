use proc_macro::TokenStream;

#[proc_macro]
pub fn make_web_link_functions(_item: TokenStream) -> TokenStream {
    let mut result = "".to_owned();

    let mut config: std::collections::BTreeMap<
        String,
        std::collections::BTreeMap<String, (u32, Option<webrogue_macro_common::ValueType>)>,
    > = std::collections::BTreeMap::new();
    let mut config_str = "".to_owned();

    let mut func_vec = vec![];
    for import in webrogue_macro_common::get_imports() {
        if !config.contains_key(&import.module) {
            config.insert(import.module.clone(), std::collections::BTreeMap::new());
        }
        let module = config.get_mut(&import.module).unwrap();
        module.insert(
            import.func_name.clone(),
            (func_vec.len() as u32, import.ret_str.clone()),
        );
        func_vec.push(import.clone());
    }
    {
        config_str += "{";
        for (module_i, (module_name, module)) in config.iter().enumerate() {
            if module_i != 0 {
                config_str += ",";
            }
            config_str += &format!("\n    \"{}\": {{", module_name);
            for (func_i, (func_name, (func_id, ret_type))) in module.iter().enumerate() {
                if func_i != 0 {
                    config_str += ",";
                }

                config_str += &format!("\n        \"{}\": {{", func_name);
                config_str += &format!(
                    "\n            \"ret_type\": \"{}\",",
                    match ret_type {
                        None => "void",
                        Some(t) => match t {
                            webrogue_macro_common::ValueType::U32 => "uint32_t",
                            webrogue_macro_common::ValueType::U64 => "uint64_t",
                        },
                    }
                );
                config_str += &format!("\n            \"func_id\": {}", func_id);
                config_str += "\n        }";
            }
            config_str += "\n    }";
        }
        config_str += "\n}";
    }
    result += &format!(
        r##"
fn config_str() -> String {{ r#"{}"#.to_owned() }}
        "##,
        config_str
    );

    result += r##"
fn get_func_vec(context: Arc<Mutex<webrogue_runtime::Context>>) -> Vec<Box<dyn Fn()>> {
vec![
"##;
    for import in func_vec {
        let args = import
            .args
            .iter()
            .enumerate()
            .map(|(i, arg)| {
                let f = match arg {
                    webrogue_macro_common::ValueType::U32 => "wr_rs_em_js_getArgUInt32",
                    webrogue_macro_common::ValueType::U64 => "wr_rs_em_js_getArgUInt64",
                };
                return format!("unsafe {{ {}({}) }},", f, i);
            })
            .collect::<Vec<_>>()
            .join("");
        let ret: String = match import.ret_str {
            None => "".to_owned(),
            Some(r) => {
                let f = match r {
                    webrogue_macro_common::ValueType::U32 => "wr_rs_em_js_writeUInt32Result",
                    webrogue_macro_common::ValueType::U64 => "wr_rs_em_js_writeUInt64Result",
                };
                format!("unsafe {{ {}(result) }}", f)
            }
        };
        result += &format!(
            r##"
    Box::new({{ 
        let context_c = context.clone(); 
        move || {{
            let mut lock = context_c.lock().unwrap();
            let result = webrogue_runtime::imported_functions::{}::{}(
                &mut *lock,
                {}
            );
            {}
        }} 
    }}),
"##,
            import.module, import.func_name, args, ret
        );
    }
    result += r##"
]
}
        "##;

    result.parse().unwrap()
}
