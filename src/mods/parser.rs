use std::{fs, vec};

use super::{
    color::ColorRBG,
    objs::{Cube, Object3D, Plane, Sphere, Triangle},
    position::{Angle, Quat, Vect3},
    render::{Camera, Light, Material, PointLight, Scene},
};

// Tokeneiser stuff

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Identifier(String),
    Number(f64),
    Colon,
    LBrace, // {
    RBrace, // }
    Newline,
}

// Parser stuff

pub struct Parser {
    tokens: Vec<Token>,
    materials: Vec<(String, Material)>,
    pos: usize,
}

impl Parser {
    pub fn new(filename: &str) -> Parser {
        println!("== Parsing file : {}", filename);
        let mut parser = Parser {
            tokens: vec![],
            materials: vec![],
            pos: 0,
        };
        // TODO : Ajouter une gestion des erreurs plus solide
        let input = fs::read_to_string(filename).unwrap();
        let slice: &str = &input;
        parser.tokenize(slice);
        parser
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos);
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: &Token) {
        let token = self.next().expect("Unexpected end of input");
        if token != expected {
            panic!("Expected {:?}, got {:?}", expected, token);
        }
    }

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
                        if c.is_alphanumeric() || c == '_' {
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

    pub fn parse_scene(&mut self) -> Scene {
        let mut cameras: Vec<Camera> = vec![];
        let mut lights: Vec<Box<dyn Light>> = vec![];
        let mut objects: Vec<Box<dyn Object3D>> = vec![];

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
                Token::Identifier(name) if name == "point_light" => {
                    self.next();
                    lights.push(Box::new(self.parse_light()));
                }
                Token::Identifier(name) if name == "sphere" => {
                    self.next();
                    objects.push(Box::new(self.parse_sphere()));
                }
                Token::Identifier(name) if name == "plane" => {
                    self.next();
                    objects.push(Box::new(self.parse_plane()));
                }
                Token::Identifier(name) if name == "triangle" => {
                    self.next();
                    objects.push(Box::new(self.parse_triangle()));
                }
                Token::Identifier(name) if name == "cube" => {
                    self.next();
                    objects.push(Box::new(self.parse_cube()));
                }
                Token::Newline => {
                    self.next();
                }
                _ => {
                    panic!("Unexpected token: {:?}", token);
                }
            }
        }

        Scene::new(cameras[0].clone(), objects, lights) // Il faudrait trouver un moyen de
                                                        // supprimer le clone.
    }

    fn parse_camera(&mut self) -> Camera {
        self.expect(&Token::LBrace);
        let mut position = Vect3::ZERO;
        let mut rotation = Quat::identity();
        let mut focal = 1.0;
        let mut fov = Angle::new(0.0);
        let mut resolution = (800, 600);

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
                Token::Identifier(name) if name == "resolution" => {
                    self.next();
                    self.expect(&Token::Colon);
                    let vals = self.parse_f64_array(2);
                    resolution = (vals[0] as u32, vals[1] as u32);
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

        Camera::build(position, rotation, focal, fov, resolution.0, resolution.1)
    }

    fn parse_f64_array(&mut self, count: usize) -> [f64; 4] {
        let mut result = [0.0; 4];
        for res in result.iter_mut().take(count) {
            *res = self.parse_number();
        }
        result
    }

    fn parse_vect3(&mut self) -> Vect3 {
        let data = self.parse_f64_array(3);
        Vect3::new(data[0], data[1], data[2])
    }

    fn parse_quat(&mut self) -> Quat {
        let data = self.parse_f64_array(4);
        Quat::new(data[0], Vect3::new(data[1], data[2], data[3]))
    }

    fn parse_number(&mut self) -> f64 {
        match self.next() {
            Some(Token::Number(n)) => *n,
            other => panic!("Expected number, got {:?}", other),
        }
    }

    fn parse_color(&mut self) -> ColorRBG {
        let data = self.parse_f64_array(3);
        ColorRBG::new(data[0], data[1], data[2])
    }

    fn parse_angle(&mut self) -> Angle {
        Angle::from_deg(self.parse_number())
    }

    fn parse_string(&mut self) -> String {
        match self.next() {
            Some(Token::Identifier(name)) => String::from(name),
            other => panic!("Expected material name, got {:?}", other),
        }
    }

    fn parse_light(&mut self) -> PointLight {
        self.expect(&Token::LBrace);
        let mut position = Vect3::ZERO;
        let mut color = ColorRBG::BLACK;

        while let Some(token) = self.peek() {
            match token {
                Token::Identifier(name) if name == "position" => {
                    self.next();
                    self.expect(&Token::Colon);
                    position = self.parse_vect3();
                }
                Token::Identifier(name) if name == "color" => {
                    self.next();
                    self.expect(&Token::Colon);
                    color = self.parse_color();
                }
                Token::RBrace => {
                    self.next();
                    break;
                }
                Token::Newline => {
                    self.next();
                }
                _ => panic!("Unexpected token in light block: {:?}", token),
            }
        }

        PointLight::new(position, color)
    }

    fn parse_material(&mut self) -> (String, Material) {
        self.expect(&Token::LBrace);
        let mut name = String::new();
        let mut ambient = ColorRBG::BLACK;
        let mut diffuse = ColorRBG::BLACK;
        let mut specular = ColorRBG::BLACK;
        let mut shininess = 30.0;
        while let Some(token) = self.peek() {
            match token {
                Token::Identifier(n) if n == "name" => {
                    self.next();
                    self.expect(&Token::Colon);
                    name = self.parse_string();
                }
                Token::Identifier(name) if name == "ambient" => {
                    self.next();
                    self.expect(&Token::Colon);
                    ambient = self.parse_color();
                }
                Token::Identifier(name) if name == "diffuse" => {
                    self.next();
                    self.expect(&Token::Colon);
                    diffuse = self.parse_color();
                }
                Token::Identifier(name) if name == "specular" => {
                    self.next();
                    self.expect(&Token::Colon);
                    specular = self.parse_color();
                }
                Token::Identifier(name) if name == "shininess" => {
                    self.next();
                    self.expect(&Token::Colon);
                    shininess = self.parse_number();
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

        (name, Material::new(ambient, diffuse, specular, shininess))
    }

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
                    //color = self.parse_color();
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

        if let Some(material) = self
            .materials
            .iter()
            .find(|(mat_name, _)| *mat_name == *name)
            .map(|(_, mat)| mat)
        {
            Sphere::new(position, radius, *material)
        } else {
            let material = Material::new(
                0.3 * ColorRBG::WHITE,
                ColorRBG::WHITE,
                1.0 * ColorRBG::WHITE,
                33.0,
            );
            println!("Matériau introuvable");
            Sphere::new(position, radius, material)
        }
    }

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
                    //color = self.parse_color();
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

        if let Some(material) = self
            .materials
            .iter()
            .find(|(mat_name, _)| *mat_name == *name)
            .map(|(_, mat)| mat)
        {
            Plane::new(point, normal, *material)
        } else {
            let material = Material::new(
                0.3 * ColorRBG::WHITE,
                ColorRBG::WHITE,
                1.0 * ColorRBG::WHITE,
                33.0,
            );
            println!("Matériau introuvable");
            Plane::new(point, normal, material)
        }
    }

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
                    //color = self.parse_color();
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

        if let Some(material) = self
            .materials
            .iter()
            .find(|(mat_name, _)| *mat_name == *name)
            .map(|(_, mat)| mat)
        {
            Triangle::new(point_1, point_2, point_3, *material)
        } else {
            let material = Material::new(
                0.3 * ColorRBG::WHITE,
                ColorRBG::WHITE,
                1.0 * ColorRBG::WHITE,
                33.0,
            );
            println!("Matériau introuvable");
            Triangle::new(point_1, point_2, point_3, material)
        }
    }

    fn parse_cube(&mut self) -> Cube {
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
                    //color = self.parse_color();
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

        if let Some(material) = self
            .materials
            .iter()
            .find(|(mat_name, _)| *mat_name == *name)
            .map(|(_, mat)| mat)
        {
            Cube::new(position, rotation, size, *material)
        } else {
            let material = Material::new(
                0.3 * ColorRBG::WHITE,
                ColorRBG::WHITE,
                1.0 * ColorRBG::WHITE,
                33.0,
            );
            println!("Matériau introuvable");
            Cube::new(position, rotation, size, material)
        }
    }
}
