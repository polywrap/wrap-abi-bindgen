use std::convert::TryFrom;
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    Context,
    DecodeError,
    EncodeError,
    Read,
    ReadDecoder,
    Write,
    WriteEncoder,
    JSON,
};
use crate::Directory;

use crate::File;

pub fn serialize_directory(args: &Directory) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: Directory".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_directory(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_directory<W: Write>(args: &Directory, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&3)?;
    writer.context().push("name", "String", "writing property");
    writer.write_string("name")?;
    writer.write_string(&args.name)?;
    writer.context().pop();
    writer.context().push("files", "Vec<File>", "writing property");
    writer.write_string("files")?;
    writer.write_array(&args.files, |writer, item| {
        File::write(item, writer)
    })?;
    writer.context().pop();
    writer.context().push("dirs", "Vec<Directory>", "writing property");
    writer.write_string("dirs")?;
    writer.write_array(&args.dirs, |writer, item| {
        Directory::write(item, writer)
    })?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_directory(args: &[u8]) -> Result<Directory, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: Directory".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_directory(&mut reader)
}

pub fn read_directory<R: Read>(reader: &mut R) -> Result<Directory, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _name: String = String::new();
    let mut _name_set = false;
    let mut _files: Vec<File> = vec![];
    let mut _files_set = false;
    let mut _dirs: Vec<Directory> = vec![];
    let mut _dirs_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "name" => {
                reader.context().push(&field, "String", "type found, reading property");
                _name = reader.read_string()?;
                _name_set = true;
                reader.context().pop();
            }
            "files" => {
                reader.context().push(&field, "Vec<File>", "type found, reading property");
                _files = reader.read_array(|reader| {
                    let object = File::read(reader)?;
                    Ok(object)
                })?;
                _files_set = true;
                reader.context().pop();
            }
            "dirs" => {
                reader.context().push(&field, "Vec<Directory>", "type found, reading property");
                _dirs = reader.read_array(|reader| {
                    let object = Directory::read(reader)?;
                    Ok(object)
                })?;
                _dirs_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_name_set {
        return Err(DecodeError::MissingField("name: String.".to_string()));
    }
    if !_files_set {
        return Err(DecodeError::MissingField("files: [File].".to_string()));
    }
    if !_dirs_set {
        return Err(DecodeError::MissingField("dirs: [Directory].".to_string()));
    }

    Ok(Directory {
        name: _name,
        files: _files,
        dirs: _dirs,
    })
}
