# immich-sync: Sync Google Photo content to Immich

## Project Goal

The goal of this project is to mirror the content of a Google Photos account into a personal Immich
server. This tool is designed to:

- Reduce dependency on Google Photos by keeping all photos locally.
- Mirror shared albums from Google Photos to Immich.

Specifically, this tool mirrors:

- All albums:
  - Private
  - Shared albums created by the user
  - Shared albums where the user is a member
- All media items (photos and videos) in those albums.

## Usage

1. **Get a Cloud API OAuth Client ID**

   - Follow the
     [instructions](https://github.com/NicholasDawson/ArchiverForGooglePhotos/blob/master/INSTRUCTIONS.md)
     to obtain a Cloud API OAuth client ID.

1. **Run the Program for the First Time**

   - Execute the program to create the authentication token. Ensure that you are running locally and
     can access `localhost:8080`.

1. **Create an Immich API Key**

   - Generate an API key in Immich.
   - Place it in a `.env` file in the following format:
     ```plaintext
     IMMICH_API_KEY=your_api_key_here
     ```

1. **Dry-Run Mode**

   - Run the program in dry-run mode to see what actions will be performed without making any
     changes.

1. **Execute the Program**

   - Run the program to actually sync photos from Google Photos to Immich, create albums, and add
     photos to albums in Immich.

1. **Run the import periodically**

   - I've set up a daily import job to copy over all shared albums.

## Principles of Operation

### Album Sync Flow

1. Start with a Google Photos album.
1. Link it with an Immich album:
   - If not found in Immich, create an Immich album.
   - Store the album-album link in the local database.
1. For every item in the Google Photos album, try to link it with an item in Immich.
1. For any items not linked (based on given strictness):
   - Copy the item to Immich.
   - Save the mapping between the two IDs.
   - Add the item to the album created in step 2.

### Syncing

#### Media Items

Before copying items from Google Photos to Immich, the tool checks if they are already in Immich
using filename search and other matching criteria. Only filenames not found in Immich will be copied
over.

Any media item copied to Immich by this tool is recorded in an internal database to avoid
duplication in future runs. The tool stores a mapping between the persistent Google Photos item ID
and Immich ID.

#### Albums

Albums are matched only by title. The tool normalizes the names and strips trailing whitespace
before matching. Once a match is found, the corresponding unique IDs (Google Photos and Immich) are
recorded in the internal database.

### Notes

#### Photo Location

The Google Photos API does not include photo location in EXIFs. This applies even to your own photos
or if location sharing is enabled on the photo/album. More information can be found in this
[Google issue](https://issuetracker.google.com/issues/80379228).

To include photo locations, it is necessary to import Google Takeout data. Unfortunately, there is
no easy solution for shared photos.

#### API Limits

Google Photo Library imposes limits of 10,000 API requests per day and 75,000 media downloads per
day (per client ID). Using different cloud project/client IDs may help, as item and album IDs are
preserved across different clients.
