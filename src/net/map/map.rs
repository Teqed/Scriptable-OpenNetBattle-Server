use super::super::{Asset, Direction};
use super::map_layer::MapLayer;
use super::map_object::{MapObject, MapObjectData};
use super::Tile;
use crate::helpers::unwrap_and_parse_or_default;

#[derive(Clone)]
pub struct TilesetInfo {
  pub first_gid: u32,
  pub path: String,
}

#[derive(Clone)]
pub struct Map {
  name: String,
  background_name: String,
  background_texture_path: String,
  background_animation_path: String,
  background_vel_x: f32,
  background_vel_y: f32,
  song_path: String,
  width: usize,
  height: usize,
  tile_width: u32,
  tile_height: u32,
  spawn_x: f32,
  spawn_y: f32,
  spawn_z: f32,
  spawn_direction: Direction,
  tilesets: Vec<TilesetInfo>,
  layers: Vec<MapLayer>,
  next_layer_id: u32,
  objects: Vec<MapObject>,
  next_object_id: u32,
  asset_stale: bool,
  cached: bool,
  cached_string: String,
}

impl Map {
  pub fn from(text: String) -> Map {
    let mut map = Map {
      name: String::new(),
      background_name: String::new(),
      background_texture_path: String::new(),
      background_animation_path: String::new(),
      background_vel_x: 0.0,
      background_vel_y: 0.0,
      song_path: String::new(),
      width: 0,
      height: 0,
      tile_width: 0,
      tile_height: 0,
      spawn_x: 0.0,
      spawn_y: 0.0,
      spawn_z: 0.0,
      spawn_direction: Direction::None,
      tilesets: Vec::new(),
      layers: Vec::new(),
      next_layer_id: 0,
      objects: Vec::new(),
      next_object_id: 0,
      asset_stale: true,
      cached: false,
      cached_string: String::from(""),
    };

    let map_element: minidom::Element = text.parse().expect("Invalid Tiled map file");

    map.width = unwrap_and_parse_or_default(map_element.attr("width"));
    map.height = unwrap_and_parse_or_default(map_element.attr("height"));
    map.tile_width = unwrap_and_parse_or_default(map_element.attr("tilewidth"));
    map.tile_height = unwrap_and_parse_or_default(map_element.attr("tileheight"));

    map.next_layer_id = unwrap_and_parse_or_default(map_element.attr("nextlayerid"));
    map.next_object_id = unwrap_and_parse_or_default(map_element.attr("nextobjectid"));

    let scale_x = 1.0 / (map.tile_width as f32 / 2.0);
    let scale_y = 1.0 / map.tile_height as f32;

    let mut object_layers = 0;

    for child in map_element.children() {
      match child.name() {
        "properties" => {
          for property in child.children() {
            let name = property.attr("name").unwrap_or_default();
            let value = property.attr("value").unwrap_or_default().to_string();

            match name {
              "Name" => {
                map.name = value;
              }
              "Background" => {
                map.background_name = value;
              }
              "Background Texture" => {
                map.background_texture_path = value;
              }
              "Background Animation" => {
                map.background_animation_path = value;
              }
              "Background Vel X" => {
                map.background_vel_x = value.parse().unwrap_or_default();
              }
              "Background Vel Y" => {
                map.background_vel_y = value.parse().unwrap_or_default();
              }
              "Song" => {
                map.song_path = value;
              }
              _ => {}
            }
          }
        }
        "tileset" => {
          let first_gid: u32 = unwrap_and_parse_or_default(child.attr("firstgid"));
          let mut path = child.attr("source").unwrap_or_default().to_string();

          const ASSETS_RELATIVE_PATH: &str = "../assets/";

          if path.starts_with(ASSETS_RELATIVE_PATH) {
            path = String::from("/server/assets/") + &path[ASSETS_RELATIVE_PATH.len()..];
          }

          map.tilesets.push(TilesetInfo { first_gid, path });
        }
        "layer" => {
          let id: u32 = unwrap_and_parse_or_default(child.attr("id"));
          let name: String = child.attr("name").unwrap_or_default().to_string();

          // map name might be missing if the file wasn't generated
          Map::test_offsets_and_warn_for_layer(
            map.name.as_str(),
            name.as_str(),
            map.layers.len(),
            child,
          );

          // actual handling
          let data: Vec<u32> = child
            .get_child("data", minidom::NSChoice::Any)
            .unwrap()
            .text()
            .split(',')
            .map(|value| value.trim().parse().unwrap_or_default())
            .collect();

          let mut layer = MapLayer::new(id, name, map.width, map.height, data);

          let visible = child.attr("visible").unwrap_or_default() != "0";
          layer.set_visible(visible);

          map.layers.push(layer);
        }
        "objectgroup" => {
          let name: &str = child.attr("name").unwrap_or_default();

          // map name might be missing if the file wasn't generated
          Map::test_offsets_and_warn_for_layer(map.name.as_str(), name, object_layers, child);

          if object_layers + 1 != map.layers.len() {
            println!("{}: Layer \"{}\" will link to layer {}! (Layer order starting from bottom is Tile, Object, Tile, Object, etc)", map.name, name, object_layers);
          }

          for object_element in child.children() {
            let map_object = MapObject::from(object_element, object_layers, scale_x, scale_y);

            if map_object.object_type == "Home Warp" {
              map.spawn_x = map_object.x + map_object.height / 2.0;
              map.spawn_y = map_object.y + map_object.height / 2.0;
              map.spawn_z = object_layers as f32;

              let direction_string = map_object
                .custom_properties
                .get("Direction")
                .map(|string| string.as_str())
                .unwrap_or_default();

              map.spawn_direction = Direction::from(direction_string);

              // make sure direction is set if the spawn is on a home warp
              // otherwise the player will immediately warp out
              if matches!(map.spawn_direction, Direction::None) {
                map.spawn_direction = Direction::UpRight;
              }
            }

            map.objects.push(map_object);
          }

          object_layers += 1;
        }
        _ => {}
      }
    }

    map
  }

