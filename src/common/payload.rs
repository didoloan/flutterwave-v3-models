use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Payload<T: Serialize> {
    Plain(T),
    ToEncrypt(T),
}