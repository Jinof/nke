extern crate clap;
use clap::{App, Arg};
use openssl::ssl::{SslConnector, SslMethod};
use std::fs::OpenOptions;
use std::io::{Read, Write};
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
            App::new("ip").arg(
                Arg::with_name("meta")
                    .help("获取服务器元外部 IP")
                    .takes_value(true),
            ),
        )
        .get_matches();

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

    if matches.is_present("ip") {
        let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
        let stream = TcpStream::connect("www.taobao.com:443").unwrap();

        let mut stream = connector.connect("www.taobao.com", stream).unwrap();

        let mut buf = [0; 1024];
        // 注意此处的多行 string 语法, rust 中多行 string 要在行末尾加 \
        // HTTP 报文以 \r\n 作为分隔符, 以 \r\n\r\n 作为结束符, 两行报文间
        // 除分隔符外不能有其他字符(比如空格), 所以 rust 换行符 \ 应紧贴 \r\n.
        stream
            .ssl_write(
                b"GET /help/getip.php HTTP/1.1\r\n\
                Host: www.taobao.com\r\n\
                user-agent: nke/1.0\r\n\
                accept: */*\r\n\r\n",
            )
            .unwrap();
        stream.ssl_read(&mut buf).unwrap();
        let resp: Vec<&str> = str::from_utf8(&buf).unwrap().split(" ").collect();
        let resp_body = resp.last().unwrap();
        let resp_body: Vec<&str> = resp_body.split("\"").collect();
        let ip = resp_body.get(1).unwrap();

        println!("external ip is: {}", ip);
    }
}
