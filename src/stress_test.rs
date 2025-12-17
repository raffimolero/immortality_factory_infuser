use crate::prelude::*;

/*
1s per cycle

1 Air Pumps.
3 Structures.
1 Connections.
74 bytes total:
> 66 bytes from structures (22 bytes/struct)
> 8 bytes from connections (8 bytes/connect)
*/
pub fn ideal() -> World {
    let mut bp = World::new();
    let pump = bp.place(AirPump, Laboratory.width(), -2);
    let door = bp.place(AbysalDoor, Laboratory.width() + AirPump.width(), -2);
    bp.connect(pump.output(0), door.input(0));
    bp
}

/*
3s per cycle

4097 Air Pumps.
4099 Structures.
1 Connections.
90186 bytes total:
> 90178 bytes from structures (22 bytes/struct)
> 8 bytes from connections (8 bytes/connect)
*/
pub fn pumps() -> World {
    let mut bp = ideal();
    for y in 0..64 {
        for x in 0..64 {
            bp.place(AirPump, x * AirPump.width(), y * AirPump.height());
        }
    }
    bp
}

/*
2.8s per cycle

1 Air Pumps.
4099 Structures.
1 Connections.
90186 bytes total:
> 90178 bytes from structures (22 bytes/struct)
> 8 bytes from connections (8 bytes/connect)
*/
pub fn buildings() -> World {
    let mut bp = ideal();
    for y in 0..64 {
        for x in 0..64 {
            bp.place(SingleStorage, x, y);
        }
    }
    bp
}

/*
8s per cycle

1 Air Pumps.
4099 Structures.
1 Connections.
90186 bytes total:
> 90178 bytes from structures (22 bytes/struct)
> 8 bytes from connections (8 bytes/connect)
*/
pub fn ports() -> World {
    let mut bp = ideal();
    for y in 0..64 {
        for x in 0..64 {
            bp.place(BigMerger, x * BigMerger.width(), y * BigMerger.height());
        }
    }
    bp
}

/*
didn't even bother running this one to a full cycle

1 Air Pumps.
4099 Structures.
12289 Connections.
188490 bytes total:
> 90178 bytes from structures (22 bytes/struct)
> 98312 bytes from connections (8 bytes/connect)
*/
pub fn connections() -> World {
    let mut bp = ideal();
    for y in 0..64 {
        for x in 0..32 {
            let split = bp.place(BigSplitter, x * 2, y * 6);
            let merge = bp.place(BigMerger, x * 2 + 1, y * 6);
            bp.connect(merge.output(0), split.input(0));
            for i in 0..5 {
                bp.connect(split.output(i), merge.input(i));
            }
        }
    }
    bp
}
