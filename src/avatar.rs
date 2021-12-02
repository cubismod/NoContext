// discord user avatar related functions
extern crate reqwest;
use serenity::{
    model::{
        id::UserId
    }
};
use color_thief::get_palette;
use std::io;
use std::io::{self, Write};
use std::path::Path;
use std::fs::File;
use std::io::ErrorKind;


mod avatar {
    /**
     * Determines the primary color of an avatar by downloading it
     * (or using an already downloaded avatar) then returns the color
     * as an unsigned value
     */
    pub async fn parse_colors(url: String, id: UserId) -> u32 {
        let pixels = get_pixels(url, id);
    }

    /**
     * returns the raw image data from an avatar
     */
    async fn get_pixels(url: String, id: UserId) -> &[u8] {
        let path = Path::new(format!("download/{}"), id.0);
        
        let f = match File::open(path) {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => 
            }
        }
        
        
        if !path.exists() {
            // the avatar.face() func seems to return 1024x1024 avatars
            // however we want a much smaller image so we do a replace to get the
            // 32x32 version
            let smol_url = url.replace("1024", "32")
            // file doesn't exist so we need to download it
            let mut resp = reqwest::get(smol_url).await?;
        }
    }

    async fn download_avatar(url: String, id: UserId) -> Result<Response> {
        
    }

}