#!/bin/bash
cat | sqlite3 "${1:-sqlite.db}" <<EOF
CREATE TABLE IF NOT EXISTS "album_album_links" (
   [gphoto_id] TEXT NOT NULL,
   [immich_id] TEXT NOT NULL,
   UNIQUE(gphoto_id)
   UNIQUE(immich_id)
   PRIMARY KEY (gphoto_id)
) STRICT;
CREATE TABLE [created_albums] (
   [immich_id] TEXT PRIMARY KEY NOT NULL,
   [creation_time] INTEGER
) STRICT;
CREATE TABLE IF NOT EXISTS "item_item_links" (
   [gphoto_id] TEXT NOT NULL,
   [immich_id] TEXT NOT NULL,
   UNIQUE(gphoto_id)
   UNIQUE(immich_id)
   PRIMARY KEY (gphoto_id)
) STRICT;
EOF
