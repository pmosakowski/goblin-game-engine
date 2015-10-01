#[test]
fn test_interpolator_creation_and_update() {
    use std::rc::Rc;
    use interpolator::{Interpolatable, Interpolator, FunctionType, Repeat};

    let inter = Rc::new(Interpolator::new(FunctionType::Linear, Repeat::Once, 0, 1000, 100));
    let inter_trait: Rc<Interpolatable> = inter.clone();

    inter_trait.update(50);
    assert_eq!(500, inter.get());

    inter_trait.update(25);
    assert_eq!(750, inter.get());

    inter_trait.update(25);
    assert_eq!(1000, inter.get());

    inter_trait.update(30);
    assert_eq!(1000, inter.get());
}

#[test]
fn test_wrap_interpolator() {
    use std::rc::Rc;
    use interpolator::{Interpolatable, Interpolator, FunctionType, Repeat};

    let inter = Rc::new(Interpolator::new(FunctionType::Linear, Repeat::Wrap, 0, 1000, 100));
    let inter_trait: Rc<Interpolatable> = inter.clone();

    inter_trait.update(50);
    assert_eq!(500, inter.get());

    inter_trait.update(25);
    assert_eq!(750, inter.get());

    inter_trait.update(25);
    assert_eq!(1000, inter.get());

    inter_trait.update(30);
    assert_eq!(300, inter.get());

    inter_trait.update(40);
    assert_eq!(700, inter.get());
}
