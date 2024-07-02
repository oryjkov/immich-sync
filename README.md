# immich-sync: Sync Google Photo content to Immich

The goal of this project is to mirror the content of one's Google Photo account in a personal Immich
server.

I wanted to find a way to rely on Google Photos as little as possible and keep all of my photos
locally. However most of my friends are using Google Photos to create shared albums of our
activities together and Immich is not really suitable for this. Hence I wanted tsharedo be able to
mirror GPhoto contents to my Immich server and keep it in sync. In particular, keep a copy of all
media items, either mine or part of an album that is shared with me, as well as all albums (shared
and non-shared) that exist in GPhotos, in Immich.

This program uses GPhoto API to get all of the media items and albums (again, shared and not
shared), then goes through all albums and tries to title-match albums that exist in Immich. This
mapping is then stored a local sqlite database (we map gphoto album id to immich album id, so
renaming an album in either photo server won't affect it). Any gphoto albums that do not exist in
immich are created there and we copy over all the associated media items.

Media items are mapped based on filename and metadata. This is not always unique, so it is possible
that wrong photos will be added to albums and/or some photos will not be reuploaded. It is possible
to tune this behaviour. However, for any media items that have been copied over, we preserve the
mapping (again based on unique ids) in the local storage.

Then the ideal usage of this program is to start with an empty immich server, the sync over all of
the content (this creates the mapping between all unique ids in both systems), then re-run the sync
periodically to transfer any new content over.

At the moment no items are deleted from Immich so any deletions in GPhotos will not be reflected in
immich.

### Notes

#### Album sync

1. Start with a google photo album
1. link it with an immich album
   1. if not found in immich, then create an immich album
   1. store the link in local db
1. for every item in the gphoto album link it with an item in immich
1. any items that were not linked (based on given strictness)
   1. copy the item to immich
   1. save the mapping between the two ids
   1. add the item to the album in step 2.

Q: Should we store mapping for items that we did not create in immich in the local DB?

Q: We could use album associations as an additional signal when linking media items.

#### Photo location

GPhoto API does not include photo location in EXIFs. Even for your own photos, or if the location
sharing is enabled on the photo/album.
[google issue](https://issuetracker.google.com/issues/80379228).

Takeout dump includes photo locations, so it is necessary to import the google takeout to have
location data on your photos.

As for shared photos - I don't know of an easy solution :(

#### API limits

Google photo library limits to 10k API reqests per day and 75k media downloads per day.

Using a different cloud project worked find and it appears that item and album ids are preserved
accross different projects.
