use super::{flags::{
    QueryClasses,
    QueryTypes
}, query};

use num_traits::FromPrimitive;


#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Answer {
    name: String,
    query_type: QueryTypes,
    query_class: QueryClasses,
    ttl: u32,
    data_length: u16,
    pub data: Vec<u8>,
}

impl Answer {
    pub (super) fn from_bytes(
        bytes: &[u8; 512],
        start_index: usize,
        query_name: &str
    ) -> Self {
        let mut index = start_index;

        let query_type = QueryTypes::from_u16(
            u16::from_be_bytes(
                (&bytes[index..index + 2]).try_into().unwrap()
            )
        ).unwrap();
        index += 2;
        let query_class = QueryClasses::from_u16(
            u16::from_be_bytes(
                (&bytes[index..index + 2]).try_into().unwrap()
            )
        ).unwrap();
        index += 2;
        let ttl = u32::from_be_bytes(
            (&bytes[index..index + 4]).try_into().unwrap()
        );
        index += 4;
        let data_length = u16::from_be_bytes(
            (&bytes[index..index + 2]).try_into().unwrap()
        );
        index += 2;
        let data = bytes[index..index + data_length as usize].to_vec();

        let data = bytes[index..index + data_length as usize].to_vec();
        Self {
            name: query_name.to_string(),
            query_type,
            query_class,
            ttl,
            data_length,
            data
        }
    }

}