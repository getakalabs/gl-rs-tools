pub mod prelude;

use chrono::{DateTime as ChronoDateTime, Utc};
pub use mongodb::bson::{oid::ObjectId, DateTime as BsonDateTime};

use crate::{MongoArray, MongoObjectId, MongoDateTime, Payload};

pub trait Decrypt {
    fn decrypt(&self) -> Self;
}

pub trait Encrypt {
    fn encrypt(&self) -> Self;
}

pub trait GetArrayObjectId {
    fn get_array_object_id(&self) -> Option<Vec<MongoObjectId>>;
}

pub trait GetArrayString {
    fn get_array_string(&self) -> Option<Vec<String>>;
}

pub trait GetArrayValue<T: Clone + GetMongoObjectId + ToJson + ToBson> {
    fn get_array_value(&self) -> Option<Vec<T>> where T: Sized;
}

pub trait GetI32 {
    fn get_i32(&self) -> Option<i32>;
}

pub trait GetI64 {
    fn get_i64(&self) -> Option<i64>;
}

pub trait GetF64 {
    fn get_f64(&self) -> Option<f64>;
}

pub trait GetBool {
    fn get_bool(&self) -> Option<bool>;
}

pub trait GetMongoArray {
    fn get_mongo_array(&self) -> Option<MongoArray>;
}

pub trait GetMongoObjectId {
    fn get_mongo_object_id(&self) -> Option<MongoObjectId>;
}

pub trait GetMongoDateTime {
    fn get_mongo_date_time(&self) -> Option<MongoDateTime>;
}

pub trait GetObjectId {
    fn get_object_id(&self) -> Option<ObjectId>;
}

pub trait GetObjectIds {
    fn get_object_ids(&self) -> Option<Vec<ObjectId>>;
}

pub trait GetSwapValue<T: Clone + GetMongoObjectId + ToJson + ToBson> {
    fn get_swap_value(&self) -> Option<T> where T: Sized;
}

pub trait GetSwitchValue<T: Clone + PartialEq> {
    fn get_switch_value(&self) -> Option<T> where T: Sized;
}

pub trait GetString {
    fn get_string(&self) -> Option<Self> where Self: Sized;
}

pub trait GetStringId {
    fn get_string_id(&self) -> Option<String>;
}

pub trait GetStringIds {
    fn get_string_ids(&self) -> Option<Vec<String>>;
}

pub trait GetBsonDateTime {
    fn get_bson_date_time(&self) -> Option<BsonDateTime>;
}

pub trait GetChronoDateTime {
    fn get_chrono_date_time(&self) -> Option<ChronoDateTime<Utc>>;
}

pub trait GetStringDateTime {
    fn get_string_date_time(&self) -> Option<String>;
}

pub trait GetSelf<T: Clone + IsEmpty> {
    fn get_self(value: T) -> Option<T> where T: Sized {
        match value.is_empty() {
            true => None,
            false => Some(value),
        }
    }
}

pub trait IsEmpty: Clone + Default + PartialEq {
    fn is_empty(&self) -> bool {
        self.clone() == Self::default()
    }
}

pub trait Normalize {
    fn normalize(&self) -> Self;
}

pub trait SetToBsonDateTime {
    fn set_to_bson_date_time(&self) -> Self;
}

pub trait SetToChronoDateTime {
    fn set_to_chrono_date_time(&self) -> Self;
}

pub trait SetToI32 {
    fn set_to_i32(&self) -> Self;
}

pub trait SetToI64 {
    fn set_to_i64(&self) -> Self;
}

pub trait SetToF64 {
    fn set_to_f64(&self) -> Self;
}

pub trait SetToBool {
    fn set_to_bool(&self) -> Self;
}

pub trait SetToMongoArray {
    fn set_to_mongo_array(&self) -> Self;
}

pub trait SetToMongoObjectId {
    fn set_to_mongo_object_id(&self) -> Self;
}

pub trait SetToMongoDateTime {
    fn set_to_mongo_date_time(&self) -> Self;
}

pub trait SetToManager {
    fn set_to_manager(&self) -> Self;
}

pub trait SetToObjectId {
    fn set_to_object_id(&self) -> Self;
}

pub trait SetToStringDateTime {
    fn set_to_string_date_time(&self) -> Self;
}

pub trait SetToSwapValue {
    fn set_to_swap_value(&self) -> Self;
}

pub trait SetToString {
    fn set_to_string(&self) -> Self;
}

pub trait Src<T> {
    fn src(value: T) -> Self;
}

pub trait ToBson {
    fn to_bson(&self) -> Option<Self> where Self: Sized;
}

pub trait ToBsonDateTime {
    fn to_bson_date_time(&self) -> Option<BsonDateTime>;
}

pub trait ToChronoDateTime {
    fn to_chrono_date_time(&self) -> Option<ChronoDateTime<Utc>>;
}

pub trait ToJson {
    fn to_json(&self) -> Option<Self> where Self: Sized;
}

pub trait ToOptString {
    fn to_opt_string(&self) -> Option<String>;
}

pub trait ToPayload {
    fn to_payload(&self, code: u16) -> Payload;
}

