use std::{
    env::args,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    process::exit,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub struct ParamData {
    p: u32,
    v: f64,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModelData {
    PacketID: u32,
    Final: bool,
    SessionUUID: String,
    TotalFolders: u32,
    TotalFiles: u32,
    TotalFileSize: u32,
    TotalFileSizeMegabyte: u32,
    TotalPacketCount: u32,
    ModelName: String,
    ModelBasePath: String,
    OrderedDirsToCreate: Vec<String>,
    OrderedRelativeFiles: Vec<String>,
    OrderedAbsoluteFiles: Vec<String>,
    OrderedAbsoluteFileSizes: Vec<String>,
    OrderedNumberOfPayloadsPerFile: Vec<String>,
    Sequence: u32,
    DataLength: u32,
    DataName: String,
    Data: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    Ver: u32,
    Time: u64,
    Type: u32,
    Command: u32,
    found: bool,
    Fwd: u32,
    USB: u32,
    Data: Vec<ParamData>,
    ModelData: ModelData,
}

fn zeros() -> impl Iterator<Item = u8> {
    pub struct Constant {
        item: u8,
    }
    impl Iterator for Constant {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            return Some(self.item);
        }
    }

    Constant { item: 0 }
}

pub fn server(address: &str) -> ! {
    let listener = TcpListener::bind(address).unwrap();
    let (mut front_stream, _addr) = listener.accept().unwrap();
    loop {
        let mut buf = [0u8; 4];

        front_stream
            .read_exact(&mut buf)
            .expect("Cannot read content length");

        let content_length = u32::from_be_bytes(buf);
        let mut buf = zeros().take(content_length as usize).collect::<Vec<u8>>();

        front_stream
            .read_exact(&mut buf)
            .expect("Unable to read bytes from front_stream");

        let s = String::from_utf8(buf).expect("Invalid utf-8 string");
        let entry: Entry = serde_json::from_str(&s).expect("Invalid data struct");
        println!("Model {}", if entry.found { "found" } else { "not found" })
    }
}

fn client(_address: &str) -> ! {
    unreachable!()
}

fn main() {
    let args = args().collect::<Vec<String>>();
    let action = match args.get(1) {
        Some(arg) => arg,
        None => {
            eprintln!("You should provide `server` or `client` as a first argument");
            exit(-1);
        }
    };
    let address = match args.get(2) {
        Some(arg) => arg,
        None => {
            eprintln!("You should provide an address as a second argument");
            exit(-1);
        }
    };
    match action.as_str() {
        "server" => server(&address),
        "client" => client(&address),
        _ => "First argument should be either `server` or `client`",
    };
}
