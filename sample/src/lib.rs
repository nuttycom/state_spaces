use std::marker::PhantomData;
use std::collections::HashMap;

trait Tuple<A, B> {
    fn cata<F, C>(&self, f: F) -> C
        where F: FnOnce(&A, &B) -> C;
}

struct T<A, B> {
    a: A,
    b: B,
}

impl<A, B> Tuple<A, B> for T<A, B> {
    fn cata<F, C>(&self, f: F) -> C where F: FnOnce(&A, &B) -> C {
        f(&self.a, &self.b)
    }
}


trait Either<A, B> {
    fn cata<L, R, C>(&self, l: L, r: R) -> C
        where L: FnOnce(&A) -> C,
              R: FnOnce(&B) -> C;
}

struct Left<A, B> {
    value: A,
    _phantom: PhantomData<B>,
}

struct Right<A, B> {
    _phantom: PhantomData<A>,
    value: B,
}

impl<A, B> Either<A, B> for Left<A, B> {
    fn cata<L, R, C>(&self, l: L, _: R) -> C
        where L: FnOnce(&A) -> C,
              R: FnOnce(&B) -> C {

        l(&self.value)
    }
}

impl<A, B> Either<A, B> for Right<A, B> {
    fn cata<L, R, C>(&self, _: L, r: R) -> C
        where L: FnOnce(&A) -> C,
              R: FnOnce(&B) -> C {

        r(&self.value)
    }
}

pub mod samples {
    pub enum Either<A, B> {
        Left(A),
        Right(B),
    }

    trait Samples {
        fn f() -> bool;

        fn g() -> u32;

        fn t<A, B>() -> (A, B);

        fn e<A, B>() -> Either<A, B>;
    }

    pub fn test() -> bool {
        true ^ false
    }

    pub fn identity<A>(a: &A) -> &A {
        a
    }
}

pub mod monoidal {
    pub trait Monoidal {
        fn mzero() -> Self;
        fn mappend(&self, b: &Self) -> Self;
    }

    pub fn reduce<A: Monoidal, V: Iterator<Item = A>>(vs: V) -> A {
        let mut result = A::mzero();
        for v in vs {
            result = result.mappend(&v);
        }

        result
    }
}

pub mod monoid {
    pub trait Monoid<A> {
        fn mzero() -> A;
        fn mappend(a: &A, b: &A) -> A;
    }

    pub fn reduce<A, M: Monoid<A>, V: Iterator<Item = A>>(vs: V) -> A {
        let mut result = M::mzero();

        for v in vs {
            result = M::mappend(&result, &v);
        }

        result
    }

    pub struct I32Add;

    impl Monoid<i32> for I32Add {
        fn mzero() -> i32 { 0 }
        fn mappend(a: &i32, b: &i32) -> i32 { a + b }
    }

    pub struct I32Mult;

    impl Monoid<i32> for I32Mult {
        fn mzero() -> i32 { 0 }
        fn mappend(a: &i32, b: &i32) -> i32 { a * b }
    }

    pub fn test() -> i32 {
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);

        reduce::<_, I32Mult, _>(vec.into_iter())
    }
}

enum JValue {
  JBool(bool),
  JNum(f64),
  JStr(String),
  JNull,
  JArray(Vec<JValue>),
  JObject(HashMap<String, JValue>),
}

struct JNull {}

type JValue0 = Either<bool, Either<f64, Either<String, Either<JNull, Either<Vec<JValue>, HashMap<String, JValue>>>>>>;

