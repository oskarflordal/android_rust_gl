#![feature(globs)]
#![allow(unused_attribute)]
#![feature(default_type_params)]
#![feature(macro_rules)]
#![allow(dead_code)]
#![allow(unused_variable)]
#![allow(unreachable_code)]

use std::cmp;
use super::to::To;
pub use std::num;
pub use std::f32::*;
pub use std::num::*;
use std::io;

/// TODO: Split into XYZW interface+VecTypes,& Vecath dependant on XYZW interfaces
/// Generic maths classes
/// member functions prefixed with 'v' for easier life without code-completion, and to distinguish from operator overloads (official langauge level "add") etc

#[deriving(Copy,Show)]
pub struct Vec2<T=f32>(pub T,pub T);

#[deriving(Copy,Show)]
pub struct Vec3<T=f32>(pub T,pub T,pub T);


#[deriving(Copy,Show)]
pub struct Vec4<T=f32>(pub T,pub T,pub T,pub T);

pub fn fmin<T:PartialOrd>(a:T,b:T)->T { if a<b{a}else{b} }
pub fn fmax<T:PartialOrd>(a:T,b:T)->T { if a>b{a}else{b} }

// 'Position' trait for  objects with a spatial centre.
// position should be an x,y,z or x,y,z,1
pub trait Pos<V=Vec3<f32>> {
	fn pos(&self)->V;
	fn set_pos(&mut self,v:&V) {fail!();}
}




// TODO half-precision type for GL..
// TODO: Packed normal 10:10:10
// TODO: 565 colors


impl<T:Copy+Num> Vec2<T> {
	pub fn cross_to_scalar(&self,other:&Vec2<T>)->T {self.0*other.1-self.1*other.0}
	pub fn cross_z(&self,z:T)->Vec2<T> { Vec2(-self.1*z,self.0*z)}
	pub fn cross_one(&self)->Vec2<T> { Vec2(-self.1,self.0)}
	pub fn map<R>(&self, f:|T|->R)->Vec2<R>{
		let Vec2(x,y)=*self;
		Vec2(f(x),f(y))
	}
	pub fn zip<X:Copy,R:Copy>(&self,&Vec2( x1, y1):&Vec2<X>,f:|T,X|->R)->Vec2<R>{
		let Vec2( x, y)=*self;
		Vec2(f(x,x1),f(y,y1))
	}
	pub fn fold<R:Copy>(&self, src:R,f:|R,T|->R)->R{
		let Vec2(x,y)=*self;
		let f2=f(src,x);
		f(f2,y)
	}
}

	// componentwise conversion
impl<T:ToPrimitive+Copy+Zero+One> Vec4<T> {
	pub fn to_f32(&self)->Vec4<f32> {
		Vec4(self.0.to_f32().unwrap(),self.1.to_f32().unwrap(),self.2.to_f32().unwrap(),self.3.to_f32().unwrap())
	}
	pub fn to_i32(&self)->Vec4<i32> {
		Vec4(self.0.to_i32().unwrap(),self.1.to_i32().unwrap(),self.2.to_i32().unwrap(),self.3.to_i32().unwrap())
	}
}
impl<T:ToPrimitive+Copy+Zero+One> Vec3<T>{
	pub fn to_f32(&self)->Vec3<f32> {
		Vec3(self.0.to_f32().unwrap(),self.1.to_f32().unwrap(),self.2.to_f32().unwrap())
	}
	pub fn to_i32(&self)->Vec3<i32> {
		Vec3(self.0.to_i32().unwrap(),self.1.to_i32().unwrap(),self.2.to_i32().unwrap())
	}
}

impl<T:Copy+Zero+One> Vec3<T> {

	pub fn from_vec2(xy:&Vec2<T>,z:T)->Vec3<T> {Vec3::<T>(xy.0,xy.1,z)}

	pub fn map<R:Copy>(&self, f:|T|->R)->Vec3<R>{
//		let Vec3(x,y,z)=*self;
		Vec3(f(self.0),f(self.1),f(self.2))
	}
	pub fn zip<X:Copy,R:Copy>(&self,&Vec3(x1,y1,z1):&Vec3<X>,f:|T,X|->R)->Vec3<R>{
		Vec3(f(self.0,x1),f(self.1,y1),f(self.2,z1))
	}
	pub fn fold<R:Copy>(&self, src:R,f:|R,T|->R)->R{
		let f2=f(src,self.0);
		let f3=f(f2,self.1);
		f(f3,self.2)
	}
}
impl<T:Copy+Zero+One> Vec4<T> {

	pub fn from_vec3(xyz:&Vec3<T>,w:T)->Vec4<T> {Vec4(xyz.0,xyz.1,xyz.2,w)}
	pub fn from_vec2(xy:&Vec2<T>,z:T,w:T)->Vec4<T> {Vec4(xy.0,xy.1,z,w)}
	pub fn from_vec2_vec2(xy:&Vec2<T>,zw:&Vec2<T>)->Vec4<T> {Vec4(xy.0,xy.1,zw.0,zw.1)}

	pub fn map<R:Copy>(&self, f:|T|->R)->Vec4<R>{
		Vec4(f(self.0),f(self.1),f(self.2),f(self.3))
	}
	pub fn zip<X:Copy,R:Copy>(&self,other:&Vec4<X>,f:|T,X|->R)->Vec4<R>{
		Vec4(f(self.0,other.0),f(self.1,other.1),f(self.2,other.2), f(self.3,other.3))
	}
	pub fn fold<R>(&self, src:R,f:|R,T|->R)->R{
		let f2=f(src,self.0);
		let f3=f(f2,self.1);
		let f4=f(f3,self.2);
		f(f4,self.3)
	}
}

pub trait XYZW<T:Zero+One+Copy=f32> :Copy{
	fn x(&self)->T;
	fn y(&self)->T;
	fn z(&self)->T;
	fn w(&self)->T;

	fn from_xyzw(x:T,y:T,z:T,w:T)->Self;
	fn from_xyz(x:T,y:T,z:T)->Self { XYZW::<T>::from_xyzw(x,y,z,zero::<T>()) }
	fn from_xyz1(x:T,y:T,z:T)->Self { XYZW::<T>::from_xyzw(x,y,z,one::<T>()) }
	fn from_xy(x:T,y:T)->Self { XYZW::<T>::from_xyzw(x,y,zero::<T>(),zero::<T>()) }
	fn xyzw(&self)->(T,T,T,T) {(self.x(),self.y(),self.z(),self.w())}

