use std::str;
use bytes::Buf;

#[repr(C)]
pub struct ArtNetReceiveData {
    pub opt_code: u16,
    pub protocol_version: u16,
    pub sequence: u8,
    pub physical: u8,
    pub universe: u16,
    pub length: u16,

    pub dmx_data: Vec<u8>
}

impl ArtNetReceiveData {
    
    pub fn new(buffer: &[u8], data_length: usize) -> Result<Self, String> {
        
        if data_length > 12 {

            let artnet = str::from_utf8(&buffer[0..7]).unwrap();
            let mut buf = &buffer[8..data_length];
            
            println!("ArtNet : {:?}", artnet);

            let opt_code = buf.get_u16_le();
            let protocol_version = buf.get_u16();

            let sequence = buf.get_u8();
            let physical = buf.get_u8();

            let universe = buf.get_u16();
            let length = buf.get_u16();

            let dmx_data = buf.to_vec();

            Ok(ArtNetReceiveData{opt_code, protocol_version, sequence, physical, universe, length, dmx_data})

        } else {
            Err(String::from(""))
        }

    }
}