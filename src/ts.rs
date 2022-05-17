use std::{path::Path};

// swc
use swc_common::{
    self, sync::Lrc, Globals, Mark, SourceMap, GLOBALS,
};
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};
use swc_ecma_transforms_base::{hygiene::hygiene, resolver};
use swc_ecma_transforms_typescript::strip;
use swc_ecma_visit::FoldWith;
// ----

pub fn convert_ts(ts_file: &str) -> String {
    let cm: Lrc<SourceMap> = Default::default();

    let fm = cm
        .load_file(Path::new(ts_file))
        .expect(&*format!("failed to load {}", ts_file));

    let lexer = Lexer::new(
        Syntax::Typescript(TsConfig {
            tsx: false,
            ..Default::default()
        }),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    let module = parser.parse_module().expect("failed to parse module.");

    let globals = Globals::default();
    GLOBALS.set(&globals, || {
        let unresolved_mark = Mark::new();
        let top_level_mark = Mark::new();

        // Optionally transforms decorators here before the resolver pass
        // as it might produce runtime declarations.

        // Conduct identifier scope analysis
        let module = module.fold_with(&mut resolver(unresolved_mark, top_level_mark, true));

        // Remove typescript types
        let module = module.fold_with(&mut strip(top_level_mark));

        // Fix up any identifiers with the same name, but different contexts
        let module = module.fold_with(&mut hygiene());

        let mut buf = vec![];

        let mut emitter = Emitter {
            cfg: swc_ecma_codegen::Config { minify: true },
            cm: cm.clone(),
            comments: None,
            wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
        };

        emitter.emit_module(&module).unwrap();

        String::from_utf8(buf).expect("non-utf8?")
    })
}