	fn splat(f:T)->Self { XYZW::<T>::from_xyzw(f,f,f,f)}
	fn splat_x(&self)->Self { XYZW::<T>::splat(self.x())}
	fn splat_y(&self)->Self { XYZW::<T>::splat(self.y())}
	fn splat_z(&self)->Self { XYZW::<T>::splat(self.z())}
	fn splat_w(&self)->Self { XYZW::<T>::splat(self.w())}

	fn set_x(&self,f:T)->Self {XYZW::<T>::from_xyzw(f,self.y(),self.z(),self.w())}
	fn set_y(&self,f:T)->Self {XYZW::<T>::from_xyzw(self.x(),f,self.z(),self.w())}
	fn set_z(&self,f:T)->Self {XYZW::<T>::from_xyzw(self.x(),self.y(),f,self.w())}
	fn set_w(&self,f:T)->Self {XYZW::<T>::from_xyzw(self.x(),self.y(),self.z(),f)}
	fn set_w1(&self,f:T)->Self {XYZW::<T>::from_xyzw(self.x(),self.y(),self.z(),one())}
	fn set_w0(&self,f:T)->Self {XYZW::<T>::from_xyzw(self.x(),self.y(),self.z(),zero())}
	fn to_point(&self)->Self {self.set_w(one())}	// synonymous with 'w=1'
	fn to_axis(&self)->Self {self.set_w(zero())}


	fn swap_yz(&self)->Self { XYZW::<T>::from_xyzw(self.x(),self.z(),self.y(),self.w())}
	fn swap_xyz(&self)->Self { XYZW::<T>::from_xyzw(self.z(),self.y(),self.x(),self.w())}
	fn swap_xyzw(&self)->Self { XYZW::<T>::from_xyzw(self.w(),self.z(),self.y(),self.x())}

}



/// VecPermute carries some type associations - the type should know about the 2,3,4 element versions
pub trait VecPermute<T:Copy+One+Zero,V2:XYZW<T>=Vec2<T>, V3:XYZW<T>=Vec3<T>,V4:XYZW<T>=Vec4<T>> : XYZW<T> {

	fn xy(&self)->V2	{ XYZW::from_xy(self.x(),self.y())}
	fn yx(&self)->V2	{ XYZW::from_xy(self.y(),self.x())}
	fn xz(&self)->V2	{ XYZW::from_xy(self.x(),self.z())}
	fn yz(&self)->V2	{ XYZW::from_xy(self.y(),self.z())}

	fn xy01(&self)->V4	{ XYZW::from_xyzw(self.x(),self.y(),zero(),one())}

	fn xyz(&self)->V3	 { XYZW::from_xyz(self.x(),self.y(),self.z())}
	fn xyz1(&self)->V4	{ XYZW::from_xyzw(self.x(),self.y(),self.z(),one())}	// vec3 to homogeneous point
	fn xyz0(&self)->V4	{ XYZW::from_xyzw(self.x(),self.y(),self.z(),zero())}	// vec3 to homogeneous offset
	fn xyzw(&self)->V4	{XYZW::from_xyzw(self.x(),self.y(),self.z(),self.w())}

	// permutes useful for swapping rgb/bgr
	fn zyx(&self)->V3	{ XYZW::from_xyz(self.z(),self.y(),self.x())}
	fn zyx0(&self)->V4	{ XYZW::from_xyzw(self.z(),self.y(),self.x(),zero())}
	fn zyx1(&self)->V4	{ XYZW::from_xyzw(self.z(),self.y(),self.x(),one())}
	fn zyxw(&self)->V4	{ XYZW::from_xyzw(self.z(),self.y(),self.x(),self.w())}

	// permutes for swapping Y or Z being up
	fn xzy(&self)->V3	{ XYZW::from_xyz(self.x(),self.z(),self.y())}
	fn xzy0(&self)->V4	{ XYZW::from_xyzw(self.x(),self.z(),self.y(),zero())}
	fn xzy1(&self)->V4	{ XYZW::from_xyzw(self.x(),self.z(),self.y(),one())}
	fn xzyw(&self)->V4	{ XYZW::from_xyzw(self.x(),self.z(),self.y(),self.w())}

	// swap all, swap pairs
	fn wzyx(&self)->V4	{ XYZW::from_xyzw(self.w(),self.z(),self.y(),self.x())}
	fn yxwz(&self)->V4	{ XYZW::from_xyzw(self.y(),self.x(),self.w(),self.z())}

	// permutes for cross-product
	// i  j   k
	// x0 y0 z0
	// x1 y1 z1

	// x'=y0*z1 - z0*y1 ,  y'=z0*x1-x0*z1,  z'=x0*y1-y0*x1
	fn yzx(&self)->V3 {XYZW::from_xyz(self.y(),self.z(),self.x())}
	fn zxy(&self)->V3 {XYZW::from_xyz(self.z(),self.x(),self.y())}
	fn yzxw(&self)->V4 {XYZW::from_xyzw(self.y(),self.z(),self.x(),self.w())}
	fn zxyw(&self)->V4 {XYZW::from_xyzw(self.z(),self.x(),self.y(),self.w())}
	fn yzx0(&self)->V4 {XYZW::from_xyzw(self.y(),self.z(),self.x(),zero())}
	fn zxy0(&self)->V4 {XYZW::from_xyzw(self.z(),self.x(),self.y(),zero())}

	// splats in permute syntax
	fn xxxx(&self)->V4	{ XYZW::from_xyzw(self.x(),self.x(),self.x(),self.x())}
	fn yyyy(&self)->V4	{ XYZW::from_xyzw(self.y(),self.y(),self.y(),self.y())}
	fn zzzz(&self)->V4	{ XYZW::from_xyzw(self.z(),self.z(),self.z(),self.z())}
	fn wwww(&self)->V4	{ XYZW::from_xyzw(self.w(),self.w(),self.w(),self.w())}
}



//pub trait VecNum<T:Num> {
//	fn from_xyz(x:T,y:T,z:T)->Self;
//}
pub trait VecCmp<T:PartialOrd> {
	fn min(&self,b:&Self)->Self;
	fn max(&self,b:&Self)->Self;
	fn max_elem_index(&self)->uint;
}

