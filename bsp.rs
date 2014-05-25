#[feature(globs)];
#[feature(macro_rules)];
#[feature(default_type_params)];
#[allow(dead_code)];

extern crate collections;
use r3d::gl::*;
use r3d::vecmath::*;
use std::vec_ng::Vec;
use std::io;
use std::intrinsics::{size_of,offset};
use std::libc::*;
use std::c_str::CString;
use std::f32;
use collections::hashmap::HashSet;
use r3d::ut::*;
//use std::c_str::*;
mod r3d;

//mod macros;
// graphical test main, immiediate debug lines

fn draw_line(&(x0,y0,z0):&(f32,f32,f32),&(x1,y1,z1):&(f32,f32,f32), color:u32) {
	unsafe {
		glBegin(GL_LINES);
		gl_color(color);
		glVertex3f(x0,y0,z0); glVertex3f(x1,y1,z1);
		glEnd();
	}
}
fn v3isometric(&(x,y,z):&(f32,f32,f32))->(f32,f32,f32) {(x+y,z+(x-y)*0.5, z)}

fn draw_line_iso(v0:&BspVec3,v1:&BspVec3,color:u32, scale:f32) {
	draw_line(&v3isometric(&v3scale(v0,scale)),&v3isometric(&v3scale(v1,scale)), color)
}
unsafe fn gl_vertex_v3(&(x,y,z):&BspVec3) {
	glVertex3f(x,y,z);
}
unsafe fn gl_tex0(&(u,v):&(f32,f32)) {
	glTexCoord2f(u,v);
}
unsafe fn gl_color(color:u32) {
	let r=(color)&255;
	let g=(color>>8)&255;
	let b=(color>>16)&255;
	glColor3f(r as f32 *(1.0/255.0),g as f32 *(1.0/255.0),b as f32 *(1.0/255.0));
}
fn draw_tri_iso(v0:&BspVec3,v1:&BspVec3,v2:&BspVec3,color:u32, scale:f32 ) {
	let tv0=v3isometric(&v3scale(v0,scale));
	let tv1=v3isometric(&v3scale(v1,scale));
	let tv2=v3isometric(&v3scale(v2,scale));
	unsafe {
		glBegin(GL_TRIANGLES);
		gl_color(color);
		gl_vertex_v3(&tv0);	
		gl_vertex_v3(&tv1);	
		gl_vertex_v3(&tv2);	
		glEnd();
	}
}
fn draw_tri_iso_tex(
		(v0,uv0):(&BspVec3,BspVec2), 
		(v1,uv1):(&BspVec3,BspVec2),
		(v2,uv2):(&BspVec3,BspVec2),
		color:u32, scale:f32 ) {
	let tv0=v3isometric(&v3scale(v0,scale));
	let tv1=v3isometric(&v3scale(v1,scale));
	let tv2=v3isometric(&v3scale(v2,scale));
	unsafe {
		glBegin(GL_TRIANGLES);
		gl_color(color);
		gl_tex0(&uv0);
		gl_vertex_v3(&tv0);
		gl_tex0(&uv1);
		gl_vertex_v3(&tv1);	
		gl_tex0(&uv2);
		gl_vertex_v3(&tv2);	
		glEnd();
	}
}


pub fn main()
{
	unsafe {
		let mut argc:c_int=0;
		let argv:Vec<*c_char> =Vec::new();
		glutInit((&mut argc) as *mut c_int,0 as **c_char );
		//::macros::test();

		glutInitDisplayMode(GLUT_RGBA);
		glutInitWindowSize(1024,1024);
		let win=glutCreateWindow("testbed".to_c_str().unwrap());

		glClear(GL_COLOR_BUFFER_BIT|GL_DEPTH_BUFFER_BIT);
		let bsp=Blob::<BspHeader>::read(&Path::new("data/e1m1.bsp"));
		let mut a=0.0;

		let mut tex_array=Vec::<GLuint>::new();
		// Load textures to GL
		bsp.visit_textures( &mut |i,_|{ // we miss do notation :(
				let (tx,img)=bsp.get_texture_image(i); 
				show_texture(tx,&img);
				let txi=create_texture((tx.width as uint,tx.height as uint), &img,8);
				tex_array.push(txi);
			}
		);
		// show the map, isometric.
		bsp.visit_triangles(
			&|_,(v0,v1,v2),(_,txinfo),(_,plane),(face_id,_)| {
				glEnable(GL_TEXTURE_2D);
				let txi=txinfo.miptex as uint;
				glBindTexture(GL_TEXTURE_2D, *tex_array.get(txi));
				
//				draw_tri_iso_tex(v0,v1,v2, random_color(face_id), 1.0/2000.0)
				fn applytx<'a>(tx:&'a TexInfo,v:&'a BspVec3)->(&'a BspVec3,(f32,f32)){
					(v, (v3dot(&tx.axis_s,v)+tx.ofs_s,v3dot(&tx.axis_t,v)+tx.ofs_t) )
				}
				draw_tri_iso_tex(applytx(txinfo,v0),applytx(txinfo,v1),applytx(txinfo,v2), 0xffffff, 1.0/2000.0)
			}
		);
		glFlush();

		while true {
			use std::f32;
			glutMainLoopEvent();
		}
	}
}


