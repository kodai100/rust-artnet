
use std::net::UdpSocket;
use std::thread;

use yew::services::ConsoleService;

mod art_net_receive_data;

const BUFFER_LENGTH: usize = 2048;

pub struct ArtNetReceiver {

}

impl ArtNetReceiver {
    pub fn start() -> std::io::Result<()> {
        let socket = UdpSocket::bind("127.0.0.1:11111")?;

        let mut buf = [0; BUFFER_LENGTH];

        loop {

            match socket.recv_from(&mut buf) {
                Ok((buf_size, src_addr)) => {
                    thread::spawn(move || {

                        ConsoleService::info(format!("{:}", "=".repeat(80)).as_ref());
                        ConsoleService::info(format!("src address: {:?}", src_addr).as_ref());
                        ConsoleService::info(format!("buffer size: {:?}", buf_size).as_ref());

                        let art_net_recv_data = art_net_receive_data::ArtNetReceiveData::new(&mut buf, buf_size);

                        match art_net_recv_data {
                            Ok(data) => {
                                ConsoleService::info(format!("Opcode {:#01x}", data.opt_code).as_ref());

                                ConsoleService::info(format!("Protocol Version Hi {:?}", data.protocol_version).as_ref());
                            
                                ConsoleService::info(format!("Sequence {:?}", data.sequence).as_ref());
                                ConsoleService::info(format!("Physical {:?}", data.physical).as_ref());
                            
                                ConsoleService::info(format!("Universe {:?}", data.universe).as_ref());
                            
                                ConsoleService::info(format!("Length {:?}", data.length).as_ref());
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

    // fn dump(&'a self, data: &art_net_receive_data::ArtNetReceiveData) {

        
    
    //     // for d in &data.dmx_data {
    //     //     print!("{:?} ", d);
    //     // }
        
    //     // print!("\n");
        
    // }
}