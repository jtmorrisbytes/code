use std::io::BufRead;
use super::Vertex4f;

const VERTEX: &str = "v";
const COMMENT: &str = "#";
const FACE: &str = "f";

pub struct Obj{
    vertices: std::collections::BTreeMap<u32,Vertex4f>
}
impl Obj {
    pub fn list_verticies(&self) -> Vec<Vertex4f> {
        self.vertices.values().into_iter().map(|v| v.to_owned()).collect()
    }
    pub fn verticies<'a>(&'a self) -> &'a std::collections::BTreeMap<u32,Vertex4f> {
        &self.vertices
    }
    
}



#[cfg(test)]
#[test]
pub fn test_parse_obj(){
    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = std::path::PathBuf::from(path).join("teapot.obj");
    parse_obj(path.to_str().unwrap()).unwrap();
}

pub fn parse_obj(path: &str) -> Result<Obj, Box<dyn std::error::Error>> {
    let mut file = std::io::BufReader::new(std::fs::File::open(path)?);
    let mut line = String::new();
    let mut vertex_index: u32 = 0;
    let mut vertices: std::collections::BTreeMap<u32,Vertex4f> = std::collections::BTreeMap::new();
    while let Ok(bytes) = file.read_line(&mut line) {
        // Ok(0) == end of file
        if bytes == 0 {
            break;
        }
        // # this is a comment
        if line.starts_with(COMMENT) {
            continue;
        }
        if line.len() <1 {
            continue;
        }
        println!("{line}");
        let command_index = line.find(" ");
        if command_index.is_none() {
            continue;
        }
        let mut split = line.trim().split(" ");
        let command = split.next().ok_or("Failed to get object type from OBJ file.")?;
        if command.len() == 0 {
            return Err("Empty command string in OBJ file".into());
        }
        match command {
            VERTEX => {
                let x:f32 = split.next().ok_or("Missing x coordinate of vertex in vertex command.")?.parse().map_err(|e| format!("Error while parsing X coordinate of Vertex in OBJ file: {e}"))?;
                let y:f32 = split.next().ok_or("Missing y coordinate of vertex in vertex command")?.parse().map_err(|e| format!("Error while parsing Y coordinate of Vertex in OBJ file: {e}"))?;
                let z:f32 = split.next().ok_or("Missing z coordinate of vertex in vertex command")?.parse().map_err(|e| format!("Error while parsing Z coordinate of Vertex in OBJ file: {e}"))?;
                let w:f32 = split.next().unwrap_or("1.0").parse().unwrap_or(1.0);
                let vertex = Vertex4f::new(x,y,z,w);
                vertices.insert(vertex_index,vertex);
                // println!("{parameters}");
                vertex_index = vertex_index + 1;
            },
            FACE=> {
                // TODO: faces
            }
            _=>{panic!("unsupported grammar '{command}'")}
        }
        line.clear();


    }
    Ok(Obj{vertices})
}