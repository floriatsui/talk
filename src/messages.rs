const BUFFER_SIZE: usize = 8;

enum Message {
    Initial { username: String, other: String },
    Data { buffer: [u8; BUFFER_SIZE] },
}

impl Message {
    fn serialize(&self, output: &mut [u8]) {
        todo!()
    }

    fn deserialize(serialized: &[u8]) -> Self {
        todo!()
    }
}
