pub fn is_in_circle(pos: &(i64, i64), r: &i64, x: &i64, y: &i64) -> bool {
    if (x - pos.0) * (x - pos.0) + (y - pos.1) * (y - pos.1) < (*r * *r) {
        true
    } else {
        false
    }
}

pub fn gradient_from_center(pos: &(i64, i64), r: &i64, x: &i64, y: &i64) -> u8 {
    if (x - pos.0) * (x - pos.0) + (y - pos.1) * (y - pos.1) < (*r * *r) {
        // TODO : Trouver une meilleure formule pour faire un gradient (racine carrée)
        let dist = ((x - pos.0) * (x - pos.0) + (y - pos.1) * (y - pos.1)) / *r;
        255 - dist as u8
    } else {
        0
    }
}
