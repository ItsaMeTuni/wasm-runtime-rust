use crate::bytecode::Bytecode;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub struct Module {
    functions: Vec<Function>,
    exports: Vec<Export>,
    types: Vec<Type>,
    bytecode: Bytecode
}

pub struct Function {
    offset: u32,
    type_idx: u32
}

pub struct Export {
    name: String,

    // Type of the export
    export_type: ExportType,

    // Idx of function, table, mem or global based on type
    exportee_idx: u32,
}

pub enum ExportType {
    Func = 0x00,
    Table = 0x01,
    Mem = 0x02,
    Global = 0x03,
}


pub struct Type {
    params: Vec<ValueType>,
    results: Vec<ValueType>
}

pub enum ValueType {
    I32 = 0x7f,
    I64 = 0x7e,
    F32 = 0x7d,
    F64 = 0x7c,
}

#[derive(FromPrimitive, PartialEq)]
enum SectionId {
    Exports = 7,
    Functions = 3,
    Types = 1,
    Code = 10,
    Unknown
}

struct Section {
    id: SectionId,
    offset: usize,
    length: u32,
}

struct MalformedBytecodeError;
type Result<T> = Result<T, MalformedBytecodeError>;

pub fn parse(bytecode: &Bytecode) -> () {
    let sections = read_sections(bytecode);

    try_read_functions(bytecode, &sections);

    //Module {}
}

fn read_sections(bytecode: &Bytecode) -> Vec<Section> {
    // start at ninth byte in order to skip magic number and version
    // (first 8 bytes)
    let mut offset: usize = 8;

    let mut sections = vec![];

    while offset < bytecode.len() {
        let section_id = bytecode.read_char(&mut offset);
        let section_len = bytecode.read_u32(&mut offset);

        let section = Section {
            id: FromPrimitive::from_u8(section_id).unwrap_or(SectionId::Unknown),
            offset,
            length: section_len
        };

        sections.push(section);
    }

    sections
}

fn try_read_functions(bytecode: &Bytecode, sections: &Vec<Section>) -> Option<Vec<Function>> {
    let functions_section = find_section_by_id(sections, SectionId::Functions)?;

    let function_body_offsets = get_function_body_offsets(bytecode, sections);

    let mut offset = functions_section.offset;
    let functions_count = bytecode.read_u32(&mut offset);

    let mut functions = vec![];

    for function_idx in 0..functions_count {
        let type_idx = bytecode.read_char(&mut offset);
        let body_offset = function_body_offsets.get(function_idx).unwrap();

        let function = Function {
            offset: body_offset,
            type_idx,
        };

        functions.push(function);
    }
    
    return Some(functions);
}

fn get_function_body_offsets(bytecode: &Bytecode, sections: &Vec<Section>) -> Vec<usize> {
    let code_section = find_section_by_id(sections, SectionId::Code)?;

    let mut offset = code_section.offset;
    let function_count = bytecode.read_u32(&mut offset);

    let offsets = vec![];

    for function_idx in 0..function_count {
        let body_size = bytecode.read_u32(&mut offset);

        // + 1 to skip local declarations
        // TODO: fix this
        offsets.push(offset + 1);

        offset += body_size as usize;
    }

    return offsets;

}

fn find_section_by_id(sections: &Vec<Section>, id: SectionId) -> Option<&Section> {
    sections.iter()
        .find(|section| section.id == id)
}
