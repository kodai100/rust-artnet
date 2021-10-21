use std::net::UdpSocket;
use std::thread;


mod ArtNetReceiveData;

const BUFFER_LENGTH: usize = 2048;

fn main() -> std::io::Result<()> {

    let socket = UdpSocket::bind("127.0.0.1:11111")?;

    let mut buf = [0; BUFFER_LENGTH];

    loop {

        match socket.recv_from(&mut buf) {
            Ok((buf_size, src_addr)) => {
                thread::spawn(move || {
                    
                    println!("src address: {:?}", src_addr);

                    let artNetReceiveData = ArtNetReceiveData::ArtNetReceiveData{buffer: &mut buf, data_length: buf_size };

                    artNetReceiveData.dump();
                  });
            }, 
            Err(e) => {
                println!("couldn't receive request: {:?}", e);
            }
        }

    }


}
