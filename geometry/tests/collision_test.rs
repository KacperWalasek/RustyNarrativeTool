use geometry::{
    angle::Angle,
    collisions::Collisions,
    point::Point2D,
    shapes::{
        affine::{Affine2D, EmbedInAffine2D, TypedAffine2D},
        circle::Circle,
        line::Line2D,
        rectangle::Rectangle,
        segment::Segment2D,
    },
    vector::Vector2D,
};

fn rect_w2_h2() -> Rectangle {
    Rectangle {
        width: 2.0,
        height: 2.0,
    }
}

fn circle_r1() -> Circle {
    Circle { radius: 1.0 }
}

fn line_deg(deg: f32) -> Line2D {
    Line2D {
        angle: Angle::degrees(deg),
    }
}

fn segment_deg(deg: f32, len: f32) -> Segment2D {
    Segment2D {
        angle: Angle::degrees(deg),
        length: len,
    }
}

mod two_points_collision {
    use super::*;
    #[test]
    fn test_same_point_collision() {
        let p1 = Point2D { x: 1.0, y: 2.0 };
        let p2 = Point2D { x: 1.0, y: 2.0 };
        assert!(p1.check_collision(&p2).unwrap());
    }

    #[test]
    fn test_different_points_collision() {
        let p1 = Point2D { x: 2.0, y: 2.0 };
        let p2 = Point2D { x: 1.0, y: 2.0 };
        assert!(!p1.check_collision(&p2).unwrap());
    }
}
mod rectangle_point_collision {
    use super::*;
    #[test]
    fn test_rectangle_point_inside_collisions() {
        let aff = Affine2D::new(2.0, 2.0, rect_w2_h2().into());
        let point = Point2D { x: 1.5, y: 2.5 };
        assert!(aff.check_collision(&point).unwrap());
        assert!(point.check_collision(&aff).unwrap());
    }

    #[test]
    fn test_rectangle_point_on_edge_collisions() {
        let aff = Affine2D::new(2.0, 2.0, rect_w2_h2().into());
        let point = Point2D { x: 1.0, y: 3.0 };
        assert!(aff.check_collision(&point).unwrap());
        assert!(point.check_collision(&aff).unwrap());
    }

    #[test]
    fn test_rectangle_point_outside_collisions() {
        let aff = Affine2D::new(2.0, 2.0, rect_w2_h2().into());
        let point = Point2D { x: 0.9, y: 3.0 };
        assert!(!aff.check_collision(&point).unwrap());
        assert!(!point.check_collision(&aff).unwrap());
    }
}

mod circle_point_collision {
    use super::*;
    #[test]
    fn test_circle_point_inside_collisions() {
        let aff = Affine2D::new(2.0, 2.0, circle_r1().into());
        let point = Point2D { x: 2.1, y: 1.9 };
        assert!(aff.check_collision(&point).unwrap());
        assert!(point.check_collision(&aff).unwrap());
    }

    #[test]
    fn test_circle_point_on_edge_collisions() {
        let aff = Affine2D::new(2.0, 2.0, circle_r1().into());
        let point = Point2D { x: 2.0, y: 3.0 };
        assert!(aff.check_collision(&point).unwrap());
        assert!(point.check_collision(&aff).unwrap());
    }

    #[test]
    fn test_circle_point_outside_collisions() {
        let aff = Affine2D::new(2.0, 2.0, circle_r1().into());
        let point = Point2D { x: 2.1, y: 3.0 };
        assert!(!aff.check_collision(&point).unwrap());
        assert!(!point.check_collision(&aff).unwrap());
    }
}

mod line_point_collision {
    use super::*;
    #[test]
    fn test_line_point_inside_collisions() {
        let line_point = Point2D { x: 2.0, y: 1.0 };
        let aff = Affine2D {
            point: line_point,
            shape: line_deg(30.0).into(),
        };
        let point = line_point + 5.0 * Vector2D::by_angle(&Angle::degrees(30.0));
        assert!(aff.check_collision(&point).unwrap());
        assert!(point.check_collision(&aff).unwrap());
    }

    #[test]
    fn test_line_point_outside_collisions() {
        let aff = Affine2D::new(2.0, 1.0, line_deg(30.0).into());
        let point = Point2D { x: 3.0, y: 2.0 };
        assert!(!aff.check_collision(&point).unwrap());
        assert!(!point.check_collision(&aff).unwrap());
    }
}

