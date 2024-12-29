CREATE TABLE gallery_images (
  gallery_id INTEGER NOT NULL REFERENCES galleries(id),
  image_id INTEGER NOT NULL REFERENCES images(id),
  PRIMARY KEY (gallery_id, image_id)
);
