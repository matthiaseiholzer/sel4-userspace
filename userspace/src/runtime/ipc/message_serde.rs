

pub trait SerDe: Sized {
    fn serialize(&self, serializer: &mut [u8]) -> Result<usize, ()>;
    fn deserialize(deserializer: &[u8]) -> Result<Self, ()> ;
}