mod segment_point_collision {
    use super::*;
    #[test]
    fn test_segment_point_inside_collisions() {
        let line_point = Point2D { x: 2.0, y: 1.0 };
        let aff = Affine2D {
            point: line_point,
            shape: segment_deg(30.0, 2.0).into(),
        };
        let point = line_point + Vector2D::by_angle(&Angle::degrees(30.0));
        assert!(aff.check_collision(&point).unwrap());
        assert!(point.check_collision(&aff).unwrap());
    }

    #[test]
    fn test_segment_point_outside_on_line_collisions() {
        let line_point = Point2D { x: 2.0, y: 1.0 };
        let aff = Affine2D {
            point: line_point,
            shape: segment_deg(30.0, 2.0).into(),
        };
        let point = line_point + 2.1 * Vector2D::by_angle(&Angle::degrees(30.0));
        assert!(!aff.check_collision(&point).unwrap());
        assert!(!point.check_collision(&aff).unwrap());
    }

    #[test]
    fn test_segment_point_outside_near_collisions() {
        let aff = Affine2D::new(2.0, 1.0, segment_deg(30.0, 2.0).into());
        let point = Point2D { x: 3.0, y: 2.0 };
        assert!(!aff.check_collision(&point).unwrap());
        assert!(!point.check_collision(&aff).unwrap());
    }
}

mod line_line_collision {
    use super::*;
    #[test]
    fn test_line_line_alighned() {
        let l1: Affine2D = Affine2D::new(2.0, 1.0, line_deg(45.0).into());
        let l2: Affine2D = Affine2D::new(3.0, 2.0, line_deg(45.0).into());
        assert!(l1.check_collision(&l2).unwrap());
        assert!(l2.check_collision(&l1).unwrap());
    }
    #[test]
    fn test_line_line_paralel_90() {
        let l1: Affine2D = Affine2D::new(2.0, 1.0, line_deg(270.0).into());
        let l2: Affine2D = Affine2D::new(3.0, 1.0, line_deg(90.0).into());
        assert!(!l1.check_collision(&l2).unwrap());
        assert!(!l2.check_collision(&l1).unwrap());
    }
    #[test]
    fn test_line_line_collide() {
        let l1: Affine2D = Affine2D::new(2.0, 1.0, line_deg(45.0).into());
        let l2: Affine2D = Affine2D::new(3.0, 1.0, line_deg(46.0).into());
        assert!(l1.check_collision(&l2).unwrap());
        assert!(l2.check_collision(&l1).unwrap());
    }
    #[test]
    fn test_line_line_paralel() {
        let l1: Affine2D = Affine2D::new(2.0, 1.0, line_deg(45.0).into());
        let l2: Affine2D = Affine2D::new(3.0, 1.0, line_deg(45.0).into());
        assert!(!l1.check_collision(&l2).unwrap());
        assert!(!l2.check_collision(&l1).unwrap());
    }
}
mod segment_segment_collision {
    use super::*;

    #[test]
    fn test_segment_segment_cross_collision() {
        let s1: Affine2D = Affine2D::new(0.0, 0.0, segment_deg(45.0, 2.0).into());
        let s2: Affine2D = Affine2D::new(1.0, 0.0, segment_deg(135.0, 2.0).into());
        assert!(s1.check_collision(&s2).unwrap());
        assert!(s2.check_collision(&s1).unwrap());
    }

    #[test]
    fn test_segment_segment_boundries_collision() {
        let s1: Affine2D = Affine2D::new(0.0, 0.0, segment_deg(0.0, 2.0).into());
        let s2: Affine2D = Affine2D::new(1.0, 1.0, segment_deg(-90.0, 2.0).into());
        assert!(s1.check_collision(&s2).unwrap());
        assert!(s2.check_collision(&s1).unwrap());
    }

    #[test]
    fn test_segment_segment_boundry_inside_collision() {
        let s1: Affine2D = Affine2D::new(0.0, 0.0, segment_deg(0.0, 4.0).into());
        let s2: Affine2D = Affine2D::new(1.0, 1.0, segment_deg(90.0, 2.0).into());
        assert!(s1.check_collision(&s2).unwrap());
        assert!(s2.check_collision(&s1).unwrap());
    }

