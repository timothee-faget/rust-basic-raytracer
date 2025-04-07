use std::{fs, vec};

use super::{
    color::ColorRBG,
    objs::{Object3D, Sphere},
    position::{Angle, Quat, Vect3},
    render::{Camera, Light, Scene},
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
    pos: usize,
}

impl Parser {
    pub fn new(filename: &str) -> Parser {
        let mut parser = Parser {
            tokens: vec![],
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
        let mut lights: Vec<Light> = vec![];
        let mut objects: Vec<Box<dyn Object3D>> = vec![];

        while let Some(token) = self.peek() {
            match token {
                Token::Identifier(name) if name == "camera" => {
                    self.next(); // consume identifier
                    cameras.push(self.parse_camera());
                }
                Token::Identifier(name) if name == "light" => {
                    self.next();
                    lights.push(self.parse_light());
                }
                Token::Identifier(name) if name == "sphere" => {
                    self.next();
                    objects.push(Box::new(self.parse_sphere()));
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

    fn parse_light(&mut self) -> Light {
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

        Light::new(position, color)
    }

    fn parse_sphere(&mut self) -> Sphere {
        self.expect(&Token::LBrace);
        let mut position = Vect3::ZERO;
        let mut radius = 0.0;
        let mut color = ColorRBG::BLACK;

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
                _ => panic!("Unexpected token in sphere block: {:?}", token),
            }
        }

        Sphere::new(position, radius, color)
    }
}
