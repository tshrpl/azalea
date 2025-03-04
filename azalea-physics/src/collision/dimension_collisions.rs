use crate::collision::{VoxelShape, AABB};
use azalea_block::BlockState;
use azalea_core::{ChunkPos, ChunkSectionPos, Cursor3d, CursorIterationType, EPSILON};
use azalea_world::entity::EntityData;
use azalea_world::{Chunk, Dimension};
use std::sync::{Arc, Mutex};

pub trait CollisionGetter {
    fn get_block_collisions<'a>(
        &'a self,
        entity: Option<&EntityData>,
        aabb: AABB,
    ) -> BlockCollisions<'a>;
}

impl CollisionGetter for Dimension {
    fn get_block_collisions<'a>(
        &'a self,
        entity: Option<&EntityData>,
        aabb: AABB,
    ) -> BlockCollisions<'a> {
        BlockCollisions::new(self, entity, aabb)
    }
}

pub struct BlockCollisions<'a> {
    pub dimension: &'a Dimension,
    // context: CollisionContext,
    pub aabb: AABB,

    pub cursor: Cursor3d,
    pub only_suffocating_blocks: bool,
}

impl<'a> BlockCollisions<'a> {
    pub fn new(dimension: &'a Dimension, _entity: Option<&EntityData>, aabb: AABB) -> Self {
        let origin_x = (aabb.min_x - EPSILON) as i32 - 1;
        let origin_y = (aabb.min_y - EPSILON) as i32 - 1;
        let origin_z = (aabb.min_z - EPSILON) as i32 - 1;

        let end_x = (aabb.max_x + EPSILON) as i32 + 1;
        let end_y = (aabb.max_y + EPSILON) as i32 + 1;
        let end_z = (aabb.max_z + EPSILON) as i32 + 1;

        let cursor = Cursor3d::new(origin_x, origin_y, origin_z, end_x, end_y, end_z);

        Self {
            dimension,
            aabb,
            cursor,
            only_suffocating_blocks: false,
        }
    }

    fn get_chunk(&self, block_x: i32, block_z: i32) -> Option<&Arc<Mutex<Chunk>>> {
        let chunk_x = ChunkSectionPos::block_to_section_coord(block_x);
        let chunk_z = ChunkSectionPos::block_to_section_coord(block_z);
        let chunk_pos = ChunkPos::new(chunk_x, chunk_z);

        // TODO: minecraft caches chunk here
        // int chunkX = SectionPos.blockToSectionCoord(blockX);
        // int chunkZ = SectionPos.blockToSectionCoord(blockZ);
        // long chunkPosLong = ChunkPos.asLong(chunkX, chunkZ);
        // if (this.cachedBlockGetter != null && this.cachedBlockGetterPos == var5) {
        //    return this.cachedBlockGetter;
        // } else {
        //    BlockGetter var7 = this.collisionGetter.getChunkForCollisions(chunkX, chunkZ);
        //    this.cachedBlockGetter = var7;
        //    this.cachedBlockGetterPos = chunkPosLong;
        //    return var7;
        // }

        self.dimension[&chunk_pos].as_ref()
    }
}

impl<'a> Iterator for BlockCollisions<'a> {
    type Item = Box<dyn VoxelShape>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.cursor.next() {
            if item.iteration_type == CursorIterationType::Corner {
                continue;
            }

            let chunk = self.get_chunk(item.pos.x, item.pos.z);
            let chunk = match chunk {
                Some(chunk) => chunk,
                None => continue,
            };
            let chunk_lock = chunk.lock().unwrap();

            let pos = item.pos;
            let block_state: BlockState = chunk_lock.get(&(&pos).into(), self.dimension.min_y());
            // let block: Box<dyn Block> = block_state.into();

            // TODO: continue if self.only_suffocating_blocks and the block is not suffocating

            let block_shape = if block_state == BlockState::Air {
                crate::collision::empty_shape()
            } else {
                crate::collision::block_shape()
            };
            // let block_shape = block.get_collision_shape();
            // if block_shape == Shapes::block() {
            if true {
                // TODO: this can be optimized
                if !self.aabb.intersects_aabb(&AABB {
                    min_x: item.pos.x as f64,
                    min_y: item.pos.y as f64,
                    min_z: item.pos.z as f64,
                    max_x: (item.pos.x + 1) as f64,
                    max_y: (item.pos.y + 1) as f64,
                    max_z: (item.pos.z + 1) as f64,
                }) {
                    continue;
                }

                return Some(block_shape.move_relative(
                    item.pos.x as f64,
                    item.pos.y as f64,
                    item.pos.z as f64,
                ));
            }

            // let block_shape = block_shape.move_relative(item.pos.x, item.pos.y, item.pos.z);
            // if (!Shapes.joinIsNotEmpty(block_shape, this.entityShape, BooleanOp.AND)) {
            //     continue;
            // }

            // return block_shape;
        }

        None
    }
}
