//! Points with dimensions known at compile-time.

#![allow(missing_doc)] // we allow missing to avoid having to document the point components.

use std::mem;
use std::num::{Zero, One, Bounded};
use std::slice::{Items, MutItems};
use std::iter::{Iterator, FromIterator};
use traits::operations::{ApproxEq, POrd, POrdering, PartialLess, PartialEqual,
                         PartialGreater, NotComparable, Axpy};
use traits::structure::{Cast, Dim, Indexable, Iterable, IterableMut, PntAsVec};
use traits::geometry::{Orig, FromHomogeneous, ToHomogeneous};
use structs::vec::{Vec1, Vec2, Vec3, Vec4, Vec5, Vec6};


/// Point of dimension 0.
#[deriving(Eq, PartialEq, Decodable, Clone, Rand, Show)]
pub struct Pnt0<N>;

impl<N> Pnt0<N> {
    /// Creates a new point.
    #[inline]
    pub fn new() -> Pnt0<N> {
        Pnt0
    }

    /// Creates a new point. The parameter is not taken in account.
    #[inline]
    pub fn new_repeat(_: N) -> Pnt0<N> {
        Pnt0
    }
}

/// Point of dimension 1.
#[deriving(Eq, PartialEq, Encodable, Decodable, Clone, Hash, Rand, Show)]
pub struct Pnt1<N> {
    /// First component of the point.
    pub x: N
}

double_dispatch_binop_decl_trait!(Pnt1, Pnt1MulRhs)
double_dispatch_binop_decl_trait!(Pnt1, Pnt1DivRhs)
double_dispatch_binop_decl_trait!(Pnt1, Pnt1AddRhs)
double_dispatch_binop_decl_trait!(Pnt1, Pnt1SubRhs)
double_dispatch_cast_decl_trait!(Pnt1, Pnt1Cast)
mul_redispatch_impl!(Pnt1, Pnt1MulRhs)
div_redispatch_impl!(Pnt1, Pnt1DivRhs)
add_redispatch_impl!(Pnt1, Pnt1AddRhs)
sub_redispatch_impl!(Pnt1, Pnt1SubRhs)
cast_redispatch_impl!(Pnt1, Pnt1Cast)
new_impl!(Pnt1, x)
orig_impl!(Pnt1, x)
ord_impl!(Pnt1, x)
vec_cast_impl!(Pnt1, Pnt1Cast, x)
as_slice_impl!(Pnt1, 1)
index_impl!(Pnt1)
indexable_impl!(Pnt1, 1)
at_fast_impl!(Pnt1, 1)
new_repeat_impl!(Pnt1, val, x)
dim_impl!(Pnt1, 1)
container_impl!(Pnt1)
pnt_as_vec_impl!(Pnt1, Vec1, x)
pnt_sub_impl!(Pnt1, Vec1, Pnt1SubRhs)
neg_impl!(Pnt1, x)
pnt_add_vec_impl!(Pnt1, Vec1, Pnt1AddRhs, x)
pnt_sub_vec_impl!(Pnt1, Vec1, Pnt1SubRhs, x)
vec_mul_scalar_impl!(Pnt1, f64, Pnt1MulRhs, x)
vec_mul_scalar_impl!(Pnt1, f32, Pnt1MulRhs, x)
vec_mul_scalar_impl!(Pnt1, u64, Pnt1MulRhs, x)
vec_mul_scalar_impl!(Pnt1, u32, Pnt1MulRhs, x)
vec_mul_scalar_impl!(Pnt1, u16, Pnt1MulRhs, x)
vec_mul_scalar_impl!(Pnt1, u8, Pnt1MulRhs, x)
vec_mul_scalar_impl!(Pnt1, i64, Pnt1MulRhs, x)
vec_mul_scalar_impl!(Pnt1, i32, Pnt1MulRhs, x)
vec_mul_scalar_impl!(Pnt1, i16, Pnt1MulRhs, x)
vec_mul_scalar_impl!(Pnt1, i8, Pnt1MulRhs, x)
vec_mul_scalar_impl!(Pnt1, uint, Pnt1MulRhs, x)
vec_mul_scalar_impl!(Pnt1, int, Pnt1MulRhs, x)
vec_div_scalar_impl!(Pnt1, f64, Pnt1DivRhs, x)
vec_div_scalar_impl!(Pnt1, f32, Pnt1DivRhs, x)
vec_div_scalar_impl!(Pnt1, u64, Pnt1DivRhs, x)
vec_div_scalar_impl!(Pnt1, u32, Pnt1DivRhs, x)
vec_div_scalar_impl!(Pnt1, u16, Pnt1DivRhs, x)
vec_div_scalar_impl!(Pnt1, u8, Pnt1DivRhs, x)
vec_div_scalar_impl!(Pnt1, i64, Pnt1DivRhs, x)
vec_div_scalar_impl!(Pnt1, i32, Pnt1DivRhs, x)
vec_div_scalar_impl!(Pnt1, i16, Pnt1DivRhs, x)
vec_div_scalar_impl!(Pnt1, i8, Pnt1DivRhs, x)
vec_div_scalar_impl!(Pnt1, uint, Pnt1DivRhs, x)
vec_div_scalar_impl!(Pnt1, int, Pnt1DivRhs, x)
vec_add_scalar_impl!(Pnt1, f64, Pnt1AddRhs, x)
vec_add_scalar_impl!(Pnt1, f32, Pnt1AddRhs, x)
vec_add_scalar_impl!(Pnt1, u64, Pnt1AddRhs, x)
vec_add_scalar_impl!(Pnt1, u32, Pnt1AddRhs, x)
vec_add_scalar_impl!(Pnt1, u16, Pnt1AddRhs, x)
vec_add_scalar_impl!(Pnt1, u8, Pnt1AddRhs, x)
vec_add_scalar_impl!(Pnt1, i64, Pnt1AddRhs, x)
vec_add_scalar_impl!(Pnt1, i32, Pnt1AddRhs, x)
vec_add_scalar_impl!(Pnt1, i16, Pnt1AddRhs, x)
vec_add_scalar_impl!(Pnt1, i8, Pnt1AddRhs, x)
vec_add_scalar_impl!(Pnt1, uint, Pnt1AddRhs, x)
vec_add_scalar_impl!(Pnt1, int, Pnt1AddRhs, x)
vec_sub_scalar_impl!(Pnt1, f64, Pnt1SubRhs, x)
vec_sub_scalar_impl!(Pnt1, f32, Pnt1SubRhs, x)
vec_sub_scalar_impl!(Pnt1, u64, Pnt1SubRhs, x)
vec_sub_scalar_impl!(Pnt1, u32, Pnt1SubRhs, x)
vec_sub_scalar_impl!(Pnt1, u16, Pnt1SubRhs, x)
vec_sub_scalar_impl!(Pnt1, u8, Pnt1SubRhs, x)
vec_sub_scalar_impl!(Pnt1, i64, Pnt1SubRhs, x)
vec_sub_scalar_impl!(Pnt1, i32, Pnt1SubRhs, x)
vec_sub_scalar_impl!(Pnt1, i16, Pnt1SubRhs, x)
vec_sub_scalar_impl!(Pnt1, i8, Pnt1SubRhs, x)
vec_sub_scalar_impl!(Pnt1, uint, Pnt1SubRhs, x)
vec_sub_scalar_impl!(Pnt1, int, Pnt1SubRhs, x)
approx_eq_impl!(Pnt1, x)
from_iterator_impl!(Pnt1, iterator)
bounded_impl!(Pnt1, x)
axpy_impl!(Pnt1, x)
iterable_impl!(Pnt1, 1)
iterable_mut_impl!(Pnt1, 1)
pnt_to_homogeneous_impl!(Pnt1, Pnt2, y, x)
pnt_from_homogeneous_impl!(Pnt1, Pnt2, y, x)

