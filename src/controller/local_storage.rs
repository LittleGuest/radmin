use std::fs::File;
use std::io::prelude::*;

use actix_multipart::Multipart;
use actix_web::{Error, HttpResponse, Responder, web};
use chrono::NaiveDateTime;
use futures::{StreamExt, TryStreamExt};
use rbatis::core::value::DateTimeNow;

use crate::commons::{LOCAL_FILE_PATH, RespBuilder};
use crate::dtos::local_storage::ToolLocalStorageQuery;
use crate::models::ToolLocalStorage;

pub struct LocalStorageController;

impl LocalStorageController {
    pub async fn page(query: web::Query<ToolLocalStorageQuery>) -> impl Responder {
        let data = ToolLocalStorage::page(query.0).await.unwrap_or_default();
        RespBuilder::ok().with_data(data).build()
    }

    pub async fn upload(
        query: web::Query<ToolLocalStorageQuery>,
        mut payload: Multipart,
    ) -> Result<HttpResponse, Error> {
        let _ = std::fs::create_dir_all(LOCAL_FILE_PATH);

        // iterate over multipart stream
        while let Ok(Some(mut field)) = payload.try_next().await {
            let content_type = field.content_disposition().unwrap();
            let filename;
            let real_filename = content_type.get_filename().unwrap().to_string();
            // let suffix = String::from_utf8(content_type.get_filename_ext().cloned().unwrap().value)
            //     .unwrap_or("".to_string());
            // TODO 文件后缀
            let suffix = "".to_string();

            if let Some(nfn) = &query.0.name {
                filename = nfn.to_string();
            } else {
                filename = real_filename.clone();
            }
            let filepath = format!(
                "{}{}",
                LOCAL_FILE_PATH,
                sanitize_filename::sanitize(&real_filename)
            );

            // File::create is blocking operation, use threadpool
            let fp = filepath.clone();
            let mut f = web::block(|| File::create(fp)).await?;

            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.next().await {
                let data = chunk?;
                // filesystem operations are blocking, we have to use threadpool
                f = web::block(move || f.write_all(&data).map(|_| f)).await?;
            }

            ToolLocalStorage::save(ToolLocalStorage {
                real_name: Some(real_filename.clone()),
                name: Some(filename),
                suffix: Some(suffix),
                path: Some(filepath.clone()),
                size: Some(f.metadata().unwrap().len().to_string()),
                ..Default::default()
            })
                .await;
        }
        // TODO
        Ok(HttpResponse::Created().into())
    }

    pub async fn delete_batch_by_ids(form: web::Path<Vec<i64>>) -> impl Responder {
        ToolLocalStorage::delete_batch_by_ids(form.0).await;
        RespBuilder::<&str>::ok().build()
    }
}