// componentwise multiplication operator for vectors
impl<F:Mul<F,F>+Copy+Zero+One> Mul<Vec2<F>,Vec2<F>> for Vec2<F> {
	fn mul(&self,b:&Vec2<F>)->Vec2<F> {
		Vec2(self.0*b.0,self.1*b.1)
	}
}
impl<F:Div<F,F>+Copy+Zero+One> Div<Vec2<F>,Vec2<F>> for Vec2<F> {
	fn div(&self,b:&Vec2<F>)->Vec2<F> {
		Vec2(self.0/b.0,self.1/b.1)
	}
}
impl<F:Div<F,F>+Copy+Zero+One> Div<Vec3<F>,Vec3<F>> for Vec3<F> {
	fn div(&self,b:&Vec3<F>)->Vec3<F> {
		Vec3(self.0/b.0,self.1/b.1,self.2/b.2)
	}
}
impl<F:Div<F,F>+Copy+Zero+One> Div<Vec4<F>,Vec4<F>> for Vec4<F> {
	fn div(&self,b:&Vec4<F>)->Vec4<F> {
		Vec4(self.0/b.0,self.1/b.1,self.2/b.2,self.3/b.3)
	}
}


pub trait PreMulVec2<T,RESULT> {
	fn pre_mul_vec2(&self,&Vec2<T>)->RESULT;
}
impl<T:Mul<T,T>+Copy+Zero+One> PreMulVec2<T,Vec2<T>> for Vec2<T> {
	fn pre_mul_vec2(&self, lhs:&Vec2<T>)->Vec2<T> { Vec2(lhs.0*self.0,lhs.1*self.1) }
}
// TODO: At the minute, this tells us 'conflicting impl' if we do for generic T:Float
impl PreMulVec2<f32,Vec2<f32>> for f32 {
	fn pre_mul_vec2(&self, lhs:&Vec2<f32>)->Vec2<f32> { Vec2(lhs.0**self,lhs.1**self) }
}
// TODO: At the minute, this tells us 'conflicting impl' if we do for generic T:Float
impl PreMulVec2<f64,Vec2<f64>> for f64 {
	fn pre_mul_vec2(&self, lhs:&Vec2<f64>)->Vec2<f64> { Vec2(lhs.x()**self,lhs.y()**self) }
}
pub trait PreDivVec2<T,RESULT> {
	fn pre_div_vec2(&self,&Vec2<T>)->RESULT;
}
impl<T:Div<T,T>+Copy+Zero+One> PreDivVec2<T,Vec2<T>> for Vec2<T> {
	fn pre_div_vec2(&self, lhs:&Vec2<T>)->Vec2<T> { Vec2(lhs.0/self.0,lhs.1/self.1) }
}
// TODO: At the minute, this tells us 'conflicting impl' if we do for generic T:Float
impl PreDivVec2<f32,Vec2<f32>> for f32 {
	fn pre_div_vec2(&self, lhs:&Vec2<f32>)->Vec2<f32> { Vec2(lhs.0/ *self,lhs.1/ *self) }
}

impl<T:Rem<T,T>+Copy+Zero+One> Rem<Vec2<T>,Vec2<T>> for Vec2<T> {
	fn rem(&self,rhs:&Vec2<T>)->Vec2<T> {
		Vec2(self.x()%rhs.x(),self.y()%rhs.y())
	}
}
impl<T:Rem<T,T>+Copy+Zero+One> Rem<Vec3<T>,Vec3<T>> for Vec3<T> {
	fn rem(&self,rhs:&Vec3<T>)->Vec3<T> {
		Vec3(self.x()%rhs.x(),self.y()%rhs.y(),self.z()%rhs.z())
	}
}
impl<T:Rem<T,T>+Copy+Zero+One> Rem<Vec4<T>,Vec4<T>> for Vec4<T> {
	fn rem(&self,rhs:&Vec4<T>)->Vec4<T> {
		Vec4(self.0%rhs.0,self.1%rhs.1,self.2%rhs.2,self.3%rhs.3)
	}
}

impl<T:Neg<T>+Copy+Zero+One> Neg<Vec2<T>> for Vec2<T> {
	fn neg(&self)->Vec2<T> { Vec2(-self.0,-self.1) }
}
impl<T:Neg<T>+Copy+Zero+One> Neg<Vec3<T>> for Vec3<T> {
	fn neg(&self)->Vec3<T> { Vec3(-self.0,-self.1,-self.2) }
}
impl<T:Neg<T>+Copy+Zero+One> Neg<Vec4<T>> for Vec4<T> {
	fn neg(&self)->Vec4<T> { Vec4(-self.0,-self.1,-self.2,-self.3) }
}

impl<T,OUT,RHS:PreMulVec3<T,OUT>> Mul<RHS,OUT> for Vec3<T> {
	fn mul(&self,b:&RHS)->OUT {
		b.pre_mul_vec3(self)
	}
}
impl<F:Mul<F,F>+Copy,OUT, RHS:PreMulVec4<F,OUT>> Mul<RHS,OUT> for Vec4<F> {
	fn mul(&self,b:&RHS)->OUT {
		b.pre_mul_vec4(self)
	}
}

pub trait Sum<T> {
	fn sum(&self)->T;
}

pub trait PreMulVec3<T,RESULT> {
	fn pre_mul_vec3(&self,&Vec3<T>)->RESULT;
}
impl<T:Float+Copy> PreMulVec3<T,Vec3<T>> for Vec3<T> {
	fn pre_mul_vec3(&self, lhs:&Vec3<T>)->Vec3<T> { Vec3(lhs.0*self.0,lhs.1*self.1,lhs.2*self.2) }
}
// TODO: At the minute, this tells us 'conflicting impl' if we do for generic T:Float
impl PreMulVec3<f64,Vec3<f64>> for f64 {
	fn pre_mul_vec3(&self/*=rhs*/, lhs:&Vec3<f64>)->Vec3<f64> { Vec3(lhs.0**self,lhs.1**self,lhs.2**self) }
}
impl PreMulVec3<f32,Vec3<f32>> for f32 {
	fn pre_mul_vec3(&self, lhs:&Vec3<f32>)->Vec3<f32> { Vec3(lhs.0**self,lhs.1**self,lhs.2**self) }
}

