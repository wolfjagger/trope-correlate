use std::{
  fmt::Display,
  str::FromStr,
};

use derive_more;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Namespace {
  // Trope
  Main,
  // Media
  Advertising, Animation, Anime,
  Art, ARG, AudioGame, AudioPlay,
  Blog,
  ComicBook, ComicStrip, Creator,
  Fanfic, Film, Franchise,
  LARP, LetsPlay, LightNovel, Literature,
  Magazine, Manga, Manhua, Manhwa, Music, Myth,
  Pinball, PlaygroundSong, Podcast,
  Radio, Ride, Roleplay,
  Series,
  TabletopGame, Theatre, Toys,
  VideoGame, VisualNovel,
  WebAnimation, Webcomic, WebOriginal, Website,
  WebVideo, WesternAnimation, Wrestling,
  // Other
  UsefulNotes,
}
use Namespace::*;
pub static ALL_NAMESPACES: [Namespace; 45] = [
  // Trope
  Main,
  // Media
  Advertising, Animation, Anime,
  Art, ARG, AudioGame, AudioPlay,
  Blog,
  ComicBook, ComicStrip, Creator,
  Fanfic, Film, Franchise,
  LARP, LetsPlay, LightNovel, Literature,
  Magazine, Manga, Manhua, Manhwa, Music, Myth,
  Pinball, PlaygroundSong, Podcast,
  Radio, Ride, Roleplay,
  Series,
  TabletopGame, Theatre, Toys,
  VideoGame, VisualNovel,
  WebAnimation, Webcomic, WebOriginal, Website,
  WebVideo, WesternAnimation, Wrestling,
  // Other
  UsefulNotes,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
  Trope,
  Media,
  Other,
}

#[derive(Debug, derive_more::Display, derive_more::Error, PartialEq, Eq)]
pub struct NamespaceParseError;


impl Namespace {
  pub fn entity_type(&self) -> EntityType {
    match self {
      Self::Main => EntityType::Trope,
      Self::Advertising | Self::Animation | Self::Anime |
      Self::Art | Self::ARG | Self::AudioGame | Self::AudioPlay |
      Self::Blog |
      Self::ComicBook | Self::ComicStrip | Self::Creator |
      Self::Fanfic | Self::Film | Self::Franchise |
      Self::LARP | Self::LetsPlay | Self::LightNovel | Self::Literature |
      Self::Magazine | Self::Manga | Self::Manhua | Self::Manhwa | Self::Music | Self::Myth |
      Self::Pinball | Self::PlaygroundSong | Self::Podcast |
      Self::Radio | Self::Ride | Self::Roleplay |
      Self::Series |
      Self::TabletopGame | Self::Theatre | Self::Toys |
      Self::VideoGame | Self::VisualNovel |
      Self::WebAnimation | Self::Webcomic | Self::WebOriginal | Self::Website |
      Self::WebVideo | Self::WesternAnimation | Self::Wrestling => EntityType::Media,
      Self::UsefulNotes => EntityType::Other,
    }
  }
}


impl Display for Namespace {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(match self {
      Namespace::Main => "main",
      Namespace::Advertising => "advertising",
      Namespace::Animation => "animation",
      Namespace::Anime => "anime",
      Namespace::Art => "art",
      Namespace::ARG => "arg",
      Namespace::AudioGame => "audiogame",
      Namespace::AudioPlay => "audioplay",
      Namespace::Blog => "blog",
      Namespace::ComicBook => "comicbook",
      Namespace::ComicStrip => "comicstrip",
      Namespace::Creator => "creator",
      Namespace::Fanfic => "fanfic",
      Namespace::Film => "film",
      Namespace::Franchise => "franchise",
      Namespace::LARP => "larp",
      Namespace::LetsPlay => "letsplay",
      Namespace::LightNovel => "lightnovel",
      Namespace::Literature => "literature",
      Namespace::Magazine => "magazine",
      Namespace::Manga => "manga",
      Namespace::Manhua => "manhua",
      Namespace::Manhwa => "manhwa",
      Namespace::Music => "music",
      Namespace::Myth => "myth",
      Namespace::Pinball => "pinball",
      Namespace::PlaygroundSong => "PlaygroundSong",
      Namespace::Podcast => "podcast",
      Namespace::Radio => "radio",
      Namespace::Ride => "ride",
      Namespace::Roleplay => "roleplay",
      Namespace::Series => "series",
      Namespace::TabletopGame => "tabletopgame",
      Namespace::Theatre => "theatre",
      Namespace::Toys => "toys",
      Namespace::VideoGame => "videogame",
      Namespace::VisualNovel => "visualnovel",
      Namespace::WebAnimation => "webanimation",
      Namespace::Webcomic => "webcomic",
      Namespace::WebOriginal => "weboriginal",
      Namespace::Website => "website",
      Namespace::WebVideo => "webvideo",
      Namespace::WesternAnimation => "westernanimation",
      Namespace::Wrestling => "wrestling",
      Namespace::UsefulNotes => "usefulnotes",
    })
  }
}

