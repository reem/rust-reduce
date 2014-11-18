use async::Callback;
use {Reduce};

include!(concat!(env!("OUT_DIR"), "/impls.rs"))

impl<'a, I> Reduce<I, I> for Vec<Box<Callback<I, I> + 'a>> {
    fn reduce(self, input: I) -> I {
        self.into_iter().fold(input, |val, fun| fun.invoke(val))
    }
}