/// Point of dimension 2.
#[deriving(Eq, PartialEq, Encodable, Decodable, Clone, Hash, Rand, Show)]
pub struct Pnt2<N> {
    /// First component of the point.
    pub x: N,
    /// Second component of the point.
    pub y: N
}

double_dispatch_binop_decl_trait!(Pnt2, Pnt2MulRhs)
double_dispatch_binop_decl_trait!(Pnt2, Pnt2DivRhs)
double_dispatch_binop_decl_trait!(Pnt2, Pnt2AddRhs)
double_dispatch_binop_decl_trait!(Pnt2, Pnt2SubRhs)
double_dispatch_cast_decl_trait!(Pnt2, Pnt2Cast)
mul_redispatch_impl!(Pnt2, Pnt2MulRhs)
div_redispatch_impl!(Pnt2, Pnt2DivRhs)
add_redispatch_impl!(Pnt2, Pnt2AddRhs)
sub_redispatch_impl!(Pnt2, Pnt2SubRhs)
cast_redispatch_impl!(Pnt2, Pnt2Cast)
new_impl!(Pnt2, x, y)
orig_impl!(Pnt2, x, y)
ord_impl!(Pnt2, x, y)
vec_cast_impl!(Pnt2, Pnt2Cast, x, y)
as_slice_impl!(Pnt2, 2)
index_impl!(Pnt2)
indexable_impl!(Pnt2, 2)
at_fast_impl!(Pnt2, 2)
new_repeat_impl!(Pnt2, val, x, y)
dim_impl!(Pnt2, 2)
container_impl!(Pnt2)
pnt_as_vec_impl!(Pnt2, Vec2, x, y)
pnt_sub_impl!(Pnt2, Vec2, Pnt2SubRhs)
neg_impl!(Pnt2, x, y)
pnt_add_vec_impl!(Pnt2, Vec2, Pnt2AddRhs, x, y)
pnt_sub_vec_impl!(Pnt2, Vec2, Pnt2SubRhs, x, y)
vec_mul_scalar_impl!(Pnt2, f64, Pnt2MulRhs, x, y)
vec_mul_scalar_impl!(Pnt2, f32, Pnt2MulRhs, x, y)
vec_mul_scalar_impl!(Pnt2, u64, Pnt2MulRhs, x, y)
vec_mul_scalar_impl!(Pnt2, u32, Pnt2MulRhs, x, y)
vec_mul_scalar_impl!(Pnt2, u16, Pnt2MulRhs, x, y)
vec_mul_scalar_impl!(Pnt2, u8, Pnt2MulRhs, x, y)
vec_mul_scalar_impl!(Pnt2, i64, Pnt2MulRhs, x, y)
vec_mul_scalar_impl!(Pnt2, i32, Pnt2MulRhs, x, y)
vec_mul_scalar_impl!(Pnt2, i16, Pnt2MulRhs, x, y)
vec_mul_scalar_impl!(Pnt2, i8, Pnt2MulRhs, x, y)
vec_mul_scalar_impl!(Pnt2, uint, Pnt2MulRhs, x, y)
vec_mul_scalar_impl!(Pnt2, int, Pnt2MulRhs, x, y)
vec_div_scalar_impl!(Pnt2, f64, Pnt2DivRhs, x, y)
vec_div_scalar_impl!(Pnt2, f32, Pnt2DivRhs, x, y)
vec_div_scalar_impl!(Pnt2, u64, Pnt2DivRhs, x, y)
vec_div_scalar_impl!(Pnt2, u32, Pnt2DivRhs, x, y)
vec_div_scalar_impl!(Pnt2, u16, Pnt2DivRhs, x, y)
vec_div_scalar_impl!(Pnt2, u8, Pnt2DivRhs, x, y)
vec_div_scalar_impl!(Pnt2, i64, Pnt2DivRhs, x, y)
vec_div_scalar_impl!(Pnt2, i32, Pnt2DivRhs, x, y)
vec_div_scalar_impl!(Pnt2, i16, Pnt2DivRhs, x, y)
vec_div_scalar_impl!(Pnt2, i8, Pnt2DivRhs, x, y)
vec_div_scalar_impl!(Pnt2, uint, Pnt2DivRhs, x, y)
vec_div_scalar_impl!(Pnt2, int, Pnt2DivRhs, x, y)
vec_add_scalar_impl!(Pnt2, f64, Pnt2AddRhs, x, y)
vec_add_scalar_impl!(Pnt2, f32, Pnt2AddRhs, x, y)
vec_add_scalar_impl!(Pnt2, u64, Pnt2AddRhs, x, y)
vec_add_scalar_impl!(Pnt2, u32, Pnt2AddRhs, x, y)
vec_add_scalar_impl!(Pnt2, u16, Pnt2AddRhs, x, y)
vec_add_scalar_impl!(Pnt2, u8, Pnt2AddRhs, x, y)
vec_add_scalar_impl!(Pnt2, i64, Pnt2AddRhs, x, y)
vec_add_scalar_impl!(Pnt2, i32, Pnt2AddRhs, x, y)
vec_add_scalar_impl!(Pnt2, i16, Pnt2AddRhs, x, y)
vec_add_scalar_impl!(Pnt2, i8, Pnt2AddRhs, x, y)
vec_add_scalar_impl!(Pnt2, uint, Pnt2AddRhs, x, y)
vec_add_scalar_impl!(Pnt2, int, Pnt2AddRhs, x, y)
vec_sub_scalar_impl!(Pnt2, f64, Pnt2SubRhs, x, y)
vec_sub_scalar_impl!(Pnt2, f32, Pnt2SubRhs, x, y)
vec_sub_scalar_impl!(Pnt2, u64, Pnt2SubRhs, x, y)
vec_sub_scalar_impl!(Pnt2, u32, Pnt2SubRhs, x, y)
vec_sub_scalar_impl!(Pnt2, u16, Pnt2SubRhs, x, y)
vec_sub_scalar_impl!(Pnt2, u8, Pnt2SubRhs, x, y)
vec_sub_scalar_impl!(Pnt2, i64, Pnt2SubRhs, x, y)
vec_sub_scalar_impl!(Pnt2, i32, Pnt2SubRhs, x, y)
vec_sub_scalar_impl!(Pnt2, i16, Pnt2SubRhs, x, y)
vec_sub_scalar_impl!(Pnt2, i8, Pnt2SubRhs, x, y)
vec_sub_scalar_impl!(Pnt2, uint, Pnt2SubRhs, x, y)
vec_sub_scalar_impl!(Pnt2, int, Pnt2SubRhs, x, y)
approx_eq_impl!(Pnt2, x, y)
from_iterator_impl!(Pnt2, iterator, iterator)
bounded_impl!(Pnt2, x, y)
axpy_impl!(Pnt2, x, y)
iterable_impl!(Pnt2, 2)
iterable_mut_impl!(Pnt2, 2)
pnt_to_homogeneous_impl!(Pnt2, Pnt3, z, x, y)
pnt_from_homogeneous_impl!(Pnt2, Pnt3, z, x, y)

