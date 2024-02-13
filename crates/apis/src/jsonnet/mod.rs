use javy::quickjs::JSValue;
use jrsonnet_evaluator::{
    apply_tla,
    function::TlaArg,
    gc::GcHashMap,
    manifest::{JsonFormat, ManifestFormat},
    tb,
    trace::{CompactFormat, PathResolver, TraceFormat},
    FileImportResolver, State,
};
use jrsonnet_parser::IStr;

use crate::JSApiSet;

pub struct VM {
    state: State,
    manifest_format: Box<dyn ManifestFormat>,
    trace_format: Box<dyn TraceFormat>,
    tla_args: GcHashMap<IStr, TlaArg>,
}

static JSONNET: &str = include_str!("./index.js");

pub(super) struct Jsonnet;

impl JSApiSet for Jsonnet {
    fn register(&self, runtime: &javy::Runtime, _config: &crate::APIConfig) -> anyhow::Result<()> {
        let context = runtime.context();
        let global = context.global_object()?;
        global.set_property(
            "__jsonnet_evaluate_snippet",
            context.wrap_callback(jsonnet_evaluate_snippet_callback())?,
        )?;

        match context.eval_module("arakoo-jsonnet", JSONNET) {
            Ok(_) => {}
            Err(err) => eprintln!("Error loading the path shim: {err}"),
        };
        Ok(())
    }
}

// fn jsonnet_make() -> VM {
//     let state = State::default();
//     state.settings_mut().import_resolver = tb!(FileImportResolver::default());
//     state.settings_mut().context_initializer = tb!(jrsonnet_stdlib::ContextInitializer::new(
//         state.clone(),
//         PathResolver::new_cwd_fallback(),
//     ));

//     VM {
//         state,
//         manifest_format: Box::new(JsonFormat::default()),
//         trace_format: Box::new(CompactFormat::default()),
//         tla_args: GcHashMap::default(),
//     }
// }

// fn jsonnet_destroy(vm: *mut VM) {
//     unsafe {
//         let dloc_vm = Box::from_raw(vm);
//         drop(dloc_vm);
//     }
// }

fn jsonnet_evaluate_snippet(filename: &str, snippet: &str) -> String {
    let vm = {
        let state = State::default();
        state.settings_mut().import_resolver = tb!(FileImportResolver::default());
        state.settings_mut().context_initializer = tb!(jrsonnet_stdlib::ContextInitializer::new(
            state.clone(),
            PathResolver::new_cwd_fallback(),
        ));

        VM {
            state,
            manifest_format: Box::new(JsonFormat::default()),
            trace_format: Box::new(CompactFormat::default()),
            tla_args: GcHashMap::default(),
        }
    };
    match vm
        .state
        .evaluate_snippet(filename, snippet)
        .and_then(|val| apply_tla(vm.state.clone(), &vm.tla_args, val))
        .and_then(|val| val.manifest(&vm.manifest_format))
    {
        Ok(v) => v,
        Err(e) => {
            let mut out = String::new();
            vm.trace_format.write_trace(&mut out, &e).unwrap();
            out
        }
    }
}

fn jsonnet_evaluate_snippet_callback() -> impl FnMut(
    &javy::quickjs::JSContextRef,
    javy::quickjs::JSValueRef,
    &[javy::quickjs::JSValueRef],
) -> anyhow::Result<javy::quickjs::JSValue> {
    move |_ctx: &javy::quickjs::JSContextRef,
          _this: javy::quickjs::JSValueRef,
          args: &[javy::quickjs::JSValueRef]| {
        if args.len() != 3 {
            return Err(anyhow::anyhow!(
                "Expecting 2 arguments, received {}",
                args.len()
            ));
        }
        let filename = args[1];
        let snippet = args[2];
        let result = jsonnet_evaluate_snippet(filename.as_str()?, snippet.as_str()?);
        Ok(JSValue::from(result))
    }
}
