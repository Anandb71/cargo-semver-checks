// Test case for issue #1781: false positive when same-named methods
// exist on different instantiations of a generic type.
//
// This simulates arrow-rs's pattern:
// type SyncBuilder<T> = Builder<SyncReader<T>>;
// type AsyncBuilder<T> = Builder<AsyncReader<T>>;
// where the same generic Builder type has different methods when instantiated
// with different wrapper types.

pub struct SyncReader<T>(T);
pub struct AsyncReader<T>(T);

pub struct Builder<R> {
    reader: R,
}

// impl on Builder<SyncReader<T>>: method takes &self
impl<T> Builder<SyncReader<T>> {
    pub fn get_data(&self) -> i32 {
        42
    }
}

// impl on Builder<AsyncReader<T>>: method takes &mut self
impl<T> Builder<AsyncReader<T>> {
    pub fn get_data(&mut self) -> i32 {
        42
    }
}

// True positive case: a method that actually changes from &self to &mut self
pub struct Changed {
    value: i32,
}

impl Changed {
    pub fn mutate(&self) -> i32 {
        self.value
    }
}
