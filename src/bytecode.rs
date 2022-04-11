use std::io::Cursor;

pub struct Bytecode {
    bin: Vec<u8>,
}

impl Bytecode {
    pub fn new(bin: Vec<u8>) -> Self {
        Self {
            bin
        }
    }

    pub fn read_char(&self, offset: &mut usize) -> u8 {
        let out = *self.bin.get(*offset).unwrap();

        *offset += 1;

        return out
    }

    pub fn read_u32(&self, offset: &mut usize) -> u32 {
        let slice = &self.bin[*offset..];
        let cursor = &mut Cursor::new(slice);
        let val = leb128::read::unsigned(cursor).unwrap();

        let len: usize = cursor.position().try_into().unwrap();
        *offset += len;

        u32::try_from(val).unwrap()
    }

    pub fn read_string(&self, offset: &mut usize) -> String {
        let len = self.read_u32(offset).try_into().unwrap();
        let chars = self.bin[*offset..len].to_vec();

        String::from_utf8(chars).unwrap()
    }

    pub fn len(&self) -> usize {
        self.bin.len()
    }
}