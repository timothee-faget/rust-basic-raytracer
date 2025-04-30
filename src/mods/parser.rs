use std::{error::Error, fs};

use console::style;

use super::{
    color::ColorRBG,
    material::Material,
    objs::{create_cube_triangles, Camera, Plane, Sphere, Triangle},
    position::{Angle, Quat, Vect3},
    render::Scene,
};

/// Tokeneiser enum
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Identifier(String),
    Number(f64),
    Colon,
    LBrace,
    RBrace,
    Newline,
}

/// Parser implementation
pub struct Parser {
    tokens: Vec<Token>,
    materials: Vec<(String, Material)>,
    pos: usize,
}

impl Parser {
    /// Build parser from text file
    pub fn build(filename: &str) -> Result<Self, Box<dyn Error>> {
        let mut parser = Self {
            tokens: vec![],
            materials: vec![],
            pos: 0,
        };

        if filename.ends_with(".rtp") {
            println!(
                "{} Parsing scene file : {}",
                style("[1/3]").bold().green(),
                style(filename).italic().dim()
            );
        } else if filename.ends_with(".obj") {
            println!(
                "        - Parsing mesh file : {}",
                style(filename).italic().dim()
            );
        }
        let file_content: &str = &fs::read_to_string(filename)?;
        parser.tokenize(file_content);
        Ok(parser)
    }

