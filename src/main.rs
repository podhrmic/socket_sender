/*
Instad of sending UDP packed with bytes, construct a full ethernet frame and send it over UDP, so it can 
be reconstructed on the other side.
*/
use std::net::UdpSocket;
use std::{thread, time};

fn main() {

    let tx = thread::spawn(move || {
        // bind to an IP address assigned to an existing interface & specific port
        let socket = UdpSocket::bind("192.168.179.1:6666").expect("couldn't bind to address");

        let s = String::from("Hello there");
        let bytes = s.into_bytes();

        loop {
            // send to an IP on the same network as the interface
            // e.g. iface is 10.0.0.1 with mask 255.255.25.0
            // Anything in range of 10.0.0.2 - 10.0.0.254 is OK
            // pick any unused port
            let len = socket
                .send_to(&bytes, "192.168.179.50:6666")
                .expect("Couldn't send data");

            println!("Sent {} data: {:?}", len, bytes);
            thread::sleep(time::Duration::from_millis(500));
        }
    });

    let rx = thread::spawn(move || {
        // bind to an IP address assigned to an existing interface & specific port
        let socket = UdpSocket::bind("192.168.179.1:6665").expect("couldn't bind to address");
        socket
            .connect("192.168.179.50:6665")
            .expect("connect function failed");
        let mut buf = vec![0; 1500];
        loop {
            match socket.recv(&mut buf) {
                Ok(received) => {
                    println!("received {} bytes: {:?}", received, String::from_utf8_lossy(&buf[0..received]));
                    println!("received {} bytes: {:?}", received, &buf[0..received]);
                }
                Err(e) => println!("recv function failed: {:?}", e),
            }
        }
    });

    println!("Waiting for threads to end");
    match rx.join() {
        Ok(_) => println!("Rx All well"),
        Err(e) => println!("Rx thread joined with an error {:?}", e), 
    }
    match tx.join() {
        Ok(_) => println!("Tx All well"),
        Err(e) => println!("Tx thread joined with an error {:?}", e), 
    }


}
