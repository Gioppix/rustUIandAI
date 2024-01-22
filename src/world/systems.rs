use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_ecs_tilemap::prelude::get_tilemap_center_transform;
use bevy_ecs_tilemap::prelude::TilemapTexture;
use bevy_ecs_tilemap::TilemapBundle;
use bevy_ecs_tilemap::prelude::TilemapSize;
use bevy_ecs_tilemap::prelude::TileStorage;
use bevy_ecs_tilemap::prelude::TilePos;
use bevy_ecs_tilemap::prelude::TileBundle;
use bevy_ecs_tilemap::prelude::TilemapId;
use bevy_ecs_tilemap::prelude::TilemapTileSize;
use bevy_ecs_tilemap::prelude::TilemapType;
use bevy_ecs_tilemap::tiles::TileTextureIndex;

use robotics_lib::world::tile::Tile;
use robotics_lib::world::tile::TileType;
use robotics_lib::world::tile::Content;
use robotics_lib::interface::robot_map;
use bevy::prelude::Resource;
use crate::world::resources::WorldRes;
use robotics_lib::runner::Runnable;
use crate::world::components::TileDraw;
use crate::world::components::GridDraw;
use crate::world::components::ContentDraw;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;

pub fn spawn_map(
    mut commands: Commands,
    mut world: ResMut<WorldRes>,
    //rendered_map: Query<Entity, (With<TileDraw>, With<GridDraw>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    #[cfg(all(not(feature = "atlas"), feature = "render"))]
    array_texture_loader: Res<ArrayTextureLoader,>,
) {

    // 
    // DRAW THE TILES ----------------------------------------
    //

    let window = window_query.get_single().unwrap();
    let texture_handle: Handle<Image> = asset_server.load("tiles4.png");

    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn_empty().id();

    let map_size = world.world_size;
    let map_size = TilemapSize { x: map_size as u32, y: map_size as u32};

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage = TileStorage::empty(map_size);

    // Spawn empty map 
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((
                    TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(12), // Here the None texture index 
                    ..Default::default()
                },
                TileDraw{}))
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }


    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();
    
    // Spawn new map
    commands.entity(tilemap_entity).insert((TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    },
    GridDraw{}));

    //
    // DRAW THE CONTENT --------------------------------------
    //

    let texture_handle: Handle<Image> = asset_server.load("contents.png");
    let tilemap_entity = commands.spawn_empty().id();
    let map_size = world.world_size;
    let map_size = TilemapSize { x: map_size as u32, y: map_size as u32};
    let mut tile_storage = TileStorage::empty(map_size);

    // Spawn empty map 
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((
                    TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(15), // Here the None texture index 
                    ..Default::default()
                },
                TileDraw{}))
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }


    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();
   
    // Spawn new map
    commands.entity(tilemap_entity).insert((TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 1.0),
        ..Default::default()
    },
    ContentDraw{}));


    // Add atlas to array texture loader so it's preprocessed before we need to use it.
    // Only used when the atlas feature is off and we are using array textures.
    #[cfg(all(not(feature = "atlas"), feature = "render"))]
    {
        array_texture_loader.add(TilemapArrayTexture {
            texture: TilemapTexture::Single(asset_server.load("tiles2.png")),
            tile_size,
            ..Default::default()
        });

        array_texture_loader.add(TilemapArrayTexture {
            texture: TilemapTexture::Single(asset_server.load("contents.png")),
            tile_size,
            ..Default::default()
        });

    }
}


