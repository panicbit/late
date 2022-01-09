type TileId = u8;
type TargetId = u8;
struct Coord(u8, u8);
struct PixelCoord(u8, u8);

enum Element {
    Tile(Coord, TileId),
    TileRow(Coord, TileId, u8),
    Warp(TargetId, PixelCoord)
}

impl Element {
    fn from_bytes(element: &[u8]) -> Result<Element, String> {
        let first = element.get(0)
            .ok_or_else(|| "Invalid element")?;

        let high: u8 = first >> 4;
        let low: u8 = first & 0xF;

        match (high, low) {
            (x @ 0x0...0x7, y @ 0x0...0x9) => {
                Tile(Coord(x, y), )
                
            }
            _ => unimplemented!()
        }

        return unimplemented!()
    }
}

struct Room {

}
