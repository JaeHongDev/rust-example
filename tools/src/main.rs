use pcap::{Capture, Device, Packet};
use std::net::IpAddr;
use std::ops::Add;
use std::thread::park;
use dns_lookup::lookup_addr;
use etherparse::{IpHeader, PacketHeaders, TransportHeader};

fn main() {
    // 1. 기본적으로 사용 가능한 네트워크 디바이스(인터페이스)를 가져옵니다.
    let device = Device::lookup().unwrap();
    println!("선택된 디바이스: {:#?}", device);

    // 2. 디바이스로부터 캡처를 생성하며, 프로미스큐어스(promiscuous) 모드와 최대 패킷 길이(snaplen)를 설정

    let mut cap = Capture::from_device(device.expect("error"))
        .expect("not found")
        .promisc(true)
        .snaplen(65535)
        .open()
        .expect("");

    let mut i = 0;

    while let Ok(packet) = cap.next_packet() {
        // 패킷 데이터에서 IP와 TCP/UDP 정보를 파싱
        if let Ok(packet_headers) = PacketHeaders::from_ethernet_slice(&packet.data) {
            if let Some(ip_header) = packet_headers.ip {
                if let Some(TransportHeader::Udp(udp_header)) = packet_headers.transport {
                    match ip_header {
                        IpHeader::Version4(header) => {
                            let sou_ip4 = IpAddr::from(header.source);
                            let des_ip4 = IpAddr::from(header.destination);

                            if i <= 1 {
                                println!("{:?}", udp_header);
                                dns_parser::Packet::parse(packet.data).expect("");
                                if let Ok(parse_result) = dns_parser::Packet::parse(packet.data){
                                    println!("변환된 질문 섹션: {:?}", parse_result);
                                    i += 1 ;
                                }
                            }

                            match (lookup_addr(&sou_ip4), lookup_addr(&des_ip4)) {
                                (_, Ok(des_host)) => println!(
                                    "목적지 도메인 이름 {} IPv4 패킷 - 출발지 IP: {:?}, 목적지 IP: {:?}",
                                    des_host, header.source, header.destination
                                ),
                                (_, Err(_)) => {}
                            }
                        }
                        IpHeader::Version6(header) => {
                            let ip6 = IpAddr::from(header.destination);
                            match lookup_addr(&ip6) {
                                Ok(hostname) => println!("도메인 이름: {}", hostname),
                                _ => {}
                            }
                            println!(
                                "IPv6 패킷 - 출발지 IP: {:?}, 목적지 IP: {:?}",
                                header.source, header.destination
                            );
                        }
                    }
                }
            }
        }
    }
}

fn extract_domain_name(data: &[u8]) -> String {
    let mut pos = 0;
    let mut domain_name = String::new();

    while pos < data.len() {
        let length = data[pos] as usize; // 레이블 길이
        if length == 0 {
            break; // 0이면 도메인 이름 끝
        }
        pos += 1;
        if !domain_name.is_empty() {
            domain_name.push('.');
        }
        let label = &data[pos..pos + length]; // 레이블 추출
        domain_name.push_str(&String::from_utf8_lossy(label));
        pos += length; // 다음 레이블로 이동
    }

    domain_name
}