struct Blob<HEADER> {
	data:~[u8],
}

impl<T> std::ops::Deref<T> for Blob<T> {
	fn deref<'s>(&'s self)->&'s T {
		unsafe {	&*(&self.data[0] as *u8 as *T)
		}
	}
}

impl<T> Blob<T> {
	fn num_bytes(&self) -> uint { self.data.len() }
	
	fn read(path:&Path)->Blob<T> {
		let data=
			match io::File::open(path).read_to_end() {
				Ok(data)=>{
					println!("read {} {} bytes", /*path_to_str*/path.as_str().unwrap_or(""), data.len());	
					data
				},
				Err(E)=>{
					println!("failed to read {}", path.as_str().unwrap_or("")); 
					//~[0,..intrinsics::size_of::<Header>()]		// still returns an empty object, hmmm.
					//vec::from_elem(0,intrinsics::size_of::<Header>())
					~[]
				}
			};
		Blob::<T>  {data:data}
	}
}


pub struct DEntry<Header, T> { 
	// we got a nasty bug passing wrong base ptr without header typeinfo here
	// &self instead of self..
	offset:u32, 
	size:u32
}
//unsafe fn byte_ofs_ref<'a,X,Y=X,I:Int=int>(base:&'a X, ofs:I)->&'a Y {
//	&*( (base as *_ as *u8).offset( ofs.to_int().unwrap() ) as *Y)
//}


impl<Header,T> DEntry<Header,T> {
	fn len(&self)->uint { unsafe {self.size as uint /  size_of::<T>()} }
	fn get<'a>(&'a self, owner:&'a Header,i:uint) -> &'a T{
		// TODO: to REALLY be safe, the sub-elements need to check safety from the blob 'owner'
		// unfortunately 'bspheader' doesn't seem to have that, although the last elements' ofs & size could be used
		// for an assert?
		unsafe {
//			&*(((owner as *Header as *u8).offset(self.offset as int) as *T).offset(i as int))
			&*(byte_ofs_ptr(owner, self.offset).offset(i as int))
		}
	}
}
pub type BspDEntry<T> =DEntry<BspHeader,T>;
pub struct BspHeader {
	version:u32,
	entities:BspDEntry<Entity>,
	planes:BspDEntry<Plane>,

	miptex:BspDEntry<MipHeader>,
	vertices:BspDEntry<BspVec3>,

	visibility:BspDEntry<VisiList>,
	nodes:BspDEntry<BspNode>,

	texinfo:BspDEntry<TexInfo>,

	faces:BspDEntry<Face>,

	lightmaps:BspDEntry<LightMap>,
	clipnodes:BspDEntry<ClipNode>,

	leafs:BspDEntry<BspLeaf>,

	marksurfaces:BspDEntry<i16>, //? no
	edges:BspDEntry<Edge>,

