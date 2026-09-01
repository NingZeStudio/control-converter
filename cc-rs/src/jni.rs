use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;

fn convert_native(input_path: &str, output_path: &str) -> Result<(), String> {
    if input_path.is_empty() || output_path.is_empty() {
        return Err("input and output paths are required".to_string());
    }
    let source = crate::jsonio::load_json_file(input_path)
        .map_err(|e| format!("failed to read input file: {}", e))?;
    let mut ctx = crate::context::ConversionContext::new();
    let result = crate::fcl_to_zl::convert_fcl_to_zl(
        &mut ctx,
        &source,
        false,
        false,
        16.0 / 9.0,
        true,
        false,
    );
    crate::jsonio::write_json_file_opts(output_path, &result, false, true)
        .map_err(|e| format!("failed to write output file: {}", e))?;
    if let Some(summary) = ctx.substitution_summary() {
        eprintln!("{}", summary);
    }
    Ok(())
}

fn convert_native_zl2fcl(input_path: &str, output_path: &str) -> Result<(), String> {
    if input_path.is_empty() || output_path.is_empty() {
        return Err("input and output paths are required".to_string());
    }
    let source = crate::jsonio::load_json_file(input_path)
        .map_err(|e| format!("failed to read input file: {}", e))?;
    let mut ctx = crate::context::ConversionContext::new();
    let result = crate::zl_to_fcl::convert_zl_to_fcl(&mut ctx, &source, false);
    crate::jsonio::write_json_file_opts(output_path, &result, false, false)
        .map_err(|e| format!("failed to write output file: {}", e))?;
    if let Some(summary) = ctx.substitution_summary() {
        eprintln!("{}", summary);
    }
    Ok(())
}

fn jni_convert_impl(env: &mut JNIEnv, input: JString, output: JString, reverse: bool) -> jstring {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let input: String = match env.get_string(&input) {
            Ok(s) => s.into(),
            Err(_) => String::new(),
        };
        let output: String = match env.get_string(&output) {
            Ok(s) => s.into(),
            Err(_) => String::new(),
        };
        if reverse {
            convert_native_zl2fcl(&input, &output)
        } else {
            convert_native(&input, &output)
        }
    }));

    let message = match result {
        Ok(Ok(())) => return std::ptr::null_mut(),
        Ok(Err(e)) => e,
        Err(_panic) => {
            let msg = "JNI conversion panicked";
            eprintln!("warning: {}", msg);
            msg.to_string()
        }
    };

    match env.new_string(&message) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_tungsten_fcl_util_LayoutConverter_convertFclToZl2Native<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    input_path: JString<'local>,
    output_path: JString<'local>,
) -> jstring {
    jni_convert_impl(&mut env, input_path, output_path, false)
}

#[no_mangle]
pub extern "system" fn Java_com_tungsten_fcl_util_LayoutConverter_convertZl2ToFclNative<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    input_path: JString<'local>,
    output_path: JString<'local>,
) -> jstring {
    jni_convert_impl(&mut env, input_path, output_path, true)
}