pub trait PreDivVec3<T,RESULT> {
	fn pre_div_vec3(&self,&Vec3<T>)->RESULT;
}
impl<T:Float+Copy> PreDivVec3<T,Vec3<T>> for Vec3<T> {
	fn pre_div_vec3(&self, lhs:&Vec3<T>)->Vec3<T> { Vec3(lhs.0/self.0,lhs.1/self.1,lhs.2/self.2) }
}
// TODO: At the minute, this tells us 'conflicting impl' if we do for generic T:Float
impl PreDivVec3<f32,Vec3<f32>> for f32 {
	fn pre_div_vec3(&self, lhs:&Vec3<f32>)->Vec3<f32> { Vec3(lhs.0/ *self,lhs.1/ *self,lhs.2/ *self) }
}



pub trait PreMulVec4<T,RESULT> {
	fn pre_mul_vec4(&self,lhs:&Vec4<T>)->RESULT;
}
impl<T:Float+Copy> PreMulVec4<T,Vec4<T>> for Vec4<T> {
	fn pre_mul_vec4(&self, lhs:&Vec4<T>)->Vec4<T> { Vec4(lhs.0*self.0,lhs.1*self.1,lhs.2*self.2,lhs.3*self.3) }
}
// TODO: At the minute, this tells us 'conflicting impl' if we do for generic T:Float
impl PreMulVec4<f32,Vec4<f32>> for f32 {
	fn pre_mul_vec4(&self, lhs:&Vec4<f32>)->Vec4<f32> { Vec4(lhs.0**self,lhs.1**self,lhs.2**self,lhs.3**self) }
}
// TODO: At the minute, this tells us 'conflicting impl' if we do for generic T:Float
impl PreMulVec4<f64,Vec4<f64>> for f64 {
	fn pre_mul_vec4(&self, lhs:&Vec4<f64>)->Vec4<f64> { Vec4(lhs.0**self,lhs.1**self,lhs.2**self,lhs.3**self) }
}

pub trait PreDivVec4<T,RESULT> {
	fn pre_div_vec4(&self,lhs:&Vec4<T>)->RESULT;
}
impl<T:Div<T,T>+Copy+Zero+One> PreDivVec4<T,Vec4<T>> for Vec4<T> {
	fn pre_div_vec4(&self, lhs:&Vec4<T>)->Vec4<T> { Vec4(lhs.0/self.0,lhs.1/self.1,lhs.2/self.2,lhs.3/self.3) }
}
// TODO: At the minute, this tells us 'conflicting impl' if we do for generic T:Float
impl PreDivVec4<f32,Vec4<f32>> for f32 {
	fn pre_div_vec4(&self, lhs:&Vec4<f32>)->Vec4<f32> { Vec4(lhs.0/ *self,lhs.1/ *self,lhs.2/ *self,lhs.3/ *self) }
}

impl<T:PartialEq+Copy+Zero+One> PartialEq for Vec2<T> {
	fn eq(&self,rhs:&Vec2<T>)->bool { return self.0==rhs.0 && self.1==rhs.1 }
}
impl<T:PartialEq+Copy+Zero+One> PartialEq for Vec3<T> {
	fn eq(&self,rhs:&Vec3<T>)->bool { return self.0==rhs.0 && self.1==rhs.1 && self.2==rhs.2 }
}
impl<T:PartialEq+Copy+Zero+One> PartialEq for Vec4<T> {
	fn eq(&self,rhs:&Vec4<T>)->bool { return self.0==rhs.0 && self.1==rhs.1 && self.2==rhs.2 && self.3==rhs.3 }
}


// todo: satisfy if Num+Clone only
impl<T:Num+Copy+Float> Num for Vec2<T> {}
impl<T:Num+Copy+Float> Num for Vec3<T> {}
impl<T:Num+Copy+Float> Num for Vec4<T> {}

// vector maths gathers primitive operations and implements more in terms of them
// This is not 
pub trait VecMath<T:Float=f32>:XYZW<T>+Num+VecCmp<T>+Sum<T> {
	fn origin()->Self	{ XYZW::<T>::from_xyzw(zero(),zero(),zero(),one()) }
	fn axis(i:int)->Self{
		match i{ 0=>XYZW::<T>::from_xyzw(one(),zero(),zero(),zero()),
                1=>XYZW::<T>::from_xyzw(zero(),one(),zero(),zero()),
                2=>XYZW::<T>::from_xyzw(zero(),zero(),one(),zero()),
                _=>XYZW::<T>::from_xyzw(zero(),zero(),zero(),one())}
	}
//	fn from_xyzw(x:T,y:T,z:T,w:T)->Self {
//		XYZW::<T>::from_xyzw(x,y,z,w)
//	}
	fn longest_axis(&self)->uint{ self.mul(self).max_elem_index()}

	// abstractions may help keeping in SIMD regs with direct impl, generic provided here
	// optimized versions might multiply by splatted x,y,z,w instead of scalar.
	fn mul_x(&self, b:&Self)->Self { self.scale(b.x()) }
	fn mul_y(&self, b:&Self)->Self { self.scale(b.y()) }
	fn mul_z(&self, b:&Self)->Self { self.scale(b.z()) }
	fn mul_w(&self, b:&Self)->Self { self.scale(b.w()) }
	fn add_mul_x(&self, b:&Self,c:&Self)->Self { self.macc(b,c.x()) }
	fn add_mul_y(&self, b:&Self,c:&Self)->Self { self.macc(b,c.y()) }
	fn add_mul_z(&self, b:&Self,c:&Self)->Self { self.macc(b,c.z()) }
	fn add_mul_w(&self, b:&Self,c:&Self)->Self { self.macc(b,c.w()) }
	// transform self by axes
	fn mul_xyzw_sum(&self, ax:&Self,ay:&Self,az:&Self,aw:&Self)->Self {
		ax.mul_x(self).add_mul_y(ay,self).add_mul_z(az,self).add_mul_w(aw,self)
	}

	// todo: 'cross' could use permutes. However, we dont want this trait to depend on 'VecPermute'
	// because that needs the v2,v3,v4 versions
	// do we have a special set of permutes for exp

	fn cross(&self,b:&Self)->Self	{XYZW::<T>::from_xyz(self.y()*b.z()-self.z()*b.y(),self.z()*b.x()-self.x()*b.z(),self.x()*b.y()-self.y()*b.x())}

	fn scale(&self,f:T)->Self	{ XYZW::<T>::from_xyzw(self.x()*f,self.y()*f,self.z()*f,self.w()*f) }
	fn dot(&self,b:&Self)->T	{self.mul(b).sum()}
	fn para(&self,vaxis:&Self)->Self {  	let dotp=self.dot(vaxis); vaxis.scale(dotp) }

