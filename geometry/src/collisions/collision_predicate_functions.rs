use core::f32;

use crate::{
    comparators::{are_equal, less_or_equal},
    point::{Point2D, dist2},
    shapes::{
        affine::{EmbedInAffine2D, TypedAffine2D},
        circle::Circle,
        line::Line2D,
        rectangle::Rectangle,
        segment::Segment2D,
    },
    vector::Vector2D,
};

pub fn check_collision_point_point(p1: &Point2D, p2: &Point2D) -> bool {
    are_equal(p1.x, p2.x) && are_equal(p1.y, p2.y)
}

pub fn check_collision_line_point(l: &TypedAffine2D<Line2D>, point: &Point2D) -> bool {
    let angle = &l.shape.angle;
    if are_equal(point.x, l.point.x) {
        let norm_angle_radians = angle.normalized().as_radians();
        return are_equal(point.y, l.point.y)
            || are_equal(norm_angle_radians, f32::consts::FRAC_PI_2)
            || are_equal(norm_angle_radians, 3.0 * f32::consts::FRAC_PI_2);
    }
    let point_vec = point.clone() - l.point;
    angle.is_inline_with(&point_vec.get_angle())
}

pub fn check_collision_segment_point(s: &TypedAffine2D<Segment2D>, point: &Point2D) -> bool {
    let line = TypedAffine2D::new(s.point, s.shape.get_line());
    check_collision_line_point(&line, point)
        && less_or_equal(
            dist2(point, &s.point),
            s.shape.length * s.shape.length / 4.0,
        )
}

pub fn check_collision_circle_point(c: &TypedAffine2D<Circle>, point: &Point2D) -> bool {
    less_or_equal(dist2(point, &c.point), c.shape.radius * c.shape.radius)
}

pub fn check_collision_rect_point(r: &TypedAffine2D<Rectangle>, point: &Point2D) -> bool {
    let left = r.point.x - r.shape.width / 2.0;
    let top = r.point.y - r.shape.height / 2.0;
    (left..=left + r.shape.width).contains(&point.x)
        && (top..=top + r.shape.height).contains(&point.y)
}

pub fn check_collision_line_line(l1: &TypedAffine2D<Line2D>, l2: &TypedAffine2D<Line2D>) -> bool {
    if !l1.shape.angle.is_inline_with(&l2.shape.angle) {
        return true;
    }
    let normalized_point_vec = (l1.point - l2.point).normalized();
    let line_vec = Vector2D::by_angle(&l1.shape.angle);
    match normalized_point_vec {
        Some(v) => v == line_vec || v == -line_vec,
        None => l1.point == l2.point,
    }
}

pub fn check_collision_line_segment(
    l: &TypedAffine2D<Line2D>,
    s: &TypedAffine2D<Segment2D>,
) -> bool {
    let p = l.get_intersection_point(&TypedAffine2D {
        point: s.point,
        shape: s.shape.get_line(),
    });
    match p {
        Some(p) => check_collision_segment_point(&s, &p),
        None => check_collision_line_point(&l, &s.point),
    }
}

pub fn check_collision_segment_segment(
    s1: &TypedAffine2D<Segment2D>,
    s2: &TypedAffine2D<Segment2D>,
) -> bool {
    let int_point = s1.get_line().get_intersection_point(&s2.get_line());
    match int_point {
        Some(int_point) => {
            check_collision_segment_point(s1, &int_point)
                && check_collision_segment_point(s2, &int_point)
        }
        None => {
            s1.is_inline_with(s2)
                && less_or_equal(
                    dist2(&s1.point, &s2.point),
                    f32::powi(s1.shape.length / 2.0 + s2.shape.length / 2.0, 2),
                )
        }
    }
}

pub fn check_collision_rect_line(r: &TypedAffine2D<Rectangle>, l: &TypedAffine2D<Line2D>) -> bool {
    r.get_segments()
        .iter()
        .any(|s| check_collision_line_segment(l, &s))
}

pub fn check_collision_rect_segment(
    r: &TypedAffine2D<Rectangle>,
    s: &TypedAffine2D<Segment2D>,
) -> bool {
    let segment_cross_border = r
        .get_segments()
        .iter()
        .any(|rect_segment| check_collision_segment_segment(rect_segment, s));
    segment_cross_border || check_collision_rect_point(r, &s.point)
}

pub fn check_collision_circle_line(c: &TypedAffine2D<Circle>, l: &TypedAffine2D<Line2D>) -> bool {
    less_or_equal(l.dist2(&c.point), f32::powi(c.shape.radius, 2))
}

pub fn check_collision_circle_segment(
    c: &TypedAffine2D<Circle>,
    s: &TypedAffine2D<Segment2D>,
) -> bool {
    let perp = s
        .get_line()
        .shape
        .get_perpendicular()
        .embed_affine(&c.point);
    let int_point = perp
        .get_intersection_point(&s.get_line())
        .expect("perpendicular lines should intersect");
    if check_collision_segment_point(s, &int_point) {
        less_or_equal(dist2(&int_point, &c.point), f32::powi(c.shape.radius, 2))
    } else {
        s.get_end_points()
            .iter()
            .any(|end| less_or_equal(dist2(end, &c.point), f32::powi(c.shape.radius, 2)))
    }
}

pub fn check_collision_circle_rect(
    c: &TypedAffine2D<Circle>,
    r: &TypedAffine2D<Rectangle>,
) -> bool {
    let closest_point = Point2D {
        x: f32::clamp(c.point.x, r.get_left(), r.get_right()),
        y: f32::clamp(c.point.y, r.get_bottom(), r.get_top()),
    };
    check_collision_circle_point(c, &closest_point)
}
pub fn check_collision_circle_circle(
    c1: &TypedAffine2D<Circle>,
    c2: &TypedAffine2D<Circle>,
) -> bool {
    less_or_equal(
        dist2(&c1.point, &c2.point),
        f32::powi(c1.shape.radius + c2.shape.radius, 2),
    )
}

pub fn check_collision_rect_rect(
    r1: &TypedAffine2D<Rectangle>,
    r2: &TypedAffine2D<Rectangle>,
) -> bool {
    less_or_equal(r1.get_left(), r2.get_right())
        && less_or_equal(r2.get_left(), r1.get_right())
        && less_or_equal(r1.get_bottom(), r2.get_top())
        && less_or_equal(r2.get_bottom(), r1.get_top())
}
