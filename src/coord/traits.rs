use std::marker::PhantomData;

pub trait Transformation<A, B> {
    fn transform(&self, in_coord: A) -> B;
    fn compose_after<C, T: Transformation<B, C>>(self, next_transform: T) -> CompositeTransformation<Self, T, B> where Self: Sized {
        CompositeTransformation { first: self, second: next_transform, intermediate: PhantomData {} }
    }
    // fn compose_before<C, T: Transformation<C, A>>(self, prev_transform: T) -> CompositeTransformation<T, Self> {
    //     return CompositeTransformation(prev_transform, self);
    // }
}

#[derive(Copy, Clone, Debug)]
pub struct CompositeTransformation<T1, T2, I> {
    pub first: T1,
    pub second: T2,
    intermediate: PhantomData<I>,
}

// see https://github.com/rust-lang/rust/issues/28271 for an explanation of how we have to deal with the intermediate type
impl<T1, T2, A, B, C> Transformation<A, C> for CompositeTransformation<T1, T2, B>
    where T1: Transformation<A, B>, T2: Transformation<B, C> {
    fn transform(&self, in_coord: A) -> C {
        let intermediate_coord: B = self.first.transform(in_coord);
        return self.second.transform(intermediate_coord);
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BidirectionalTransform<T1, T2, A, B> {
    pub forward: T1,
    pub backward: T2,
    // TODO: figure out how to get rid of these without screwing up the transform_forward() and transform_backward() impls !
    input: PhantomData<A>,
    output: PhantomData<B>,
}


impl<A, B, T1, T2> BidirectionalTransform<T1, T2, A, B> where T1: Transformation<A, B>, T2: Transformation<B, A> {
    pub(crate) fn new(forward: T1, backward: T2) -> Self {
        Self { forward, backward, input: PhantomData {}, output: PhantomData {} }
    }
}

impl<A, B, T1, T2> BidirectionalTransform<T1, T2, A, B> where T1: Transformation<A, B> {
    fn transform_forward(&self, in_coord: A) -> B {
        self.forward.transform(in_coord)
    }
}

impl<A, B, T1, T2> BidirectionalTransform<T1, T2, A, B> where T2: Transformation<B, A> {
    fn transform_backward(&self, in_coord: B) -> A {
        self.backward.transform(in_coord)
    }
}

// impl<A, B, C, T1, T2, T3, T4> BidirectionalTransform<T1, T2, A, C>
//     where T1: Transformation<A, B>,
//           T2: Transformation<B, A>,
//           T3: Transformation<B, C>,
//           T4: Transformation<C, B> {
//     fn compose_after(self, next_transform: BidirectionalTransform<T3, T4, B, C>) -> BidirectionalTransform<CompositeTransformation<T1, T3, B>, CompositeTransformation<T4, T2, B>, A, C> {
//         BidirectionalTransform { forward: self.forward.compose_after(next_transform.forward), backward: next_transform.backward.compose_after(self.backward) }
//     }
// }

// impl<A, B, T1, T2> Transformation<A, B> for BidirectionalTransform<T1, T2> where T1: Transformation<A, B> {
//     fn transform(&self, in_coord: A) -> B {
//         self.forward(in_coord)
//     }
// }
//
// impl<A, B, T1, T2> Transformation<B, A> for BidirectionalTransform<T1, T2> where T2: Transformation<B, A> {
//     fn transform(&self, in_coord: B) -> A {
//         self.transform_backward(in_coord)
//     }
// }
