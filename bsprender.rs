use r3d::*;
use bsp::*;

// renderable version of a bsp..
pub struct BspRender {
	bsp:Box<Blob<BspHeader>>,
	textures:Vec<GLuint>,
	extents:Extents<Vec3<f32>>,
	centre:Vec3,
	radius:f32
	// vertex/index arrays would go here
}
impl BspRender {
	pub fn new(bsp:Box<Blob<BspHeader>>)->BspRender {
		let mut tex_array=Vec::new();

		bsp.visit_textures( &mut |i,_|{
				let (tx,img)=bsp.get_texture_image(i); 
				let txsize=(tx.width as u32,tx.height as u32);
				let txi=create_texture((tx.width,tx.height), &img,8);
				tex_array.push(txi);
			}
		);
		let (ext,c,r)=bsp.extents();

		BspRender{
			bsp:bsp,
			textures:tex_array,
			extents:ext,
			centre:c,
			radius:r
		}
	}

	pub fn render(&self) {
		self.bsp.visit_triangles(
			&mut |_,(v0,v1,v2),(_,txinfo),(_,plane),(face_id,_)| {
				unsafe {
					let txi=txinfo.miptex as uint;
					draw_set_texture(0,*self.textures.get(txi));
			
					fn applytx<'a>(tx:&'a TexInfo,v:&'a BspVec3)->(&'a BspVec3,(f32,f32)){
						(v, (v3dot(&tx.axis_s,v)+tx.ofs_s,v3dot(&tx.axis_t,v)+tx.ofs_t) )
					}
					let scale=1.0f32/2000.0f32;
					draw_tri_tex(applytx(txinfo,v0),applytx(txinfo,v1),applytx(txinfo,v2), 0xffffff,scale)
				}
			}
		);
	}
}