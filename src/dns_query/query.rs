use super::{answer::Answer, flags::{
    Flags, QueryClasses, QueryTypes
}, MessageType, QueryType, RecursionAvailable, RecursionDesired, ResponseCode, Truncation};

use num_traits::FromPrimitive;
use std::{net::{Ipv4Addr, UdpSocket}, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct DnsQuery {
    pub identification: u16,
    pub flags: Flags,
    pub number_of_questions: u16,
    pub number_of_answer_records: u16,
    pub number_of_resource_records: u16,
    pub number_of_additional_records: u16,
    pub query_name: String,
    pub query_type: QueryTypes,
    pub query_class: QueryClasses,
    pub answer: Option<Answer>,
}

impl DnsQuery {
    pub fn from_bytes(bytes: &[u8; 512]) -> Self{
        let identification = u16::from_be_bytes(
            (&bytes[0..2]).try_into().unwrap()
        );
        let flags = Flags::from_bytes(
            (&bytes[2..4]).try_into().unwrap()
        );
        let number_of_questions = u16::from_be_bytes(
            (&bytes[4..6]).try_into().unwrap()
        );
        let number_of_answer_records = u16::from_be_bytes(
            (&bytes[6..8]).try_into().unwrap()
        );
        let number_of_resource_records = u16::from_be_bytes(
            (&bytes[8..10]).try_into().unwrap()
        );
        let number_of_additional_records = u16::from_be_bytes(
            (&bytes[10..12]).try_into().unwrap()
        );
        let mut query_name = String::new();
        let mut index = 12;
        loop {
            let length = bytes[index] as usize;
            if length == 0 {
                break;
            }
            query_name.push_str(
                std::str::from_utf8(
                    &bytes[index + 1..index + 1 + length]
                ).unwrap()
            );
            query_name.push('.');
            index += length + 1;
        }
        query_name.pop();
        index += 1;

        let query_type = QueryTypes::from_u16(
            u16::from_be_bytes(
                (&bytes[index..index + 2]).try_into().unwrap()
            )
        ).unwrap();
        let query_class = QueryClasses::from_u16(
            u16::from_be_bytes(
                (&bytes[index + 2..index + 4]).try_into().unwrap()
            )
        ).unwrap();
        let mut answer = None;
        if number_of_answer_records > 0 {
            answer = Some(Answer::from_bytes(bytes, index + 6, &query_name));
        }
        
        Self {
            identification,
            flags,
            number_of_questions,
            number_of_answer_records,
            number_of_resource_records,
            number_of_additional_records,
            query_name,
            query_type,
            query_class,
            answer
        }
    }
    pub fn to_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(
            &self.identification.to_be_bytes()
        );
        bytes.extend_from_slice(
            &self.flags.flags.to_be_bytes()
        );
        bytes.extend_from_slice(
            &self.number_of_questions.to_be_bytes()
        );
        bytes.extend_from_slice(
            &self.number_of_answer_records.to_be_bytes()
        );
        bytes.extend_from_slice(
            &self.number_of_resource_records.to_be_bytes()
        );
        bytes.extend_from_slice(
            &self.number_of_additional_records.to_be_bytes()
        );
        for part in self.query_name.split(".") {
            bytes.push(part.len() as u8);
            bytes.extend_from_slice(
                part.as_bytes()
            )
        };
        bytes.push(0);
        bytes.extend_from_slice(
            &(self.query_type as u16).to_be_bytes()
        );
        bytes.extend_from_slice(
            &(self.query_class as u16).to_be_bytes()
        );

        bytes
    }

    pub fn normal_query(
        query_name: &str,
        dns_address: &str
    ) -> Ipv4Addr {
        let mut flags = Flags::new();
        flags.set_message_type(MessageType::Query);
        flags.set_query_type(QueryType::StandardQuery);
        flags.set_truncation(Truncation::NotExceeds512);
        flags.set_recursion(RecursionDesired::Yes);
        flags.set_recursion_available(RecursionAvailable::Yes);
        flags.set_response_code(ResponseCode::NoError);
        
        let query = DnsQuery {
            identification: 567,
            flags,
            number_of_questions: 1,
            number_of_answer_records: 0,
            number_of_resource_records: 0,
            number_of_additional_records: 0,
            query_name: query_name.to_string(),
            query_type: QueryTypes::A,
            query_class: QueryClasses::IN,
            answer: None,
        };

        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).unwrap();
        socket.connect(dns_address).unwrap();
        let query_bytes = query.to_bytes();
        socket.send(&query_bytes).unwrap();

        let mut buffer = [0; 512];
        let (_, _) = socket.recv_from(&mut buffer).unwrap();
        let response = DnsQuery::from_bytes(&buffer);
        // to ip
        let mut ip = String::new();
        for byte in response.answer.unwrap().data {
            ip.push_str(&byte.to_string());
            ip.push('.');
        };
        ip.pop();
        Ipv4Addr::from_str(&ip).unwrap()



    }
}
