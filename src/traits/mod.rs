pub mod prelude;

use chrono::{DateTime as ChronoDateTime, Utc};
use mongodb::bson::{oid::ObjectId, DateTime as BsonDateTime};

use crate::Payload;

pub trait Decrypt {
    fn decrypt(&self) -> Option<Self> where Self: Sized;
}

pub trait Dedup {
    fn dedup(&self) -> Self where Self: Sized;
}

pub trait Encrypt {
    fn encrypt(&self) -> Option<Self> where Self: Sized;
}

pub trait GetArrayObject<T:Clone + GetObjectId + ToJson + ToBson + IsEmpty + PartialEq + Default> {
    fn get_array_object(&self) -> Option<Vec<T>> where T: Sized;
}

pub trait GetArrayObjectId {
    fn get_array_object_id(&self) -> Vec<ObjectId>;
}

pub trait GetArrayString {
    fn get_array_string(&self) -> Option<Vec<String>>;
}

pub trait GetBool {
    fn get_bool(&self) -> Option<bool>;
}

pub trait GetDateTimeBson {
    fn get_date_time_bson(&self) -> Option<BsonDateTime>;
}

pub trait GetDateTimeChrono {
    fn get_date_time_chrono(&self) -> Option<ChronoDateTime<Utc>>;
}

pub trait GetF64 {
    fn get_f64(&self) -> Option<f64>;
}

pub trait GetI32 {
    fn get_i32(&self) -> Option<i32>;
}

pub trait GetObjectId {
    fn get_object_id(&self) -> Option<ObjectId>;
}

pub trait GetString {
    fn get_string(&self) -> Option<String>;
}

pub trait GetSwap<T:Clone + GetObjectId + ToJson + ToBson + IsEmpty + PartialEq + Default> {
    fn get_swap(&self) -> Option<T> where T: Sized;
}

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

pub trait MutateClear {
    fn mutate_clear(&self);
}

pub trait MutateUpdate {
    fn mutate_update(&self, form: &Self) where Self: Sized;
}

pub trait Normalize {
    fn normalize(&self) -> Self where Self: Sized;
}

pub trait SetToCipher {
    fn set_to_cipher(&self) -> Self where Self: Sized;
}

pub trait SetToDateTimeBson {
    fn set_to_date_time_bson(&self) -> Self where Self: Sized;
}

pub trait SetToDateTimeChrono {
    fn set_to_date_time_chrono(&self) -> Self where Self: Sized;
}

pub trait SetToI32 {
    fn set_to_i32(&self) -> Self where Self: Sized;
}

pub trait SetToObjectId {
    fn set_to_object_id(&self) -> Self where Self: Sized;
}

pub trait SetToString {
    fn set_to_string(&self) -> Self where Self: Sized;
}

pub trait ToBson {
    fn to_bson(&self) -> Option<Self> where Self: Sized;
}

pub trait ToJson {
    fn to_json(&self) -> Option<Self> where Self: Sized;
}

pub trait ToOption {
    fn to_option(&self) -> Option<Self> where Self: Sized;
}

pub trait ToPayload {
    fn to_payload(&self, code: usize) -> Payload;
}