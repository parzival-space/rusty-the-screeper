use screeps::Part::Move;
use screeps::{Creep, Position};
use std::cmp::min;

/// Computes the move speed of a creep. Currently, always assumes a ground of type plain land.
pub fn get_creep_move_speed(creep: &Creep) -> usize {
    // get count of MOVE parts and other parts
    let (move_parts, fatigue_parts) = creep.body().iter()
        .fold((0, 0), |(move_parts, fatigue_parts), part| {
            if part.part().eq(&Move) {
                (move_parts + 1, fatigue_parts)
            } else {
                (move_parts, fatigue_parts + 1)
            }
        });

    // Each body part (except MOVE) generates fatigue points when the creep moves:
    // - 1 point per body part on roads
    // - 2 on plain land
    // - 10 on swamp
    // Each MOVE body part decreases fatigue points by 2 per tick.
    // Maximum movement speed is 1
    const FATIGUE_GENERATION: usize = 2; // todo: currently always assumes plain land
    min(1, (fatigue_parts * FATIGUE_GENERATION) - (move_parts * 2))
}

/// Computes the Chebyshev distance between two `RoomPosition`s.
pub fn chebyshev_pos(a: Position, b: Position) -> f64 {

    chebyshev(
        &[
            a.x().u8() as f64 * a.room_name().x_coord() as f64,
            a.y().u8() as f64 * a.room_name().y_coord() as f64
        ],
        &[
            (b.x().u8() as f64) * b.room_name().x_coord() as f64,
            (b.y().u8() as f64) * b.room_name().y_coord() as f64
        ],
    )
}

/// Computes the Chebyshev distance (l-infinity norm) between two vectors `x` and `y`.
pub fn chebyshev(x: &[f64], y: &[f64]) -> f64 {
    assert_eq!(x.len(), y.len(), "Vectors must be of the same length");

    let mut result = 0.0;
    for i in 0..x.len() {
        let diff = (x[i] - y[i]).abs();
        if diff > result {
            result += diff;
        }
    }
    result
}