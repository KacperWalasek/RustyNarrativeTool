// use crate::shapes::{affine::*, simple::Rectangle};

// trait CollisionRule {
//     fn can_check(&self, s1: &Affine, s2: &Affine) -> bool;
//     fn check(&self, s1: &Affine, s2: &Affine) -> Option<bool>;
// }

// struct RectanglePointRule {}
// impl CollisionRule for RectanglePointRule {
//     fn can_check(&self, s1: &Affine, s2: &Affine) -> bool {
//         todo!()
//     }

//     fn check(&self, s1: &Affine, s2: &Affine) -> Option<bool> {
//         let dog = s1.shape.as_any().downcast_ref::<Rectangle>().unwrap();
//         None
//     }
// }

// struct CollisionChecker {
//     rules: Vec<Box<dyn CollisionRule>>,
// }

// impl CollisionChecker {
//     pub fn check_collision(&self, s1: &Affine, s2: &Affine) -> bool {
//         self.rules
//             .iter()
//             .find(|rule| rule.check(s1, s2).is_some_and(|x| x))
//             .is_some()
//     }
// }