    /// Tokenize text file
    fn tokenize(&mut self, input: &str) {
        let mut chars = input.chars().peekable();

        while let Some(&ch) = chars.peek() {
            match ch {
                c if c.is_whitespace() && c != '\n' => {
                    chars.next();
                }
                '\n' => {
                    self.tokens.push(Token::Newline);
                    chars.next();
                }
                ':' => {
                    self.tokens.push(Token::Colon);
                    chars.next();
                }
                '{' => {
                    self.tokens.push(Token::LBrace);
                    chars.next();
                }
                '}' => {
                    self.tokens.push(Token::RBrace);
                    chars.next();
                }
                c if c.is_ascii_digit() || c == '-' => {
                    let mut num_str = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_ascii_digit() || c == '.' || c == '-' {
                            num_str.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    let value: f64 = num_str.parse().unwrap();
                    self.tokens.push(Token::Number(value));
                }
                c if c.is_alphabetic() => {
                    let mut ident = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_alphanumeric() || c == '_' || c == '.' || c == '/' {
                            ident.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    self.tokens.push(Token::Identifier(ident));
                }
                '#' => {
                    for c in chars.by_ref() {
                        if c == '\n' {
                            self.tokens.push(Token::Newline);
                            break;
                        }
                    }
                }
                _ => {
                    panic!("Unknown character: {}", ch);
                }
            }
        }
    }

    // Iteration functions

    /// Peek current token
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    /// Get next token
    fn next(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos);
        self.pos += 1;
        tok
    }

    /// Expect token
    fn expect(&mut self, expected: &Token) {
        let token = self.next().expect("Unexpected end of input");
        if token != expected {
            panic!("Expected {:?}, got {:?}", expected, token);
        }
    }

    // Specific parsers

    /// Parse Scene
    pub fn parse_scene(&mut self) -> Scene {
        let mut cameras: Vec<Camera> = vec![];
        //let mut objects: Vec<Object> = vec![];
        let mut spheres: Vec<Sphere> = vec![];
        let mut planes: Vec<Plane> = vec![];
        let mut triangles: Vec<Triangle> = vec![];

        while let Some(token) = self.peek() {
            match token {
                Token::Identifier(name) if name == "camera" => {
                    self.next(); // consume identifier
                    cameras.push(self.parse_camera());
                }
                Token::Identifier(name) if name == "material" => {
                    self.next();
                    let material = self.parse_material();
                    self.materials.push(material);
                }
                Token::Identifier(name) if name == "sphere" => {
                    self.next();
                    spheres.push(self.parse_sphere());
                }
                Token::Identifier(name) if name == "plane" => {
                    self.next();
                    planes.push(self.parse_plane());
                }
                Token::Identifier(name) if name == "triangle" => {
                    self.next();
                    triangles.push(self.parse_triangle());
                }
                Token::Identifier(name) if name == "cube" => {
                    self.next();
                    let cube = self.parse_cube();
                    for tri in cube {
                        triangles.push(tri);
                    }
                }
                Token::Identifier(name) if name == "mesh" => {
                    self.next();
                    let mesh_triangles = self.parse_mesh();
                    for tri in mesh_triangles {
                        triangles.push(tri);
                    }
                }
                Token::Newline => {
                    self.next();
                }
                _ => {
                    panic!("Unexpected token: {:?}", token);
                }
            }
        }

        let scene = Scene::new(cameras[0].clone(), spheres, planes, triangles);
        scene.get_info();
        scene
    }

    /// Parse Camera
    fn parse_camera(&mut self) -> Camera {
        self.expect(&Token::LBrace);
        let mut position = Vect3::ZERO;
        let mut rotation = Quat::identity();
        let mut focal = 1.0;
        let mut fov = Angle::new(0.0);

        while let Some(token) = self.peek() {
            match token {
                Token::Identifier(name) if name == "position" => {
                    self.next();
                    self.expect(&Token::Colon);
                    position = self.parse_vect3();
                }
                Token::Identifier(name) if name == "rotation" => {
                    self.next();
                    self.expect(&Token::Colon);
                    rotation = self.parse_quat();
                }
                Token::Identifier(name) if name == "focal_length" => {
                    self.next();
                    self.expect(&Token::Colon);
                    focal = self.parse_number();
                }
                Token::Identifier(name) if name == "fov" => {
                    self.next();
                    self.expect(&Token::Colon);
                    fov = self.parse_angle();
                }

                Token::RBrace => {
                    self.next();
                    break;
                }
                Token::Newline => {
                    self.next();
                }
                _ => panic!("Unexpected token in camera block: {:?}", token),
            }
        }

        Camera::build(position, rotation, focal, fov, 160, 90)
    }

    /// Parse Material
    fn parse_material(&mut self) -> (String, Material) {
        self.expect(&Token::LBrace);
        let mut name = String::new();
        let mut color = ColorRBG::BLACK;
        let mut emission_color = ColorRBG::BLACK;
        let mut specular_color = ColorRBG::BLACK;
        let mut emission_strength = 0.0;
        let mut smoothness = 0.0;
        let mut specular_prob = 0.0;

        while let Some(token) = self.peek() {
            match token {
                Token::Identifier(n) if n == "name" => {
                    self.next();
                    self.expect(&Token::Colon);
                    name = self.parse_string();
                }
                Token::Identifier(name) if name == "color" => {
                    self.next();
                    self.expect(&Token::Colon);
                    color = self.parse_color();
                }
                Token::Identifier(name) if name == "emission_color" => {
                    self.next();
                    self.expect(&Token::Colon);
                    emission_color = self.parse_color();
                }
                Token::Identifier(name) if name == "specular_color" => {
                    self.next();
                    self.expect(&Token::Colon);
                    specular_color = self.parse_color();
                }
                Token::Identifier(name) if name == "emission_strength" => {
                    self.next();
                    self.expect(&Token::Colon);
                    emission_strength = self.parse_number();
                }
                Token::Identifier(name) if name == "smoothness" => {
                    self.next();
                    self.expect(&Token::Colon);
                    smoothness = self.parse_number();
                }
                Token::Identifier(name) if name == "specular_prob" => {
                    self.next();
                    self.expect(&Token::Colon);
                    specular_prob = self.parse_number();
                }
                Token::RBrace => {
                    self.next();
                    break;
                }
                Token::Newline => {
                    self.next();
                }
                _ => panic!("Unexpected token in material block: {:?}", token),
            }
        }

        (
            name,
            Material::new(
                color,
                emission_color,
                specular_color,
                emission_strength,
                smoothness,
                specular_prob,
            ),
        )
    }

    /// Parse Sphere
    fn parse_sphere(&mut self) -> Sphere {
        self.expect(&Token::LBrace);
        let mut position = Vect3::ZERO;
        let mut radius = 0.0;
        let mut name = String::new();

        while let Some(token) = self.peek() {
            match token {
                Token::Identifier(name) if name == "position" => {
                    self.next();
                    self.expect(&Token::Colon);
                    position = self.parse_vect3();
                }
                Token::Identifier(name) if name == "radius" => {
                    self.next();
                    self.expect(&Token::Colon);
                    radius = self.parse_number();
                }
                Token::Identifier(n) if n == "mat" => {
                    self.next();
                    self.expect(&Token::Colon);
                    name = self.parse_string();
                }
                Token::RBrace => {
                    self.next();
                    break;
                }
                Token::Newline => {
                    self.next();
                }
                _ => panic!("Unexpected token in sphere block: {:?}", token),
            }
        }

        Sphere::new(position, radius, self.get_material(name))
    }

    /// Parse Plane
    fn parse_plane(&mut self) -> Plane {
        self.expect(&Token::LBrace);
        let mut point = Vect3::ZERO;
        let mut normal = Vect3::ZERO;
        let mut name = String::new();

        while let Some(token) = self.peek() {
            match token {
                Token::Identifier(name) if name == "point" => {
                    self.next();
                    self.expect(&Token::Colon);
                    point = self.parse_vect3();
                }
                Token::Identifier(name) if name == "normal" => {
                    self.next();
                    self.expect(&Token::Colon);
                    normal = self.parse_vect3();
                }
                Token::Identifier(n) if n == "mat" => {
                    self.next();
                    self.expect(&Token::Colon);
                    name = self.parse_string();
                }
                Token::RBrace => {
                    self.next();
                    break;
                }
                Token::Newline => {
                    self.next();
                }
                _ => panic!("Unexpected token in sphere block: {:?}", token),
            }
        }

        Plane::new(point, normal, self.get_material(name))
    }

    /// Parse Triangle
    fn parse_triangle(&mut self) -> Triangle {
        self.expect(&Token::LBrace);
        let mut point_1 = Vect3::ZERO;
        let mut point_2 = Vect3::ZERO;
        let mut point_3 = Vect3::ZERO;
        let mut name = String::new();

        while let Some(token) = self.peek() {
            match token {
                Token::Identifier(name) if name == "point_1" => {
                    self.next();
                    self.expect(&Token::Colon);
                    point_1 = self.parse_vect3();
                }
                Token::Identifier(name) if name == "point_2" => {
                    self.next();
                    self.expect(&Token::Colon);
                    point_2 = self.parse_vect3();
                }
                Token::Identifier(name) if name == "point_3" => {
                    self.next();
                    self.expect(&Token::Colon);
                    point_3 = self.parse_vect3();
                }
                Token::Identifier(n) if n == "mat" => {
                    self.next();
                    self.expect(&Token::Colon);
                    name = self.parse_string();
                }
                Token::RBrace => {
                    self.next();
                    break;
                }
                Token::Newline => {
                    self.next();
                }
                _ => panic!("Unexpected token in sphere block: {:?}", token),
            }
        }

        Triangle::new(point_1, point_2, point_3, self.get_material(name))
    }

    /// Parse Cube
    fn parse_cube(&mut self) -> Vec<Triangle> {
        self.expect(&Token::LBrace);
        let mut position = Vect3::ZERO;
        let mut rotation = Quat::identity();
        let mut size = 0.0;
        let mut name = String::new();

        while let Some(token) = self.peek() {
            match token {
                Token::Identifier(name) if name == "position" => {
                    self.next();
                    self.expect(&Token::Colon);
                    position = self.parse_vect3();
                }
                Token::Identifier(name) if name == "rotation" => {
                    self.next();
                    self.expect(&Token::Colon);
                    rotation = self.parse_quat();
                }
                Token::Identifier(name) if name == "size" => {
                    self.next();
                    self.expect(&Token::Colon);
                    size = self.parse_number();
                }
                Token::Identifier(n) if n == "mat" => {
                    self.next();
                    self.expect(&Token::Colon);
                    name = self.parse_string();
                }
                Token::RBrace => {
                    self.next();
                    break;
                }
                Token::Newline => {
                    self.next();
                }
                _ => panic!("Unexpected token in sphere block: {:?}", token),
            }
        }

        create_cube_triangles(position, rotation, size, self.get_material(name))
    }

    /// Parse mesh
    fn parse_mesh(&mut self) -> Vec<Triangle> {
        self.expect(&Token::LBrace);
        let mut position = Vect3::ZERO;
        let mut rotation = Quat::identity();
        let mut triangles: Vec<Triangle> = vec![];
        let mut mat_name = String::new();

        while let Some(token) = self.peek() {
            match token {
                Token::Identifier(name) if name == "position" => {
                    self.next();
                    self.expect(&Token::Colon);
                    position = self.parse_vect3();
                }
                Token::Identifier(name) if name == "rotation" => {
                    self.next();
                    self.expect(&Token::Colon);
                    rotation = self.parse_quat();
                }
                Token::Identifier(name) if name == "obj_file" => {
                    self.next();
                    self.expect(&Token::Colon);
                    let file_name = self.parse_string();
                    let mut obj_parser = Parser::build(&file_name).unwrap();
                    triangles = obj_parser.parse_obj();
                }
                Token::Identifier(n) if n == "mat" => {
                    self.next();
                    self.expect(&Token::Colon);
                    mat_name = self.parse_string();
                }
                Token::RBrace => {
                    self.next();
                    break;
                }
                Token::Newline => {
                    self.next();
                }
                _ => panic!("Unexpected token in camera block: {:?}", token),
            }
        }

        let mat = self.get_material(mat_name);

        for triangle in triangles.iter_mut() {
            triangle.rotate(rotation, position);
            triangle.set_material(mat);
        }

        triangles
    }

    // Small parsers

    /// Parse .obj file
    pub fn parse_obj(&mut self) -> Vec<Triangle> {
        let mut vertices: Vec<Vect3> = vec![];
        let mut triangles: Vec<Triangle> = vec![];

        while let Some(token) = self.peek() {
            match token {
                Token::Identifier(name) if name == "v" => {
                    self.next();
                    vertices.push(self.parse_vect3());
                }
                Token::Identifier(name) if name == "f" => {
                    self.next();
                    let face = self.parse_face();
                    let point_1 = vertices[face[0] - 1];
                    let point_2 = vertices[face[2] - 1];
                    let point_3 = vertices[face[1] - 1];
                    triangles.push(Triangle::new(
                        point_1,
                        point_2,
                        point_3,
                        Material::default(),
                    ));
                }
                Token::Newline => {
                    self.next();
                }
                _ => {
                    panic!("Unexpected token: {:?}", token);
                }
            }
        }

        triangles
    }

    /// Parse f64 array
    fn parse_f64_array(&mut self, count: usize) -> [f64; 4] {
        let mut result = [0.0; 4];
        for res in result.iter_mut().take(count) {
            *res = self.parse_number();
        }
        result
    }

    /// Parse Vect3
    fn parse_vect3(&mut self) -> Vect3 {
        let data = self.parse_f64_array(3);
        Vect3::new(data[0], data[1], data[2])
    }

    /// Parse obj face
    fn parse_face(&mut self) -> [usize; 3] {
        let mut face = [0; 3];
        for res in face.iter_mut().take(3) {
            *res = self.parse_int();
        }
        face
    }

    /// Parse Quat
    fn parse_quat(&mut self) -> Quat {
        let data = self.parse_f64_array(4);
        Quat::new(data[0], Vect3::new(data[1], data[2], data[3]))
    }

    /// Parse f64 number
    fn parse_number(&mut self) -> f64 {
        match self.next() {
            Some(Token::Number(n)) => *n,
            other => panic!("Expected number, got {:?}", other),
        }
    }

    /// Parse usize number
    fn parse_int(&mut self) -> usize {
        match self.next() {
            Some(Token::Number(n)) => *n as usize,
            other => panic!("Expected number, got {:?}", other),
        }
    }

    /// Parse ColorRBG
    fn parse_color(&mut self) -> ColorRBG {
        let data = self.parse_f64_array(3);
        ColorRBG::new(data[0], data[1], data[2])
    }

    /// Parse Angle
    fn parse_angle(&mut self) -> Angle {
        Angle::from_deg(self.parse_number())
    }

    /// Parse String
    fn parse_string(&mut self) -> String {
        match self.next() {
            Some(Token::Identifier(name)) => String::from(name),
            other => panic!("Expected material name, got {:?}", other),
        }
    }

    // Helpers

    /// Get material from its name
    fn get_material(&self, material_name: String) -> Material {
        if let Some(material) = self
            .materials
            .iter()
            .find(|(name, _)| *name == *material_name)
            .map(|(_, mat)| mat)
        {
            *material
        } else {
            Material::default()
        }
    }
}
