#[derive(Debug,Clone)]
pub struct PtrTable(Vec<usize>);

impl PtrTable {
    pub fn from_bytes(table: &[u8], ptr_offset: usize) -> Self {
        let ptrs = table
            .chunks(2)
            .filter(|s| s.len() >= 2)
            .map(|s| {
                let a = s[0] as usize;
                let b = s[1] as usize;
                ptr_offset + a + 0x100 * b
            })
            .collect();

        PtrTable(ptrs)
    }
}

pub struct PtrTableDesc {
    offset: usize,
    ptr_offset: usize,
    len: usize,
}

impl PtrTableDesc {
    pub fn get(&self, rom: &[u8]) -> PtrTable {
        PtrTable::from_bytes(&rom[self.offset..][..self.len], self.ptr_offset)
    }
}

const OVERWORLD_TOP_ROOMS: PtrTableDesc = PtrTableDesc {
    offset: 0x24000,
    ptr_offset: 0x20000,
    len: 0x100,
};

const OVERWORLD_BOTTOM_ROOMS: PtrTableDesc = PtrTableDesc {
    offset: 0x24100,
    ptr_offset: 0x64000,
    len: 0x100,
};

const UNDERWORLD1_ROOMS: PtrTableDesc = PtrTableDesc {
    offset: 0x28000,
    ptr_offset: 0x24000,
    len: 0x200,
};

const UNDERWORLD2_ROOMS: PtrTableDesc = PtrTableDesc {
    offset: 0x2C000,
    ptr_offset: 0x28000,
    len: 0x200,
};
