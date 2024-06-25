sqlite-utils create-table sqlite.db media_items \
    rowid integer \
    filename text \
    mime_type text \
    metadata text \
    contributor text \
    description text \
    id text \
    local_file text \
    --not-null id \
    --not-null local_file \
    --pk=rowid \
    --strict

sqlite-utils create-table sqlite.db albums \
    rowid integer \
    title text \
    media_items_count integer \
    share_info text \
    id text \
    cover_photo_media_item_id text \
    --not-null id \
    --pk=rowid \
    --strict

sqlite-utils create-table sqlite.db album_items \
    album_id integer \
    media_item_id integer \
    --not-null album_id \
    --not-null media_item_id \
    --pk=album_id,media_item_id \
    --strict

sqlite-utils add-foreign-keys sqlite.db \
    album_items album_id albums rowid \
    album_items media_item_id media_items rowid
