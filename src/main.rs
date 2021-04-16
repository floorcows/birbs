use birbs::vectorize;

fn main() {
    println!("Hello, world!");
}

struct Vector_f32 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector_f32 {
    fn norm (&self) -> f32 {

    }
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

impl Chunk {
    fn update_birbs_list(&mut self) ->Chunk{
        //je sais plus trop la forme optimale pour les methodes d'initialisation

    }

    fn close_chunks(&self) -> Vec<Chunk> {
        //pour l'instant c'est juste celui dans lequel on est vu qu'il y en a qu'un
        
    }

    fn close_birbs(&self) -> Vec<Birb>{

    }

    fn update_birbs_pos(birbs: Vec<Birb>) -> birbs: Vec<Birb>{
        
    }


}

struct World {
    worldsize: u32,
    chunks: [[[Chunk; worldsize]; worldsize]; worldsize],
    birbs: Vec<Birb>,
}

impl World {
    fn loop(&mut?? self) -> World {

    }
    fn update_birbs_pos(chunks: ????, birbs: Vec<Birb>) -> birbs: Vec<Birb>{

    }

    fn update_birbs_list(chunks:????) -> chunks??? {

    }

}