	surfedges:BspDEntry<i32>, // ? no
	models:BspDEntry<Model>,
}
fn random_color3(a:uint,b:uint,c:uint)->u32 {
	(a*b*c ^(a<<3)^(b<<8)*(c<<2)^(a<<19)^(b<<22)*(c<<13) )as u32
}
fn random_color(a:uint)->u32 {
	(a^(a<<3)^(a<<8)*(a<<2)^(a<<19)^(a<<22)*(a<<13) )as u32
}
macro_rules! get {
	($obj:ident . $field:ident [ $id:expr ] )=>($obj . $field . get( $obj , $id as uint ))
}
impl BspHeader {
	fn dump_vertices(&self) {	
		println!("{}",self.vertices.len());
		let mut i:uint=0;
		let vtlen=self.vertices.len();
		while i<vtlen { 
			let vtref= self.vertices.get(self,i as uint);
			println!("vertex{}/{}:owner={:p} vertex= {:p} ,({},{},{})",
				i,vtlen, self, vtref,
				vtref.val0(),vtref.val1(),vtref.val2());
			i+=1;
			let v=*vtref;
		}
	
	}
	fn dump(&self) {
		println!("vertices: {:u}", self.vertices.len());
		self.dump_vertices();
	}
	// some convinient accessors. - TODO autogenerate from a macro
	fn visit_triangles<'a,'b,R>(
			&'a self,
			fn_apply_to_tri:
				&'b|	tri_indices:(uint,uint,uint),
						tri_vertices:(&'a BspVec3,&'a BspVec3,&'a BspVec3),
						texinfo:(uint,&'a TexInfo),
						plane:(uint,&'a Plane),
						face_id:(uint,&'a Face)|->R
			)->Vec<R>
	{
		let mut return_val:Vec<R> =Vec::new();	// todo: reserve
		for face_id in range(0,self.faces.len()) {
			let face=self.faces.get(self, face_id);
			let eii = face.firstedge;
			let first_ei= *get!{self.surfedges[eii]};
			let first_edge= get!{self.edges[if first_ei>=0{first_ei}else{-first_ei}]};
			let iv0=(if first_ei>=0 {first_edge.vertex0}else{first_edge.vertex1})  as uint;
			let v0 = self.vertices.get(self, iv0 as uint) ;
			
			// todo: iterate as strips, not fans.
			for esubi in range(0, face.num_edges) {
				let ei = *get!{self.surfedges[eii+esubi as i32]};
				let edge=get!{self.edges[ei]};
				let edge=get!{self.edges[if ei>0{ei}else{-ei}]};
				let (iv1,iv2)=if ei>=0{ 
					(edge.vertex0 as uint,edge.vertex1 as uint)
				} else {
					(edge.vertex1 as uint,edge.vertex0 as uint)
				};
				let mut v1=self.vertices.get(self, iv1 as uint);
				let mut v2=self.vertices.get(self, iv2 as uint);

				let tri_result=
				(*fn_apply_to_tri) (
					(iv0,iv1,iv2),
					(v0,v1,v2),	
					(face.texinfo as uint,	get!{self.texinfo[face.texinfo]} ),
					(face.plane as uint,	get!{self.planes[face.plane]} ),
					(face_id, face)
				);
				return_val.push(tri_result);
			}
		}
		return_val
	}
	fn visit_faces<'a>(&'a self, f:&mut 'a |i:uint, f:&Face |) {
		for i in range(0, self.faces.len()) {
			(*f)(i, get!{self.faces[i]} );
		}
	}

	fn get_used_textures(&self)->HashSet<uint> {
		let mut used_tx= HashSet::<uint>::new();
		self.visit_faces( &mut |i:uint,face:&Face|{used_tx.insert(face.texinfo as uint);});
		used_tx
	}

	fn get_texture<'a>(&'a self, i:uint)->&'a MipTex {
		let txh=self.miptex.get(self,0);
		let tx = unsafe {&*(
			(txh as *_ as *u8).offset(*txh.miptex_offset.unsafe_ref(i as uint) as int) as *MipTex
		)};
		tx
	}

	fn visit_textures<'a>(&'a self, mut tex_fn:&'a|i:uint,tx:&MipTex|) {
		let txh =self.miptex.get(self,0);
		for i in range(0,txh.numtex) {
			let tx=self.get_texture(i as uint);
			unsafe {
				println!("tx: {} {} {}",
					i,
					// ::std::c_str::CString::new(&tx.name[0],false).as_str().unwrap_or(""), 
					tx.width, tx.height);
			}
			(*tex_fn)( i as uint, tx );
		}
	}

}