	fn neg(&self)->Self {self.scale(-one::<T>())}
	fn avr(&self,b:&Self)->Self {self.add(b).scale(one::<T>()/(one::<T>()+one::<T>()))}
	fn macc(&self,b:&Self,f:T)->Self	{self.add(&b.scale(f))} //'Multiply-Accumulate' we prefer base+ofs*scale to a*b+c
	fn add_scale(&self,b:&Self,f:T)->Self{self.macc(b,f)} // synonymous. 
	fn add_mul(&self,b:&Self,c:&Self)->Self{self.add(&b.mul(c))} // 
	fn lerp(&self,b:&Self,f:T)->Self	{self.macc(&b.sub(self),f)}
	fn sqr(&self)->T { self.dot(self)} //todo:ambiguous, maybe a*a which is componentwise.
	fn length(&self)->T { self.sqr().sqrt()}
	fn length_squared(&self)->T { self.dot(self)}
	fn inv_length(&self)->T { one::<T>()/self.sqr().sqrt()}
	fn scale_to_length(&self,length:T)->Self { self.scale(length/self.sqr().sqrt()) }
	fn normalize(&self)->Self { self.scale(one::<T>()/self.sqr().sqrt()) }
	fn perp(&self,axis:&Self)->Self { let vpara =self.para(axis); self.sub(&vpara)}
	fn cross_norm(&self, b:&Self)->Self { self.cross(b).normalize() }
	fn sub_norm(&self,b:&Self)->Self { self.sub(b).normalize() }
	//pub fn axisScale(i:int,f:VScalar)->Self;
	// { VecOps::axis(i).vscale(f)} how?
	fn reflect(&self,a:&Self)->Self { self.macc(a, self.dot(a)*(one::<T>()+one::<T>())) }

	fn para_perp(&self,vaxis:&Self)->(Self,Self) {
		let vpara=self.para(vaxis);
		(vpara,self.sub(&vpara))
	}
	fn dist(&self,b:&Self)->T {self.sub(b).length()}
	fn dist_squared(&self,b:&Self)->T {self.sub(b).sqr()}
}
impl<T:Float,V2:XYZW<T>,V3:XYZW<T>,V4:XYZW<T>, V:Copy+VecCmp<T>+Num+VecPermute<T,V2,V3,V4>+Mul<V,V>+Sum<T>> VecMath<T> for V {} 


//todo: HALF
fn bilerp<F:Float,V:VecMath<F>>(((v00,v01),(v10,v11)):((V,V),(V,V)),(s,t):(F,F))->V{
	(v00.lerp(&v01,s)).lerp(&v10.lerp(&v10,s), t)
}


// free function interface to vec maths
pub fn vadd<T:Float,V:VecMath<T>>(a:&V,b:&V)->V { a.add(b)}
pub fn vsub<T:Float,V:VecMath<T>>(a:&V,b:&V)->V { a.sub(b)}
pub fn vmacc<T:Float,V:VecMath<T>>(a:&V,b:&V,f:T)->V { a.macc(b,f)}
pub fn vmul<T:Float,V:VecMath<T>>(a:&V,b:&V)->V { a.mul(b)}
pub fn vsqr<T:Float,V:VecMath<T>>(a:&V)->T { a.sqr()}
pub fn vlerp<T:Float,V:VecMath<T>>( a:&V,b:&V,f:T)->V { vmacc(a, &vsub(b,a), f) }
pub fn vdot<T:Float,V:VecMath<T>>( a:&V,b:&V)->T { a.dot(b)}
pub fn vlength<T:Float,V:VecMath<T>>( a:&V)->T { a.mul(a).sum().sqrt()}
pub fn vnormalize<T:Float,V:VecMath<T>>( a:&V)->V { a.normalize() }
pub fn vsub_norm<T:Float,V:VecMath<T>>(a:&V,b:&V)->V { a.sub(b).normalize() }
pub fn vcross<T:Float,V:VecMath<T>>(a:&V,b:&V)->V { a.cross(b)}
pub fn vcross_norm<T:Float,V:VecMath<T>>(a:&V,b:&V)->V { a.cross(b).normalize()}
pub fn vpara_perp<T:Float,V:VecMath<T>>(a:&V,b:&V)->(V,V) { a.para_perp(b)}



//  wtf this does,t work now
impl<T:Add<T,T>+Copy+Zero+One> Add<Vec2<T>,Vec2<T>> for Vec2<T> {
	fn add(&self,rhs:&Vec2<T>)->Vec2<T> { 
		Vec2(self.0+rhs.0, self.1+rhs.1)
	}
}
impl<T:Add<T,T>+Copy+Zero+One> Add<Vec3<T>,Vec3<T>> for Vec3<T> {
	fn add(&self,rhs:&Vec3<T>)->Vec3<T> { 
		Vec3(self.0+rhs.0   , self.1+rhs.1, self.2+rhs.2)
	}
}
impl<T:Add<T,T>+Copy+Zero+One> Add<Vec4<T>,Vec4<T>> for Vec4<T> {
	fn add(&self,rhs:&Vec4<T>)->Vec4<T> { 
		Vec4(self.0+rhs.0   , self.1+rhs.1, self.2+rhs.2, self.3+rhs.3)
	}
}

//  wtf this does,t work now
impl<T:Sub<T,T>+Copy+Zero+One> Sub<Vec2<T>,Vec2<T>> for Vec2<T> {
	fn sub(&self,rhs:&Vec2<T>)->Vec2<T> { 
		Vec2(self.0-rhs.0, self.1-rhs.1)
	}
}
impl<T:Sub<T,T>+Copy+Zero+One> Sub<Vec3<T>,Vec3<T>> for Vec3<T> {
	fn sub(&self,rhs:&Vec3<T>)->Vec3<T> { 
		Vec3(self.0-rhs.0   , self.1-rhs.1, self.2-rhs.2)
	}
}
impl<T:Sub<T,T>+Copy+Zero+One> Sub<Vec4<T>,Vec4<T>> for Vec4<T> {
	fn sub(&self,rhs:&Vec4<T>)->Vec4<T> { 
		Vec4(self.0-rhs.0   , self.1-rhs.1, self.2-rhs.2, self.3-rhs.3)
	}
}


