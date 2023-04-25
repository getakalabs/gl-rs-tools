use actix_web::Result;
use mongodb::{bson::{doc, from_document, Bson, oid::ObjectId, Document, Regex}, options::AggregateOptions, Collection};
use serde::de::DeserializeOwned;
use std::str::FromStr;
use futures::{StreamExt, TryStreamExt};

use crate::traits::*;
use crate::Payload;

#[derive(Debug, Default, Clone)]
pub struct Pipeline {
    queries: Vec<Document>
}

impl Pipeline {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn build(&self) -> Vec<Document> {
        self.queries.clone()
    }

    pub async fn aggregate_result_many<T>(&self,  collection: &Collection<T>) -> Result<Vec<T>>
        where T: IsEmpty + DeserializeOwned + ToJson + Default
    {
        let  cursor = match collection.clone().aggregate(self.queries.to_owned(), AggregateOptions::builder().build()).await {
            Ok(cursor) => cursor,
            Err(error) => return Err(Payload::error(error))
        };

        let data = cursor.map_ok(|value| {
            from_document::<T>(value).unwrap_or_default().to_json().unwrap_or_default()
        }).try_collect::<Vec<T>>().await;

        match data {
            Ok(data) => Ok(data),
            Err(error) => Err(Payload::error(error))
        }
    }

    pub async fn aggregate_result_many_string<T, U>(&self,  collection: &Collection<T>, key: U) -> Result<Vec<String>>
        where T: IsEmpty + DeserializeOwned + ToJson + Default,
              U: ToString
    {
        let  cursor = match collection.clone().aggregate(self.queries.to_owned(), AggregateOptions::builder().build()).await {
            Ok(cursor) => cursor,
            Err(error) => return Err(Payload::error(error))
        };

        let data = cursor.map_ok(|value| {
            value.get(&key.to_string()).and_then(Bson::as_str).unwrap_or_default().to_owned()
        }).try_collect::<Vec<String>>().await;

        match data {
            Ok(data) => Ok(data),
            Err(error) => Err(Payload::error(error))
        }
    }

    pub async fn aggregate_result_many_i64<T, U>(&self,  collection: &Collection<T>, key: U) -> Result<Vec<i64>>
        where T: IsEmpty + DeserializeOwned + ToJson + Default,
              U: ToString
    {
        let  cursor = match collection.clone().aggregate(self.queries.to_owned(), AggregateOptions::builder().build()).await {
            Ok(cursor) => cursor,
            Err(error) => return Err(Payload::error(error))
        };

        let data = cursor.map_ok(|value| {
            value.get(&key.to_string()).and_then(Bson::as_i64).unwrap_or_default().to_owned()
        }).try_collect::<Vec<i64>>().await;

        match data {
            Ok(data) => Ok(data),
            Err(error) => Err(Payload::error(error))
        }
    }

    pub async fn aggregate_result_one<T>(&self,  collection: &Collection<T>) -> Result<T>
        where T: IsEmpty + DeserializeOwned,
    {
        let mut cursor = match collection.clone().aggregate(self.queries.to_owned(), None).await {
            Ok(cursor) => cursor,
            Err(error) => return Err(Payload::error(error))
        };

        if let Some(value) = cursor.next().await {
            match value {
                Ok(value) => match from_document::<T>(value) {
                    Ok(data) => if !data.is_empty() { return Ok(data); },
                    Err(error) => return Err(Payload::error(error))
                },
                Err(error) => return Err(Payload::error(error))
            }
        }

        Err(Payload::error("No matching record was found in the database. Please check your input and try again"))
    }

    pub fn custom(&mut self, value: Document) -> &mut Self {
        self.queries.push(value);

        self
    }

    pub fn filters(&mut self, value: &[Document]) -> &mut Self {
        let value = value.to_owned();

        if !value.is_empty() {
            self.queries.push(doc! { "$match": { "$and": value } });
        }

        self
    }

    pub fn global_search(&mut self, search: &[Document]) -> &mut Self {
        if !search.is_empty() {
            self.queries.push(doc! {
                "$match": {
                    "$or": search.to_owned()
                }
            });
        }

        self
    }

