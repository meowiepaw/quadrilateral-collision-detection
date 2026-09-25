#[derive(PartialEq)]
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
    fn get_collision_points(quad_a: &Quadrilateral, quad_b: &Quadrilateral) -> (bool, Vec<Point>) {
        let quad_a_linears = quad_a.get_linears();
        let quad_b_linears = quad_b.get_linears();
        let mut collision_points: Vec<Point> = Vec::new();
        let mut collision_detected = false;
        for a in &quad_a_linears {
            for b in &quad_b_linears {
                let is_colliding = Linear::get_collision(a, b);
                if is_colliding.0 {
                    collision_detected = true;
                    match is_colliding.1 {
                        None => (),
                        Some(value) => {
                            if !collision_points.contains(&value) {
                                collision_points.push(value)
                            }
                        },
                    }
                }
            }
        }
        return (collision_detected, collision_points);
    }
    fn check_collision(quad_a: &Quadrilateral, quad_b: &Quadrilateral) -> (bool, Option<Point>) {
        let quad_a_linears = quad_a.get_linears();
        let quad_b_linears = quad_b.get_linears();
        for a in &quad_a_linears {
            for b in &quad_b_linears {
                let is_colliding = Linear::get_collision(a, b);
                if is_colliding.0 {
                    return (true, is_colliding.1);
                }
            }
        }
        (false, None)
    }
}

struct Linear {
    m: LinearM,
    t: LinearT,
    dlimits: (f64, f64),
    wlimits: (f64, f64),
}

enum LinearOverlapValue {
    One(Point),
    All,
    None,
}

enum LinearT {
    ValueT(f64),
    ValueX(f64),
}

enum LinearM {
    Value(f64),
    Vertical,
    Horizontal,
}

