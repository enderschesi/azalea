use azalea_core::position::BlockPos;

use crate::pathfinder::{astar, costs::*};

use super::{Edge, ExecuteCtx, IsReachedCtx, MoveData, PathfinderCtx};

pub fn teleport_move(ctx: &mut PathfinderCtx, node: BlockPos) {

}

fn execute_teleport_move(mut ctx: ExecuteCtx) {

}

#[must_use]
pub fn teleport_is_reached(
    IsReachedCtx {
        position, target, ..
    }: IsReachedCtx,
) -> bool {
    // 0.094 and not 0 for lilypads
    BlockPos::from(position) == target && (position.y - target.y as f64) < 0.094
}