    #[test]
    fn test_segment_segment_inline_boundry_collision() {
        let s1: Affine2D = Affine2D::new(0.0, 0.0, segment_deg(90.0, 2.0).into());
        let s2: Affine2D = Affine2D::new(0.0, 2.0, segment_deg(90.0, 2.0).into());
        assert!(s1.check_collision(&s2).unwrap());
        assert!(s2.check_collision(&s1).unwrap());
    }
    #[test]
    fn test_segment_segment_inline_no_collision() {
        let s1: Affine2D = Affine2D::new(0.0, 0.0, segment_deg(90.0, 1.9).into());
        let s2: Affine2D = Affine2D::new(0.0, 2.0, segment_deg(90.0, 2.0).into());
        assert!(!s1.check_collision(&s2).unwrap());
        assert!(!s2.check_collision(&s1).unwrap());
    }
}
mod line_segment_collision {
    use super::*;

    #[test]
    fn test_line_segment_aligned() {
        let l: Affine2D = Affine2D::new(2.0, 1.0, line_deg(30.0).into());
        let s: Affine2D = Affine2D::new(2.0, 1.0, segment_deg(30.0, 2.0).into());
        assert!(l.check_collision(&s).unwrap());
        assert!(s.check_collision(&l).unwrap());
    }

    #[test]
    fn test_line_segment_collision_inside() {
        let l: Affine2D = TypedAffine2D::new(
            Point2D::zero(),
            Line2D {
                angle: Angle::degrees(45.0),
            },
        )
        .into();
        let s: Affine2D = TypedAffine2D::new(
            Point2D { x: 2.0, y: 0.0 },
            Segment2D {
                angle: Angle::degrees(135.0),
                length: 3.0,
            },
        )
        .into();
        assert!(l.check_collision(&s).unwrap());
        assert!(s.check_collision(&l).unwrap());
    }

    #[test]
    fn test_line_segment_collision_on_boundry() {
        let l: Affine2D = TypedAffine2D::new(
            Point2D::zero(),
            Line2D {
                angle: Angle::degrees(45.0),
            },
        )
        .into();
        let s: Affine2D = TypedAffine2D::new(
            Point2D { x: 1.0, y: 0.0 },
            Segment2D {
                angle: Angle::degrees(90.0),
                length: 2.0,
            },
        )
        .into();
        assert!(l.check_collision(&s).unwrap());
        assert!(s.check_collision(&l).unwrap());
    }

    #[test]
    fn test_line_segment_no_collision() {
        let l: Affine2D = TypedAffine2D::new(
            Point2D::zero(),
            Line2D {
                angle: Angle::degrees(45.0),
            },
        )
        .into();
        let s: Affine2D = TypedAffine2D::new(
            Point2D { x: 1.0, y: 0.0 },
            Segment2D {
                angle: Angle::degrees(90.0),
                length: 1.5,
            },
        )
        .into();
        assert!(!l.check_collision(&s).unwrap());
        assert!(!s.check_collision(&l).unwrap());
    }
}

mod rectangle_line_collision {
    use super::*;

    #[test]
    fn test_no_collision() {
        let rect = rect_w2_h2().embed_affine(&Point2D::zero());
        let line = line_deg(0.0).embed_affine(&Point2D { x: 0.0, y: 2.0 });
        assert!(!rect.check_collision(&line).unwrap());
        assert!(!line.check_collision(&rect).unwrap());
    }
    #[test]
    fn test_collision() {
        let rect = rect_w2_h2().embed_affine(&Point2D::zero());
        let line = line_deg(15.0).embed_affine(&Point2D { x: 0.5, y: 0.0 });
        assert!(rect.check_collision(&line).unwrap());
        assert!(line.check_collision(&rect).unwrap());
    }
}
mod rectangle_segmnent_collision {
    use super::*;

    #[test]
    fn test_rectangle_segment_border_collision() {
        let rect = rect_w2_h2().embed_affine(&Point2D::zero());
        let segment = segment_deg(0.0, 2.0).embed_affine(&Point2D { x: 1.5, y: 0.0 });
        assert!(rect.check_collision(&segment).unwrap());
        assert!(segment.check_collision(&rect).unwrap());
    }
    #[test]
    fn test_rectangle_segment_segment_inside() {
        let rect = rect_w2_h2().embed_affine(&Point2D::zero());
        let segment = segment_deg(15.0, 0.5).embed_affine(&Point2D { x: 0.1, y: 0.0 });
        assert!(rect.check_collision(&segment).unwrap());
        assert!(segment.check_collision(&rect).unwrap());
    }
    #[test]
    fn test_rectangle_segment_segment_outside() {
        let rect = rect_w2_h2().embed_affine(&Point2D::zero());
        let segment = segment_deg(15.0, 0.5).embed_affine(&Point2D { x: 2.0, y: 0.0 });
        assert!(!rect.check_collision(&segment).unwrap());
        assert!(!segment.check_collision(&rect).unwrap());
    }
}
mod circle_line_collision {
    use super::*;

