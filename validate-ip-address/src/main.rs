pub fn valid_ip_address(query_ip: String) -> String {
    let ipv4 = query_ip.split('.').collect::<Vec<_>>();
    let ipv6 = query_ip.split(':').collect::<Vec<_>>();

    if ipv4.len() == 4 && ipv4.into_iter().all(|c| valid_ip_v4(c)) {
        "IPv4"
    } else if ipv6.len() == 8 && ipv6.into_iter().all(|c| valid_ip_v6(c)) {
        "IPv6"
    } else {
        "Neither"
    }
    .to_string()
}

pub fn valid_ip_v4(ip: &str) -> bool {
    ip == "0" || (!ip.starts_with('0') && ip.len() < 4 && ip.parse::<i32>().unwrap_or(256) < 256)
}

pub fn valid_ip_v6(ip: &str) -> bool {
    ip.len() > 0 && ip.len() < 5 && ip.chars().all(|c| c.is_digit(16))
}


#[cfg(test)]
mod tests {
    use crate::valid_ip_address;

    #[test]
    fn test_01() {
        let query_ip = String::from("172.16.254.1");
        let r = String::from("IPv4");
        assert_eq!(valid_ip_address(query_ip), r);
    }


    #[test]
    fn test_02() {
        let query_ip = String::from("2001:0db8:85a3:0:0:8A2E:0370:7334");
        let r = String::from("IPv6");
        assert_eq!(valid_ip_address(query_ip), r);
    }

    #[test]
    fn test_03() {
        let query_ip = String::from("256.256.256.256");
        let r = String::from("Neither");
        assert_eq!(valid_ip_address(query_ip), r);
    }
}

fn main() {
    println!("Hello, world!");
}
