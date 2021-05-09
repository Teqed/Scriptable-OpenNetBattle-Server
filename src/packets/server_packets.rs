// Increment VERSION_ITERATION src/packets/mod.rs if packets are added or modified

use super::bytes::*;
use super::{VERSION_ID, VERSION_ITERATION};
use crate::net::{Asset, AssetData, BbsPost, Direction};

#[derive(Debug)]
pub enum ServerPacket<'a> {
  Pong {
    max_payload_size: usize,
  },
  Ack {
    reliability: u8,
    id: u64,
  },
  Login {
    ticket: String,
    warp_in: bool,
    spawn_x: f32,
    spawn_y: f32,
    spawn_z: f32,
    spawn_direction: Direction,
  },
  TransferWarp,
  TransferStart,
  TransferComplete {
    warp_in: bool,
    direction: Direction,
  },
  TransferServer {
    address: String,
    port: u16,
    data: String,
    warp_out: bool,
  },
  Kick {
    reason: String,
  },
  RemoveAsset {
    path: String,
  },
  AssetStreamStart {
    name: String,
    asset: &'a Asset,
  },
  AssetStream {
    data: &'a [u8],
  },
  Preload {
    asset_path: String,
  },
  MapUpdate {
    map_path: String,
  },
  PlaySound {
    path: String,
  },
  ExcludeObject {
    id: u32,
  },
  IncludeObject {
    id: u32,
  },
  MoveCamera {
    x: f32,
    y: f32,
    z: f32,
    hold_time: f32,
  },
  SlideCamera {
    x: f32,
    y: f32,
    z: f32,
    duration: f32,
  },
  UnlockCamera,
  LockInput,
  UnlockInput,
  Teleport {
    warp: bool,
    x: f32,
    y: f32,
    z: f32,
    direction: Direction,
  },
  Message {
    message: String,
    mug_texture_path: String,
    mug_animation_path: String,
  },
  Question {
    message: String,
    mug_texture_path: String,
    mug_animation_path: String,
  },
  Quiz {
    option_a: String,
    option_b: String,
    option_c: String,
    mug_texture_path: String,
    mug_animation_path: String,
  },
  OpenBoard {
    current_depth: u8,
    name: String,
    color: (u8, u8, u8),
    posts: &'a [BbsPost],
  },
  PrependPosts {
    current_depth: u8,
    reference: Option<String>,
    posts: &'a [BbsPost],
  },
  AppendPosts {
    current_depth: u8,
    reference: Option<String>,
    posts: &'a [BbsPost],
  },
  RemovePost {
    current_depth: u8,
    id: String,
  },
  PostSelectionAck,
  InitiatePvp {
    address: String,
  },
  ActorConnected {
    ticket: String,
    name: String,
    texture_path: String,
    animation_path: String,
    direction: Direction,
    x: f32,
    y: f32,
    z: f32,
    solid: bool,
    warp_in: bool,
  },
  ActorDisconnected {
    ticket: String,
    warp_out: bool,
  },
  ActorSetName {
    ticket: String,
    name: String,
  },
  ActorMove {
    ticket: String,
    x: f32,
    y: f32,
    z: f32,
    direction: Direction,
  },
  ActorSetAvatar {
    ticket: String,
    texture_path: String,
    animation_path: String,
  },
  ActorEmote {
    ticket: String,
    emote_id: u8,
  },
  ActorAnimate {
    ticket: String,
    state: String,
    loop_animation: bool,
  },
}

pub fn build_unreliable_packet(packet: &ServerPacket) -> Vec<u8> {
  let mut buf = build_packet(packet);
  buf.insert(0, 0);
  buf
}

