use birbs::vectorize;

fn main() {
    println!("Hello, world!");
}

struct Vector_f32 {
    x: f32,
    y: f32,
    z: f32,
}

struct Vector_i32 {
    x: i32,
    y: i32,
    z: i32,
}

struct Individuality {}

struct Birb {
    pos: Vector_f32,
    spd: Vector_f32,
    acc: Vector_f32,
    ind: Individuality,
}

struct Chunk {
    size: u32,
    pos: Vector_i32,
    birbs: Vec<Birb>,
    field: [[[Vector_f32; 100]; 100]; 100], /*alloué dynamiquement pour le momenet à voir si c'est bien */
}

struct World {
    worldsize: u32,
    chunks: [[[Chunk; worldsize]; worldsize]; worldsize],
    birbs: Vec<Birb>,
}
