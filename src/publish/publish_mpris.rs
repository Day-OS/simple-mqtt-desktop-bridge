use rumqttc::QoS;
use serde::Serialize;
use crate::{publish, topics};
use crate::publish::IPublishQuery;
use mpris::{Metadata, PlayerFinder};

#[derive(Serialize)]
struct MprisStatus{
    status: Status,
    music_info: MusicInfo
}

#[derive(Serialize)]
enum Status{
    Playing,
    Paused,
    Stopped
}

impl Status{
    fn from(status: mpris::PlaybackStatus) -> Self{
        match status {
            mpris::PlaybackStatus::Playing => Self::Playing,
            mpris::PlaybackStatus::Paused => Self::Paused,
            mpris::PlaybackStatus::Stopped => Self::Stopped,
        }
    }
}

#[derive(Serialize)]
struct MusicInfo{
    title: Option<String>,
    artists: Option<Vec<String>>,
    art_url: Option<String>
}

impl MusicInfo {
    fn from(metadata: Metadata)-> Self{
        Self{
            title: metadata.title().map(|a| a.to_string()),
            artists: metadata.artists().map(|vec| vec.iter().map(|a| a.to_string()).collect()),
            art_url: metadata.art_url().map(|a| a.to_string()),
        }
    }
}


publish!(UpdateMPRIS, topics::TOPIC_MPRIS.to_string(), QoS::AtLeastOnce, true, || -> Vec<u8> {
    let playerfinder = PlayerFinder::new();

    if let Err(e) = playerfinder {
        log::error!("{e}");
        return vec![]
    }
    let player = playerfinder.unwrap().find_active();

    if let Err(e) = player {
        log::error!("{e}");
        return vec![]
    }
    let player = player.unwrap();

    match player.get_playback_status() {
    Ok(status) => {
        match player.get_metadata() {
            Ok(metadata) => {
                let status = MprisStatus{ 
                    status: Status::from(status), 
                    music_info: MusicInfo::from(metadata)
                };
                serde_json::to_string(&status).unwrap().as_bytes().to_vec()
                
            },
            Err(e) => {log::error!("{e}"); return vec![]},
        }
    },
    Err(e) => {log::error!("{e}"); return vec![]}
    }
});