    pub fn graph_lookup<T, U, V, W>(
        &mut self,
        from: T,
        start_with: U,
        connect_from_field: V,
        connect_to_field: W,
        as_field: W,
        max_depth: usize
    ) -> &mut Self
        where T: ToString,
              U: ToString,
              V: ToString,
              W: ToString
    {
        self.queries.push(doc! {
            "$graphLookup": {
                "from": from.to_string(),
                "startWith": start_with.to_string(),
                "connectFromField": connect_from_field.to_string(),
                "connectToField": connect_to_field.to_string(),
                "as": as_field.to_string(),
                "maxDepth": max_depth as i32
            }
        });

        self
    }

    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.queries.push(doc! {
            "$limit": limit as i32
        });

        self
    }

    pub fn lookup<T, U, V, W>(&mut self, from: T, local_field: U, foreign_field: V, as_field: W) -> &mut Self
        where T: ToString,
              U: ToString,
              V: ToString,
              W: ToString
    {
        self.queries.push(doc! {
            "$lookup": {
                "from": from.to_string(),
                "localField": local_field.to_string(),
                "foreignField": foreign_field.to_string(),
                "as": as_field.to_string()
            }
        });

        self
    }

    pub fn match_and<T, U>(&mut self, fields: Vec<(T, U, bool)>, status: Option<Vec<String>>) -> &mut Self
        where T: ToString,
              U: ToString
    {
        let mut array = Vec::new();

        for (field, value, mut is_object_id) in fields {
            let field = field.to_string();
            let value = value.to_string();

            if field.as_str() == "_id" {
                is_object_id = true;
            }

            match is_object_id {
                true => match ObjectId::from_str(&value) {
                    Ok(object_id) => array.push(doc!{ field.clone(): object_id }),
                    Err(_) => array.push(doc!{ field.clone(): { "$exists": false } })
                },
                false => match value.trim().is_empty() {
                    false => match field.as_str() == "name" || field.as_str() == "parents.name" {
                        true => array.push(doc!{ field.clone(): { "$regex": value.to_lowercase().to_string(), "$options": "i" } }),
                        false => array.push(doc!{ field.clone(): value.to_string() })
                    },
                    true => array.push(doc!{ field.clone(): { "$exists": false } })
                }
            }
        }

        if let Some(status) = status {
            array.push(doc!{ "status": { "$in": status.iter().map(|s| s.to_string()).collect::<Vec<String>>() } });
        }

        if !array.is_empty() {
            self.queries.push(doc!{
                "$match": {
                    "$and": array
                }
            });
        }

        self
    }

    pub fn match_and_in<T, U>(&mut self, field:T, values: Vec<U>, is_object_id: bool, status: Option<Vec<String>>) -> &mut Self
        where T: ToString,
              U: ToString
    {
        let field = field.to_string();
        let mut array = Vec::new();

        match is_object_id {
            true => {
                let mut items = Vec::new();

                for i in values {
                    let item = i.to_string().trim().to_string();

                    if let Ok(value) = ObjectId::from_str(&item) {
                        items.push(value)
                    }
                }

                if !items.is_empty() {
                    array.push(doc!{ field: { "$in": items} })
                }
            },
            false => {
                let mut items = Vec::new();

                for i in values {
                    let item = i.to_string().trim().to_string();

                    items.push(Regex{ pattern: item, options: "i".to_string() })
                }

                if !items.is_empty() {
                    array.push(doc!{ field: { "$in": items} })
                }
            }
        };

        if let Some(status) = status {
            array.push(doc!{ "status": { "$in": status.iter().map(|s| s.to_string()).collect::<Vec<String>>() } });
        }

        if !array.is_empty() {
            self.queries.push(doc!{
                "$match": {
                    "$and": array
                }
            });
        }

        self
    }

    pub fn match_or<T, U>(&mut self, fields: Vec<(T, U, bool)>, status: Option<Vec<String>>) -> &mut Self
        where T: ToString,
              U: ToString
    {
        let mut array = Vec::new();

        for (field, value, mut is_object_id) in fields {
            let field = field.to_string();
            let value = value.to_string();

            if field.as_str() == "_id" {
                is_object_id = true;
            }

            match is_object_id {
                true => match ObjectId::from_str(&value) {
                    Ok(object_id) => array.push(doc!{ field.clone(): object_id }),
                    Err(_) => array.push(doc!{ field.clone(): { "$exists": false } })
                },
                false => match value.trim().is_empty() {
                    false => array.push(doc!{ field.clone(): value.to_string() }),
                    true => array.push(doc!{ field.clone(): { "$exists": false } })
                }
            }
        }

        if let Some(status) = status {
            array.push(doc!{ "status": { "$in": status.iter().map(|s| s.to_string()).collect::<Vec<String>>() } });
        }

        if !array.is_empty() {
            self.queries.push(doc!{
                "$match": {
                    "$or": array
                }
            });
        }

        self
    }

    pub fn projection(&mut self, projection: Document) -> &mut Self {
        self.queries.push(projection);

        self
    }

    pub fn replace_root<T>(&mut self, new_root: T) -> &mut Self
        where T: ToString
    {
        self.queries.push(doc! {
            "$replaceRoot": {
                "newRoot": new_root.to_string()
            }
        });

        self
    }

    pub fn slug<V, S>(&mut self, value: V, status: &[S]) -> &mut Self
        where V: ToString,
              S: ToString
    {
        let value = value.to_string().to_lowercase();

        let query = match ObjectId::from_str(&value) {
            Ok(oid) => doc! {
                "$match": {
                    "$and": [
                        { "_id": { "$eq": oid } },
                        { "status": { "$in": status.iter().map(|s| s.to_string()).collect::<Vec<String>>() } }
                    ]
                }
            },
            Err(_) => doc! {
                "$match": {
                    "$and": [
                        { "slug": { "$eq": value } },
                        { "status": { "$in": status.iter().map(|s| s.to_string()).collect::<Vec<String>>() } }
                    ]
                }
            }
        };

        self.queries.push(query);
        self
    }

    pub fn unwind<T>(&mut self, field: T, preserve_null: bool) -> &mut Self
        where T: ToString
    {
        self.queries.push(doc! {
            "$unwind": {
                "path": field.to_string(),
                "preserveNullAndEmptyArrays": preserve_null
            }
        });

        self
    }

    pub fn unwind_path<T>(&mut self, field: T) -> &mut Self
        where T: ToString
    {
        self.queries.push(doc! {
            "$unwind": {
                "path": field.to_string()
            }
        });

        self
    }

    pub fn role<T>(&mut self, role: &Option<T>) -> &mut Self
        where T: ToString
    {
        match role {
            Some(role) => {
                self.queries.push(doc! {
                    "$match": {
                        "$and": [
                            { "role": { "$eq": role.to_string() } },
                            { "role": { "$ne": "Controller" } }
                        ]
                    }
                });
            },
            None => {
                self.queries.push(doc! {
                    "$match": {
                        "role": { "$ne": "Controller" }
                    }
                });
            }
        }

        self
    }

    pub fn sort<T>(&mut self, field: T, order: i32) -> &mut Self
        where T: ToString
    {
        self.queries.push(doc! {
            "$sort": {
                field.to_string(): order
            }
        });

        self
    }

    pub fn status_in<T>(&mut self, statuses:&[T]) -> &mut Self
        where T: ToString
    {
        self.queries.push(doc! {
            "$match": {
                "status": {
                    "$in": statuses.iter().map(|s| Bson::String(s.to_string())).collect::<Vec<_>>()
                }
            }
        });

        self
    }

    pub fn table_facet(&mut self, current_page: &i32, per_page: &i32) -> &mut Self {
        self.queries.push(doc!{
            "$facet": {
                "list":[
                    { "$skip": (*current_page - 1) * *per_page },
                    { "$limit": *per_page }
                ],
                "count": [ { "$count": "total" } ]
            }
        });

        self
    }

    pub fn table_pagination(&mut self, current_page: &i32, per_page: &i32) -> &mut Self {
        self.queries.push(doc!{
             "$project": {
                "list": 1,
                "total": { "$arrayElemAt": ["$count.total", 0] },
                "current_page": { "$literal": *current_page },
                "per_page": { "$literal": *per_page },
                "pages": { "$toInt": { "$ceil": { "$divide": [ { "$arrayElemAt": [ "$count.total", 0 ] }, *per_page ] } } },
                "total_results":  { "$arrayElemAt": [ "$count.total", 0 ] }
            }
        });

        self
    }
}
