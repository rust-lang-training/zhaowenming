use neon::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

// #[neon::main]
// fn main(mut cx: ModuleContext) -> NeonResult<()> {
//     cx.export_function("hello", hello)?;
//     Ok(())
// }


fn digest_file(mut cx: FunctionContext) -> JsResult<JsString> {
    let file_name = cx.argument::<JsString>(0)?.value(&mut cx);
    let file = File::open(&file_name).unwrap();
    let mut reader = BufReader::new(file);
    let mut ctx =
    ring::digest::Context::new(&ring::digest::SHA256);
    let mut buffer = [0; 1024 * 64];
    loop {
    let n = reader.read(&mut buffer).unwrap();
    if n == 0 {
    break;
     }
    ctx.update(&buffer[..n]);
     }
     let result = ctx.finish();
     let hash_hex = hex::encode(result);
     Ok(cx.string(hash_hex))
     }
   

  #[neon::main]
     fn main(mut cx: ModuleContext) -> NeonResult<()> {
     cx.export_function("hello", hello)?;
    //  cx.export_function("requestStsToken", request_sts_token)?;
     cx.export_function("digestFile", digest_file)?;
    Ok(())
}

