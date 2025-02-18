pub mod types {
    use derive_more::Display;

    #[derive(Hash, Clone, Debug, PartialEq, Eq, Display)]
    pub struct ImmichItemId(pub String);
    #[derive(Hash, Clone, Debug, PartialEq, Eq, Display)]
    pub struct GPhotoItemId(pub String);
    #[derive(Hash, Clone, Debug, PartialEq, Eq, Display)]
    pub struct ImmichAlbumId(pub String);
    #[derive(Hash, Clone, Debug, PartialEq, Eq, Display)]
    pub struct GPhotoAlbumId(pub String);
}

pub mod gpclient;
pub mod immich_client;
pub mod match_metadata;