impl BspHeader {
	fn draw_edges(&self) {
		let scale=1.0f32/3000.0f32;
		let mut i=0u;
		while i < self.edges.len() {
			let e= get!{self.edges[i]};
			let v0 = get!{self.vertices[e.vertex0]};
			let v1 = get!{self.vertices[e.vertex1]};
			draw_line_iso(v0,v1,0xffffff, scale);
			i+=1;
		}
	}
	fn draw_faces(&self) {
		let scale=1.0f32/3000.0f32;
		self.visit_triangles(
			&|(i0,i1,i2),(v0,v1,v2),(_,txinfo),_,(face_id,_)| draw_tri_iso(v0,v1,v2, random_color(face_id), scale)
		);
		
	}
	fn draw_all_surface_edges(&self)
	{
		for i in range(0, self.surfedges.len()) {
			self.draw_edge(*(self.surfedges.get(self, i))  as int);
		}
	}
	fn draw_edge(&self, mut ei:int) {
		let scale=1.0f32/3000.0f32;
		if ei<0 {ei=-ei}
		let edge=self.edges.get(self, ei as uint);
		let v0 = self.vertices.get(self, edge.vertex0 as uint);
		let v1 = self.vertices.get(self, edge.vertex1 as uint);
		draw_line_iso(v0,v1,0xffffff, scale);
	}
}


// minimal tuple vmath
pub fn v3scale(&(x,y,z):&BspVec3,s:f32)->BspVec3 {	(x*s,y*s,z*s)}
pub fn v3add(&(x0,y0,z0):&BspVec3,&(x1,y1,z1):&BspVec3)->BspVec3 {	(x0+x1,y0+y1,z0+z1)}
pub fn v3sub(&(x0,y0,z0):&BspVec3,&(x1,y1,z1):&BspVec3)->BspVec3 {	(x0-x1,y0-y1,z0-z1)}
pub fn v3mad(v0:&BspVec3,v1:&BspVec3,f:f32)->BspVec3 { v3add(v0,&v3scale(v1,f))}
pub fn v3lerp(v0:&BspVec3,v1:&BspVec3,f:f32)->BspVec3 { v3add(v0,&v3scale(&v3sub(v1,v0),f))}
pub fn v3dot(&(x0,y0,z0):&BspVec3,&(x1,y1,z1):&BspVec3)->f32 {	x0*x1+y0*y1+z0*z1}
pub fn v3cross(&(x0,y0,z0):&BspVec3,&(x1,y1,z1):&BspVec3)->BspVec3 { ((y0*z1-z0*y1),(z0*x1-z1*x0),(x0*y1-x1*y0)) }
pub fn v3norm(v0:&BspVec3)->BspVec3{ v3scale(v0,1.0/f32::sqrt(v3dot(v0,v0))) }

pub type Point3s=(i16,i16,i16);
pub type BBox=(Point3s,Point3s);
pub struct Entity;
pub struct Plane {
	normal:BspVec3,
	dist:f32,
	plane_type:u32	// 0,1,2 = axial planes x,y,z; 3,4,5 = x,y,z predominant..
}
pub struct MipTex {
	name:[c_char,..16],
	width:u32, height:u32, offset1:u32, offset2:u32, offset4:u32, offset8:u32
}
pub struct MipHeader {
	numtex:u32, 
	miptex_offset:[u32,..0]	// actual size is..
}
impl MipHeader {
	pub unsafe fn tex_offsets(&self)->*u32 {
		(self as *_).offset(1) as *u32
	}
	pub unsafe fn tex_offset(&self, i:int)->u32 {
		let ofs=self.tex_offsets();
		*ofs.offset(i)
	}
	pub fn get_texture(&self, i:int)->&MipTex {
		unsafe {
			&*((self as *_ as *u8).offset( self.tex_offset(i) as int ) as *MipTex)
		}
	}
}

pub type BspVec3=(f32,f32,f32);
pub type BspVec2=(f32,f32);
pub struct VisiList;
pub struct BspNode {
	plane_id:u32,
	priv children:[i16,..2],
	bbox:BBox,
	firstface:u16,
	numfaces:u16
}
enum BspNodeChild {
	ChildNode(i16),ChildLeaf(i16)
}
impl BspNode {
	pub fn child_node(&self, i:int)->BspNodeChild {
		match self.children[i] {
			x if x>=0 => ChildNode(x),
			x  =>ChildLeaf(-(self.children[i]+1))
		}
	}
}