  fn test_offsets_and_warn_for_layer(
    map_name: &str,
    layer_name: &str,
    layer_index: usize,
    layer_element: &minidom::Element,
  ) {
    // warnings
    let manual_horizontal_offset: i32 = unwrap_and_parse_or_default(layer_element.attr("offsetx"));
    let manual_vertical_offset: i32 = unwrap_and_parse_or_default(layer_element.attr("offsety"));
    let correct_vertical_offset = layer_index as i32 * -16;

    if manual_horizontal_offset != 0 {
      println!(
        "{}: Layer \"{}\" has incorrect horizontal offset! (Should be 0)",
        map_name, layer_name
      );
    }

    if manual_vertical_offset != correct_vertical_offset {
      println!(
        "{}: Layer \"{}\" has incorrect vertical offset! (Should be {})",
        map_name, layer_name, correct_vertical_offset
      );
    }
  }

  pub fn get_tilesets(&self) -> &Vec<TilesetInfo> {
    &self.tilesets
  }

  pub fn get_name(&self) -> &String {
    &self.name
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
    self.cached = false;
  }

  pub fn get_song_path(&self) -> &String {
    &self.song_path
  }

  pub fn set_song_path(&mut self, path: String) {
    self.song_path = path;
    self.cached = false;
  }

  pub fn get_background_name(&self) -> &String {
    &self.background_name
  }

  pub fn set_background_name(&mut self, name: String) {
    self.background_name = name;
    self.cached = false;
  }

  pub fn get_custom_background_texture_path(&self) -> &String {
    &self.background_texture_path
  }

  pub fn set_custom_background_texture_path(&mut self, path: String) {
    self.background_texture_path = path;
    self.cached = false;
  }

  pub fn get_custom_background_animation_path(&self) -> &String {
    &self.background_animation_path
  }

