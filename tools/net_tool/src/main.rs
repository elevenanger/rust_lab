use std::collections::HashMap;
use rustdns::Message;
use rustdns::types::*;
use std::net::UdpSocket;
use std::time::Duration;
use ipinfo::{IpDetails, IpError, IpInfo, IpInfoConfig};

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

fn main() {
    let domain = "www.landbank.com.tw";
    let query_type = Type::AAAA;
    let dns_result = udp(domain, query_type).unwrap();
    let ips = dns_message_to_ip_vec(dns_result);
    println!("records => {:?}", &ips);

    let details =
        get_ip_info(ips.iter().map(String::as_str).collect())
            .unwrap();

    format_ip_details(details);
}