    #[test]
    fn test_chord_of_circle() {
        let c = circle_r1().embed_affine(&Point2D::zero());
        let l = line_deg(0.0).embed_affine(&Point2D { x: 10.0, y: 0.5 });
        assert!(c.check_collision(&l).unwrap());
        assert!(l.check_collision(&c).unwrap());
    }

    #[test]
    fn test_diameter_of_circle() {
        let c = circle_r1().embed_affine(&Point2D::zero());
        let l = line_deg(0.0).embed_affine(&Point2D { x: 10.0, y: 0.0 });
        assert!(c.check_collision(&l).unwrap());
        assert!(l.check_collision(&c).unwrap());
    }

    #[test]
    fn test_tangent_to_circle() {
        let c = circle_r1().embed_affine(&Point2D::zero());
        let l = line_deg(0.0).embed_affine(&Point2D { x: 10.0, y: 1.0 });
        assert!(c.check_collision(&l).unwrap());
        assert!(l.check_collision(&c).unwrap());
    }
    #[test]
    fn test_no_collision() {
        let c = circle_r1().embed_affine(&Point2D::zero());
        let l = line_deg(0.0).embed_affine(&Point2D { x: 10.0, y: 1.1 });
        assert!(!c.check_collision(&l).unwrap());
        assert!(!l.check_collision(&c).unwrap());
    }
}
mod circle_segment_collision {
    use super::*;

    #[test]
    fn test_segment_inside() {
        let c = circle_r1().embed_affine(&Point2D::zero());
        let s = segment_deg(10.0, 0.2).embed_affine(&Point2D { x: 0.1, y: 0.0 });
        assert!(c.check_collision(&s).unwrap());
        assert!(s.check_collision(&c).unwrap());
    }

    #[test]
    fn test_segment_cross() {
        let c = circle_r1().embed_affine(&Point2D { x: 0.0, y: 1.0 });
        let s = segment_deg(0.0, 1.0).embed_affine(&Point2D { x: 0.0, y: 0.1 });
        assert!(c.check_collision(&s).unwrap());
        assert!(s.check_collision(&c).unwrap());
    }

    #[test]
    fn test_segment_tangent() {
        let c = circle_r1().embed_affine(&Point2D { x: 0.0, y: 1.0 });
        let s = segment_deg(0.0, 1.0).embed_affine(&Point2D::zero());
        assert!(c.check_collision(&s).unwrap());
        assert!(s.check_collision(&c).unwrap());
    }
    #[test]
    fn test_segment_outside() {
        let c = circle_r1().embed_affine(&Point2D { x: 0.0, y: 1.1 });
        let s = segment_deg(0.0, 1.0).embed_affine(&Point2D::zero());
        assert!(!c.check_collision(&s).unwrap());
        assert!(!s.check_collision(&c).unwrap());
    }
}
mod rectangle_rectangle_collision {
    use super::*;