pub fn update_map(
    mut commands: Commands,
    mut world: ResMut<WorldRes>,
    // Tiles Map
    mut tile_storage: Query<&mut TileStorage, (Without<ContentDraw>, With<GridDraw>)>,
    mut tilemap_entity_query: Query<Entity, (With<GridDraw>, Without<ContentDraw>)>,
    //rendered_map: Query<Entity, (With<TileDraw>, With<GridDraw>)>,
    // Content Map
    mut content_storage: Query<&mut TileStorage, (Without<GridDraw>, With<ContentDraw>)>,
    mut content_entity_query: Query<Entity, (With<ContentDraw>, Without<GridDraw>)>,
    //rendered_content: Query<Entity, (With<TileDraw>, With<ContentDraw>, Without<GridDraw>)>,
    // Other Queries
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    #[cfg(all(not(feature = "atlas"), feature = "render"))]
    array_texture_loader: Res<ArrayTextureLoader,>,
) {
        

    // Recieve the map or return
    let res = match world.rx.lock().unwrap().try_recv() {
         Ok(w) => Some(w),
         Err(_) => None,
    };
    if let None = res {
        return;
    }
    let ((map, (robot_x, robot_y)), environmental_conditions, score) = res.unwrap();
    let map_size = world.world_size;
    world.environmental_conditions = environmental_conditions;
    world.score = score;
    world.elevation = map[1][1].clone().unwrap().elevation; 
    //
    // Draw Tiles ---------------------------------------------
    //

    let mut tile_map = &mut tile_storage.get_single_mut().unwrap();
    let tilemap_entity = tilemap_entity_query.get_single().unwrap();

    // Doing this for all nine tiles
   // Remember that the robot coordinates as swapped
   // ROBOT 
   // ------------------> y 
   // |
   // |
   // |
   // |
   // |
   // V 
   // x
   //
   // WORLD 
   // y 
   // ^
   // |
   // |
   // |
   // |
   // |
   // |
   // -------------------> x 

    // TOP LEFT
    if is_inside(robot_x as i32 - 1, robot_y as i32 - 1, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32 - 1,
            y:  map_size as u32 -1 - robot_x as u32 + 1,
        };
        let new_tile_id = get_texture_id(&map[0][0]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }
 
    // TOP
    if is_inside(robot_x as i32 - 1, robot_y as i32, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32,
            y:  map_size as u32 -1 - robot_x as u32 + 1,
        };
        let new_tile_id = get_texture_id(&map[0][1]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }
      
    
    // TOP RIGHT
    if is_inside(robot_x as i32 - 1, robot_y as i32 + 1, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32 + 1,
            y:  map_size as u32 -1 - robot_x as u32 + 1,
        };
        let new_tile_id = get_texture_id(&map[0][2]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }


    // BOTTOM LEFT
    if is_inside(robot_x as i32 + 1, robot_y as i32 - 1, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32 - 1,
            y:  map_size as u32 -1- robot_x as u32 - 1,
        };
        let new_tile_id = get_texture_id(&map[2][0]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }
 
    // BOTTOM
    if is_inside(robot_x as i32 + 1, robot_y as i32, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32,
            y:  map_size as u32 -1- robot_x as u32 - 1,
        };
        let new_tile_id = get_texture_id(&map[2][1]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }
      
    
    // BOTTOM RIGHT
    if is_inside(robot_x as i32 + 1, robot_y as i32 + 1, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32 + 1,
            y:  map_size as u32 -1- robot_x as u32 - 1,
        };
        let new_tile_id = get_texture_id(&map[2][2]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }


    // MIDDLE LEFT
    if is_inside(robot_x as i32, robot_y as i32 - 1, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32 - 1,
            y: map_size as u32-1 - robot_x as u32,

        };
        let new_tile_id = get_texture_id(&map[1][0]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }
      
    
    // MIDDLE RIGHT
    if is_inside(robot_x as i32, robot_y as i32 + 1, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32 + 1,
            y: map_size as u32 -1- robot_x as u32,
        };
        let new_tile_id = get_texture_id(&map[1][2]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }
    
    // MIDDLE
    if is_inside(robot_x as i32, robot_y as i32, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32,
            y: map_size as u32 -1- robot_x as u32,
        };
        let new_tile_id = get_texture_id(&map[1][1]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }

    
    //
    // Draw Content ---------------------------------------------
    //

    let mut tile_map = &mut content_storage.get_single_mut().unwrap();
    let tilemap_entity = content_entity_query.get_single().unwrap();

    // TOP LEFT
    if is_inside(robot_x as i32 - 1, robot_y as i32 - 1, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32 - 1,
            y:  map_size as u32 -1 - robot_x as u32 + 1,
        };
        let new_tile_id = get_texture_id_content(&map[0][0]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }
 
    // TOP
    if is_inside(robot_x as i32 - 1, robot_y as i32, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32,
            y:  map_size as u32 -1 - robot_x as u32 + 1,
        };
        let new_tile_id = get_texture_id_content(&map[0][1]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }
      
    
    // TOP RIGHT
    if is_inside(robot_x as i32 - 1, robot_y as i32 + 1, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32 + 1,
            y:  map_size as u32 -1 - robot_x as u32 + 1,
        };
        let new_tile_id = get_texture_id_content(&map[0][2]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }


    // BOTTOM LEFT
    if is_inside(robot_x as i32 + 1, robot_y as i32 - 1, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32 - 1,
            y:  map_size as u32 -1- robot_x as u32 - 1,
        };
        let new_tile_id = get_texture_id_content(&map[2][0]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }
 
    // BOTTOM
    if is_inside(robot_x as i32 + 1, robot_y as i32, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32,
            y:  map_size as u32 -1- robot_x as u32 - 1,
        };
        let new_tile_id = get_texture_id_content(&map[2][1]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }
      
    
    // BOTTOM RIGHT
    if is_inside(robot_x as i32 + 1, robot_y as i32 + 1, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32 + 1,
            y:  map_size as u32 -1- robot_x as u32 - 1,
        };
        let new_tile_id = get_texture_id_content(&map[2][2]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }


    // MIDDLE LEFT
    if is_inside(robot_x as i32, robot_y as i32 - 1, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32 - 1,
            y: map_size as u32-1 - robot_x as u32,

        };
        let new_tile_id = get_texture_id_content(&map[1][0]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }
      
    
    // MIDDLE RIGHT
    if is_inside(robot_x as i32, robot_y as i32 + 1, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32 + 1,
            y: map_size as u32 -1- robot_x as u32,
        };
        let new_tile_id = get_texture_id_content(&map[1][2]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }
    
    // MIDDLE
    if is_inside(robot_x as i32, robot_y as i32, map_size as i32) {
        // World coordinates
        let position = TilePos {
            x: robot_y as u32,
            y: map_size as u32 -1- robot_x as u32,
        };
        let new_tile_id = get_texture_id_content(&map[1][1]);
        if let Some(old_tile) = tile_map.get(&position) {
            // Despawn old tile 
            commands.entity(old_tile).despawn();
            // Spawn a new tile
            let tile_entity = commands
                    .spawn((
                        TileBundle {
                        position: position,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(new_tile_id),
                        ..Default::default()
                    },
                    TileDraw{}))
                    .id();
                tile_map.set(&position, tile_entity);
        }
    }


}

