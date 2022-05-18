use std::{path::Path};
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::env;

// swc
use swc_common::{
    self, sync::Lrc, Globals, Mark, SourceMap, GLOBALS,
};
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};
use swc_ecma_transforms_base::{hygiene::hygiene, resolver};
use swc_ecma_transforms_typescript::strip;
use swc_ecma_visit::FoldWith;
use walkdir::WalkDir;
use crate::dist;
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

pub fn dist_ts() {
    let dir = env::current_dir().unwrap();

    for entry in WalkDir::new(dir.join("src").to_str().unwrap()) {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if file_name == "src" {
            continue;
        }

        if path.is_file() {
            if !file_name.ends_with(".ts") {
                continue;
            }

            let file_path = path.to_str().unwrap().split("src").collect::<Vec<&str>>()[1];
            let f_path_dist = format!(r"{}/dist/{}", dir.to_str().unwrap(), file_path);
            let path_without_file = f_path_dist.split(file_name).collect::<Vec<&str>>()[0];

            if !Path::new(&path_without_file).exists() {
                create_dir_all(&path_without_file).unwrap();
            }
            let f_path_dist = dist::change_ext(&f_path_dist, ".js");

            File::create(Path::new(&f_path_dist)).expect("Couldn't create dist file.");

            let mut file = File::open(path).expect("Couldn't open file.");
            let mut content = String::new();
            file.read_to_string(&mut content).expect("Couldn't read file.");
            let content = convert_ts(path.to_str().unwrap());
            let mut file = File::create(Path::new(&f_path_dist)).expect("Couldn't create dist file.");
            file.write_all(content.as_bytes()).expect("Couldn't write file.");
        }
    }
}