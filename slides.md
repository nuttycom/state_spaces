% Parametricity, And The State Space of Programs
% Kris Nuttycombe (@nuttycom) - April, 2020

# Resources

- Slides: [nuttycom.github.io/state_spaces/index.html](http://nuttycom.github.io/state_spaces/index.html)

# The Problem

<div class="incremental"><div>
* Programming is hard, maintenance is harder.
* Churn and inflexibility are a problem.

* So how to fix it?
    * Write down small, general facts about the universe.
    * Compose small facts to obtain more specific results.
    * Use good tools to check that we've written things down right.
</div></div>

<div class="notes">

A few years ago I wrote a piece of software, and had set up deployment for it
using Docker. Just a little thing, it ran on a DigitalOcean instance, standard
app + database + nginx sort of configuration. When I went to freshen it up a
bit, I found that basically nothing in the build or deployment infrastructure I
had set up worked any more - the build system's dependency resolution mechanism
(a standard 3rd-party component) had had a bug some time in the past few years
that meant I could no longer download some of the dependencies, the Docker base
images that I had used were no longer able to install the updated build
tooling; basically nothing worked. It took me a solid week of evenings to get
it all just to be deployable again - and this is with essentially no changes
to the code.

Our industry is kind of broken in the way that we think about software. We
expect it to break, to need to be replaced all the time. Now, in some sense
this is good; we should be continuously evolving and moving forward. But at
the same time, operations that are semantically the same shouldn't require
updating; if you've written down a fact about what you want to happen, that
fact doesn't change.

So what can we do? I think that what is required is just a small shift in
how we think about the programs that we're writing.

</div>

# Outline

* Types and the state space of programs
    * Sum and product types
    * Functions

* Types as sets of capabilities
    * Newtypes
    * A basic introduction to parametricity.
    * IO capabilities and mocking

* *This presentation is meant to be interactive.*

# Terminology

Function
: A transformation on data that maps inputs to outputs. Given the same input,
  a function will always produce the same output, and the state of the universe
  will be the same after the function has been evaluated as before. Just a
  fact that relates some types to one another.

Procedure
: Instructions for a computer to make some change to the state of the universe
  and/or return a value, given some inputs and the ambient state of the universe.

# Types as state spaces

How many different values can this function possibly return?

~~~rust
fn f() -> bool
~~~

# Types as state spaces

How many different values can this function possibly return?

~~~rust
fn f() -> bool // 2 possible values
~~~

# Types as state spaces

How many different values can this function possibly return?

~~~rust
fn f() -> bool // 2 possible values
~~~

~~~rust
fn g() -> u32
~~~

# Types as state spaces

How many different values can this function possibly return?

~~~rust
fn f() -> bool // 2 possible values
~~~

~~~rust
fn g() -> u32 // 2^32 possible values
~~~

# Types as state spaces

How many different values can this function possibly return?

~~~rust
fn f() -> bool // 2 possible values
~~~

~~~rust
fn g() -> u32 // 2^32 possible values
~~~

We refer to the number of inhabitants of a type T as its cardinality,
and denote it |T|, so |bool| == 2

# Cardinality

~~~rust
|(bool, u32)|
~~~

# Cardinality

~~~rust
|(bool, u32)| == 2 * 2^32 
~~~

# Cardinality

~~~rust
|(bool, u32)| == 2 * 2^32 
~~~

~~~rust
|(u32, u32)| == 2^32 * 2^32 == 2^64
~~~

# Cardinality

~~~rust
|(bool, u32)| == 2 * 2^32
~~~

~~~rust
|(u32, u32)| == 2^32 * 2^32 == 2^64
~~~

~~~rust
|(A, B)| == A * B
~~~

We refer to tuple-like or struct-like types as **product types**. The
2-tuple is the fundamental product type; larger products can be built
from 2-tuples.

# Cardinality

~~~rust
|Either<bool, u32>|
~~~

# Cardinality

~~~rust
|Either<bool, u32>| == 2 + 2^32
~~~

# Cardinality

~~~rust
|Either<bool, u32>| == 2 + 2^32
~~~

~~~rust
|Either<A, B>| == A + B
~~~

We refer to Either-like types as **sum types**. Either is the fundamental
sum type; larger sums can be built from nested Eithers.

# Cardinality

~~~rust
|Either<bool, u32>| == 2 + 2^32
~~~

~~~rust
|Either<A, B>| == A + B
~~~

We refer to Either-like types as **sum types**. Either is the fundamental
sum type; larger sums can be built from nested Eithers.

~~~rust
enum JValue {
  JBool(bool),
  JNum(f64),
  JStr(String),
  JNull,
  JArray(Vec<JValue>),
  JObject(HashMap<String, JValue>),
}
~~~

# Cardinality

~~~rust
|Either<bool, u32>| == 2 + 2^32
~~~

~~~rust
|Either<A, B>| == A + B
~~~

We refer to Either-like types as **sum types**. Either is the fundamental
sum type; larger sums can be built from nested Eithers.

~~~rust
enum JValue {
  JBool(bool),
  JNum(f64),
  JStr(String),
  JNull,
  JArray(Vec<JValue>),
  JObject(HashMap<String, JValue>),
}
~~~

~~~rust
type JValue = 
  Either<bool, 
    Either<f64, 
      Either<String, 
        Either<JNull, 
          Either<Vec<JValue>, 
                 HashMap<String, JValue>>>>>>;
~~~

# Cardinality

~~~rust
|(bool, u32)| == 2 * 2^32 
~~~

~~~rust
|Either<u32, u32>|
~~~

# Cardinality

~~~rust
|(bool, u32)| == 2 * 2^32
~~~

~~~rust
|Either<u32, u32>| == 2^32 + 2^32
~~~

# Cardinality

~~~rust
|(bool, u32)| == 2 * 2^32 == 2^33
~~~

~~~rust
|Either<u32, u32>| == 2^32 + 2^32 == 2^33
~~~

These occupy the same state space, so they are isomorphic. However,
their implications in terms of program safety are rather different.

You can forget to check a boolean flag, but the compiler won't let
you forget to analyze a variant (constructor).

# Cardinality

~~~rust
// rust

enum Maybe<A> {
  Nothing,
  Just(A),
}
~~~

~~~rust
|Maybe<A>| = |A| + 1
~~~

Without sum types, it can be difficult to accurately represent a
type that has exactly `|A| + 1` inhabitants.

# Cardinality

~~~rust
|Fn(bool) -> u32|
~~~

# Cardinality

~~~rust
|Fn(bool) -> u32|
~~~

~~~rust
f(true) = 1
f(false) = 1
f(true) = 2
f(false) = 2
...
~~~

# Cardinality

~~~rust
|Fn(bool) -> u32| == (2^32) ^ 2
~~~

~~~rust
f(true) = 1
f(false) = 1
f(true) = 2
f(false) = 2
...
~~~

# Cardinality

~~~rust
|Fn(bool) -> u32| == (2^32) ^ 2
~~~

~~~rust
f(true) = 1
f(false) = 1
f(true) = 2
f(false) = 2
...
~~~

~~~rust
|Fn(A) -> B| == |B| ^ |A|
~~~

# Cardinality

~~~rust
|String|
~~~

# Cardinality

~~~rust
|String| == 😬😖😩😷
~~~

* Never use strings.

# Sum Types

~~~rust
// rust

enum Either<A, B> {
  Left(A),
  Right(B),
}
~~~

# Sum Types

~~~rust
// rust

enum Either<A, B> {
  Left(A),
  Right(B),
}
~~~

~~~haskell
-- haskell

data Either a b = Left a | Right b
~~~

# Sum Types

~~~java
// java

interface Either<A, B> {
  <C> C accept(EitherVisitor<A, B, C> c);
}

interface EitherVisitor<A, B, C> {
  C whenLeft(A a);
  C whenRight(B b);
}

class Left<A, B> implements Either<A, B> {
  private A a;
  public Left(A a) { this.a = a; }

  <C> C accept(EitherVisitor<A, B, C> v) {
    return v.whenLeft(this.a);
  }
}

class Right<A, B> ...
~~~

# Sum Types

~~~go
// golang

// ... sorry, try back later
~~~

# Types as sets of operations
~~~rust
// bool

&& // Logical AND
|| // Logical OR
!  // Logical Not
&  // AND
|  // OR
^  // XOR
~~~

# Types as sets of operations

~~~rust
// i32

+   // Addition
-   // Subtraction
*   // Multiplication
/   // Division
%   // Modulus
>   // Greater than
<   // Lesser than
>=  // Greater than or equal to
<=  // Lesser than or equal to
==  // Equality (A == B)
!=  // Not equal  (A != B)
&   // Bitwise AND
|   // Bitwise OR
^   // Bitwise XOR
!   // Bitwise Not
<<  // Left Shift
>>  // Right shift
>>> // Right shift with zero fill

// also... arbitrary imported traits!
~~~

# Types as sets of operations

~~~rust
// String

🤮
~~~

* Never use strings.

# Types as sets of operations

> * If you have a value of a type, you have access to all the operations defined
>   on that type.
> * A lot of the time, most of those operations are not sensible given the context.
> * In order to reduce the operations available in a given scope to a manageable
>   set, we have two good tools: newtypes, and parametricity.

# Newtypes

> * The purpose of a newtype is to reduce the number of operations available on
>   a value.

> * ~~~haskell
>   -- haskell
>   newtype IpAddr = IpAddr Word32
>   ~~~
>
> * ~~~rust
>   // rust
>   struct IpAddr(u32)
>   ~~~

> * `u32` correctly represents the set of possible ip addresses, but it
>   exposes too many operations! If you left-shift an IP address, you get
>   something that is no longer an IP address, or at least doesn't have a
>   meaningful relationship to the original.
>
> * ~~~rust
>   impl IpAddr {
>     fn maskWith(i: u32) -> u32 { ... }
>   }
>   ~~~

> * Prune off everything, then add back the operations you need.

# Newtypes

~~~rust
struct IpAddr(String)

impl IpAddr {
  fn maskWith(i: u32) -> u32 { ... }
}
~~~

A `String` is obviously a lousy representation for an IP address,
but if it supports the operations that we need, we don't really care.

# Parsing > Validation

> * Validation is verifying that a value of a type conforms to its
>   semantic constraints.
>
> * Parsing is ensuring that you never produce values that could
>   violate those constraints in the first place.
>
> * ~~~rust
>   fn parseIpAddr(s: String): Result<IpAddr, ParseError>;
>   ~~~
>
> * We reduce a set with infinite cardinality and infinite operations to 
>   (2^32 + |ParseError|) with a restricted set of operations - and if 
>   `ParseError` is also a well-formed sum type, its cardinality will be small.

# The Vampire Policy

<div align="center">
<img width="600" src="./images/bela-lugosi.jpg"/>

> "Bug fixing strategy: forbid yourself to fix the bug. Instead, render
> the bug impossible by construction."
> --[Paul Phillips](https://twitter.com/extempore2/status/417366903209091073)
</div>

# Parametricity

~~~rust
fn identity<A>(a: &A): &A
~~~

# Parametricity

~~~rust
fn identity<A>(a: &A): &A
~~~

~~~rust
|identity| == ∞ ^ 0 == 1
~~~

# Parametricity

~~~rust
fn identity<A>(a: &A): &A
~~~

~~~rust
|identity| == ∞ ^ 0 == 1
~~~

~~~rust
fn absurd<A>(): &A
~~~

# Parametricity

~~~rust
fn identity<A>(a: &A): &A
~~~

~~~rust
|identity| == ∞ ^ 0 == 1
~~~

~~~rust
fn absurd<A>(): &A
~~~

~~~rust
|absurd| == ∞ ^ 1 == ∞ 
~~~

> * Parametricity allows us to forbid the implementation of a function
>   from performing any operations on a value for which we were not
>   explicitly provided a capability.
>
> * We ensure the maximal possible space of invocations for the caller,
>   and the minimal possible space of implementations for the implementer.
>
> * Principle of Least Power


# Parametricity

~~~rust
trait Monoidal {
    fn mzero() -> Self;
    fn mappend(&self, b: &Self) -> Self;
}
~~~

# Parametricity

~~~rust
trait Monoidal {
    fn mzero() -> Self;
    fn mappend(&self, b: &Self) -> Self;
}
~~~

~~~rust
fn reduce<A: Monoidal>(vs: &Vec<A>) -> A {






}
~~~

# Parametricity

~~~rust
trait Monoidal {
    fn mzero() -> Self;
    fn mappend(&self, b: &Self) -> Self;
}
~~~

~~~rust
fn reduce<A: Monoidal>(vs: &Vec<A>) -> A {
    let mut result = A::mzero();
    for v in vs {
        result = result.mappend(v);
    }

    result
}
~~~

# Parametricity

~~~rust
trait Monoidal {
    fn mzero() -> Self;
    fn mappend(&self, b: &Self) -> Self;
}
~~~

~~~rust
fn reduce<A: Monoidal, V: Iterator<Item = A>>(vs: V) -> A {
    let mut result = A::mzero();
    for v in vs {
        result = result.mappend(&v);
    }

    result
}
~~~

# Parametricity

~~~rust
trait Monoid<A> {
    fn mzero() -> A;
    fn mappend(a: &A, b: &A) -> A;
}
~~~

# Parametricity

~~~rust
trait Monoid<A> {
    fn mzero() -> A;
    fn mappend(a: &A, b: &A) -> A;
}
~~~

~~~rust
fn reduce<A, M: Monoid<A>, V: Iterator<Item = A>>(vs: V) -> A {
    let mut result = M::mzero();

    for v in vs {
        result = M::mappend(&result, &v);
    }

    result
}
~~~

# Parametricity

~~~rust
trait Monoid<A> {
    fn mzero() -> A;
    fn mappend(a: &A, b: &A) -> A;
}
~~~

~~~rust
fn reduce<A, M: Monoid<A>, V: Iterator<Item = A>>(vs: V) -> A {
    ...
}
~~~

~~~rust
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

let res = reduce::<_, I32Mult, _>(vec.into_iter())
~~~

# IO Capabilities

There is another other big category of operation that we want
to think about where we need to respect the Principle of Least Power:

**Low-level IO operations (and the high-level ops built on them)**

<div class="notes">
Most languages expose operations - for accessing the filesystem,
network interfaces, etc. etc. - that require no referent or capability
to access. So, as a matter of discipline, we want to fix this in the
code that *we* write, so that we can have more control over how the
procedures that we write behave.

The example that I always use is... the system clock.
</div>

# IO Capabilities

~~~rust
trait Clock {
    fn now(&self) -> std::time::Instant;
}

struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> std::Time::Instant {
        std::time::Instant::now()
    }
}

fn acceptToMemoryPool<C: Clock>(clock: C, pool: TxMemPool, ...) -> ...
~~~

* **NEVER** use the system clock. Pass a capability!

> * The same goes for any other IO.
>
> * Ideally, the same for access to mutable variables.
>
> * Make it possible to know everything that a procedure can do simply
>   by looking at its inputs.
> 
> * Incidentally, this also makes unit testing vastly easier. Pass mock
>   implementations to procedures under unit testing.

# IO Capabilities

* A caveat: a capability interface should reflect **domain concerns**,
  not implementation concerns.

**BAD**

~~~rust
trait FileCap {
  fn fopen(path: &str, mode: FileMode): Result<Handle>
}

fn getTransaction<F: FileCap>(fileCap: &F, hash: &u256, ...) -> ...
~~~

**GOOD**

~~~rust
trait BlockSource {
  fn readBlock(idx: &BlockIndex) -> Result<Block>
}

fn getTransaction<B: BlockSource>(blockSource: &B, hash: &u256, ...) -> ...
~~~


# Final principles 

> - Make invalid states unrepresentable.
> - Give the minimum possible power to a function's implementer, and the maximum possible flexibility to its caller. Least Power!
> - Type polymorphism reduces the number of things a function can possibly do. Use it.
> - **Strings should appear in your program only where they're being show to a human being.**

# Extra bonus quiz!

~~~haskell
type T a b = forall c. (a -> b -> c) -> c

type E a b = forall c. (a -> c) -> (b -> c) -> c
~~~

~~~haskell
|T Bool Int32|

|E Bool Int32|
~~~

# Extra bonus quiz!

~~~haskell
type T a b = forall c. (a -> b -> c) -> c

type E a b = forall c. (a -> c) -> (b -> c) -> c
~~~

~~~haskell
|T Bool Int32| == 2 * 2^32 = 2^33

|E Bool Int32| == 2 + 2^32
~~~

# Extra bonus quiz (Rustlang Edition)

~~~rust
trait T<A, B> {
    fn cata<F, C>(&self, f: F) -> C
        where F: FnOnce(&A, &B) -> C;
}
~~~

~~~rust
trait E<A, B> {
    fn cata<L, R, C>(&self, l: L, r: R) -> C
        where L: FnOnce(&A) -> C,
              R: FnOnce(&B) -> C;
}
~~~
