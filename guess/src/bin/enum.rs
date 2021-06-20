enum Kind {
    MALE,
    FEMALE,
}
enum Ip {
    IPv4(String),
    IPv6(String),
}
fn main() {
    let kind2 = Kind::MALE;
    // let kind2 = Kind::FEMALE;
    match kind2 {
       Kind::MALE => println!("{}","MALE") ,
       // Kind::FEMALE => println!("{}","FEMALE"),
       // default => println!("defautl")
       _ => println!("________"),

    }
    let kind = Kind::MALE;
    let kind2 = Kind::FEMALE;
    match kind {
        Kind::MALE => println!("{}","MALE"),
        _ => println!("=========="),
    }
    match kind2 {
        Kind::MALE => println!("{}","MALE"),
        _ => println!("=========="),
    }
    let ip4 = Ip::IPv4(String::from("127.0.0.1"));
    let ip6 = Ip::IPv6(String::from("::1"));
    match ip4 {
        Ip::IPv4(ip) => println!("ipv4:{}",ip),
        Ip::IPv6(ip) => println!("ipv6:{}",ip),
    }

    match ip6 {
        Ip::IPv4(ip) => println!("ipv4:{}",ip),
        Ip::IPv6(ip) => println!("ipv6:{}",ip),
    }

    let ip5 = Ip::IPv4(String::from("127.0.0.1"));
    let ip7 = Ip::IPv6(String::from("::1"));
    // if let enum Name 
    
     if let  Ip::IPv4(ip) = ip5 {
         println!("ipv4:{}",ip);
     }
     else if let Ip::IPv6(ip) = ip5{
         println!("ipv6:{}",ip);
     }
     if let  Ip::IPv4(ip) = ip7 {
         println!("ipv4:{}",ip);
     }else if let Ip::IPv6(ip) = ip7{
         println!("ipv6:{}",ip);
     }


}
