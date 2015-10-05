extern crate num;
use self::num::traits::{Num, FromPrimitive, ToPrimitive};

#[cfg(test)]
mod tests;

// enums used in interpolator constructor
pub enum FunctionType {
    Static,
    Linear,
    Quadratic,
    Cubic,
}

pub enum Repeat {
    Once,
    Bounce,
    Wrap,
}

mod control {
    use std::rc::Rc;
    use std::cell::Cell;
    use super::function::{Function, Linear};
    use super::{Param, Repeat};
    use super::num::traits::{Num, ToPrimitive, FromPrimitive};

    pub fn enum_to_control<'a,T:'a + Copy + Num + ToPrimitive + FromPrimitive>(repeat: Repeat, params: Param<T>) -> Rc<Control<T> + 'a> {
        match repeat {
            Repeat::Once => Once::new(params),
            Repeat::Wrap => Wrap::new(params),
            _            => Once::new(params),
        }
    }

    // controls repeat and flow
    pub trait Control<T> {
        fn control(&self, timedelta: u32) -> T;
    }

    pub struct Once<'a,T> where T: 'a {
        endval: T,
        elapsed: Cell<u32>,
        duration: u32,
        function: Rc<Function<T> + 'a>,
    }

    impl<'a,T> Once<'a,T> where T: Copy + Num + ToPrimitive + FromPrimitive {
        pub fn new(params: Param<T>) -> Rc<Once<'a,T>> {
            Rc::new(
                Once {
                    endval: params.endval,
                    elapsed: Cell::new(0),
                    duration: params.duration,
                    function: Linear::new(params.startval, params.slope),
                })
        }
    }

    impl<'a,T> Control<T> for Once<'a,T> where T:Copy {
        fn control(&self, timedelta: u32) -> T {
            self.elapsed.set(self.elapsed.get() + timedelta);

            // if we run out of time return the final value
            // otherwise calculate based on timedelta
            if self.elapsed.get() >= self.duration {
                self.elapsed.set(self.duration);
                self.endval.clone()
            } else {
                self.function.calculate(self.elapsed.get())
            }
        }
    }

    pub struct Wrap<'a,T> where T: 'a {
        endval: T,
        duration: u32,
        elapsed: Cell<u32>,
        function: Rc<Function<T> + 'a>,
    }

    impl<'a,T> Wrap<'a,T>  where T: Copy + Num + ToPrimitive + FromPrimitive {
        pub fn new(params: Param<T>) -> Rc<Wrap<'a,T>> {
            Rc::new(
                Wrap {
                    endval: params.endval,
                    duration: params.duration,
                    elapsed: Cell::new(0),
                    function: Linear::new(params.startval, params.slope),
                }
            )
        }
    }

    impl<'a,T> Control<T> for Wrap<'a,T> {
        fn control(&self, timedelta: u32) -> T {
            self.elapsed.set(self.elapsed.get() + timedelta);

            if self.elapsed.get() > self.duration {
                self.elapsed.set(self.elapsed.get() - self.duration);
            }

            self.function.calculate(self.elapsed.get())
        }
    }
}

mod function {
    use std::rc::Rc;
    use super::{FunctionType, Param};
    use super::num::traits::{Num, FromPrimitive, ToPrimitive};

    fn enum_to_function<'a,T: Copy + Num + ToPrimitive + FromPrimitive>(function: FunctionType, params: &'a Param<T>) -> Rc<Function<T> + 'a> {
        match function {
            FunctionType::Static => Static::new(params.endval),
            FunctionType::Linear => Linear::new(params.startval,params.slope),
            _                    => Static::new(params.endval),
        }
    }

    pub trait Function<T> {
        fn calculate(&self, timedelta: u32) -> T;
    }

    // interpolates linearly from start to end
    pub struct Linear<T> {
        startval: T,
        slope: f32,
    }

    impl<T> Linear<T> {
        pub fn new(startval: T, slope: f32) -> Rc<Linear<T>> {
            Rc::new(
                Linear {
                    startval: startval,
                    slope: slope,
                }
            )
        }
    }

    impl<T> Function<T> for Linear<T> where T: Copy + Num + ToPrimitive + FromPrimitive{
        fn calculate(&self, timedelta: u32) -> T {
            let timedelta_f = timedelta as f32;
        //let elapsed_f = elapsed as f32;
        //let duration_f = self.duration as f32;
        //let progress_f = elapsed_f / duration_f;
        //let distance_f = self.distance.to_f32().unwrap();
        ////let new_val: T  = self.startval * self.duration;
        //let val: T = FromPrimitive::from_f32(progress_f * distance_f).unwrap();
        //let new_val = val + self.value.get();
            let new_val: T = FromPrimitive::from_f32(self.slope.to_f32().unwrap()*timedelta_f).unwrap();
            new_val + self.startval
        }
    }

    // TODO: implement
    struct Quadratic;

    // TODO: implement
    struct Cubic;

    // always returns a predefined value
    pub struct Static<T> {
        value: T,
    }

    impl<T> Static<T> {
        pub fn new(value: T) -> Rc<Static<T>> {
            Rc::new(
                Static {
                    value: value,
                }
            )
        }
    }

    impl<T> Function<T> for Static<T> where T: Copy {
        fn calculate(&self, timedelta: u32) -> T {
            self.value
        }
    }
}

use std::rc::Rc;
use std::cell::Cell;
use self::function::Function;
use self::control::Control;

#[derive(Clone,Copy)]
pub struct Param<T>  where T: Copy {
    startval: T,
    endval: T,
    distance: T,
    slope: f32,
    duration: u32,
}

pub struct Interpolator<'a,T> where T: 'a + Copy {
    function: Rc<Function<T> + 'a>,
    repeat: Rc<Control<T> + 'a>,
    value: Cell<T>,
    params: Param<T>,
}

impl<'a,T> Interpolator<'a,T> where T: 'a + Copy + Num + ToPrimitive + FromPrimitive {
    pub fn new(func: FunctionType, rep: Repeat, startval: T, endval: T, duration: u32) -> Interpolator<'a,T> {
        use self::control::enum_to_control;
        use self::function::Static;
        use self::control::Once;

        let distance = endval - startval;
        let slope = distance.to_f32().unwrap() / duration as f32;

        let params = Param {
            startval: startval,
            endval: endval,
            distance: distance,
            slope: slope,
            duration: duration,
        };

        Interpolator {
            function: Static::new(endval),
            repeat: enum_to_control(rep, params),
            value: Cell::new(startval),
            params: params,
        }
    }
    
    pub fn get(&self) -> T {
        self.value.get()
    }
}

pub trait Interpolatable {
    fn update(&self, _timedelta: u32);
}

impl<'a,T> Interpolatable for Interpolator<'a,T> where T: Copy {
    fn update(&self, timedelta: u32) {
        use self::function::Function;

        self.value.set(self.repeat.control(timedelta));
    }
}