// Using robot coordinates
fn is_inside(x: i32, y: i32, map_size: i32) -> bool {
    x >= 0 && y >= 0 && y < map_size && x < map_size 
}

// Gets the tilemap id given a TyleType 
fn get_texture_id(tile: &Option<Tile>) -> u32 {
    match tile {         
        Some(tile) => {
            match tile.tile_type {
                TileType::DeepWater => 0,
                TileType::ShallowWater => 1,
                TileType::Sand => 2,
                TileType::Grass => 3,
                TileType::Street => 4,
                TileType::Hill => 5,
                TileType::Mountain => 7,
                TileType::Snow => 8,
                TileType::Lava => 9,
                TileType::Teleport(_) => 10,
                TileType::Wall => 11,
                _ => 12,
            }
        },
        None => 12,
    }
    
}

// Gets the tilemap id given a TyleType for the content 
fn get_texture_id_content(tile: &Option<Tile>) -> u32 {
    match tile {         
        Some(tile) => {
            match tile.content {
                Content::Rock(_) => 0,
                Content::Tree(_) => 1,
                Content::Garbage(_) => 2,
                Content::Fire => 3,
                Content::Coin(_) => 4,
                Content::Bin(_) => 5,
                Content::Crate(_) => 5,
                Content::Bank(_) => 6,
                Content::Water(_) => 7,
                Content::Market(_) => 8,
                Content::Fish(_) => 9,
                Content::Building => 10,
                Content::Bush(_) => 11,
                Content::JollyBlock(_) => 12,
                Content::Scarecrow => 13,
                Content::None => 14,
                _ => 14,
            }
        },
        None => 14,
    }
    
}
