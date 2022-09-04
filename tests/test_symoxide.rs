// Copyright (c) 2022 Kaushik Kulkarni
// Copyright (c) 2009-2013 Andreas Kloeckner (for regressions in pymbolic)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::collections::HashSet;
use sym::scalar;
use symoxide as sym;
use symoxide::operations as ops;
use symoxide::parse::parse_expr as parse;

#[test]
fn test_parser() {
    let (a,) = sym::variables!("a");
    parse(concat!("(2*a[1]*b[1]+2*a[0]*b[0])*(hankel_1(-1,sqrt(a[1]**2+a[0]**2)*k) ",
                  "-hankel_1(1,sqrt(a[1]**2+a[0]**2)*k))*k /(4*sqrt(a[1]**2+a[0]**2)) ",
                  "+hankel_1(0,sqrt(a[1]**2+a[0]**2)*k)"));
    assert_eq!(parse("d4knl0"), sym::var("d4knl0"));
    assert_eq!(parse("0."), scalar!(0.));
    assert_eq!(parse("0.e1"), scalar!(0.));
    assert_eq!(parse("1e-12"), scalar!(1e-12));

    assert_eq!(parse("a >= 1"), ops::greater_equal(&a, &1));
    assert_eq!(parse("a <= 1"), ops::less_equal(&a, &1));

    let (foo, bar) = sym::variables!("foo bar");
    assert_eq!(parse("foo[2, bar]"), ops::index(foo, [scalar!(2), bar]));

    let (foo, bar, baz) = sym::variables!("foo bar baz");
    assert_eq!(parse("foo[2, bar, baz]"),
               ops::index(foo, vec![scalar!(2), bar, baz]));
}

#[test]
fn test_get_dependencies() {
    let expr = parse("2*foo(bar, baz[1.0, quux])");
    assert_eq!(sym::get_dependencies(&expr),
               HashSet::from(["foo".to_string(),
                              "bar".to_string(),
                              "baz".to_string(),
                              "quux".to_string()]))
}

#[test]
fn test_get_num_nodes() {
    let expr = parse("2*foo(bar, baz[1.0, quux]) - 1729");
    assert_eq!(sym::get_num_nodes(&expr), 11);
}