impl Linear {
    fn new(m: LinearM, t: LinearT, dlimits: (f64, f64), wlimits: (f64, f64)) -> Linear {
        Linear { m, t, dlimits, wlimits }
    }
    fn from_points(point_a: &Point, point_b: &Point) -> Linear {
        let height_difference = point_a.y - point_b.y;
        let length_difference = point_a.x - point_b.x;
        let fp_linear_m: LinearM;
        let fp_linear_t ;
        if length_difference == 0f64 {
            fp_linear_m = LinearM::Vertical;
            fp_linear_t = LinearT::ValueX(point_a.x);
        } else if height_difference == 0f64 {
            fp_linear_m = LinearM::Horizontal;
            fp_linear_t = LinearT::ValueT(point_a.y);
        } else {
            let temp_m = height_difference / length_difference;
            fp_linear_m = LinearM::Value(temp_m);
            fp_linear_t = LinearT::ValueT(point_a.y - (temp_m * point_a.x));
        }
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
    fn calc_overlap_hv(linear_a: &Linear, linear_b: &Linear) -> LinearOverlapValue {
        let linear_a_t = match linear_a.t {
            LinearT::ValueT(value) => value,
            _ => 0f64,
        };
        let linear_b_x = match linear_b.t {
            LinearT::ValueX(value) => value,
            _ => 0f64,
        };
        return LinearOverlapValue::One(Point::new(linear_b_x, linear_a_t));
    }
    fn calc_overlap_hx(linear_a: &Linear, linear_b: &Linear) -> LinearOverlapValue {
        let overlap_y = match linear_a.t {
            LinearT::ValueT(value) => value,
            _ => 0f64,
        };
        let linear_b_m = match linear_b.m {
            LinearM::Value(value) => value,
            _ => 0f64
        };
        let linear_b_t = match linear_b.t {
            LinearT::ValueT(value) => value,
            _ => 0f64,
        };
        let overlap_x = (overlap_y - linear_b_t) / linear_b_m;
        return LinearOverlapValue::One(Point::new(overlap_x, overlap_y));
    }
    fn calc_overlap_vx(linear_a: &Linear, linear_b: &Linear) -> LinearOverlapValue {
        let overlap_x = match linear_a.t {
            LinearT::ValueX(value) => value,
            _ => 0f64,
        };
        let linear_b_m = match linear_b.m {
            LinearM::Value(value) => value,
            _ => 0f64
        };
        let linear_b_t = match linear_b.t {
            LinearT::ValueT(value) => value,
            _ => 0f64,
        };
        let overlap_y = (linear_b_m * overlap_x) + linear_b_t;
        return LinearOverlapValue::One(Point::new(overlap_x, overlap_y));
    }
    fn get_overlap(linear_a: &Linear, linear_b: &Linear) -> LinearOverlapValue {
        match linear_a.m {
            LinearM::Horizontal => {
                match linear_b.m {
                    LinearM::Horizontal => {
                        let value_t_a = match linear_a.t {
                            LinearT::ValueT(value) => value,
                            _ => 0f64
                        };
                        let value_t_b = match linear_b.t {
                            LinearT::ValueT(value) => value,
                            _ => 0f64,
                        };
                        if value_t_a == value_t_b {
                            return LinearOverlapValue::All;
                        }
                        return LinearOverlapValue::None;
                    }
                    LinearM::Vertical => {return Linear::calc_overlap_hv(&linear_a, &linear_b);}
                    LinearM::Value(_) => {return Linear::calc_overlap_hx(&linear_a, &linear_b);}
                }
            }
            LinearM::Vertical => {
                match linear_b.m {
                    LinearM::Horizontal => {return Linear::calc_overlap_hv(&linear_b, &linear_a);}
                    LinearM::Vertical => {
                        let value_x_a = match linear_a.t {
                            LinearT::ValueX(value) => value,
                            _ => 0f64,
                        };
                        let value_x_b = match linear_b.t {
                            LinearT::ValueX(value) => value,
                            _ => 0f64,
                        };
                        if value_x_a == value_x_b {
                            return LinearOverlapValue::All;
                        }
                        return LinearOverlapValue::None;
                    }
                    LinearM::Value(_) => {return Linear::calc_overlap_vx(&linear_a, &linear_b);}
                }
            }
            LinearM::Value(value_m_a) => {
                match linear_b.m {
                    LinearM::Horizontal => {return Linear::calc_overlap_hx(&linear_b, &linear_a);}
                    LinearM::Vertical => {return Linear::calc_overlap_vx(&linear_b, &linear_a);}
                    LinearM::Value(value_m_b) => {
                        let linear_m_subtract = value_m_a - value_m_b; // m1x1 + t1 = m2x2 + t >>> mTxT + tT = 0
                        let linear_t_a = match linear_a.t {
                            LinearT::ValueT(value) => value,
                            _ => 0f64,
                        };
                        let linear_t_b = match linear_b.t {
                            LinearT::ValueT(value) => value,
                            _ => 0f64,
                        };
                        let linear_t_subtract = linear_t_a - linear_t_b; // m1x1 + t1 = m2x2 + t >>> mTxT + tT = 0
                        let overlap_x = -(linear_t_subtract / linear_m_subtract); // mx + t = 0 >>> x = -t/m
                        let overlap_y = (value_m_a * overlap_x) + linear_t_a; // y = mx + t
                        return LinearOverlapValue::One(Point::new(overlap_x, overlap_y));
                    }
                }
            }
        };
    }
    fn is_point_in_bb(&self, point: &Point) -> bool {
        if point.x >= self.dlimits.0 && point.x <= self.dlimits.1 && point.y >= self.wlimits.0 && point.y <= self.wlimits.1 {
            return true;
        }
        false
    }
    fn get_collision(linear_a: &Linear, linear_b: &Linear) -> (bool, Option<Point>) {
        let overlap = Linear::get_overlap(linear_a, linear_b);
        let overlap_point = match overlap {
            LinearOverlapValue::All => {return (true, None);}
            LinearOverlapValue::None => {return (false, None);}
            LinearOverlapValue::One(value) => value
        };
        if linear_a.is_point_in_bb(&overlap_point) && linear_b.is_point_in_bb(&overlap_point) {
            return (true, Some(overlap_point));
        }
        (false, None)
    }
    fn get_desmos_draw(&self) -> String {
        match &self.m {
            LinearM::Horizontal => {
                return format!("y = {}", match &self.t {LinearT::ValueT(value) => value, _ => &0f64});
            }
            LinearM::Vertical => {
                return format!("x = {}", match &self.t {LinearT::ValueX(value) => value, _ => &0f64});
            }
            LinearM::Value(value) => {
                return format!("y = {}x + {}", value, match &self.t {LinearT::ValueT(value) => value, _ => &0f64})
            }
        }
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
    let square_1 = Quadrilateral::new(
        Point::new(0f64, 0f64),
        Point::new(5f64, 1f64), 
        Point::new(6f64, 9f64),
        Point::new(2f64, 6f64),
    );
    let square_2 = Quadrilateral::new_square(Point::new(4f64, 4f64), 6f64);
    let sq1sq2_collisions = Quadrilateral::get_collision_points(&square_1, &square_2);
    
    println!("is_col? {} times col: {}", sq1sq2_collisions.0, sq1sq2_collisions.1.len());
    
    draw_list.push(Object::Quadrilateral(&square_1));
    draw_list.push(Object::Quadrilateral(&square_2));

    for x in &sq1sq2_collisions.1 {
        println!("{}", x.get_desmos_draw());
    }
    for x in &square_1.get_linears() {
        println!("{}", x.get_desmos_draw());
    }
    for x in &square_2.get_linears() {
        println!("{}", x.get_desmos_draw());
    }

    draw_all(&draw_list);
}
