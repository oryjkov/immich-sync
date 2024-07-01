cat | sqlite3 sqlite.db <<EOF
CREATE TABLE [media_items] (
   [filename] TEXT,
   [mime_type] TEXT,
   [metadata] TEXT,
   [contributor] TEXT,
   [description] TEXT,
   [id] TEXT PRIMARY KEY NOT NULL,
   [local_file] TEXT NOT NULL
) STRICT;
CREATE TABLE [albums] (
   [title] TEXT,
   [media_items_count] INTEGER,
   [share_info] TEXT,
   [is_shared] INTEGER,
   [id] TEXT PRIMARY KEY NOT NULL,
   [cover_photo_media_item_id] TEXT
) STRICT;
CREATE TABLE IF NOT EXISTS "album_items" (
   [album_id,media_item_id] INTEGER PRIMARY KEY,
   [album_id] TEXT NOT NULL REFERENCES [albums]([id]),
   [media_item_id] TEXT NOT NULL REFERENCES [media_items]([id])
) STRICT;
CREATE TABLE IF NOT EXISTS "album_album_links" (
   [gphoto_id] TEXT NOT NULL REFERENCES [albums]([id]),
   [immich_id] TEXT NOT NULL,
   UNIQUE(gphoto_id, immich_id)
   PRIMARY KEY (gphoto_id)
) STRICT;
CREATE TABLE [created_albums] (
   [immich_id] TEXT PRIMARY KEY NOT NULL,
   [creation_time] INTEGER
) STRICT;
CREATE TABLE IF NOT EXISTS "item_item_links" (
   [gphoto_id] TEXT NOT NULL REFERENCES [media_items]([id]),
   [immich_id] TEXT NOT NULL,
   UNIQUE(gphoto_id, immich_id)
   PRIMARY KEY (gphoto_id)
) STRICT;
EOF
