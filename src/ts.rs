use std::{path::Path};
use std::fs::{File, read_dir};
use std::io::Write;
use std::path::PathBuf;

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

pub fn conv_ts_to_js(ts_file: &str) {
    let file = Path::new(ts_file);
    let file_name = file.file_name().unwrap().to_str().unwrap();

    let file_js = convert_ts(&ts_file.to_string());

    let mut file_js_path = PathBuf::new();
    file_js_path.push(file.parent().unwrap().parent().unwrap());
    file_js_path.push("dist");
    file_js_path.push(file_name);
    file_js_path.set_extension("js");

    let mut file_js_file = File::create(file_js_path).unwrap();
    file_js_file.write_all(file_js.as_bytes()).unwrap();
}

pub fn conv_dir_ts_to_js(dir: &PathBuf) {
    // load ts files to vec
    let mut ts_files = Vec::new();
    let src_dir = dir.join("src");

    for entry in read_dir(&src_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.extension().unwrap() == "ts" {
            ts_files.push(path);
        }
    }

    // convert the ts files to js files before run
    for entry in ts_files {
        let ts_file_name = entry.to_str().unwrap();
        conv_ts_to_js(ts_file_name);
    }
}