impl<T:Zero+Copy+One> Zero for Vec2<T> {
	fn zero()->Vec2<T> {Vec2(zero::<T>(),zero::<T>())}
	fn is_zero(&self)->bool { let Vec2(ref x,ref y)=*self; x.is_zero() && y.is_zero()}
}
fn vec_axis_scale<T:Float,V:VecMath<T>>(i:int,f:T)->V { let ret:V; ret=VecMath::axis(i); ret.scale(f) }

impl<T:PartialOrd+Copy+Zero+One> VecCmp<T> for Vec2<T> {
	fn min(&self,b:&Vec2<T>)->Vec2<T>	{Vec2(fmin(self.x(),b.x()),fmin(self.y(),b.y()))}
	fn max(&self,b:&Vec2<T>)->Vec2<T>	{Vec2(fmax(self.x(),b.x()),fmax(self.y(),b.y()))}
	fn max_elem_index(&self)->uint { if self.x()>self.y() {0}else{1}}
	
}

impl<T:Add<T,T>+Zero+Copy+One> Sum<T> for Vec2<T> {
	fn sum(&self)->T	{self.x()+self.y()}
}


impl<T> Vec2<T> {
	pub fn ref0<'a>(&'a self)->&'a T { let Vec2(ref x,ref y)=*self; x}
	pub fn ref1<'a>(&'a self)->&'a T { let Vec2(ref x,ref y)=*self; y}
}

impl<T:Copy+Zero+One> XYZW<T> for Vec2<T> {
	fn x(&self)->T	{ let Vec2(x,y)=*self;x}
	fn y(&self)->T	{ let Vec2(x,y)=*self;y}
	fn z(&self)->T	{ zero::<T>()}
	fn w(&self)->T	{ zero::<T>()}
	fn from_xyzw(x:T,y:T,_:T,_:T)->Vec2<T> { Vec2(x,y) }
}

impl<T:Copy+Zero+One> Zero for Vec3<T> {
	fn zero()->Vec3<T>{Vec3(zero(),zero(),zero())}
	fn is_zero(&self)->bool  { self.x().is_zero() && self.y().is_zero() && self.z().is_zero()}
}

impl<T:Copy+One+Zero+Float> One for Vec2<T> {
	fn one()->Vec2<T>{Vec2(one(),one())}
}
impl<T:Copy+One+Zero+Float> One for Vec3<T> {
	fn one()->Vec3<T>{Vec3(one(),one(),one())}
}
impl<T:Copy+One+Zero+Float> One for Vec4<T> {
	fn one()->Vec4<T>{Vec4(one(),one(),one(),one())}
}


impl<T:Copy+One+Zero> VecPermute<T,Vec2<T>,Vec3<T>,Vec4<T>> for Vec2<T> {}
impl<T:Copy+One+Zero> VecPermute<T,Vec2<T>,Vec3<T>,Vec4<T>> for Vec3<T> {}
impl<T:Copy+One+Zero> VecPermute<T,Vec2<T>,Vec3<T>,Vec4<T>> for Vec4<T> {}

impl<T:Copy+One+Zero> VecPermute<T,(T,T),(T,T,T),(T,T,T,T)> for (T,T) {}
impl<T:Copy+One+Zero> VecPermute<T,(T,T),(T,T,T),(T,T,T,T)> for (T,T,T) {}
impl<T:Copy+One+Zero> VecPermute<T,(T,T),(T,T,T),(T,T,T,T)> for (T,T,T,T) {}

impl<T:Copy+One+Zero> VecPermute<T,[T,..2],[T,..3],[T,..4]> for [T,..2] {}
impl<T:Copy+One+Zero> VecPermute<T,[T,..2],[T,..3],[T,..4]> for [T,..3] {}
impl<T:Copy+One+Zero> VecPermute<T,[T,..2],[T,..3],[T,..4]> for [T,..4] {}


impl<T:Copy+PartialOrd+Zero+One> VecCmp<T> for Vec3<T> {
	fn min(&self,b:&Vec3<T>)->Vec3<T>	{
		let x=fmin(self.0, b.0);
		let y=fmin(self.1, b.1);
		let z=fmin(self.2, b.2);
		Vec3(x,y,z)}
	fn max(&self,b:&Vec3<T>)->Vec3<T>	{Vec3(
									fmax(self.0,b.0),
									fmax(self.1,b.1),
									fmax(self.2,b.2))}
	fn max_elem_index(&self)->uint { if self.0>self.1 {if self.0>self.2{0}else{2}}
									else{if self.1>self.2{1}else{2}}}
}

impl<T:Add<T,T>+Copy+Zero+One> Sum<T> for Vec3<T> {
	fn sum(&self)->T	{self.0+self.1+self.2}
}
impl<T:Add<T,T>> Sum<T> for [T,..2] {
	fn sum(&self)->T	{self[0]+self[1]}
}
impl<T:Add<T,T>> Sum<T> for [T,..3] {
	fn sum(&self)->T	{self[0]+self[1]+self[2]}
}
impl<T:Add<T,T>> Sum<T> for [T,..4] {
	fn sum(&self)->T	{self[0]+self[1]+self[2]+self[3]}
}

impl<T>  Vec3<T> {
	pub fn ref0<'a>(&'a self)->&'a T { &self.0}
	pub fn ref1<'a>(&'a self)->&'a T { &self.1}
	pub fn ref2<'a>(&'a self)->&'a T { &self.2}
}
impl<T:Copy+Zero+One> XYZW<T> for Vec3<T> {
	fn x(&self)->T	{ self.0}
	fn y(&self)->T	{ self.1}
	fn z(&self)->T	{ self.2}
	fn w(&self)->T	{ zero()}
	fn from_xyzw(x:T,y:T,z:T,_:T)->Vec3<T> { Vec3(x,y,z) }
}

impl<T:Copy+Zero+One> Zero for Vec4<T> {
	fn zero()->Vec4<T>{ XYZW::splat(zero::<T>())}
	fn is_zero(&self)->bool  {self.0.is_zero() && self.1.is_zero() && self.2.is_zero() && self.3.is_zero()}
}

// Converting Vec2,Vec3,Vec4 to/from tuples & arrays