pub(super) fn build_packet(packet: &ServerPacket) -> Vec<u8> {
  let mut buf = Vec::new();

  match packet {
    ServerPacket::Pong { max_payload_size } => {
      write_u16(&mut buf, 0);
      write_str(&mut buf, VERSION_ID);
      write_u64(&mut buf, VERSION_ITERATION);
      write_u16(&mut buf, *max_payload_size as u16);
    }
    ServerPacket::Ack { reliability, id } => {
      write_u16(&mut buf, 1);
      buf.push(*reliability);
      write_u64(&mut buf, *id);
    }
    ServerPacket::Login {
      ticket,
      warp_in,
      spawn_x,
      spawn_y,
      spawn_z,
      spawn_direction,
    } => {
      write_u16(&mut buf, 2);
      write_string(&mut buf, ticket);
      write_bool(&mut buf, *warp_in);
      write_f32(&mut buf, *spawn_x);
      write_f32(&mut buf, *spawn_y);
      write_f32(&mut buf, *spawn_z);
      buf.push(translate_direction(*spawn_direction));
    }
    ServerPacket::TransferWarp => {
      write_u16(&mut buf, 3);
    }
    ServerPacket::TransferStart => {
      write_u16(&mut buf, 4);
    }
    ServerPacket::TransferComplete { warp_in, direction } => {
      write_u16(&mut buf, 5);
      write_bool(&mut buf, *warp_in);
      buf.push(translate_direction(*direction));
    }
    ServerPacket::TransferServer {
      address,
      port,
      data,
      warp_out,
    } => {
      write_u16(&mut buf, 6);
      write_string(&mut buf, address);
      write_u16(&mut buf, *port);
      write_string(&mut buf, data);
      write_bool(&mut buf, *warp_out);
    }
    ServerPacket::Kick { reason } => {
      write_u16(&mut buf, 7);
      write_string(&mut buf, reason);
    }
    ServerPacket::RemoveAsset { path } => {
      write_u16(&mut buf, 8);
      write_string(&mut buf, path);
    }
    ServerPacket::AssetStreamStart { name, asset } => {
      write_u16(&mut buf, 9);
      write_string(&mut buf, name);
      write_u64(&mut buf, asset.last_modified);
      write_bool(&mut buf, asset.cachable);

      match asset.data {
        AssetData::Text(_) => {
          buf.push(0);
        }
        AssetData::Texture(_) => {
          buf.push(1);
        }
        AssetData::Audio(_) => {
          buf.push(2);
        }
      }

      write_u64(&mut buf, asset.len() as u64);
    }
    ServerPacket::AssetStream { data } => {
      write_u16(&mut buf, 10);
      write_u16(&mut buf, data.len() as u16);
      write_data(&mut buf, data);
    }
    ServerPacket::Preload { asset_path } => {
      write_u16(&mut buf, 11);
      write_string(&mut buf, asset_path);
    }
    ServerPacket::MapUpdate { map_path } => {
      write_u16(&mut buf, 12);
      write_string(&mut buf, map_path);
    }
    ServerPacket::PlaySound { path } => {
      write_u16(&mut buf, 13);
      write_string(&mut buf, path);
    }
    ServerPacket::ExcludeObject { id } => {
      write_u16(&mut buf, 14);
      write_u32(&mut buf, *id);
    }
    ServerPacket::IncludeObject { id } => {
      write_u16(&mut buf, 15);
      write_u32(&mut buf, *id);
    }
    ServerPacket::MoveCamera { x, y, z, hold_time } => {
      write_u16(&mut buf, 16);
      write_f32(&mut buf, *x);
      write_f32(&mut buf, *y);
      write_f32(&mut buf, *z);
      write_f32(&mut buf, *hold_time);
    }
    ServerPacket::SlideCamera { x, y, z, duration } => {
      write_u16(&mut buf, 17);
      write_f32(&mut buf, *x);
      write_f32(&mut buf, *y);
      write_f32(&mut buf, *z);
      write_f32(&mut buf, *duration);
    }
    ServerPacket::UnlockCamera => {
      write_u16(&mut buf, 18);
    }
    ServerPacket::LockInput => {
      write_u16(&mut buf, 19);
    }
    ServerPacket::UnlockInput => {
      write_u16(&mut buf, 20);
    }
    ServerPacket::Teleport {
      warp,
      x,
      y,
      z,
      direction,
    } => {
      write_u16(&mut buf, 21);
      write_bool(&mut buf, *warp);
      write_f32(&mut buf, *x);
      write_f32(&mut buf, *y);
      write_f32(&mut buf, *z);
      buf.push(translate_direction(*direction));
    }
    ServerPacket::Message {
      message,
      mug_texture_path,
      mug_animation_path,
    } => {
      write_u16(&mut buf, 22);
      write_string(&mut buf, message);
      write_string(&mut buf, mug_texture_path);
      write_string(&mut buf, mug_animation_path);
    }
    ServerPacket::Question {
      message,
      mug_texture_path,
      mug_animation_path,
    } => {
      write_u16(&mut buf, 23);
      write_string(&mut buf, message);
      write_string(&mut buf, mug_texture_path);
      write_string(&mut buf, mug_animation_path);
    }
    ServerPacket::Quiz {
      option_a,
      option_b,
      option_c,
      mug_texture_path,
      mug_animation_path,
    } => {
      write_u16(&mut buf, 24);
      write_string(&mut buf, option_a);
      write_string(&mut buf, option_b);
      write_string(&mut buf, option_c);
      write_string(&mut buf, mug_texture_path);
      write_string(&mut buf, mug_animation_path);
    }
    ServerPacket::OpenBoard {
      current_depth,
      name,
      color,
      posts,
    } => {
      write_u16(&mut buf, 25);
      buf.push(*current_depth);
      write_string(&mut buf, name);
      buf.push(color.0);
      buf.push(color.1);
      buf.push(color.2);

      write_u16(&mut buf, posts.len() as u16);

      for post in *posts {
        write_string(&mut buf, &post.id);
        write_bool(&mut buf, post.read);
        write_string(&mut buf, &post.title);
        write_string(&mut buf, &post.author);
      }
    }
    ServerPacket::PrependPosts {
      current_depth,
      reference,
      posts,
    } => {
      write_u16(&mut buf, 26);
      buf.push(*current_depth);
      write_bool(&mut buf, reference.is_some());

      if reference.is_some() {
        write_string(&mut buf, reference.as_ref().unwrap());
      }

      write_u16(&mut buf, posts.len() as u16);

      for post in *posts {
        write_string(&mut buf, &post.id);
        write_bool(&mut buf, post.read);
        write_string(&mut buf, &post.title);
        write_string(&mut buf, &post.author);
      }
    }
    ServerPacket::AppendPosts {
      current_depth,
      reference,
      posts,
    } => {
      write_u16(&mut buf, 27);
      buf.push(*current_depth);
      write_bool(&mut buf, reference.is_some());

      if reference.is_some() {
        write_string(&mut buf, reference.as_ref().unwrap());
      }

      write_u16(&mut buf, posts.len() as u16);

      for post in *posts {
        write_string(&mut buf, &post.id);
        write_bool(&mut buf, post.read);
        write_string(&mut buf, &post.title);
        write_string(&mut buf, &post.author);
      }
    }
    ServerPacket::RemovePost { current_depth, id } => {
      write_u16(&mut buf, 28);
      buf.push(*current_depth);
      write_string(&mut buf, id);
    }
    ServerPacket::PostSelectionAck => {
      write_u16(&mut buf, 29);
    }
    ServerPacket::InitiatePvp { address } => {
      write_u16(&mut buf, 30);
      write_string(&mut buf, &address);
    }
    ServerPacket::ActorConnected {
      ticket,
      name,
      texture_path,
      animation_path,
      direction,
      x,
      y,
      z,
      solid,
      warp_in,
    } => {
      write_u16(&mut buf, 31);
      write_string(&mut buf, ticket);
      write_string(&mut buf, name);
      write_string(&mut buf, texture_path);
      write_string(&mut buf, animation_path);
      buf.push(translate_direction(*direction));
      write_f32(&mut buf, *x);
      write_f32(&mut buf, *y);
      write_f32(&mut buf, *z);
      write_bool(&mut buf, *solid);
      write_bool(&mut buf, *warp_in);
    }
    ServerPacket::ActorDisconnected { ticket, warp_out } => {
      write_u16(&mut buf, 32);
      write_string(&mut buf, ticket);
      write_bool(&mut buf, *warp_out);
    }
    ServerPacket::ActorSetName { ticket, name } => {
      write_u16(&mut buf, 33);
      write_string(&mut buf, ticket);
      write_string(&mut buf, name);
    }
    ServerPacket::ActorMove {
      ticket,
      x,
      y,
      z,
      direction,
    } => {
      write_u16(&mut buf, 34);
      write_string(&mut buf, ticket);
      write_f32(&mut buf, *x);
      write_f32(&mut buf, *y);
      write_f32(&mut buf, *z);
      buf.push(translate_direction(*direction));
    }
    ServerPacket::ActorSetAvatar {
      ticket,
      texture_path,
      animation_path,
    } => {
      write_u16(&mut buf, 35);
      write_string(&mut buf, ticket);
      write_string(&mut buf, texture_path);
      write_string(&mut buf, animation_path);
    }
    ServerPacket::ActorEmote { ticket, emote_id } => {
      write_u16(&mut buf, 36);
      buf.push(*emote_id);
      write_string(&mut buf, ticket);
    }
    ServerPacket::ActorAnimate {
      ticket,
      state,
      loop_animation,
    } => {
      write_u16(&mut buf, 37);
      write_string(&mut buf, ticket);
      write_string(&mut buf, state);
      write_bool(&mut buf, *loop_animation)
    }
  }

  buf
}