pub struct TexInfo {
	axis_s:BspVec3, ofs_s:f32,
	axis_t:BspVec3, ofs_t:f32,
	miptex:i32,
	flags:i32
}
pub struct Faces;
pub struct LightMap; //{ 	texels:[u8]} ??
pub struct ClipNode {
	planenum:u32,
	front:u16, back:u16,
}
pub struct BspLeaf {
	contents:u32, 
	visofs:u32, 
	min:Point3s,
	max:Point3s,
	firstmarksurface:u16,
	ambient_level:[u8,..AmbientNum]
}

pub struct Edge {
	vertex0:u16,vertex1:u16
}

enum Max{
	MaxMapHulls=4
}
static AmbientNum:int = 4;
enum Ambient {
	AmbientWater=0,AmbientSky,AmbientSlime,AmbientLava
}

pub struct Model {
	bound:BBox,
	origin:BspVec3,
	headnode:[i32,..MaxMapHulls],
	visileafs:i32,
	firstface:i32,
	numfaces:i32
	
}
pub struct Face {
	plane:u16,
	side:u16,
	firstedge:i32,
	num_edges:u16,
	texinfo:u16,
//	typelight:u8,
//	baselight:u8,
	light:[u8,..2],
	lightmap_ofs:i32, // [styles*sursize] samples..
}
/*
unsafe fn unpalettize_image_256(pixels:*u8, palette:&[u32,..256],   xsize:uint, ysize:uint)->Vec<u32> {
	buffer=Vec3::<u32>::from_fn(xsize*ysize,
		|i|{ pixels[i]
		);
}
*/

/*
unsafe fn ofs_u8_ptr<T,I:Int>(p:&T, ofs:I)->*u8 {
	(p as *T as *u8).offset(ofs.to_int().unwrap()) as *u8
}
unsafe fn ofs_void_ptr<T,I:Int>(p:&T, ofs:I)->*c_void {
	(p as *T as *u8).offset(ofs.to_int().unwrap()) as *c_void
}
unsafe fn ofs_ref<'a, T,I:Int>(p:&'a T, ofs:I)->&'a T {
	&*((p as *T).offset(ofs.to_int().unwrap()))
}
unsafe fn ofs_ptr<'a, T,I:Int>(p:*T, ofs:I)->*T {
	(p.offset(ofs.to_int().unwrap()))
}

unsafe fn void_ptr<T>(p:&T)->*c_void {
	ofs_u8_ptr(p,0) as *c_void
}
*/

/// return a reference to a different type at a byte offset from the given base object reference
unsafe fn byte_ofs_ref<'a,X,Y,I:Int=int>(base:&'a X, ofs:I)->&'a Y {	&*( (base as *_ as *u8).offset( ofs.to_int().unwrap() ) as *Y) }
/// return a raw ptr to a different type at a byte offset from the given base object reference
unsafe fn byte_ofs_ptr<'a,X,Y,I:Int=int>(base:&'a X, ofs:I)->*Y {
	(base as *_ as *u8).offset( ofs.to_int().unwrap() ) as *Y
}

trait ToVoidPtr {
	/// Get a void pointer for the contents of a collection
	unsafe fn void_ptr(&self)->*std::libc::c_void;
	/// Get a void pointer for the contents of a collection, with a byte offset
	unsafe fn byte_ofs<I:Int>(&self, ofs:I)->*std::libc::c_void;
}
impl<T> ToVoidPtr for Vec<T> {
	unsafe fn void_ptr(&self)->*std::libc::c_void {
		self.get(0) as *_ as *c_void
	}
	unsafe fn byte_ofs<I:Int>(&self,ofs:I)->*std::libc::c_void {
		self.void_ptr().offset(ofs.to_int().unwrap())
	}
}
impl<'a,T> ToVoidPtr for &'a T {
	unsafe fn void_ptr(&self)->*std::libc::c_void {
		// NOTE special handling of self, self here is &&T, we deref to get &T
		(*self) as *_ as *std::libc::c_void
	}	
	unsafe fn byte_ofs<I:Int>(&self,ofs:I)->*std::libc::c_void {
		// NOTE special handling of self, self here is &&T, we deref to get &T
		(*self as *_ as *u8).offset(ofs.to_int().unwrap()) as *c_void
	}
}
impl ToVoidPtr for *c_void {
	unsafe fn void_ptr(&self)->*std::libc::c_void {
		// NOTE special handling of self, self here is &&T, we deref to get &T
		*self
	}	
	unsafe fn byte_ofs<I:Int>(&self,ofs:I)->*std::libc::c_void {
		// NOTE special handling of self, self here is &&T, we deref to get &T
		(*self as *u8).offset(ofs.to_int().unwrap()) as *c_void
	}
}

