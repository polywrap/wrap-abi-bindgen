use polywrap_wasm_rs::JSON;
use crate::imported::ArgsGenerateEmbeds;
pub use crate::wrap::*;

pub fn generate_embeds(embeds_json: JSON::Value) -> Result<Directory, String> {
    let embeds: Vec<EmbedWasmEmbed> = JSON::from_value(embeds_json).map_err(|e| e.to_string())?;
    let embed_output: EmbedOutput = EmbedModule::generate_embeds(&ArgsGenerateEmbeds { embeds })?;

    Ok(Directory {
        name: "embeds".to_string(),
        files: map_files(embed_output.files.clone()),
        dirs: map_dirs(embed_output.dirs.clone())
    })
}

fn map_files(files: Vec<EmbedFile>) -> Vec<File> {
    files.iter().map(|file| {
        File {
            name: file.name.clone(),
            data: file.data.clone()
        }
    }).collect()
}

fn map_dirs(dirs: Vec<EmbedDirectory>) -> Vec<Directory> {
    dirs.iter().map(|dir| {
        Directory {
            name: dir.name.clone(),
            files: map_files(dir.files.clone()),
            dirs: map_dirs(dir.dirs.clone())
        }
    }).collect()
}