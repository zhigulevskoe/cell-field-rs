
struct Node {
    face: String,
    has_collision: bool
}

impl Node {
    fn get_collision(&self) -> bool {
        self.has_collision
    } // get_collision

    fn get_face(&self) -> String {
        self.face.clone()
    } // get_face
}


fn main() {

    let tree = Node {
        face: String::from("T"),
        has_collision: true
    };

    render_field(tree);

} // main


fn render_field(node: Node) {
    println!("current face:         {}", node.get_face());
    println!("current collision:    {}", node.get_collision());
} // render_field
