use crate::bytecode::Bytecode;

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
    func = 0x00,
    table = 0x01,
    mem = 0x02,
    global = 0x03,
}


pub struct Type {
    params: Vec<ValueType>,
    results: Vec<ValueType>
}

pub enum ValueType {
    i32 = 0x7f,
    i64 = 0x7e,
    f32 = 0x7d,
    f64 = 0x7c,
}

struct Section {
    id: u8,
    offset: usize,
    length: u32,
}

pub fn parse(bytecode: &Bytecode) -> Module {
    let sections = read_sections(bytecode);

    Module {}
}

pub fn read_sections(bytecode: &Bytecode) -> Vec<Section> {
    // start at ninth byte in order to skip magic number and version
    // (first 8 bytes)
    let mut offset: usize = 8;

    let mut sections = vec![];

    while offset < bytecode.len() {
        let section_id = bytecode.read_char(&mut offset);
        let section_len = bytecode.read_u32(&mut offset);

        let section = Section {
            id: section_id,
            offset,
            length: section_len
        };

        sections.push(section);
    }

    sections
}
