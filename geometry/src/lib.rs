use shapes::simple::{*};
use shapes::affine::{*};


//pub mod shapes;
// pub mod collisions;
pub mod shapes;

struct RectanglePointRule {}
impl RectanglePointRule {
    fn check(&self, s1: &Affine, s2: &Affine) ->bool{
        let c = s1.shape.as_any().downcast_ref::<Circle>().unwrap();
        let d = s2.shape.as_any().downcast_ref::<Circle>().unwrap();
        true
    }
}


#[cfg(test)]
mod tests {
    // use crate::shapes::affine::{Affine, Point};
    // use crate::shapes::simple::Rectangle;

    use crate::{*};

    #[test]
    fn affine_shape_construct() {
        let c = Rectangle {width: 12.0, heigth: 1.0};
        let d = Circle {
            radius: 1.0
        };

        let mut zoo = Vec::<Affine>::new();
        zoo.push(Affine::new(Point{x: 0.0,y:0.0}, Box::new(c)));
        zoo.push(Affine::new(Point{x: 0.0,y:0.0}, Box::new(d)));

        let rule = RectanglePointRule{};
        // How do I now get the dog's name?
        let x = &zoo[1];
        rule.check(x,x);
        let circ = x.shape.as_any().downcast_ref::<Circle>().unwrap();
        println!("{}", circ.radius); // works!
    }
}
