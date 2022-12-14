use rand::Rng;

const MAP_N: usize = 25;

fn main() {
    let mut rng = rand::thread_rng();

    let mut maze = [[0; MAP_N]; MAP_N];

    for n in 0..MAP_N {
        maze[n][0] = 1;
        maze[0][n] = 1;
        maze[n][MAP_N - 1] = 1;
        maze[MAP_N - 1][n] = 1;
    }

    for x in 2..MAP_N - 2 {
        for y in 2..MAP_N - 2 {
            if x % 2 == 1 || y % 2 == 1 {
                continue;
            }
            maze[x][y] = 1;
            let r = rng.gen_range(0..=3);
            match r {
                0 => maze[x][y - 1] = 1,
                1 => maze[x + 1][y] = 1,
                2 => maze[x][y + 1] = 1,
                3 => maze[x - 1][y] = 1,
                _ => (),
            }
        }
    }

    let tiles = ["  ", "##"];
    for x in 0..MAP_N {
        for y in 0..MAP_N {
            print!("{}", tiles[maze[x][y] as usize]);
        }
        println!("");
    }
}
