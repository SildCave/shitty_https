use num_traits::FromPrimitive;

use super::fields::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Flags {
    pub flags: u16
}

impl Flags {
    pub fn new() -> Self{
        Self {
            flags: 0
        }
    }

    pub fn from_bytes(bytes: &[u8; 2]) -> Self {
        let flags = u16::from_be_bytes(
            (&bytes[0..2]).try_into().unwrap()
        );
        Self {
            flags
        }
    }

    pub fn get_message_type(
        &self
    ) -> MessageType {
        match !self.flags >> 15 {
            0 => {
                MessageType::Response
            }
            1 => {
                MessageType::Query
            },
            _ => {
                unimplemented!()
            }
        }
    }

    pub fn set_message_type(
        &mut self,
        message_type: MessageType
    ) {
        match message_type {
            MessageType::Response => {
                self.flags |= 1 << 15
            },
            MessageType::Query => {
                self.flags &= !(1 << 15)
            }
        }
    }

    pub fn get_query_type(
        &self,
    ) -> QueryType {
        FromPrimitive::from_u16((self.flags << 1) >> 12).unwrap()

    }

    pub fn set_query_type(
        &mut self,
        query_type: QueryType
    ) {
        self.flags |= (query_type as u16) << 11_u16;
    }

    fn get_authoritative(
        &self,
    ) -> AuthoritativeAnswer {
        FromPrimitive::from_u16((self.flags << 5) >> 15).unwrap()
    }

    pub fn set_authoritative(
        &mut self,
        authoritative: AuthoritativeAnswer
    ) {
        match authoritative {
            AuthoritativeAnswer::Authoritative => {
                self.flags |= 1 << 10
            },
            AuthoritativeAnswer::NonAuthoritative => {
                self.flags &= !(1 << 10)
            }
        }
    }

    pub fn get_truncation(
        &self,
    ) -> Truncation {
        FromPrimitive::from_u16((self.flags << 6) >> 15).unwrap()
    }

    pub fn set_truncation(
        &mut self,
        truncation: Truncation
    ) {
        match truncation {
            Truncation::Exceeds512 => {
                self.flags |= 1 << 9
            },
            Truncation::NotExceeds512 => {
                self.flags &= !(1 << 9)
            }
        }
    }

    pub fn get_recursion(
        &self,
    ) -> RecursionDesired {
        FromPrimitive::from_u16((self.flags << 7) >> 15).unwrap()
    }

    pub fn set_recursion(
        &mut self,
        recursion: RecursionDesired
    ) {
        match recursion {
            RecursionDesired::Yes => {
                self.flags |= 1 << 8
            },
            RecursionDesired::No => {
                self.flags &= !(1 << 8)
            }
        }
    }

    pub fn get_recursion_available(
        &self,
    ) -> RecursionAvailable {
        FromPrimitive::from_u16((self.flags << 8) >> 15).unwrap()
    }

    pub fn set_recursion_available(
        &mut self,
        recursion_available: RecursionAvailable
    ) {
        match recursion_available {
            RecursionAvailable::Yes => {
                self.flags |= 1 << 7
            },
            RecursionAvailable::No => {
                self.flags &= !(1 << 7)
            }
        }
    }

    pub fn get_response_code(
        &self,
    ) -> ResponseCode {
        FromPrimitive::from_u16((self.flags << 12) >> 12).unwrap()
    }

    pub fn set_response_code(
        &mut self,
        response_code: ResponseCode
    ) {
        self.flags |= response_code as u16;
    }
}
