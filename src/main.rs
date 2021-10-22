use std::net::UdpSocket;
use std::thread;

use art_net_recv_data::ArtNetReceiveData;

mod art_net_recv_data;

const BUFFER_LENGTH: usize = 2048;

fn main() -> std::io::Result<()> {

    let socket = UdpSocket::bind("127.0.0.1:11111")?;

    let mut buf = [0; BUFFER_LENGTH];

    loop {

        match socket.recv_from(&mut buf) {
            Ok((buf_size, src_addr)) => {
                thread::spawn(move || {

                    
                    println!("{:}", "=".repeat(80));
                    println!("src address: {:?}", src_addr);
                    println!("buffer size: {:?}", buf_size);

                    let art_net_recv_data = art_net_recv_data::ArtNetReceiveData::new(&mut buf, buf_size);

                    match art_net_recv_data {
                        Ok(data) => {
                            dump(&data);
                        },
                        Err(e) => {}
                    }
                    

                  });
            }, 
            Err(e) => {
                println!("couldn't receive request: {:?}", e);
            }
        }

    }


}

fn dump(data: &ArtNetReceiveData) {
    println!("Opcode {:#01x}", data.opt_code);

    println!("Protocol Version Hi {:?}", data.protocol_version);

    println!("Sequence {:?}", data.sequence);
    println!("Physical {:?}", data.physical);

    println!("Universe {:?}", data.universe);

    println!("Length {:?}", data.length);

    for d in &data.dmx_data {
        print!("{:?} ", d);
    }
    
    print!("\n");
    
}
