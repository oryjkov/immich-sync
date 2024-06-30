sqlite-utils create-table sqlite.db media_items \
    filename text \
    mime_type text \
    metadata text \
    contributor text \
    description text \
    id text \
    local_file text \
    --not-null id \
    --not-null local_file \
    --pk=id \
    --strict

sqlite-utils create-table sqlite.db albums \
    title text \
    media_items_count integer \
    share_info text \
    is_shared integer \
    id text \
    cover_photo_media_item_id text \
    --not-null id \
    --pk=id \
    --strict

sqlite-utils create-table sqlite.db album_items \
    album_id text \
    media_item_id text \
    --not-null album_id \
    --not-null media_item_id \
    --pk=album_id,media_item_id \
    --strict

sqlite-utils add-foreign-keys sqlite.db \
    album_items album_id albums id \
    album_items media_item_id media_items id

# Links between gphoto albums and immich albums (either created by us or
# already existing).
sqlite-utils create-table sqlite.db album_album_links \
    gphoto_id text \
    immich_id text \
    --not-null gphoto_id \
    --not-null immich_id \
    --pk=gphoto_id,immich_id \
    --strict

sqlite-utils add-foreign-keys sqlite.db \
    album_album_links gphoto_id albums id

# Albums that we created
sqlite-utils create-table sqlite.db created_albums \
    immich_id text \
    creation_time integer \
    --not-null immich_id \
    --pk=immich_id \
    --strict

# Links between gphoto items and immich items that we uploaded.
sqlite-utils create-table sqlite.db item_item_links \
    gphoto_id text \
    immich_id text \
    --not-null gphoto_id \
    --not-null immich_id \
    --pk=gphoto_id,immich_id \
    --strict

sqlite-utils add-foreign-keys sqlite.db \
    item_item_links gphoto_id media_items id

CREATE TABLE IF NOT EXISTS "item_item_links" (
   [gphoto_id] TEXT NOT NULL REFERENCES [media_items]([id]),
   [immich_id] TEXT NOT NULL,
   PRIMARY KEY (gphoto_id, immich_id)
) STRICT;
