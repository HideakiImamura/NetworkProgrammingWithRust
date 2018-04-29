use std::net::TcpStream;
use std::str;
use std::io::{self, BufRead, BufReader, Write};

fn main(){
    let mut stream = TcpStream::connect("127.0.0.1:8888").expect("サーバーにアクセスできないよ");
    loop{
        let mut input = String::new();
        let mut buffer: Vec<u8> = Vec::new();
        io::stdin().read_line(&mut input).expect("stdinを読めないよ");
        stream.write(input.as_bytes()).expect("サーバーに書き込めないよ");

        let mut reader = BufReader::new(&stream);
        
        reader.read_until(b'\n', &mut buffer).expect("バッファを読み込めないよ");
        print!("{}", str::from_utf8(&buffer).expect("バッファにstrとして書き込めないよ"));
    }
}