    #[test]
    fn test_one_inside_another() {
        let r1 = rect_w2_h2().embed_affine(&Point2D::zero());
        let r2 = Rectangle {
            width: 1.0,
            height: 1.0,
        }
        .embed_affine(&Point2D::zero());
        assert!(r1.check_collision(&r2).unwrap());
        assert!(r2.check_collision(&r1).unwrap());
    }
    #[test]
    fn test_edges_cross() {
        let r1 = rect_w2_h2().embed_affine(&Point2D::zero());
        let r2 = Rectangle {
            width: 1.0,
            height: 1.0,
        }
        .embed_affine(&Point2D { x: 1.0, y: 1.0 });
        assert!(r1.check_collision(&r2).unwrap());
        assert!(r2.check_collision(&r1).unwrap());
    }
    #[test]
    fn test_edges_touch() {
        let r1 = rect_w2_h2().embed_affine(&Point2D::zero());
        let r2 = rect_w2_h2().embed_affine(&Point2D { x: 2.0, y: 0.0 });
        assert!(r1.check_collision(&r2).unwrap());
        assert!(r2.check_collision(&r1).unwrap());
    }
    #[test]
    fn test_verticles_touch() {
        let r1 = rect_w2_h2().embed_affine(&Point2D::zero());
        let r2 = rect_w2_h2().embed_affine(&Point2D { x: 2.0, y: 2.0 });
        assert!(r1.check_collision(&r2).unwrap());
        assert!(r2.check_collision(&r1).unwrap());
    }
    #[test]
    fn test_verticles_no_collision() {
        let r1 = rect_w2_h2().embed_affine(&Point2D::zero());
        let r2 = rect_w2_h2().embed_affine(&Point2D { x: 3.0, y: 3.0 });
        assert!(!r1.check_collision(&r2).unwrap());
        assert!(!r2.check_collision(&r1).unwrap());
    }
}
mod circle_circle_collision {
    use super::*;

    #[test]
    fn test_one_inside_another() {
        let c1 = Circle { radius: 2.0 }.embed_affine(&Point2D::zero());
        let c2 = Circle { radius: 1.0 }.embed_affine(&Point2D::zero());
        assert!(c1.check_collision(&c2).unwrap());
        assert!(c2.check_collision(&c1).unwrap());
    }

    #[test]
    fn test_cross() {
        let c1 = Circle { radius: 1.0 }.embed_affine(&Point2D::zero());
        let c2 = Circle { radius: 1.0 }.embed_affine(&Point2D { x: 0.5, y: 1.0 });
        assert!(c1.check_collision(&c2).unwrap());
        assert!(c2.check_collision(&c1).unwrap());
    }

    #[test]
    fn test_touch() {
        let c1 = Circle { radius: 1.0 }.embed_affine(&Point2D::zero());
        let c2 = Circle { radius: 1.0 }.embed_affine(&Point2D { x: 2.0, y: 0.0 });
        assert!(c1.check_collision(&c2).unwrap());
        assert!(c2.check_collision(&c1).unwrap());
    }
    #[test]
    fn test_no_collision() {
        let c1 = Circle { radius: 1.0 }.embed_affine(&Point2D::zero());
        let c2 = Circle { radius: 1.0 }.embed_affine(&Point2D { x: 3.0, y: 0.0 });
        assert!(!c1.check_collision(&c2).unwrap());
        assert!(!c2.check_collision(&c1).unwrap());
    }
}
mod circle_rectange_collision {
    use super::*;
    #[test]
    fn test_circle_inside_rectangle() {
        let c = Circle { radius: 0.5 }.embed_affine(&Point2D::zero());
        let r = rect_w2_h2().embed_affine(&Point2D::zero());
        assert!(c.check_collision(&r).unwrap());
        assert!(r.check_collision(&c).unwrap());
    }
    #[test]
    fn test_rectangle_inside_circle() {
        let c = Circle { radius: 2.0 }.embed_affine(&Point2D::zero());
        let r = rect_w2_h2().embed_affine(&Point2D::zero());
        assert!(c.check_collision(&r).unwrap());
        assert!(r.check_collision(&c).unwrap());
    }
    #[test]
    fn test_edge_crosses_circle() {
        let c = Circle { radius: 2.0 }.embed_affine(&Point2D::zero());
        let r = rect_w2_h2().embed_affine(&Point2D { x: 2.0, y: 0.0 });
        assert!(c.check_collision(&r).unwrap());
        assert!(r.check_collision(&c).unwrap());
    }
    #[test]
    fn test_edge_touch_circle() {
        let c = Circle { radius: 2.0 }.embed_affine(&Point2D::zero());
        let r = rect_w2_h2().embed_affine(&Point2D { x: 3.0, y: 0.0 });
        assert!(c.check_collision(&r).unwrap());
        assert!(r.check_collision(&c).unwrap());
    }
    #[test]
    fn test_vertex_touch_circle() {
        let c = Circle { radius: 2.0 }.embed_affine(&Point2D::zero());
        let r = rect_w2_h2().embed_affine(&Point2D { x: 3.0, y: 1.0 });
        assert!(c.check_collision(&r).unwrap());
        assert!(r.check_collision(&c).unwrap());
    }
}