impl<T:Copy+Zero+One> XYZW<T> for [T,..2] {
	fn x(&self)->T { self[0] }
	fn y(&self)->T { self[1] }
	fn z(&self)->T { zero() }
	fn w(&self)->T { zero() }
	fn from_xyzw(x:T,y:T,z:T,w:T)->[T,..2] { [x,y] }

}
impl<T:Copy+Zero+One> XYZW<T> for [T,..3] {
	fn x(&self)->T { self[0] }
	fn y(&self)->T { self[1] }
	fn z(&self)->T { self[2] }
	fn w(&self)->T { zero() }
	fn from_xyzw(x:T,y:T,z:T,w:T)->[T,..3]{ [x,y,z] }
}
impl<T:Copy+Zero+One> XYZW<T> for [T,..4] {
	fn x(&self)->T { self[0] }
	fn y(&self)->T { self[1] }
	fn z(&self)->T { self[2] }
	fn w(&self)->T { self[3] }
	fn from_xyzw(x:T,y:T,z:T,w:T)->[T,..4]{ [x,y,z,w] }
}

impl<T:Copy+Zero+One> XYZW<T> for (T,T) {
	fn x(&self)->T { let(v,_,)=*self; v }
	fn y(&self)->T { let(_,v,)=*self;v }
	fn z(&self)->T { zero() }
	fn w(&self)->T { zero() }
	fn from_xyzw(x:T,y:T,z:T,w:T)->(T,T) { (x,y) }
}
impl<T:Copy+Zero+One> XYZW<T> for (T,T,T) {
	fn x(&self)->T { let(v,_,_)=*self; v }
	fn y(&self)->T { let(_,v,_)=*self;v }
	fn z(&self)->T { let(_,_,v)=*self;v }
	fn w(&self)->T { zero() }
	fn from_xyzw(x:T,y:T,z:T,w:T)->(T,T,T) { (x,y,z) }

}
impl<T:Copy+Zero+One> XYZW<T> for (T,T,T,T) {
	fn x(&self)->T { let(v,_,_,_)=*self; v }
	fn y(&self)->T { let(_,v,_,_)=*self;v }
	fn z(&self)->T { let(_,_,v,_)=*self;v }
	fn w(&self)->T { let(_,_,_,v)=*self;v }
	fn from_xyzw(x:T,y:T,z:T,w:T)->(T,T,T,T) { (x,y,z,w) }
}

impl<T:Copy+Zero+One> Vec4<T> {
	pub fn to_array(&self)->[T,..4] { [self.0,self.1,self.2,self.3] }
	pub fn to_tuple(&self)->(T,T,T,T) { (self.0,self.1,self.2,self.3) }

	pub fn from_array([x,y,z,w]:[T,..4])->Vec4<T> {Vec4(x,y,z,w)}
	pub fn from_tuple((x,y,z,w):(T,T,T,T))->Vec4<T> {
		Vec4(x,y,z,w)
	}
}
impl<T:Copy+Zero+One> Vec3<T> {
	pub fn to_array(&self)->[T,..3] { [self.0,self.1,self.2] }
	fn to_tuple(&self)->(T,T,T) { (self.0,self.1,self.2) }

	pub fn from_array([x,y,z]:[T,..3])->Vec3<T> {Vec3(x,y,z)}
	fn from_tuple((x,y,z):(T,T,T))->Vec3<T> { Vec3(x,y,z) }
}
impl<T:Copy+Zero+One> Vec2<T> {
	pub fn to_array(&self)->[T,..2] { [self.0,self.1] }
	fn to_tuple(&self)->(T,T) { (self.0,self.1) }
	pub fn from_array([x,y]:[T,..2])->Vec2<T> {Vec2(x,y)}
	fn from_tuple((x,y):(T,T))->Vec2<T> { Vec2(x,y) }
}


impl<T:Copy+PartialOrd+Zero+One> VecCmp<T> for Vec4<T> {
	fn min(&self,b:&Vec4<T>)->Vec4<T>	{Vec4(
									fmin(self.0,b.0),
									fmin(self.1,b.1),
									fmin(self.2,b.2),
									fmin(self.3,b.3))}
	fn max(&self,b:&Vec4<T>)->Vec4<T>	{Vec4(
									fmax(self.0,b.0),
									fmax(self.1,b.1),
									fmax(self.2,b.2),
									fmax(self.3,b.3))}
	fn max_elem_index(&self)->uint { 
		let (f0,max_xy)=if self.0>self.1 {(self.0,0)}else{(self.1,1)};
		let (f1,max_zw)=if self.2>self.3 {(self.2,2)}else{(self.3,3)};
		if f0>f1 {max_xy} else{max_zw}
	}
}

//impl<T:Clone+Num> VecNum<T> for Vec4<T> {
//	fn from_xyz(x:T,y:T,z:T)->Vec4<T>{Vec4(x,y,z,zero::<T>())}
//}



impl<T:Num+Copy> Sum<T> for Vec4<T> {
	fn sum(&self)->T	{self.x()+self.y()+self.z()+self.w()}
}

impl<T> Vec4<T> {
	pub fn ref0<'a>(&'a self)->&'a T { let Vec4(ref x,ref y,ref z,ref w)=*self; x}
	pub fn ref1<'a>(&'a self)->&'a T { let Vec4(ref x,ref y,ref z,ref w)=*self; y}
	pub fn ref2<'a>(&'a self)->&'a T { let Vec4(ref x,ref y,ref z,ref w)=*self; z}
	pub fn ref3<'a>(&'a self)->&'a T { let Vec4(ref x,ref y,ref z,ref  w)=*self; w}
}
impl<T:Copy+Zero+One> XYZW<T> for Vec4<T>
{
	fn x(&self)->T {let Vec4(x,y,z,w)=*self;x}
	fn y(&self)->T {let Vec4(x,y,z,w)=*self;y}
	fn z(&self)->T {let Vec4(x,y,z,w)=*self;z}
	fn w(&self)->T {let Vec4(x,y,z,w)=*self;w}
	fn from_xyzw(x:T,y:T,z:T,w:T)->Vec4<T> { Vec4(x,y,z,w) }
}

// todo - math UT, ask if they can go in the stdlib.

fn clamp<T:PartialOrd>(x:T, lo:T, hi:T)->T {
	fmax(fmin(x,hi),lo)
}
fn clamp_s<T:PartialOrd+Num>(value:T, limit:T)->T {
	clamp(value,-limit,limit)
}
fn deadzone<T:PartialOrd+Zero>(value:T, deadzone:T)->T {
	if value<deadzone || value>deadzone { value }
	else {zero()}
}