  pub fn set_custom_background_animation_path(&mut self, path: String) {
    self.background_animation_path = path;
    self.cached = false;
  }

  pub fn get_custom_background_velocity(&self) -> (f32, f32) {
    (self.background_vel_x, self.background_vel_y)
  }

  pub fn set_custom_background_velocity(&mut self, x: f32, y: f32) {
    self.background_vel_x = x;
    self.background_vel_y = y;
    self.cached = false;
  }

  pub fn get_width(&self) -> usize {
    self.width
  }

  pub fn get_height(&self) -> usize {
    self.height
  }

  pub fn get_tile_width(&self) -> u32 {
    self.tile_width
  }

  pub fn get_tile_height(&self) -> u32 {
    self.tile_height
  }

  pub fn get_spawn(&self) -> (f32, f32, f32) {
    (self.spawn_x, self.spawn_y, self.spawn_z)
  }

  pub fn set_spawn(&mut self, x: f32, y: f32, z: f32) {
    self.spawn_x = x;
    self.spawn_y = y;
    self.spawn_z = z;
  }

  pub fn get_spawn_direction(&self) -> Direction {
    self.spawn_direction
  }

  pub fn set_spawn_direction(&mut self, direction: Direction) {
    self.spawn_direction = direction;
  }

  pub fn get_tile(&self, x: usize, y: usize, z: usize) -> Tile {
    if self.layers.len() <= z || self.width <= x || self.height <= y {
      Tile::default()
    } else {
      self.layers[z].get_tile(x, y)
    }
  }

  pub fn set_tile(&mut self, x: usize, y: usize, z: usize, tile: Tile) {
    // todo: expand world instead of rejecting
    if self.width <= x || self.height <= y || self.layers.len() <= z {
      return;
    }
    let layer: &mut MapLayer = &mut self.layers[z];

    if layer.get_tile(x, y) != tile {
      layer.set_tile(x, y, tile);
      self.cached = false;
    }
  }

  pub fn get_objects(&self) -> &Vec<MapObject> {
    &self.objects
  }

  pub fn get_object_by_id(&self, id: u32) -> Option<&MapObject> {
    self.objects.iter().find(|&o| o.id == id)
  }

  pub fn get_object_by_name(&self, name: &str) -> Option<&MapObject> {
    self.objects.iter().find(|&o| o.name == name)
  }

  #[allow(clippy::too_many_arguments)]
  pub fn create_object(
    &mut self,
    name: String,
    object_type: String,
    x: f32,
    y: f32,
    layer: usize,
    width: f32,
    height: f32,
    rotation: f32,
    data: MapObjectData,
  ) -> u32 {
    use std::collections::HashMap;

    let id = self.next_object_id;
    let map_object = MapObject {
      id,
      name,
      object_type,
      x,
      y,
      visible: true,
      layer,
      width,
      height,
      rotation,
      data,
      custom_properties: HashMap::new(),
    };

    self.objects.push(map_object);

    self.next_object_id += 1;
    self.cached = false;

    id
  }

  pub fn remove_object(&mut self, id: u32) {
    if let Some(index) = self.objects.iter().position(|object| object.id == id) {
      self.objects.remove(index);

      self.cached = false;
    }
  }

  pub fn set_object_name(&mut self, id: u32, name: String) {
    if let Some(object) = self.objects.iter_mut().find(|object| object.id == id) {
      object.name = name;

      self.cached = false;
    }
  }

  pub fn set_object_type(&mut self, id: u32, object_type: String) {
    if let Some(object) = self.objects.iter_mut().find(|object| object.id == id) {
      object.object_type = object_type;

      self.cached = false;
    }
  }

  pub fn set_object_custom_property(&mut self, id: u32, name: String, value: String) {
    if let Some(object) = self.objects.iter_mut().find(|object| object.id == id) {
      object.custom_properties.insert(name, value);

      self.cached = false;
    }
  }

