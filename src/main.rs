struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
    fn get_desmos_draw(&self) -> String {
        return format!("({},{})", self.x, self.y);
    }
}

struct Quadrilateral {
    a: Point,
    b: Point,
    c: Point,
    d: Point,
}

impl Quadrilateral {
    fn new(a: Point, b: Point, c: Point, d: Point) -> Quadrilateral {
        Quadrilateral { a, b, c, d }
    }
    fn get_desmos_draw(&self) -> String {
        return format!("polygon(({},{}),({},{}),({},{}),({},{}))", self.a.x, self.a.y, self.b.x, self.b.y, self.c.x, self.c.y, self.d.x, self.d.y);
    }
    fn new_square(center: Point, size: f64) -> Quadrilateral {
        let x = center.x;
        let y = center.y;
        let half_size = size / 2f64;
        Quadrilateral::new(
            Point::new(
                x - half_size,
                y - half_size
            ),
            Point::new(
                x + half_size,
                y - half_size
            ),
            Point::new(
                x + half_size,
                y + half_size
            ),
            Point::new(
                x - half_size,
                y + half_size
            )
        )
        
    }
    fn get_linears(&self) -> Vec<Linear> {
        let mut linears: Vec<Linear> = Vec::new();
        linears.push(Linear::from_points(&self.a, &self.b));
        linears.push(Linear::from_points(&self.b, &self.c));
        linears.push(Linear::from_points(&self.c, &self.d));
        linears.push(Linear::from_points(&self.d, &self.a));
        linears
    }
    fn check_collision(quad_a: Quadrilateral, quad_b: Quadrilateral) -> bool {
        let quad_a_linears = quad_a.get_linears();
        let quad_b_linears = quad_b.get_linears();
        for a in &quad_a_linears {
            for b in &quad_b_linears {
                if Linear::check_collision(a, b) {
                    return true;
                }
            }
        }
        false
    }
}

struct Linear {
    m: f64,
    t: f64,
    dlimits: (f64, f64),
    wlimits: (f64, f64),
}

enum LinearOverlapValue {
    One(Point),
    All,
    None,
}

impl Linear {
    fn new(m: f64, t: f64, dlimits: (f64, f64), wlimits: (f64, f64)) -> Linear {
        Linear { m, t, dlimits, wlimits }
    }
    fn from_points(point_a: &Point, point_b: &Point) -> Linear {
        let height_difference = point_a.y - point_b.y;
        let length_difference = point_a.x - point_b.x;
        let fp_linear_m = height_difference / length_difference;
        let fp_linear_t = point_a.y - (fp_linear_m * point_a.x);
        let fp_linear_dlimit_s: f64;
        let fp_linear_dlimit_b: f64;
        if point_a.x <= point_b.x {
            fp_linear_dlimit_s = point_a.x;
            fp_linear_dlimit_b = point_b.x;
        } else {
            fp_linear_dlimit_s = point_b.x;
            fp_linear_dlimit_b = point_a.x;
        }
        let fp_linear_wlimit_s: f64;
        let fp_linear_wlimit_b: f64;
        if point_a.y <= point_b.y {
            fp_linear_wlimit_s = point_a.y;
            fp_linear_wlimit_b = point_b.y;
        } else {
            fp_linear_wlimit_s = point_b.y;
            fp_linear_wlimit_b = point_a.y;
        }

        Linear::new(fp_linear_m, fp_linear_t, (fp_linear_dlimit_s, fp_linear_dlimit_b), (fp_linear_wlimit_s, fp_linear_wlimit_b))
    }
    fn get_overlap(linear_a: &Linear, linear_b: &Linear) -> LinearOverlapValue {
        if linear_a.m == linear_b.m {
            if linear_a.t == linear_b.t {
                return LinearOverlapValue::All;
            } else {
                return LinearOverlapValue::None;
            }
        }
        let linear_subt_m = linear_a.m - linear_b.m;
        let linear_subt_t = linear_a.t - linear_b.t;
        let overlap_x = -(linear_subt_t / linear_subt_m);
        let overlap_y = (linear_a.m * overlap_x) + linear_a.t;
        LinearOverlapValue::One(Point::new(overlap_x, overlap_y))
    }
    fn is_point_in_bb(&self, point: &Point) -> bool {
        if point.x >= self.dlimits.0 && point.x <= self.dlimits.1 {
            return true;
        }
        false
    }
    fn check_collision(linear_a: &Linear, linear_b: &Linear) -> bool {
        let overlap = Linear::get_overlap(linear_a, linear_b);
        let overlap_point = match overlap {
            LinearOverlapValue::All => {return true;}
            LinearOverlapValue::None => {return false;}
            LinearOverlapValue::One(value) => value
        };
        if linear_a.is_point_in_bb(&overlap_point) && linear_b.is_point_in_bb(&overlap_point) {
            return true;
        }
        false
    }
    fn get_desmos_draw(&self) -> String {
        return format!("y = {}x + {}", self.m, self.t);
    }
}

enum Object<'a> {
    Point(&'a Point),
    Linear(&'a Linear),
    Quadrilateral(&'a Quadrilateral),
}

fn draw_all(draw_list: &Vec<Object>) {
    for object in draw_list {
        match object {
            &Object::Linear(value) => {println!("{}", value.get_desmos_draw())}
            &Object::Point(value) => {println!("{}", value.get_desmos_draw())}
            &Object::Quadrilateral(value) => {println!("{}", value.get_desmos_draw())}
        }
    }
}

fn main() {
    let mut draw_list: Vec<Object> = Vec::new();
    let square_1 = Quadrilateral::new_square(Point::new(2f64, 2f64), 4f64);
    let square_2 = Quadrilateral::new_square(Point::new(4f64, 4f64), 6f64);
    
    draw_list.push(Object::Quadrilateral(&square_1));
    draw_list.push(Object::Quadrilateral(&square_2));

    draw_all(&draw_list);
}