/// Point of dimension 3.
#[deriving(Eq, PartialEq, Encodable, Decodable, Clone, Hash, Rand, Show)]
pub struct Pnt3<N> {
    /// First component of the point.
    pub x: N,
    /// Second component of the point.
    pub y: N,
    /// Third component of the point.
    pub z: N
}

double_dispatch_binop_decl_trait!(Pnt3, Pnt3MulRhs)
double_dispatch_binop_decl_trait!(Pnt3, Pnt3DivRhs)
double_dispatch_binop_decl_trait!(Pnt3, Pnt3AddRhs)
double_dispatch_binop_decl_trait!(Pnt3, Pnt3SubRhs)
double_dispatch_cast_decl_trait!(Pnt3, Pnt3Cast)
mul_redispatch_impl!(Pnt3, Pnt3MulRhs)
div_redispatch_impl!(Pnt3, Pnt3DivRhs)
add_redispatch_impl!(Pnt3, Pnt3AddRhs)
sub_redispatch_impl!(Pnt3, Pnt3SubRhs)
cast_redispatch_impl!(Pnt3, Pnt3Cast)
new_impl!(Pnt3, x, y, z)
orig_impl!(Pnt3, x, y, z)
ord_impl!(Pnt3, x, y, z)
vec_cast_impl!(Pnt3, Pnt3Cast, x, y, z)
as_slice_impl!(Pnt3, 3)
index_impl!(Pnt3)
indexable_impl!(Pnt3, 3)
at_fast_impl!(Pnt3, 3)
new_repeat_impl!(Pnt3, val, x, y, z)
dim_impl!(Pnt3, 3)
container_impl!(Pnt3)
pnt_as_vec_impl!(Pnt3, Vec3, x, y, z)
pnt_sub_impl!(Pnt3, Vec3, Pnt3SubRhs)
neg_impl!(Pnt3, x, y, z)
pnt_add_vec_impl!(Pnt3, Vec3, Pnt3AddRhs, x, y, z)
pnt_sub_vec_impl!(Pnt3, Vec3, Pnt3SubRhs, x, y, z)
vec_mul_scalar_impl!(Pnt3, f64, Pnt3MulRhs, x, y, z)
vec_mul_scalar_impl!(Pnt3, f32, Pnt3MulRhs, x, y, z)
vec_mul_scalar_impl!(Pnt3, u64, Pnt3MulRhs, x, y, z)
vec_mul_scalar_impl!(Pnt3, u32, Pnt3MulRhs, x, y, z)
vec_mul_scalar_impl!(Pnt3, u16, Pnt3MulRhs, x, y, z)
vec_mul_scalar_impl!(Pnt3, u8, Pnt3MulRhs, x, y, z)
vec_mul_scalar_impl!(Pnt3, i64, Pnt3MulRhs, x, y, z)
vec_mul_scalar_impl!(Pnt3, i32, Pnt3MulRhs, x, y, z)
vec_mul_scalar_impl!(Pnt3, i16, Pnt3MulRhs, x, y, z)
vec_mul_scalar_impl!(Pnt3, i8, Pnt3MulRhs, x, y, z)
vec_mul_scalar_impl!(Pnt3, uint, Pnt3MulRhs, x, y, z)
vec_mul_scalar_impl!(Pnt3, int, Pnt3MulRhs, x, y, z)
vec_div_scalar_impl!(Pnt3, f64, Pnt3DivRhs, x, y, z)
vec_div_scalar_impl!(Pnt3, f32, Pnt3DivRhs, x, y, z)
vec_div_scalar_impl!(Pnt3, u64, Pnt3DivRhs, x, y, z)
vec_div_scalar_impl!(Pnt3, u32, Pnt3DivRhs, x, y, z)
vec_div_scalar_impl!(Pnt3, u16, Pnt3DivRhs, x, y, z)
vec_div_scalar_impl!(Pnt3, u8, Pnt3DivRhs, x, y, z)
vec_div_scalar_impl!(Pnt3, i64, Pnt3DivRhs, x, y, z)
vec_div_scalar_impl!(Pnt3, i32, Pnt3DivRhs, x, y, z)
vec_div_scalar_impl!(Pnt3, i16, Pnt3DivRhs, x, y, z)
vec_div_scalar_impl!(Pnt3, i8, Pnt3DivRhs, x, y, z)
vec_div_scalar_impl!(Pnt3, uint, Pnt3DivRhs, x, y, z)
vec_div_scalar_impl!(Pnt3, int, Pnt3DivRhs, x, y, z)
vec_add_scalar_impl!(Pnt3, f64, Pnt3AddRhs, x, y, z)
vec_add_scalar_impl!(Pnt3, f32, Pnt3AddRhs, x, y, z)
vec_add_scalar_impl!(Pnt3, u64, Pnt3AddRhs, x, y, z)
vec_add_scalar_impl!(Pnt3, u32, Pnt3AddRhs, x, y, z)
vec_add_scalar_impl!(Pnt3, u16, Pnt3AddRhs, x, y, z)
vec_add_scalar_impl!(Pnt3, u8, Pnt3AddRhs, x, y, z)
vec_add_scalar_impl!(Pnt3, i64, Pnt3AddRhs, x, y, z)
vec_add_scalar_impl!(Pnt3, i32, Pnt3AddRhs, x, y, z)
vec_add_scalar_impl!(Pnt3, i16, Pnt3AddRhs, x, y, z)
vec_add_scalar_impl!(Pnt3, i8, Pnt3AddRhs, x, y, z)
vec_add_scalar_impl!(Pnt3, uint, Pnt3AddRhs, x, y, z)
vec_add_scalar_impl!(Pnt3, int, Pnt3AddRhs, x, y, z)
vec_sub_scalar_impl!(Pnt3, f64, Pnt3SubRhs, x, y, z)
vec_sub_scalar_impl!(Pnt3, f32, Pnt3SubRhs, x, y, z)
vec_sub_scalar_impl!(Pnt3, u64, Pnt3SubRhs, x, y, z)
vec_sub_scalar_impl!(Pnt3, u32, Pnt3SubRhs, x, y, z)
vec_sub_scalar_impl!(Pnt3, u16, Pnt3SubRhs, x, y, z)
vec_sub_scalar_impl!(Pnt3, u8, Pnt3SubRhs, x, y, z)
vec_sub_scalar_impl!(Pnt3, i64, Pnt3SubRhs, x, y, z)
vec_sub_scalar_impl!(Pnt3, i32, Pnt3SubRhs, x, y, z)
vec_sub_scalar_impl!(Pnt3, i16, Pnt3SubRhs, x, y, z)
vec_sub_scalar_impl!(Pnt3, i8, Pnt3SubRhs, x, y, z)
vec_sub_scalar_impl!(Pnt3, uint, Pnt3SubRhs, x, y, z)
vec_sub_scalar_impl!(Pnt3, int, Pnt3SubRhs, x, y, z)
approx_eq_impl!(Pnt3, x, y, z)
from_iterator_impl!(Pnt3, iterator, iterator, iterator)
bounded_impl!(Pnt3, x, y, z)
axpy_impl!(Pnt3, x, y, z)
iterable_impl!(Pnt3, 3)
iterable_mut_impl!(Pnt3, 3)
pnt_to_homogeneous_impl!(Pnt3, Pnt4, w, x, y, z)
pnt_from_homogeneous_impl!(Pnt3, Pnt4, w, x, y, z)

