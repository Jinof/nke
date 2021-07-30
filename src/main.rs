extern crate clap;
use clap::{App, Arg};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;

fn main() {
    let matches = App::new("NKE")
        .version("0.0.0")
        .author("Jinof <Jinof@foxmail.com>")
        .about("NKE (NCUHOME Kubernets Engine) 是协助你管理 K8s 的工具")
        .subcommand(
            App::new("welcome").arg(
                Arg::with_name("welcome")
                    .help("设置服务器登录欢迎信息")
                    .takes_value(true),
            ),
        )
        .subcommand(
            App::new("meta").arg(
                Arg::with_name("meta")
                    .help("获取服务器元数据例如 NETWORK, CPU, MEMORY")
                    .takes_value(true),
            ),
        )
        .get_matches();

    println!(
        "{} {}",
        matches.is_present("welcome"),
        matches.is_present("meta")
    );

    if matches.is_present("welcome") {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open("/etc/motd")
            .unwrap();
        file.write(String::from("你已进入NCUHOME CLOUD").as_bytes())
            .unwrap();
        file.sync_all().unwrap();
    };

    // TODO: Add ssl connection before GET request.
    if matches.is_present("meta") {
        let mut stream =
            TcpStream::connect("www.taobao.com:443").expect("unbale to connect to www.taobao.com");

        let mut buf = [0; 128];
        stream
            .write(
                "
            GET /help/getip.php HTTP/2
            Host: www.taobao.com
            user-agent: nke/1.0
            "
                .as_bytes(),
            )
            .expect("write error");
        stream.read(&mut buf).expect("read error");
        println!("{}", str::from_utf8(&buf).unwrap());
    }
}
