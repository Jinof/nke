extern crate clap;
use clap::{App, Arg};
use std::fs::OpenOptions;
use std::{fs::File, io::Write};
fn main() {
    let matches = App::new("NKE")
        .version("0.0.0")
        .author("Jinof <Jinof@foxmail.com>")
        .about("NKE (NCUHOME Kubernets Engine) 是协助你管理 K8s 的工具")
        .arg(Arg::with_name("welcome").help("设置服务器登录欢迎信息"))
        .get_matches();

    // 添加欢迎信息到 /etc/motd  TODO: 添加 ip 的输出. 支持 master 节点直接修改子节点 /etc/motd
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
}
