extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate csv;
extern crate failure;
extern crate gm_types;
extern crate chrono;

use std::io;
use gm_types::events::Event;
use gm_types::content::Content;
use gm_types::content::room::Member;
use gm_types::messages::Message;
use std::collections::BTreeMap;
use chrono::*;

#[derive(Debug, Deserialize)]
pub struct EventJsonLine {
    pub event_id: String,
    pub room_id: String,
    pub meta: String,
    pub event: String
}
#[derive(Debug, Deserialize)]
pub struct Metadata {
    #[serde(default)]
    pub stream_ordering: i64
}
fn main() -> Result<(), failure::Error> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_reader(io::stdin());
    let mut bmap = BTreeMap::new();
    for rec in rdr.deserialize() {
        let rec: EventJsonLine = rec?;
        let meta: Metadata = match ::serde_json::from_str(&rec.meta) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("warning: failed to parse metadata {}: {}", rec.meta, e);
                continue;
            }
        };
        let event: Event = match ::serde_json::from_str(&rec.event.replace("\\\\", "\\")) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("warning: failed to parse event {}: {}", rec.event, e);
                continue;
            }
        };
        //println!("{:?}", event);
        if let Some(ts) = event.room_data.as_ref().map(|x| x.origin_server_ts) {
            bmap.insert(ts, event);
        }
    }
    let mut members = BTreeMap::new();
    for (_, evt) in bmap {
        if let Some(rd) = evt.room_data {
            let ts: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp((rd.origin_server_ts / 1000) as _, 0), Utc);
            let name = members.get(&rd.sender).and_then(|x: &Member| x.displayname.clone()).unwrap_or(rd.sender.clone());
            print!("{}\t", ts);
            match evt.content {
                Content::RoomCreate(_) => {
                    println!("--\t{} created the room (ID: {})]", name, rd.room.unwrap().id);
                },
                Content::RoomMember(rm) => {
                    if let Some(sd) = evt.state_data {
                        println!("--\t{} updated membership state for {}: {:?}", name, sd.state_key, rm);
                        members.insert(rd.sender.clone(), rm.clone());
                    }
                },
                Content::RoomTopic(t) => {
                    println!("--\t{} set the room topic to: {}", name, t.topic);
                },
                Content::RoomMessage(m) => {
                    match m {
                        Message::Text { body, .. } => println!("{}\t{}", name, body),
                        Message::Notice { body, .. } => println!("{}\t{}", name, body),
                        Message::Image { body, .. } => println!("--\t{} sent an image: '{}'", name, body),
                        Message::File { body, .. } => println!("--\t{} sent a file: '{}'", name, body),
                        Message::Emote { body } => println!("*\t{} {}", name, body),
                        m => println!("--\t{} sent message event {:?}", name, m)
                    }
                }
                u => {
                    println!("--\t{} sent event {:?}", name, u);
                }
            }
        }
    }
    Ok(())
}
