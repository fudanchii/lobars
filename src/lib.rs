#[cfg(not(test))]
use gloo_render::{request_animation_frame, AnimationFrame};

use std::rc::Rc;
use std::cell::RefCell;

use yew::{functional::use_mut_ref, hook};

#[cfg(test)]
use tests::{AnimationFrame, request_animation_frame};

pub struct RequestAnimationFrame(Rc<RefCell<Option<AnimationFrame>>>);

fn raf_callback<P>(rafcell: Rc<RefCell<Option<AnimationFrame>>>, callback: P, frame: f64)
where
    P: Fn(f64) -> bool + 'static,
{
    let rafcell_clone = rafcell.clone();
    if !callback(frame) {
        *rafcell.borrow_mut() = None;
        return;
    }

    *rafcell.borrow_mut() = Some(
        request_animation_frame(move |f| raf_callback(rafcell_clone, callback, f))
    );
}

#[hook]
pub fn use_request_animation_frame() -> RequestAnimationFrame {
    RequestAnimationFrame(use_mut_ref(|| None))
}

impl RequestAnimationFrame {
    pub fn each<Q>(self, callback: Q)
    where
        Q: Fn(f64) -> bool + 'static
    {
        let raf_clone = self.0.clone();
        request_animation_frame(move |f| raf_callback(raf_clone, callback, f));
    }
}



#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::RequestAnimationFrame;

    pub struct AnimationFrame;

    pub fn request_animation_frame<P>(f: P) -> AnimationFrame
    where
        P: FnOnce(f64) + 'static
    {
        f(0f64);
        AnimationFrame
    }

    #[test]
    fn test_raf_each_called_n_times() {
        let test_val = Rc::new(RefCell::new(0));
        let raf = RequestAnimationFrame(Rc::new(RefCell::new(None)));
        {
            let test_val_clone = test_val.clone();
            raf.each(move |_| {
                *test_val_clone.borrow_mut() += 1;
                !(*test_val_clone.borrow() == 3)
            });
        }
        assert_eq!(*test_val.borrow(), 3);
    }
}