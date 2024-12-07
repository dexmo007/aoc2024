pub type Direction = (isize, isize);
pub const UP: Direction = (-1, 0);
pub const RIGHT: Direction = (0, 1);
pub const DOWN: Direction = (1, 0);
pub const LEFT: Direction = (0, -1);

#[inline]
pub fn turn_90_deg((dy, dx): Direction) -> Direction {
    (dx, -dy)
}

#[inline]
pub fn move_and_get<P>(
    map: &Vec<Vec<P>>,
    y: usize,
    x: usize,
    (dy, dx): Direction,
) -> Option<(usize, usize, &P)> {
    let ny = y.checked_add_signed(dy)?;
    let nx = x.checked_add_signed(dx)?;
    let row = map.get(ny)?;
    let pos = row.get(nx)?;
    Some((ny, nx, pos))
}