  pub fn resize_object(&mut self, id: u32, width: f32, height: f32) {
    if let Some(object) = self.objects.iter_mut().find(|object| object.id == id) {
      if matches!(object.data, MapObjectData::Point) {
        // cant resize a point
        return;
      }

      object.width = width;
      object.height = height;

      self.cached = false;
    }
  }

  pub fn set_object_rotation(&mut self, id: u32, rotation: f32) {
    if let Some(object) = self.objects.iter_mut().find(|object| object.id == id) {
      object.rotation = rotation;

      self.cached = false;
    }
  }

  pub fn set_object_visibility(&mut self, id: u32, visibility: bool) {
    if let Some(object) = self.objects.iter_mut().find(|object| object.id == id) {
      object.visible = visibility;

      self.cached = false;
    }
  }

  pub fn move_object(&mut self, id: u32, x: f32, y: f32, layer: usize) {
    if let Some(object) = self.objects.iter_mut().find(|object| object.id == id) {
      object.x = x;
      object.y = y;
      object.layer = layer;

      self.cached = false;
    }
  }

  pub fn render(&mut self) -> String {
    if !self.cached {
      let mut text = vec![format!(
        "\
        <?xml version=\"1.0\" encoding=\"UTF-8\"?>\
          <map version=\"1.4\" tiledversion=\"1.4.1\" orientation=\"isometric\" \
               renderorder=\"right-down\" compressionlevel=\"0\" \
               width=\"{}\" height=\"{}\" tilewidth=\"{}\" tileheight=\"{}\" \
               infinite=\"0\" nextlayerid=\"{}\" nextobjectid=\"{}\">\
            <properties>\
              <property name=\"Name\" value=\"{}\"/>\
              <property name=\"Background\" value=\"{}\"/>\
              <property name=\"Background Texture\" value=\"{}\"/>\
              <property name=\"Background Animation\" value=\"{}\"/>\
              <property name=\"Background Vel X\" value=\"{}\"/>\
              <property name=\"Background Vel Y\" value=\"{}\"/>\
              <property name=\"Song\" value=\"{}\"/>\
            </properties>\
        ",
        self.width,
        self.height,
        self.tile_width,
        self.tile_height,
        self.next_layer_id,
        self.next_object_id,
        self.name,
        self.background_name,
        self.background_texture_path,
        self.background_animation_path,
        self.background_vel_x,
        self.background_vel_y,
        self.song_path
      )];

      for tileset in &self.tilesets {
        text.push(format!(
          "<tileset firstgid=\"{}\" source=\"{}\"/>",
          tileset.first_gid, tileset.path
        ));
      }

      let scale_x = 1.0 / (self.tile_width as f32 / 2.0);
      let scale_y = 1.0 / self.tile_height as f32;

      for layer_index in 0..self.layers.len() {
        text.push(self.layers[layer_index].render());

        text.push(String::from("<objectgroup>"));
        for object in &mut self.objects {
          if object.layer >= layer_index && object.layer < layer_index + 1 {
            text.push(object.render(scale_x, scale_y));
          }
        }
        text.push(String::from("</objectgroup>"));
      }

      text.push(String::from("</map>"));

      self.cached_string = text.join("");
      self.cached = true;
    }

    self.cached_string.clone()
  }

  pub(in super::super) fn asset_is_stale(&self) -> bool {
    self.asset_stale
  }

  pub fn generate_asset(&mut self) -> Asset {
    use super::super::AssetData;

    self.asset_stale = false;

    let tileset_paths = self.tilesets.iter().map(|tileset| &tileset.path);

    let dependencies = tileset_paths
      .chain(std::iter::once(&self.background_texture_path))
      .chain(std::iter::once(&self.background_animation_path))
      .chain(std::iter::once(&self.song_path))
      .filter(|path| path.starts_with("/server/")) // provided by server
      .cloned()
      .collect();

    Asset {
      data: AssetData::Text(self.render()),
      dependencies,
      last_modified: 0,
      cachable: false,
    }
  }
}
