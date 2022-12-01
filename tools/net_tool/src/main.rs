use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use rustdns::Message;
use rustdns::types::*;
use std::net::UdpSocket;
use std::time::Duration;
use curl::easy::Easy;
use ipinfo::{IpDetails, IpError, IpInfo, IpInfoConfig};

type GenericError = Box<dyn Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

fn udp(domain: &str, query_type: Type) -> std::io::Result<Message> {
    // A DNS Message can be easily constructed
    let mut m = Message::default();
    m.add_question(domain, query_type, Class::Internet);
    m.add_extension(Extension {   // Optionally add a EDNS extension
        payload_size: 4096,       // which supports a larger payload size.
        ..Default::default()
    });

    // Setup a UDP socket for sending to a DNS server.
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_read_timeout(Some(Duration::new(10, 0)))?;
    socket.connect("8.8.8.8:53")?; // Google's Public DNS Servers

    // Encode the DNS Message as a Vec<u8>.
    let question = m.to_vec()?;

    // Send to the server.
    socket.send(&question)?;

    // Wait for a response from the DNS server.
    let mut resp = [0; 4096];
    let len = socket.recv(&mut resp)?;

    // Take the response bytes and turn it into another DNS Message.
    let answer = Message::from_slice(&resp[0..len])?;

    // Now do something with `answer`, in this case print it!
    println!("DNS Response:\n{}", &answer);

    Ok(answer)

}

fn get_ip_info(ips: Vec<&str>) -> Result<HashMap<String, IpDetails>, IpError>{
    // Setup token and other configurations.
    let config = IpInfoConfig { token: Some("988a789b27b1f1".to_string()), ..Default::default() };
    // Setup IpInfo structure and start looking up IP addresses.
    let mut ipinfo = IpInfo::new(config).expect("输入正确的 token ");
    ipinfo.lookup(ips.as_slice())
}

fn is_ip_record(record: &&Record) -> bool {
    match record.resource.r#type() {
        Type::A     => true,
        Type::AAAA  => true,
        _           => false
    }
}

fn dns_message_to_ip_vec(message: Message) -> Vec<String> {
        message.answers.iter()
            .filter(is_ip_record)
            .map(|record| record.resource.to_string())
            .collect()
}

fn format_ip_details(details: HashMap<String, IpDetails>) {
    details.iter().for_each(
        |(key, details)| {
            println!("{} info =>", key);
            println!("{}", serde_json::to_string_pretty(details).unwrap())
        }
    )
}

fn get_local_ip_addr() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect("8.8.8.8:83").unwrap();
    match socket.local_addr() {
        Ok(addr) => Some(addr.ip().to_string()),
        Err(_) => None
    }
}

fn get_public_ip() -> GenericResult<String> {
    let mut buf = File::create("./ip.dat")?;
    let mut handle = Easy::new();
    handle.url("https://ifconfig.me/")?;

    let mut transfer = handle.transfer();
    transfer.write_function(|data| {
        write!(buf, "{}", String::from_utf8(data.to_vec()).unwrap()).unwrap();
        Ok(data.len())
    })?;
    transfer.perform()?;

    let mut reader = File::open("./ip.dat")?;
    let mut s = String::new();
    reader.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let domain = "mgs.dgcb.com.cn";
    let query_type = Type::ANY;
    let dns_result = udp(domain, query_type).unwrap();
    let mut ips = dns_message_to_ip_vec(dns_result);
    ips.push(get_public_ip().expect(""));
    println!("records => {:?}", &ips);

    let details =
        get_ip_info(ips.iter().map(String::as_str).collect())
            .unwrap();

    format_ip_details(details);

}