/// Point of dimension 4.
#[deriving(Eq, PartialEq, Encodable, Decodable, Clone, Hash, Rand, Show)]
pub struct Pnt4<N> {
    /// First component of the point.
    pub x: N,
    /// Second component of the point.
    pub y: N,
    /// Third component of the point.
    pub z: N,
    /// Fourth component of the point.
    pub w: N
}

double_dispatch_binop_decl_trait!(Pnt4, Pnt4MulRhs)
double_dispatch_binop_decl_trait!(Pnt4, Pnt4DivRhs)
double_dispatch_binop_decl_trait!(Pnt4, Pnt4AddRhs)
double_dispatch_binop_decl_trait!(Pnt4, Pnt4SubRhs)
double_dispatch_cast_decl_trait!(Pnt4, Pnt4Cast)
mul_redispatch_impl!(Pnt4, Pnt4MulRhs)
div_redispatch_impl!(Pnt4, Pnt4DivRhs)
add_redispatch_impl!(Pnt4, Pnt4AddRhs)
sub_redispatch_impl!(Pnt4, Pnt4SubRhs)
cast_redispatch_impl!(Pnt4, Pnt4Cast)
new_impl!(Pnt4, x, y, z, w)
orig_impl!(Pnt4, x, y, z, w)
ord_impl!(Pnt4, x, y, z, w)
vec_cast_impl!(Pnt4, Pnt4Cast, x, y, z, w)
as_slice_impl!(Pnt4, 4)
index_impl!(Pnt4)
indexable_impl!(Pnt4, 4)
at_fast_impl!(Pnt4, 4)
new_repeat_impl!(Pnt4, val, x, y, z, w)
dim_impl!(Pnt4, 4)
container_impl!(Pnt4)
pnt_as_vec_impl!(Pnt4, Vec4, x, y, z, w)
pnt_sub_impl!(Pnt4, Vec4, Pnt4SubRhs)
neg_impl!(Pnt4, x, y, z, w)
pnt_add_vec_impl!(Pnt4, Vec4, Pnt4AddRhs, x, y, z, w)
pnt_sub_vec_impl!(Pnt4, Vec4, Pnt4SubRhs, x, y, z, w)
vec_mul_scalar_impl!(Pnt4, f64, Pnt4MulRhs, x, y, z, w)
vec_mul_scalar_impl!(Pnt4, f32, Pnt4MulRhs, x, y, z, w)
vec_mul_scalar_impl!(Pnt4, u64, Pnt4MulRhs, x, y, z, w)
vec_mul_scalar_impl!(Pnt4, u32, Pnt4MulRhs, x, y, z, w)
vec_mul_scalar_impl!(Pnt4, u16, Pnt4MulRhs, x, y, z, w)
vec_mul_scalar_impl!(Pnt4, u8, Pnt4MulRhs, x, y, z, w)
vec_mul_scalar_impl!(Pnt4, i64, Pnt4MulRhs, x, y, z, w)
vec_mul_scalar_impl!(Pnt4, i32, Pnt4MulRhs, x, y, z, w)
vec_mul_scalar_impl!(Pnt4, i16, Pnt4MulRhs, x, y, z, w)
vec_mul_scalar_impl!(Pnt4, i8, Pnt4MulRhs, x, y, z, w)
vec_mul_scalar_impl!(Pnt4, uint, Pnt4MulRhs, x, y, z, w)
vec_mul_scalar_impl!(Pnt4, int, Pnt4MulRhs, x, y, z, w)
vec_div_scalar_impl!(Pnt4, f64, Pnt4DivRhs, x, y, z, w)
vec_div_scalar_impl!(Pnt4, f32, Pnt4DivRhs, x, y, z, w)
vec_div_scalar_impl!(Pnt4, u64, Pnt4DivRhs, x, y, z, w)
vec_div_scalar_impl!(Pnt4, u32, Pnt4DivRhs, x, y, z, w)
vec_div_scalar_impl!(Pnt4, u16, Pnt4DivRhs, x, y, z, w)
vec_div_scalar_impl!(Pnt4, u8, Pnt4DivRhs, x, y, z, w)
vec_div_scalar_impl!(Pnt4, i64, Pnt4DivRhs, x, y, z, w)
vec_div_scalar_impl!(Pnt4, i32, Pnt4DivRhs, x, y, z, w)
vec_div_scalar_impl!(Pnt4, i16, Pnt4DivRhs, x, y, z, w)
vec_div_scalar_impl!(Pnt4, i8, Pnt4DivRhs, x, y, z, w)
vec_div_scalar_impl!(Pnt4, uint, Pnt4DivRhs, x, y, z, w)
vec_div_scalar_impl!(Pnt4, int, Pnt4DivRhs, x, y, z, w)
vec_add_scalar_impl!(Pnt4, f64, Pnt4AddRhs, x, y, z, w)
vec_add_scalar_impl!(Pnt4, f32, Pnt4AddRhs, x, y, z, w)
vec_add_scalar_impl!(Pnt4, u64, Pnt4AddRhs, x, y, z, w)
vec_add_scalar_impl!(Pnt4, u32, Pnt4AddRhs, x, y, z, w)
vec_add_scalar_impl!(Pnt4, u16, Pnt4AddRhs, x, y, z, w)
vec_add_scalar_impl!(Pnt4, u8, Pnt4AddRhs, x, y, z, w)
vec_add_scalar_impl!(Pnt4, i64, Pnt4AddRhs, x, y, z, w)
vec_add_scalar_impl!(Pnt4, i32, Pnt4AddRhs, x, y, z, w)
vec_add_scalar_impl!(Pnt4, i16, Pnt4AddRhs, x, y, z, w)
vec_add_scalar_impl!(Pnt4, i8, Pnt4AddRhs, x, y, z, w)
vec_add_scalar_impl!(Pnt4, uint, Pnt4AddRhs, x, y, z, w)
vec_add_scalar_impl!(Pnt4, int, Pnt4AddRhs, x, y, z, w)
vec_sub_scalar_impl!(Pnt4, f64, Pnt4SubRhs, x, y, z, w)
vec_sub_scalar_impl!(Pnt4, f32, Pnt4SubRhs, x, y, z, w)
vec_sub_scalar_impl!(Pnt4, u64, Pnt4SubRhs, x, y, z, w)
vec_sub_scalar_impl!(Pnt4, u32, Pnt4SubRhs, x, y, z, w)
vec_sub_scalar_impl!(Pnt4, u16, Pnt4SubRhs, x, y, z, w)
vec_sub_scalar_impl!(Pnt4, u8, Pnt4SubRhs, x, y, z, w)
vec_sub_scalar_impl!(Pnt4, i64, Pnt4SubRhs, x, y, z, w)
vec_sub_scalar_impl!(Pnt4, i32, Pnt4SubRhs, x, y, z, w)
vec_sub_scalar_impl!(Pnt4, i16, Pnt4SubRhs, x, y, z, w)
vec_sub_scalar_impl!(Pnt4, i8, Pnt4SubRhs, x, y, z, w)
vec_sub_scalar_impl!(Pnt4, uint, Pnt4SubRhs, x, y, z, w)
vec_sub_scalar_impl!(Pnt4, int, Pnt4SubRhs, x, y, z, w)
approx_eq_impl!(Pnt4, x, y, z, w)
from_iterator_impl!(Pnt4, iterator, iterator, iterator, iterator)
bounded_impl!(Pnt4, x, y, z, w)
axpy_impl!(Pnt4, x, y, z, w)
iterable_impl!(Pnt4, 4)
iterable_mut_impl!(Pnt4, 4)
pnt_to_homogeneous_impl!(Pnt4, Pnt5, a, x, y, z, w)
pnt_from_homogeneous_impl!(Pnt4, Pnt5, a, x, y, z, w)

