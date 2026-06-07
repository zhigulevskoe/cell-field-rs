// ========== NODE ==========

#[derive(Clone, Copy)]
struct Node {
    face: char,
    has_collision: bool,
    is_empty: bool,
    x: usize,
    y: usize,
} // Node{}

impl Node {

    // ========== CONSTRUCTORS ==========

    #[deprecated]
    fn new(face_: char, pos_x: usize, pos_y: usize,
            empty: bool, collide: bool) -> Self {

        if empty && collide {
            panic!("Tile can't be empty and have collision at the same time!!!");
        } // if

        Self {
            face: face_,
            x: pos_x,
            y: pos_y,
            is_empty: empty,
            has_collision: collide,
        } // Self
    } // new()


    fn empty(face_: char, pos_x: usize, pos_y: usize) -> Self {
        Self {
            face: face_,
            x: pos_x,
            y: pos_y,
            has_collision: false,
            is_empty: true,
        } // Self
    } // empty()
    

    fn solid(face_: char, pos_x: usize, pos_y: usize) -> Self {
        Self {
            face: face_,
            x: pos_x,
            y: pos_y,
            has_collision: true,
            is_empty: false,
        } // Self
    } // solid()
    
    // ========== /CONSTRUCTORS ==========
} // impl

// ========== /NODE ==========


// ========== TILE ==========

#[derive(Clone, Copy)]
enum Tile {
    Base(Node),
    Empty,
}

impl Tile {

    fn get_face (&self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::Base(b) => b.face
        }
    }

} // Tile

// ========== /TILE ==========

fn draw(field: &Vec<Vec<Tile>>) {

    for row in field {
        for cell in row {
            print!(" {}", cell.get_face());
        } //for cell
        println!();
    } // for row
    println!();
} // draw()

fn process(field: &mut Vec<Vec<Tile>>, entity_list: &mut Vec<Tile>) {
    println!("field length: {}", field.len());
} // process()

fn init_field(width: usize, height: usize) -> Vec<Vec<Tile>> {

    let field = vec![vec![Tile::Empty; width]; height];

    field
} // init_field()

















fn main() {
    let mut field = init_field(27, 27);
    let mut entity_list = Vec::new();

    process(&mut field, &mut entity_list);
    draw(&field);
}
