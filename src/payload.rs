use serde::Serialize;

#[derive(Debug)]
pub enum Payload<T: Serialize> {
    Plain(T),
    ToEncrypt(T),
}