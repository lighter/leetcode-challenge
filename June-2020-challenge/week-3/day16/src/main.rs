fn main() {
    // println!("Hello, world!");
    let input = String::from("127.0.0.1");
    let result = Solution::valid_ip_address(input);
    println!("{:?}", result);
}

struct Solution {}

impl Solution {
    pub fn valid_ip_address(ip: String) -> String {
        let ipv6_str = "0123456789abcdef";
        let ip_str = ip.as_str();
        let find_result = ip_str.find(".");

        if find_result != None { // IPv4
            let ips: Vec<&str> = ip_str.split('.').collect();

            if ips.len() != 4 {
                return String::from("Neither");
            }
            for i in ips.iter(){
                if i.is_empty() {
                    return String::from("Neither");
                }

                if !(i.chars().all(char::is_numeric)) || (i.len() > 1 && i.chars().next().unwrap() == '0') {
                    return String::from("Neither");
                }

                let i_num = i.parse::<i32>();
                match i_num {
                    Ok(val) => {
                        if ips[0] == String::from("0") || val > 255 || val < 0 {
                            return String::from("Neither");
                        }                        
                    },
                    Err(_why) => {
                        return String::from("Neither");
                    }
                }
            }

            return String::from("IPv4");
        } else { // IPv6
            let ips: Vec<&str> = ip_str.split(':').collect();

            if ips.len() != 8 {
                return String::from("Neither");
            }

            for i in ips.iter() {
                let il = i.to_lowercase();

                if il.len() > 4 {
                    return String::from("Neither");                    
                }

                for ili in il.chars() {
                    if !ipv6_str.contains(ili) {
                        return String::from("Neither");
                    }
                }
            }

            return String::from("IPv6");
        }
    }

    // pub fn valid_ip_address(ip: String) -> String {

    //     let ipv6 = Regex::new(r"(([0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:|([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|:((:[0-9a-fA-F]{1,4}){1,7}|:)|fe80:(:[0-9a-fA-F]{0,4}){0,4}%[0-9a-zA-Z]{1,}|::(ffff(:0{1,4}){0,1}:){0,1}((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])|([0-9a-fA-F]{1,4}:){1,4}:((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9]))").unwrap();
    //     let ipv4 = Regex::new(r"((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])").unwrap();
        
    //     if ipv4.is_match(&ip) {
    //         return String::from("IPv4");
    //     } else if ipv6.is_match(&ip) {
    //         return String::from("IPv6");
    //     } else {
    //         return String::from("Neither");
    //     }
    // }
}