impl FromStr for Namespace {
  type Err = NamespaceParseError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "main" => Ok(Namespace::Main),
      "advertising" => Ok(Namespace::Advertising),
      "animation" => Ok(Namespace::Animation),
      "anime" => Ok(Namespace::Anime),
      "art" => Ok(Namespace::Art),
      "arg" => Ok(Namespace::ARG),
      "audiogame" => Ok(Namespace::AudioGame),
      "audioplay" => Ok(Namespace::AudioPlay),
      "blog" => Ok(Namespace::Blog),
      "comicbook" => Ok(Namespace::ComicBook),
      "comicstrip" => Ok(Namespace::ComicStrip),
      "creator" => Ok(Namespace::Creator),
      "fanfic" => Ok(Namespace::Fanfic),
      "film" => Ok(Namespace::Film),
      "franchise" => Ok(Namespace::Franchise),
      "larp" => Ok(Namespace::LARP),
      "letsplay" => Ok(Namespace::LetsPlay),
      "lightnovel" => Ok(Namespace::LightNovel),
      "literature" => Ok(Namespace::Literature),
      "magazine" => Ok(Namespace::Magazine),
      "manga" => Ok(Namespace::Manga),
      "manhua" => Ok(Namespace::Manhua),
      "manhwa" => Ok(Namespace::Manhwa),
      "music" => Ok(Namespace::Music),
      "myth" => Ok(Namespace::Myth),
      "pinball" => Ok(Namespace::Pinball),
      "PlaygroundSong" => Ok(Namespace::PlaygroundSong),
      "podcast" => Ok(Namespace::Podcast),
      "radio" => Ok(Namespace::Radio),
      "ride" => Ok(Namespace::Ride),
      "roleplay" => Ok(Namespace::Roleplay),
      "series" => Ok(Namespace::Series),
      "tabletopgame" => Ok(Namespace::TabletopGame),
      "theatre" => Ok(Namespace::Theatre),
      "toys" => Ok(Namespace::Toys),
      "videogame" => Ok(Namespace::VideoGame),
      "visualnovel" => Ok(Namespace::VisualNovel),
      "webanimation" => Ok(Namespace::WebAnimation),
      "webcomic" => Ok(Namespace::Webcomic),
      "weboriginal" => Ok(Namespace::WebOriginal),
      "website" => Ok(Namespace::Website),
      "webvideo" => Ok(Namespace::WebVideo),
      "westernanimation" => Ok(Namespace::WesternAnimation),
      "wrestling" => Ok(Namespace::Wrestling),
      "usefulnotes" => Ok(Namespace::UsefulNotes),
      _ => Err(NamespaceParseError)
    }
  }
}


impl Display for EntityType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(match self {
      Self::Trope => "trope",
      Self::Media => "media",
      Self::Other => "other",
    })
  }
}

impl FromStr for EntityType {
  type Err = NamespaceParseError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "trope" => Ok(Self::Trope),
      "media" => Ok(Self::Media),
      "other" => Ok(Self::Other),
      _ => Err(NamespaceParseError)
    }
  }
}
