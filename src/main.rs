mod dns_query;

use dns_query::DnsQuery;

fn main() {

    let response = DnsQuery::normal_query(
        "www.google.com",
        "1.1.1.1:53"
    );

    println!("{:?}", response);


}





