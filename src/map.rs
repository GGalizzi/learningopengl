use vek::{Aabb, Vec3};

use crate::component::BoundingBox;

const STR_MAP: [&'static str; 2] = [
    r#"
    #################### 
    ####################
    ####################
    ####################
    ####################
    ####################
    ####################
    ####################
    ####################
    ####################
"#,
    r#"
    ........#...#....... 
    ........#...........
    ........#...........
    ........#...........
    ........####.#######
    .............#......
    .............#......
    ..#..........#......
    .............#......
    ....................
"#];

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Tile {
    Floor,
    Wall,
    StairUp,
}

impl Tile {
    fn from_char(ch: char) -> Tile {
        match ch {
            '#' => Tile::Wall,
            '<' => Tile::StairUp,
            _ => Tile::Floor,
        }
    }

    pub fn is_wall(&self) -> bool {
        *self == Tile::Wall
    }
}

#[derive(Debug)]
pub struct Area {
    pub tiles: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}

const voxels_per_world: f32 = 10f32;
impl Area {
    pub fn debug() -> Area {
        let mut world_tiles = vec![];
        for y in &STR_MAP {
            world_tiles.push({
                let level: Vec<Tile> = y
                    .to_owned()
                    .replace('\n', "")
                    .replace(' ', "")
                    .chars()
                    .map(|ch| Tile::from_char(ch))
                    .collect();
                level
            });
        }

        //expand_tiles(Area { tiles: world_tiles, width: 20, height: 5, depth: 10 }, 10)
        Area { tiles: world_tiles, width: 20, height: 2, depth: 10 }
    }
    
    pub fn debug2() -> Area {
        let width = 50;
        let height = 3;
        let depth = 80;
        
        let mut levels = Vec::with_capacity(height);
        for y in 0..height {
            let mut tiles = Vec::with_capacity(height * depth);
            for z in 0..depth {
                for x in 0..width {
                    if (z == 0 || x == 0) {
                        tiles.push(Tile::Wall);
                    } else {
                        tiles.push(Tile::Floor);
                    }
                }
            }
            levels.push(tiles);
        }
        let walls: Vec<Tile> = levels.iter().flatten().cloned().filter(|t| t.is_wall()).collect();
        println!("walls {:?}", walls.len());
        
        
        let unexpanded = Area {
            tiles: levels,
            width, height, depth,
        };
        expand_tiles(unexpanded, 5)
    }
    
    pub fn tile_at(&self, x: usize, y: usize, z: usize) -> Tile {
        self.tiles[y][z * self.width + x]
    }

    pub fn blocks_at(
        &self,
        point: Vec3<i32>,
    ) -> Option<Tile> {
        if point.x < 0 || point.y < 0 || point.z < 0 {
            return None;
        }
        let vpw = voxels_per_world as usize;
        if let Some(level) = self
            .tiles
            .get(point.y as usize * 20 * vpw * 8 * vpw)
        {
            if let Some(tile) = level.get(
                20 * vpw * point.z as usize +
                    point.x as usize,
            ) {
                if match *tile {
                    Tile::Wall => true,
                    Tile::StairUp => true,
                    _ => false,
                } {
                    return Some(*tile);
                }
            }
        }
        None
    }

    pub fn blocks_around(
        &self,
        point: Vec3<f32>,
    ) -> Option<(Aabb<f32>, Aabb<f32>, Vec3<i32>)> {
        return None;

        /*
        let p = (point * voxels_per_world as f32).map(|e| e.floor() as i32);

        if self.blocks_at(p).is_some() {
            println!("yup {:?}", p);
            return None;
        } else {
            println!("nope");
        }*/

        let aabb = Aabb {
            min: Vec3::new(
                point.x - 0.25,
                point.y - 0.15,
                point.z - 0.25,
            ),
            max: Vec3::new(
                point.x + 0.25,
                point.y + 0.15,
                point.z + 0.25,
            ),
        };

        let area_rect = Aabb {
            min: aabb.min * voxels_per_world,
            max: aabb.max * voxels_per_world,
        };
        let area_rect = area_rect.map(|e| (e.floor()) as i32);

        print!("checking {:?}", aabb);
        for my in area_rect.min.y..=area_rect.max.y {
            for mx in area_rect.min.x..=area_rect.max.x {
                for mz in area_rect.min.z..=area_rect.max.z {
                    let thisblocks =
                        self.blocks_at((mx, my, mz).into());
                    if let Some(tile) = thisblocks {
                        print!("blocked_at");
                        let block_aabb = Aabb {
                            min: Vec3::new(
                                mx as f32 * 0.1,
                                my as f32 * 0.1,
                                mz as f32 * 0.1,
                            ),
                            max: Vec3::new(
                                mx as f32 * 0.1 + 0.1,
                                my as f32 * 0.1 + 0.1,
                                mz as f32 * 0.1 + 0.1,
                            ),
                        };
                        print!(" against {:?}", block_aabb);
                        if aabb.collides_with_aabb(block_aabb)
                        {
                            println!("and collides");
                            return Some((
                                block_aabb,
                                aabb,
                                Vec3::new(mx, my, mz),
                            ));
                        }
                    }
                }
            }
        }
        println!("");

        None
    }
}

fn expand_tiles(area: Area, expand: usize) -> Area {
    let mut levels = Vec::with_capacity(area.height * expand);
    for y in 0..area.height * expand {
        let mut level = Vec::with_capacity((area.width * expand) * (area.depth * expand));
        let uy = (y as f32 / expand as f32).floor() as usize;
        for z in 0..area.depth * expand {
            let uz = (z as f32 / expand as f32).floor() as usize;
            for x in 0..area.width * expand {
                let ux = (x as f32 / expand as f32).floor() as usize;
                let tile = area.tiles[uy][uz * area.width + ux];
                level.push(tile);
            }
        }
        levels.push(level);
    }
    
    Area {
        width: area.width * expand,
        height: area.height * expand,
        depth: area.depth * expand,
        tiles: levels,
    }
}