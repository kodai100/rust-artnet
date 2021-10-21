use std::str;
use bytes::Buf;

pub struct ArtNetReceiveData<'a> {

    pub buffer : &'a mut [u8],
    pub data_length: usize

}

impl ArtNetReceiveData<'_> {
    
    pub fn is_valid(&self) -> bool {
        self.data_length > 12
    }

    pub fn dump(&self) {

        if self.is_valid() {
            let artnet = str::from_utf8(&self.buffer[0..7]).unwrap();
            let mut buf = &self.buffer[8..self.data_length];

            println!("{:}", "=".repeat(80));
            println!("buffer size: {:?}", self.data_length);
            
            println!("ArtNet : {:?}", artnet);
            println!("Opcode {:#01x}", buf.get_u16_le());

            println!("Protocol Version Hi {:?}", buf.get_u16());

            println!("Sequence {:?}", buf.get_u8());
            println!("Physical {:?}", buf.get_u8());

            println!("Universe {:?}", buf.get_u16());

            let length = buf.get_u16();
            println!("Length {:?}", &length);

            for i in 0..length {
                print!("{:?} ", buf.get_u8());
            }

            print!("\n");

        } else {

        }

        

    }
}