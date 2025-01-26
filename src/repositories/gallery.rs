use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::models::gallery::{Gallery, NewGallery, NewGalleryImage, UpdateGallery};
use crate::schema::galleries;
use crate::schema::gallery_images;

pub struct GalleryRepository;

impl GalleryRepository {
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Gallery> {
        galleries::table.find(id).get_result(c).await
    }

    pub async fn all(c: &mut AsyncPgConnection) -> QueryResult<Vec<Gallery>> {
        galleries::table.load(c).await
    }

    pub async fn create(
        c: &mut AsyncPgConnection,
        new_gallery: NewGallery,
    ) -> QueryResult<Gallery> {
        diesel::insert_into(galleries::table)
            .values(new_gallery)
            .get_result(c)
            .await
    }

    pub async fn update(
        c: &mut AsyncPgConnection,
        id: i32,
        gallery: UpdateGallery,
    ) -> QueryResult<Gallery> {
        diesel::update(galleries::table.find(id))
            .set(&gallery)
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(galleries::table.find(id)).execute(c).await
    }

    pub async fn add_images(
        c: &mut AsyncPgConnection,
        gallery_id: i32,
        image_ids: Vec<i32>,
    ) -> QueryResult<usize> {
        let gallery_images: Vec<NewGalleryImage> = image_ids
            .into_iter()
            .map(|image_id| NewGalleryImage {
                gallery_id,
                image_id,
            })
            .collect();

        diesel::insert_into(gallery_images::table)
            .values(gallery_images)
            .on_conflict_do_nothing()
            .execute(c)
            .await
    }

    pub async fn remove_images(
        c: &mut AsyncPgConnection,
        gallery_id: i32,
        image_ids: Vec<i32>,
    ) -> QueryResult<usize> {
        diesel::delete(
            gallery_images::table.filter(
                gallery_images::gallery_id
                    .eq(gallery_id)
                    .and(gallery_images::image_id.eq_any(image_ids)),
            ),
        )
        .execute(c)
        .await
    }
}