/// Point of dimension 5.
#[deriving(Eq, PartialEq, Encodable, Decodable, Clone, Hash, Rand, Show)]
pub struct Pnt5<N> {
    /// First component of the point.
    pub x: N,
    /// Second component of the point.
    pub y: N,
    /// Third component of the point.
    pub z: N,
    /// Fourth component of the point.
    pub w: N,
    /// Fifth of the point.
    pub a: N
}

double_dispatch_binop_decl_trait!(Pnt5, Pnt5MulRhs)
double_dispatch_binop_decl_trait!(Pnt5, Pnt5DivRhs)
double_dispatch_binop_decl_trait!(Pnt5, Pnt5AddRhs)
double_dispatch_binop_decl_trait!(Pnt5, Pnt5SubRhs)
double_dispatch_cast_decl_trait!(Pnt5, Pnt5Cast)
mul_redispatch_impl!(Pnt5, Pnt5MulRhs)
div_redispatch_impl!(Pnt5, Pnt5DivRhs)
add_redispatch_impl!(Pnt5, Pnt5AddRhs)
sub_redispatch_impl!(Pnt5, Pnt5SubRhs)
cast_redispatch_impl!(Pnt5, Pnt5Cast)
new_impl!(Pnt5, x, y, z, w, a)
orig_impl!(Pnt5, x, y, z, w, a)
ord_impl!(Pnt5, x, y, z, w, a)
vec_cast_impl!(Pnt5, Pnt5Cast, x, y, z, w, a)
as_slice_impl!(Pnt5, 5)
index_impl!(Pnt5)
indexable_impl!(Pnt5, 5)
at_fast_impl!(Pnt5, 5)
new_repeat_impl!(Pnt5, val, x, y, z, w, a)
dim_impl!(Pnt5, 5)
container_impl!(Pnt5)
pnt_as_vec_impl!(Pnt5, Vec5, x, y, z, w, a)
pnt_sub_impl!(Pnt5, Vec5, Pnt5SubRhs)
neg_impl!(Pnt5, x, y, z, w, a)
pnt_add_vec_impl!(Pnt5, Vec5, Pnt5AddRhs, x, y, z, w, a)
pnt_sub_vec_impl!(Pnt5, Vec5, Pnt5SubRhs, x, y, z, w, a)
vec_mul_scalar_impl!(Pnt5, f64, Pnt5MulRhs, x, y, z, w, a)
vec_mul_scalar_impl!(Pnt5, f32, Pnt5MulRhs, x, y, z, w, a)
vec_mul_scalar_impl!(Pnt5, u64, Pnt5MulRhs, x, y, z, w, a)
vec_mul_scalar_impl!(Pnt5, u32, Pnt5MulRhs, x, y, z, w, a)
vec_mul_scalar_impl!(Pnt5, u16, Pnt5MulRhs, x, y, z, w, a)
vec_mul_scalar_impl!(Pnt5, u8, Pnt5MulRhs, x, y, z, w, a)
vec_mul_scalar_impl!(Pnt5, i64, Pnt5MulRhs, x, y, z, w, a)
vec_mul_scalar_impl!(Pnt5, i32, Pnt5MulRhs, x, y, z, w, a)
vec_mul_scalar_impl!(Pnt5, i16, Pnt5MulRhs, x, y, z, w, a)
vec_mul_scalar_impl!(Pnt5, i8, Pnt5MulRhs, x, y, z, w, a)
vec_mul_scalar_impl!(Pnt5, uint, Pnt5MulRhs, x, y, z, w, a)
vec_mul_scalar_impl!(Pnt5, int, Pnt5MulRhs, x, y, z, w, a)
vec_div_scalar_impl!(Pnt5, f64, Pnt5DivRhs, x, y, z, w, a)
vec_div_scalar_impl!(Pnt5, f32, Pnt5DivRhs, x, y, z, w, a)
vec_div_scalar_impl!(Pnt5, u64, Pnt5DivRhs, x, y, z, w, a)
vec_div_scalar_impl!(Pnt5, u32, Pnt5DivRhs, x, y, z, w, a)
vec_div_scalar_impl!(Pnt5, u16, Pnt5DivRhs, x, y, z, w, a)
vec_div_scalar_impl!(Pnt5, u8, Pnt5DivRhs, x, y, z, w, a)
vec_div_scalar_impl!(Pnt5, i64, Pnt5DivRhs, x, y, z, w, a)
vec_div_scalar_impl!(Pnt5, i32, Pnt5DivRhs, x, y, z, w, a)
vec_div_scalar_impl!(Pnt5, i16, Pnt5DivRhs, x, y, z, w, a)
vec_div_scalar_impl!(Pnt5, i8, Pnt5DivRhs, x, y, z, w, a)
vec_div_scalar_impl!(Pnt5, uint, Pnt5DivRhs, x, y, z, w, a)
vec_div_scalar_impl!(Pnt5, int, Pnt5DivRhs, x, y, z, w, a)
vec_add_scalar_impl!(Pnt5, f64, Pnt5AddRhs, x, y, z, w, a)
vec_add_scalar_impl!(Pnt5, f32, Pnt5AddRhs, x, y, z, w, a)
vec_add_scalar_impl!(Pnt5, u64, Pnt5AddRhs, x, y, z, w, a)
vec_add_scalar_impl!(Pnt5, u32, Pnt5AddRhs, x, y, z, w, a)
vec_add_scalar_impl!(Pnt5, u16, Pnt5AddRhs, x, y, z, w, a)
vec_add_scalar_impl!(Pnt5, u8, Pnt5AddRhs, x, y, z, w, a)
vec_add_scalar_impl!(Pnt5, i64, Pnt5AddRhs, x, y, z, w, a)
vec_add_scalar_impl!(Pnt5, i32, Pnt5AddRhs, x, y, z, w, a)
vec_add_scalar_impl!(Pnt5, i16, Pnt5AddRhs, x, y, z, w, a)
vec_add_scalar_impl!(Pnt5, i8, Pnt5AddRhs, x, y, z, w, a)
vec_add_scalar_impl!(Pnt5, uint, Pnt5AddRhs, x, y, z, w, a)
vec_add_scalar_impl!(Pnt5, int, Pnt5AddRhs, x, y, z, w, a)
vec_sub_scalar_impl!(Pnt5, f64, Pnt5SubRhs, x, y, z, w, a)
vec_sub_scalar_impl!(Pnt5, f32, Pnt5SubRhs, x, y, z, w, a)
vec_sub_scalar_impl!(Pnt5, u64, Pnt5SubRhs, x, y, z, w, a)
vec_sub_scalar_impl!(Pnt5, u32, Pnt5SubRhs, x, y, z, w, a)
vec_sub_scalar_impl!(Pnt5, u16, Pnt5SubRhs, x, y, z, w, a)
vec_sub_scalar_impl!(Pnt5, u8, Pnt5SubRhs, x, y, z, w, a)
vec_sub_scalar_impl!(Pnt5, i64, Pnt5SubRhs, x, y, z, w, a)
vec_sub_scalar_impl!(Pnt5, i32, Pnt5SubRhs, x, y, z, w, a)
vec_sub_scalar_impl!(Pnt5, i16, Pnt5SubRhs, x, y, z, w, a)
vec_sub_scalar_impl!(Pnt5, i8, Pnt5SubRhs, x, y, z, w, a)
vec_sub_scalar_impl!(Pnt5, uint, Pnt5SubRhs, x, y, z, w, a)
vec_sub_scalar_impl!(Pnt5, int, Pnt5SubRhs, x, y, z, w, a)
approx_eq_impl!(Pnt5, x, y, z, w, a)
from_iterator_impl!(Pnt5, iterator, iterator, iterator, iterator, iterator)
bounded_impl!(Pnt5, x, y, z, w, a)
axpy_impl!(Pnt5, x, y, z, w, a)
iterable_impl!(Pnt5, 5)
iterable_mut_impl!(Pnt5, 5)
pnt_to_homogeneous_impl!(Pnt5, Pnt6, b, x, y, z, w, a)
pnt_from_homogeneous_impl!(Pnt5, Pnt6, b, x, y, z, w, a)

