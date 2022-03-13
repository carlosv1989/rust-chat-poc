use std::io::{self, ErrorKind, Read, Write};
use std::fs::File;
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const MSG_SIZE: usize = 64;

fn request_receiver_key(key_id:String) -> String{
    //let body = reqwest::get("https://www.rust-lang.org")
    //.await?
    //.text()
    //.await?;
    //println!("body = {:?}", body);
    fn encrypt_message(mut message:String,key:String) -> String{
    message.push_str(&key);
    message
}
    String::from("DEF|1")
}

fn request_sender_key() ->  String{
    //let body = reqwest::get("https://www.rust-lang.org")
    //.await?
    //.text()
    //.await?;
    //println!("body = {:?}", body);
    let sender_key =String::from("ABC|1");
    write_key(sender_key)
}

fn check_local_key_availability() -> bool{
   let local_key=get_local_key();
   if local_key != ""{
       return true;
   }else{
       return false;
   }
    
}

fn get_local_key() -> String{
    let mut local_key  = String::new();
    let mut f = File::open("storage.key").unwrap();
    f.read_to_string(&mut local_key);
    local_key
}

fn write_key(key:String) -> String{
    let mut file = std::fs::File::create("storage.key").expect("create failed");
    file.write_all(key.as_bytes()).expect("write failed");
    key
}

fn decrypt_message(mut message:String,key:String) -> String{
    String::from("Test")
}

fn encrypt_message(mut message:String,key:String) -> String{
    message.push_str(&key);
    message
}

fn main() {
    println!("Enter the chat Server IP:");
    let mut input  = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let server_ip= input.trim();
    let mut client = TcpStream::connect(server_ip).expect("Stream failed to connect");
    client.set_nonblocking(true).expect("failed to initiate non-blocking");

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                let receiver_key = request_receiver_key(String::from("IDTEST"));
                println!("message recv {:?}", msg);
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("connection with server was severed");
                break;
            }
        }

        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("writing to socket failed");
                println!("message sent {:?}", msg);
            }, 
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }

        thread::sleep(Duration::from_millis(100));
    });

    println!("Write a Message:");
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("reading from stdin failed");
        let msg = buff.trim().to_string();
        if msg == ":quit" {
            println!("bye bye!");
            break;
        }else if check_local_key_availability()==true {
            let encrypted_message: String = encrypt_message(msg,get_local_key());
            tx.send(encrypted_message).is_err();
        }else{
            let encrypted_message: String = encrypt_message(msg,request_sender_key());
            tx.send(encrypted_message).is_err();
        }
    

    }   
}