static g_palette:&'static [u8]=include_bin!("data/palette.lmp");

impl BspHeader {
	fn get_texture_image<'a>(&'a self, i:uint)->(&'a MipTex, Vec<u32>) {
		unsafe {
			let tx=self.get_texture(i);
//			let txp = void_ptr(tx);

			let mip0 = tx.byte_ofs(tx.offset1);

			println!("size={}x{} miptex offsets {} {} {} {}",
				tx.width, tx.height, 
				tx.offset1, tx.offset2, tx.offset4, tx.offset8);

			let image = Vec::<u32>::from_fn(
				(tx.width*tx.height) as uint, 
				|i|{
					let x = mip0.byte_ofs(i);
					let color_index=*(x as *u8) as uint;
					let rgb_index=color_index*3;
					let r=g_palette[rgb_index+0] as u32;
					let g=g_palette[rgb_index+1] as u32;
					let b=g_palette[rgb_index+2] as u32;
					(r|(g<<8)|(b<<16)|(if color_index<255{0xff000000}else{0})) as u32
				}
			);
			(tx,image)
		}
	}

}

fn	show_texture(tx:&MipTex, image:&Vec<u32>) {
	unsafe {
//		let (tx,image)= self.get_texture_image(i);
		let i = (tx as *_ as int);
		let a= ((i^(i<<4)^(i>>7)*i) as f32) *0.57;
		let x = f32::sin(a)*0.5;
		let y = f32::cos(a*0.551)*0.5;

		glRasterPos2f(x,y);
		glDrawPixels(tx.width as GLsizei,tx.height as GLsizei, GL_RGBA, GL_UNSIGNED_BYTE, image.as_ptr() as *c_void);
		glFlush();
	}
}

fn get_format(bytes_per_pixel:uint, alpha_bits:uint)->(GLenum,GLenum) {
	match (bytes_per_pixel,alpha_bits) {
		(4,_) => (GL_RGBA,GL_UNSIGNED_BYTE),
		(3,0) => (GL_RGB,GL_UNSIGNED_BYTE),
		(2,4) => (GL_RGBA, GL_UNSIGNED_SHORT_4_4_4_4),
		(2,1) => (GL_RGBA, GL_UNSIGNED_SHORT_5_5_5_1),
		(2,0) => (GL_RGB, GL_UNSIGNED_SHORT_5_6_5),
		(1,8) => (GL_RGB, GL_UNSIGNED_BYTE_3_3_2),	// todo:should mean compressed.
		(1,_) => (GL_RGB, GL_UNSIGNED_BYTE_3_3_2),	// todo:should mean compressed.
		_ => (GL_RGBA, GL_UNSIGNED_BYTE)
	}
}

fn create_texture<Texel>((w,h):(uint,uint), raw_pixels:&Vec<Texel>, alpha_bits:uint)->GLuint {
	// todo: generic over format, u16->1555, u32->8888 u8->dxt5 and so on
	unsafe {
		let (fmt,fmt2)=get_format(size_of::<Texel>(), alpha_bits);
		assert!(w*h==raw_pixels.len())
		let mut tx:[GLuint,..1]=[0,..1];
		glGenTextures(1,tx.as_mut_ptr());
		glBindTexture(GL_TEXTURE_2D,tx[0]);
		glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER,GL_LINEAR as GLint);
		glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER,GL_LINEAR as GLint);
		glTexImage2D(GL_TEXTURE_2D, 0, fmt as GLint, w as GLsizei,h as GLsizei, 0, fmt, fmt2, raw_pixels.as_ptr() as *c_void); 
//		glGenerateMipMaps(GL_TEXTURE_2D);
		tx[0]
	}
}





	
 