pub trait ToVec2<T> {
	fn to_vec2(&self)->Vec2<T>;
}
pub trait ToVec3<T> {
	fn to_vec3(&self)->Vec3<T>;
}
pub trait ToVec4<T> {
	fn to_vec4(&self)->Vec4<T>;
	fn to_vec4_pos(&self)->Vec4<T>;
}

impl<T:Copy+Zero+One,V:Vec3To<T>> To<V> for Vec3<T>{
	fn to(&self)->V { Vec3To::vec3_to(self)}
}
impl<T:Copy+Zero+One,V:Vec4To<T>> To<V> for Vec4<T>{
	fn to(&self)->V { Vec4To::vec4_to(self)}
}
trait Vec3To<T> {
	fn vec3_to(s:&Vec3<T>)->Self;
}
trait Vec4To<T> {
	fn vec4_to(s:&Vec4<T>)->Self;
}
impl<T:Copy+Zero+One> Vec3To<T> for Vec3<T>{
	fn vec3_to(s:&Vec3<T>)->Vec3<T> { *s}
}
impl<T:Copy+Zero+One> Vec4To<T> for Vec4<T>{
	fn vec4_to(s:&Vec4<T>)->Vec4<T> { *s}
}
// Componentwise conversion for vector
/*
impl<A:To<B>+Clone+Zero+One, B:Clone+Zero+One> To<Vec3<B>> for Vec3<A> {
	fn to(&self)->Vec3<B> { Vec3( self.x().to(),	self.y().to(),  self.z().to() )}
}
impl<A:To<B>+Clone+Zero+One, B:Clone+Zero+One> To<Vec4<B>> for Vec4<A> {
	fn to(&self)->Vec4<B> { Vec4( self.x().to(),	self.y().to(),  self.z().to(), self.w().to() )}
}
*/


impl<T:Copy+Zero+One> Vec3To<T> for Vec4<T>{
	fn vec3_to(s:&Vec3<T>)->Vec4<T> { Vec4(s.0,s.1,s.2,zero())}
}
impl<T:Copy+Zero+One> Vec4To<T> for Vec3<T>{
	fn vec4_to(s:&Vec4<T>)->Vec3<T> { Vec3(s.0,s.1,s.2)}
}


impl<T:Copy+Zero> ToVec2<T> for (T,T){
	fn to_vec2(&self)->Vec2<T>{Vec2(self.0,self.1)}
}
impl<T:Copy+Zero> ToVec3<T> for (T,T,T){
	fn to_vec3(&self)->Vec3<T>{Vec3(self.0,self.1,self.2)}
}
impl<T:Copy+Zero+One> ToVec4<T> for (T,T,T,T){
	fn to_vec4(&self)->Vec4<T>{Vec4(self.0,self.1,self.2,self.3)}
	fn to_vec4_pos(&self)->Vec4<T>{Vec4(self.0,self.1,self.2,one())}
}

impl<T:Copy+Zero> ToVec2<T> for [T,..2]{
	fn to_vec2(&self)->Vec2<T>{Vec2(self[0],self[1])}
}
impl<T:Copy+Zero> ToVec3<T> for [T,..3]{
	fn to_vec3(&self)->Vec3<T>{Vec3(self[0],self[1],self[2])}
 }
impl<T:Copy+Zero+One> ToVec4<T> for [T,..4]{
	fn to_vec4(&self)->Vec4<T>{Vec4(self[0],self[1],self[2],self[3])}
	fn to_vec4_pos(&self)->Vec4<T>{Vec4(self[0],self[1],self[2],one())}
}
impl<T:Copy+Zero+One> ToVec3<T> for Vec4<T> {
	fn to_vec3(&self)->Vec3<T>{ Vec3(self.0,self.1,self.2) }
}
impl<T:Copy+Zero+One> ToVec3<T> for Vec3<T> {
	fn to_vec3(&self)->Vec3<T>{ Vec3(self.0,self.1,self.2) }
}
impl<T:Copy+Zero+One> ToVec3<T> for Vec2<T> {
	fn to_vec3(&self)->Vec3<T>{ Vec3(self.0,self.1,zero()) }
}
impl<T:Copy+Zero+One> ToVec4<T> for Vec4<T> {
	fn to_vec4(&self)->Vec4<T>{ Vec4(self.0,self.1,self.2,self.3) }
	fn to_vec4_pos(&self)->Vec4<T>{ Vec4(self.0,self.1,self.2,one()) }
}
impl<T:Copy+Zero+One> ToVec4<T> for Vec3<T> {
	fn to_vec4(&self)->Vec4<T>{ Vec4(self.0,self.1,self.2,zero()) }
	fn to_vec4_pos(&self)->Vec4<T>{ Vec4(self.0,self.1,self.2,one()) }
}


impl<T:Copy+Zero+One> ToVec4<T> for (Vec3<T>,T){
	fn to_vec4(&self)->Vec4<T>{let (Vec3(x,y,z), w)=*self;Vec4(x,y,z,w)}
	fn to_vec4_pos(&self)->Vec4<T>{let (Vec3(x,y,z), w)=*self;Vec4(x,y,z,one())}
}
impl<T:Copy+Zero+One> ToVec4<T> for ((T,T,T),T){
	fn to_vec4(&self)->Vec4<T>{let ((x,y,z),w)=*self;Vec4(x,y,z,w)}
	fn to_vec4_pos(&self)->Vec4<T>{let ((x,y,z),w)=*self;Vec4(x,y,z,one()) }
}
impl<T:Copy+Zero+One> ToVec4<T> for ([T,..3],T){
	fn to_vec4(&self)->Vec4<T>{let ([x,y,z],w)=*self;Vec4(x,y,z,w)}
	fn to_vec4_pos(&self)->Vec4<T>{let ([x,y,z],w)=*self;Vec4(x,y,z,one())}
}



// app_render
#[cfg(run)]
fn main() {
	io::println("Vec Math Test");
	dump!(Vec3(1.0f32,2.0f32,3.0f32)*2.0f32);
	dump!(Vec3(1.0f32,2.0f32,3.0f32)*Vec3(3.0f32,2.0f32,1.0f32));
	dump!(1i,2i,3i,4i);
	let x:Vec4<i32>= Vec4(0u32,1u32,2u32,3u32).to(); 
	dump!(x);
}






