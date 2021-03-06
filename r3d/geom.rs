use super::macros::*;
use super::ut::*;
use super::matrix::*;
use super::vecmath::*;

#[deriving(Clone,Show)]
pub struct Extents<T=Vec3<f32>> {  
	min:T,max:T	
}
pub type AABB=Extents<Vec3<f32>>;


struct Sphere<T=f32> {
	pos:Vec3<T>,
	radius:T
}

impl Pos for Sphere {
	fn pos(&self)->Vec3 { self.pos}
}

struct Ray<T=f32> {
	pos:Vec3<T>,
	dir:Vec3<T>
}


struct Plane<T=f32> {
	norm:Vec3<T>,
	dist:T
}

struct OOBB {
	mat:Matrix44,
	ext:Extents<Vec3f>,
}

struct Bounds {	// combined sphere & bounding-box.
	centre:Vec3,
	size:Vec3,
	radius:f32
}
impl Pos for Bounds {
	fn pos(&self)->Vec3 { self.centre}
}


trait ToAABB {
	fn to_aabb(&self)->AABB;
}
fn pos_radius_to_aabb(pos:&Vec3, r:f32)->Extents{
	let size=Vector::splat(r);
	Extents{min:pos-size, max:pos+size}
}

impl ToAABB for Sphere {
	fn to_aabb(&self)->AABB {
		pos_radius_to_aabb(&self.pos, self.radius)
	}
}

struct Triangle<V>{
	v0:V, v1:V, v2:V
}

impl<T:Float+Copy> Plane<T> {
	fn from_point(v0:&Vec3<T>, norm:&Vec3<T>)->Plane<T> {
		Plane{norm:*norm,dist:v0.dot(norm)}
	}
	fn from_triangle(v0:&Vec3<T>,v1:&Vec3<T>,v2:&Vec3<T>)->Plane<T> {
		let norm=(v1.sub(v0)).cross(&(v2.sub(v0))).normalize();
		Plane{norm:norm,dist:norm.dot(v0)}
	}
}


struct Contact {
	pos:Vec3,
	norm:Vec3
}

impl Pos for Contact {
	fn pos(&self)->Vec3 { self.pos}
}


impl<T:Clone> Extents<T> {
	pub fn init(v:&T)->Extents<T>{ Extents::<T>{min:v.clone(),max:v.clone()}}
}
impl Extents<Vec3<f32>>{
	pub fn new()->Extents<Vec3<f32>> {
		let f=1000000.0f32;//todo: FLT_MAX
		Extents{min:Vec3(f,f,f),max:Vec3(-f,-f,-f)}
	}
	fn from_vertices<V:Pos>(vertices:&[V])->Extents {
		let mut m=Extents::new();
		for v in vertices.iter() {
			m.include(&v.pos());
		}
		m
	}
}
impl Bounds {
	fn from_vertices<V:Pos>(vertices:&[V])->Bounds {
		let ext = Extents::from_vertices(vertices);
		let centre = ext.centre();

		let mut max_d2=zero();
		for v in vertices.iter() {
			max_d2=fmax(max_d2,v.pos().dist_squared(&centre));
		}
		Bounds{
			centre:centre,
			size:ext.max-centre,
			radius:max_d2.sqrt()
		}
	}
}

pub trait Centre<V> {
	fn centre(&self)->V;
}
impl<V:Num> Centre<V> for Extents<V> {
	fn centre(&self)->V { (self.min+self.max)*(one::<V>()/(one::<V>()+one::<V>())) }
}
impl<T:Copy> Centre<Vec3<T>> for Sphere<T> {
	fn centre(&self)->Vec3<T> { self.pos }
}

impl<T:Num+PartialOrd,V:VecCmp<T>> Extents<V> { 
	pub fn include(&mut self, v:&V) {
		self.min=self.min.min(v);
		self.max=self.max.max(v);
	}
}

pub fn triangle_norm<T:Float>((v0,v1,v2):(&Vec4<T>,&Vec4<T>,&Vec4<T>))->Vec4<T>{
	let edge01=*v1-*v0;
	let edge12=*v2-*v1;
	return edge01.cross(&edge12);
}
pub fn triangle_extents<T:PartialOrd+Num+Clone,V:VecCmp<T>+Clone+Num>((v0,v1,v2):(&V,&V,&V))->Extents<V>{
	let mut ex=Extents::<V>::init(v0);
	ex.include(v1);
	ex.include(v2);
	ex
}

pub fn cuboid_vertices(mat:&Matrix44, size:&Vec3)->[Vec4,..8] {
	let vx = mat.0.scale(size.0);
	let vy = mat.1.scale(size.1);
	let vz = mat.2.scale(size.2);

	let v0 = mat.3 .sub(&vx);
	let v1 = mat.3 .add(&vx);
	let v00 = v0 -vy;
	let v01 = v0 +vy;
	let v10 =v1 - vy;
	let v11 =v1 + vy;

	[	v00 - vz,v00 + vz,
		v01 - vz,v01 + vz,
		v10 - vz,v10 + vz,
		v11 - vz,v11 + vz]
}

pub static g_cuboid_edges:[[uint,..2],..12]=[
	[0,1],[0,2],[1,3],[2,3],
	[4,5],[4,6],[5,7],[6,7],
	[0,4],[1,5],[2,6],[3,7]];

// entity:moving bounds.

#[deriving(Clone,Copy,Show)]
pub struct Entity {
	pub matrix:Matrix44<f32>,
	pub vel:Vec3,
}
impl Pos for Entity {
	fn pos(&self)->Vec3 { self.matrix.pos().to_vec3()	}
	fn set_pos(&mut self,v:&Vec3) {self.matrix.set_pos(&v.to_vec4_pos())}
}
impl Entity {
	
}