/// Point of dimension 6.
#[deriving(Eq, PartialEq, Encodable, Decodable, Clone, Hash, Rand, Show)]
pub struct Pnt6<N> {
    /// First component of the point.
    pub x: N,
    /// Second component of the point.
    pub y: N,
    /// Third component of the point.
    pub z: N,
    /// Fourth component of the point.
    pub w: N,
    /// Fifth of the point.
    pub a: N,
    /// Sixth component of the point.
    pub b: N
}

double_dispatch_binop_decl_trait!(Pnt6, Pnt6MulRhs)
double_dispatch_binop_decl_trait!(Pnt6, Pnt6DivRhs)
double_dispatch_binop_decl_trait!(Pnt6, Pnt6AddRhs)
double_dispatch_binop_decl_trait!(Pnt6, Pnt6SubRhs)
double_dispatch_cast_decl_trait!(Pnt6, Pnt6Cast)
mul_redispatch_impl!(Pnt6, Pnt6MulRhs)
div_redispatch_impl!(Pnt6, Pnt6DivRhs)
add_redispatch_impl!(Pnt6, Pnt6AddRhs)
sub_redispatch_impl!(Pnt6, Pnt6SubRhs)
cast_redispatch_impl!(Pnt6, Pnt6Cast)
new_impl!(Pnt6, x, y, z, w, a, b)
orig_impl!(Pnt6, x, y, z, w, a, b)
ord_impl!(Pnt6, x, y, z, w, a, b)
vec_cast_impl!(Pnt6, Pnt6Cast, x, y, z, w, a, b)
as_slice_impl!(Pnt6, 6)
index_impl!(Pnt6)
indexable_impl!(Pnt6, 6)
at_fast_impl!(Pnt6, 6)
new_repeat_impl!(Pnt6, val, x, y, z, w, a, b)
dim_impl!(Pnt6, 6)
container_impl!(Pnt6)
pnt_as_vec_impl!(Pnt6, Vec6, x, y, z, w, a, b)
pnt_sub_impl!(Pnt6, Vec6, Pnt6SubRhs)
neg_impl!(Pnt6, x, y, z, w, a, b)
pnt_add_vec_impl!(Pnt6, Vec6, Pnt6AddRhs, x, y, z, w, a, b)
pnt_sub_vec_impl!(Pnt6, Vec6, Pnt6SubRhs, x, y, z, w, a, b)
vec_mul_scalar_impl!(Pnt6, f64, Pnt6MulRhs, x, y, z, w, a, b)
vec_mul_scalar_impl!(Pnt6, f32, Pnt6MulRhs, x, y, z, w, a, b)
vec_mul_scalar_impl!(Pnt6, u64, Pnt6MulRhs, x, y, z, w, a, b)
vec_mul_scalar_impl!(Pnt6, u32, Pnt6MulRhs, x, y, z, w, a, b)
vec_mul_scalar_impl!(Pnt6, u16, Pnt6MulRhs, x, y, z, w, a, b)
vec_mul_scalar_impl!(Pnt6, u8, Pnt6MulRhs, x, y, z, w, a, b)
vec_mul_scalar_impl!(Pnt6, i64, Pnt6MulRhs, x, y, z, w, a, b)
vec_mul_scalar_impl!(Pnt6, i32, Pnt6MulRhs, x, y, z, w, a, b)
vec_mul_scalar_impl!(Pnt6, i16, Pnt6MulRhs, x, y, z, w, a, b)
vec_mul_scalar_impl!(Pnt6, i8, Pnt6MulRhs, x, y, z, w, a, b)
vec_mul_scalar_impl!(Pnt6, uint, Pnt6MulRhs, x, y, z, w, a, b)
vec_mul_scalar_impl!(Pnt6, int, Pnt6MulRhs, x, y, z, w, a, b)
vec_div_scalar_impl!(Pnt6, f64, Pnt6DivRhs, x, y, z, w, a, b)
vec_div_scalar_impl!(Pnt6, f32, Pnt6DivRhs, x, y, z, w, a, b)
vec_div_scalar_impl!(Pnt6, u64, Pnt6DivRhs, x, y, z, w, a, b)
vec_div_scalar_impl!(Pnt6, u32, Pnt6DivRhs, x, y, z, w, a, b)
vec_div_scalar_impl!(Pnt6, u16, Pnt6DivRhs, x, y, z, w, a, b)
vec_div_scalar_impl!(Pnt6, u8, Pnt6DivRhs, x, y, z, w, a, b)
vec_div_scalar_impl!(Pnt6, i64, Pnt6DivRhs, x, y, z, w, a, b)
vec_div_scalar_impl!(Pnt6, i32, Pnt6DivRhs, x, y, z, w, a, b)
vec_div_scalar_impl!(Pnt6, i16, Pnt6DivRhs, x, y, z, w, a, b)
vec_div_scalar_impl!(Pnt6, i8, Pnt6DivRhs, x, y, z, w, a, b)
vec_div_scalar_impl!(Pnt6, uint, Pnt6DivRhs, x, y, z, w, a, b)
vec_div_scalar_impl!(Pnt6, int, Pnt6DivRhs, x, y, z, w, a, b)
vec_add_scalar_impl!(Pnt6, f64, Pnt6AddRhs, x, y, z, w, a, b)
vec_add_scalar_impl!(Pnt6, f32, Pnt6AddRhs, x, y, z, w, a, b)
vec_add_scalar_impl!(Pnt6, u64, Pnt6AddRhs, x, y, z, w, a, b)
vec_add_scalar_impl!(Pnt6, u32, Pnt6AddRhs, x, y, z, w, a, b)
vec_add_scalar_impl!(Pnt6, u16, Pnt6AddRhs, x, y, z, w, a, b)
vec_add_scalar_impl!(Pnt6, u8, Pnt6AddRhs, x, y, z, w, a, b)
vec_add_scalar_impl!(Pnt6, i64, Pnt6AddRhs, x, y, z, w, a, b)
vec_add_scalar_impl!(Pnt6, i32, Pnt6AddRhs, x, y, z, w, a, b)
vec_add_scalar_impl!(Pnt6, i16, Pnt6AddRhs, x, y, z, w, a, b)
vec_add_scalar_impl!(Pnt6, i8, Pnt6AddRhs, x, y, z, w, a, b)
vec_add_scalar_impl!(Pnt6, uint, Pnt6AddRhs, x, y, z, w, a, b)
vec_add_scalar_impl!(Pnt6, int, Pnt6AddRhs, x, y, z, w, a, b)
vec_sub_scalar_impl!(Pnt6, f64, Pnt6SubRhs, x, y, z, w, a, b)
vec_sub_scalar_impl!(Pnt6, f32, Pnt6SubRhs, x, y, z, w, a, b)
vec_sub_scalar_impl!(Pnt6, u64, Pnt6SubRhs, x, y, z, w, a, b)
vec_sub_scalar_impl!(Pnt6, u32, Pnt6SubRhs, x, y, z, w, a, b)
vec_sub_scalar_impl!(Pnt6, u16, Pnt6SubRhs, x, y, z, w, a, b)
vec_sub_scalar_impl!(Pnt6, u8, Pnt6SubRhs, x, y, z, w, a, b)
vec_sub_scalar_impl!(Pnt6, i64, Pnt6SubRhs, x, y, z, w, a, b)
vec_sub_scalar_impl!(Pnt6, i32, Pnt6SubRhs, x, y, z, w, a, b)
vec_sub_scalar_impl!(Pnt6, i16, Pnt6SubRhs, x, y, z, w, a, b)
vec_sub_scalar_impl!(Pnt6, i8, Pnt6SubRhs, x, y, z, w, a, b)
vec_sub_scalar_impl!(Pnt6, uint, Pnt6SubRhs, x, y, z, w, a, b)
vec_sub_scalar_impl!(Pnt6, int, Pnt6SubRhs, x, y, z, w, a, b)
approx_eq_impl!(Pnt6, x, y, z, w, a, b)
from_iterator_impl!(Pnt6, iterator, iterator, iterator, iterator, iterator, iterator)
bounded_impl!(Pnt6, x, y, z, w, a, b)
axpy_impl!(Pnt6, x, y, z, w, a, b)
iterable_impl!(Pnt6, 6)
iterable_mut_impl!(Pnt6, 6)