pub fn create_asset_stream<'a>(
  max_payload_size: usize,
  name: &str,
  asset: &'a Asset,
) -> Vec<ServerPacket<'a>> {
  // reliability type + id + packet type + data size
  const HEADER_SIZE: usize = 1 + 8 + 2 + 2 + 16;

  let mut packets = vec![ServerPacket::AssetStreamStart {
    name: name.to_string(),
    asset,
  }];

  let mut bytes = match &asset.data {
    AssetData::Text(data) => data.as_bytes(),
    AssetData::Texture(data) => &data,
    AssetData::Audio(data) => &data,
  };

  let mut remaining_bytes = bytes.len();

  while remaining_bytes > 0 {
    let available_room = max_payload_size - HEADER_SIZE;
    let size = if remaining_bytes < available_room {
      remaining_bytes
    } else {
      available_room
    };

    packets.push(ServerPacket::AssetStream {
      data: &bytes[..size],
    });

    bytes = &bytes[size..];

    remaining_bytes -= size;
  }

  packets
}

fn translate_direction(direction: Direction) -> u8 {
  match direction {
    Direction::Up => 0x01,
    Direction::Left => 0x02,
    Direction::Down => 0x04,
    Direction::Right => 0x08,
    Direction::UpLeft => 0x10,
    Direction::UpRight => 0x20,
    Direction::DownLeft => 0x40,
    Direction::DownRight => 0x80,
    _ => 0x00,
  }
}
