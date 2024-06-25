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
