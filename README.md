# immich-sync: Sync Google Photo content to Immich

The goal of this project is to mirror the content of one's Google Photo account into a personal
Immich server.

I wanted to find a way to rely on Google Photos as little as possible and keep all of my photos
locally, however most of my friends are using Google Photos to create shared albums of our
activities together and Immich is not a suitable replacement for this. Hence I wanted a tool that
will mirror all the content accessible in my Google Photos account into immich. In particular this
tool mirrors:

- all albums:
  - private,
  - shared albums created by me,
  - and shared albums that I'm a member of,
- all media items (photos and videos) that are in those albums.

## Usage

1. Get a Cloud API OAuth client ID.
   [Instructions](https://github.com/NicholasDawson/ArchiverForGooglePhotos/blob/master/INSTRUCTIONS.md)
1. Run the program for the first tima and create the authentication token. The tool assumes that
   you're running locally and can access `localhost:8080`.
1. Create an Immich API key and place it in a `.env` file, like: `IMMICH_API_KEY=...`
1. Run in dry-run mode first to see what will be done.
1. Run it for reals. This may copy photos from google photos to immich, create albums in immich and
   add photos to albums in immich.

## Principles of operation

### Album sync flow

1. Start with a google photo album
1. link it with an immich album
   1. if not found in immich, then create an immich album
   1. store the link in local db
1. for every item in the gphoto album link it with an item in immich
1. any items that were not linked (based on given strictness)
   1. copy the item to immich
   1. save the mapping between the two ids
   1. add the item to the album in step 2.

This program uses GPhoto API to get all of the media items and albums (again, shared and not
shared), then goes through all albums and tries to title-match albums that exist in Immich. This
mapping is then stored a local sqlite database (we map gphoto album id to immich album id, so
renaming an album in either photo server won't affect it). Any gphoto albums that do not exist in
immich are created there and we copy over all the associated media items.

Media items are mapped based on filename and metadata. This is not always unique, so it is possible
that wrong photos will be added to albums and/or some photos will not be reuploaded. It is possible
to tune this behaviour. However, for any media items that have been copied over, we preserve the
mapping (again based on unique ids) in the local storage.

At the moment no items are deleted from Immich so any deletions in GPhotos will not be reflected in
immich.

### Syncing

#### Media items

Before copying items from Gphoto to Immich the tool tries to find if it is in Immich already. This
is done using filename search and then some match. At the moment only filenames that are not found
in Immich will be copied over.

Any media items that was copied to Immich by this tool gets recorded in an internal database so that
it does not need to be copied on future runs. We store mapping between the (persistent) gphotos item
id and immich id.

#### Albums

Albums are matched only by Title. The tool unicode-normalizes the names and strips trailing
whitespace before matching.

Once a match is done the corresponding unique ids (gphoto and immich) is recorded in the internal
db. (This is different from what is done when matching media items where we only record links when
we create a new item in Immich.)

### Notes

#### Photo location

GPhoto API does not include photo location in EXIFs. Even for your own photos, or if the location
sharing is enabled on the photo/album.
[google issue](https://issuetracker.google.com/issues/80379228).

Takeout dump includes photo locations, so it is necessary to import the google takeout to have
location data on your photos.

As for shared photos - I don't know of an easy solution :(

#### API limits

Google photo library limits to 10k API reqests per day and 75k media downloads per day (per client
id).

One could use different cloud project/client ids - it appears that item and album ids are preserved
accross different clients.
