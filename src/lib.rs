
pub struct Vertices;
pub struct Indices;

pub struct Mesh(Vertices, Indices);
pub struct Vec3(f32, f32, f32);
pub struct Bounds((f32, f32, f32), (f32, f32, f32));

impl Mesh {
    pub fn new(verts : Vec<Vec3>, inds : Vec<usize>) -> Mesh {
        unimplemented!();
    }
    pub fn empty() -> Mesh { unimplemented!(); }
}

pub trait GeomField {
    fn f(&self, x: f32, y: f32, z: f32) -> f32;
}

struct SphereField(f32);
impl SphereField {
    pub fn new(r: f32) -> SphereField {
        SphereField(r)
    }
}
impl GeomField for SphereField {
    fn f(&self, x: f32, y: f32, z: f32) -> f32 {
        let &SphereField(r) = self;
        x * x + y * y + z * z - r * r
    }
}

pub fn create_mesh(field: &GeomField, bounds: &Bounds, grid_size: &Vec3) -> Mesh {
    unimplemented!();
}

fn tessellate_cube(min : &Vec3, max : &Vec3, field : &GeomField) -> Mesh {
    let p = [
        Vec3(min.0, min.1, min.2),
        Vec3(max.0, min.1, min.2),
        Vec3(max.0, min.1, max.2),
        Vec3(min.0, min.1, max.2),
        Vec3(min.0, max.1, min.2),
        Vec3(max.0, max.1, min.2),
        Vec3(max.0, max.1, max.2),
        Vec3(min.0, max.1, max.2),
    ];
    let f = [
        field.f(p[0].0, p[0].1, p[0].2),
        field.f(p[1].0, p[1].1, p[1].2),
        field.f(p[2].0, p[2].1, p[2].2),
        field.f(p[3].0, p[3].1, p[3].2),
        field.f(p[4].0, p[4].1, p[4].2),
        field.f(p[5].0, p[5].1, p[5].2),
        field.f(p[6].0, p[6].1, p[6].2),
        field.f(p[7].0, p[7].1, p[7].2),
    ];
    tessellate_corners(&p,&f)
}

fn tessellate_corners(p : &[Vec3], f : &[f32]) -> Mesh {
    let corners_in = (0..8).filter(|i| f[*i as usize] < 0.0).fold(0, |c, i| c | (1<<i));
    let edges = EDGE_TABLE[corners_in];
    if edges == 0 {
        Mesh::empty()
    } else {
        let vmap = [
            edge_intersection(edges, 0, p, f), edge_intersection(edges, 1, p, f),
            edge_intersection(edges, 2, p, f), edge_intersection(edges, 3, p, f),
            edge_intersection(edges, 4, p, f), edge_intersection(edges, 5, p, f),
            edge_intersection(edges, 6, p, f), edge_intersection(edges, 7, p, f),
            edge_intersection(edges, 8, p, f), edge_intersection(edges, 9, p, f),
            edge_intersection(edges, 10, p, f), edge_intersection(edges, 11, p, f),
        ];

        let mut inds = Vec::with_capacity(15);
        let mut verts = Vec::with_capacity(15);

        for t in TRI_TABLE[edges].iter() {
            let t = t.unwrap();
            inds.push(verts.len()+0);
            inds.push(verts.len()+1);
            inds.push(verts.len()+2);
            let v0 = vmap[t.0].unwrap();
            let v1 = vmap[t.1].unwrap();
            let v2 = vmap[t.2].unwrap();
            verts.push(Vec3(v0.0,v0.1,v0.2));
            verts.push(Vec3(v1.0,v1.1,v1.2));
            verts.push(Vec3(v2.0,v2.1,v2.2));
        }

        Mesh::new(verts, inds)
    }
}

#[inline]
fn edge_intersection(edges : usize, i : usize, p : &[Vec3], f : &[f32]) -> Option<(f32, f32, f32)> {
    if (edges >> i) & 1 == 0 {
        None
    } else {
        let (v0, v1) = EDGES[i];
        let (ref p0, ref p1) = (&p[v0], &p[v1 ]);
        let (f0, f1) = (f[v0], f[v1]);
        Some( (
            p0.0 - f0 * (p1.0 - p0.0) / (f1 - f0),
            p0.1 - f0 * (p1.1 - p0.1) / (f1 - f0),
            p0.2 - f0 * (p1.2 - p0.2) / (f1 - f0)
        ) )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] #[should_panic]
    fn it_works() {
        let sfield = SphereField::new(10.0);
        let _mesh = create_mesh(
            &sfield,
            &Bounds((-10.0, -10.0, -10.0), (10.0, 10.0, 10.0)),
            &Vec3(1.0, 1.0, 1.0),
        );
    }

    #[test]
    fn test_edge_isect() {
        let p = [
            Vec3(0.0,0.0,0.0),
            Vec3(1.0,0.0,0.0),
            Vec3(1.0,0.0,1.0),
            Vec3(0.0,0.0,1.0),

            Vec3(0.0,1.0,0.0),
            Vec3(1.0,1.0,0.0),
            Vec3(1.0,1.0,1.0),
            Vec3(0.0,1.0,1.0),
        ];

        // Intersection is on YZ plane in middle of the unit cube
        assert_eq!( edge_intersection(1, 0, &p, &[
            -1.0, 1.0, 1.0, -1.0,
            -1.0, 1.0, 1.0, -1.0,
        ]), Some((0.5, 0.0, 0.0)) );
        assert_eq!( edge_intersection(0x3ff, 2, &p, &[
            -1.0, 1.0, 1.0, -1.0,
            -1.0, 1.0, 1.0, -1.0,
        ]), Some((0.5, 0.0, 1.0)) );
        assert_eq!( edge_intersection(0x3ff, 4, &p, &[
            -1.0, 1.0, 1.0, -1.0,
            -1.0, 1.0, 1.0, -1.0,
        ]), Some((0.5, 1.0, 0.0)) );
        assert_eq!( edge_intersection(0x3ff, 6, &p, &[
            -1.0, 1.0, 1.0, -1.0,
            -1.0, 1.0, 1.0, -1.0,
        ]), Some((0.5, 1.0, 1.0)) );

        // Intersection is on XY plane in middle of unit cube
        assert_eq!( edge_intersection(0x3ff, 1, &p, &[
            -1.0,-1.0, 1.0, 1.0,
            -1.0,-1.0, 1.0, 1.0,
        ]), Some((1.0, 0.0, 0.5)) );
        assert_eq!( edge_intersection(0x3ff, 3, &p, &[
            -1.0,-1.0, 1.0, 1.0,
            -1.0,-1.0, 1.0, 1.0,
        ]), Some((0.0, 0.0, 0.5)) );
        assert_eq!( edge_intersection(0x3ff, 5, &p, &[
            -1.0,-1.0, 1.0, 1.0,
            -1.0,-1.0, 1.0, 1.0,
        ]), Some((1.0, 1.0, 0.5)) );
        assert_eq!( edge_intersection(0x3ff, 7, &p, &[
            -1.0,-1.0, 1.0, 1.0,
            -1.0,-1.0, 1.0, 1.0,
        ]), Some((0.0, 1.0, 0.5)) );

        // Intersection is on XZ plane in the middle of unit cube
        assert_eq!( edge_intersection(0x3ff, 8, &p, &[
            -1.0,-1.0,-1.0,-1.0,
             1.0, 1.0, 1.0, 1.0,
        ]), Some((0.0, 0.5, 0.0)) );
        assert_eq!( edge_intersection(0x3ff, 9, &p, &[
            -1.0,-1.0,-1.0,-1.0,
            1.0, 1.0, 1.0, 1.0,
        ]), Some((1.0, 0.5, 0.0)) );
        assert_eq!( edge_intersection(0xfff, 10, &p, &[
            -1.0,-1.0,-1.0,-1.0,
            1.0, 1.0, 1.0, 1.0,
        ]), Some((1.0, 0.5, 1.0)) );
        assert_eq!( edge_intersection(0xfff, 11, &p, &[
            -1.0,-1.0,-1.0,-1.0,
            1.0, 1.0, 1.0, 1.0,
        ]), Some((0.0, 0.5, 1.0)) );

    }

}

const EDGES : [(usize,usize);12] = [
    (0,1), (1,2), (2,3), (3,0),
    (4,5), (5,6), (6,7), (7,4),
    (0,4), (1,5), (2,6), (3,7)
];

const EDGE_TABLE: [usize; 256] = [
    0x0,
    0x109,
    0x203,
    0x30a,
    0x406,
    0x50f,
    0x605,
    0x70c,
    0x80c,
    0x905,
    0xa0f,
    0xb06,
    0xc0a,
    0xd03,
    0xe09,
    0xf00,
    0x190,
    0x99,
    0x393,
    0x29a,
    0x596,
    0x49f,
    0x795,
    0x69c,
    0x99c,
    0x895,
    0xb9f,
    0xa96,
    0xd9a,
    0xc93,
    0xf99,
    0xe90,
    0x230,
    0x339,
    0x33,
    0x13a,
    0x636,
    0x73f,
    0x435,
    0x53c,
    0xa3c,
    0xb35,
    0x83f,
    0x936,
    0xe3a,
    0xf33,
    0xc39,
    0xd30,
    0x3a0,
    0x2a9,
    0x1a3,
    0xaa,
    0x7a6,
    0x6af,
    0x5a5,
    0x4ac,
    0xbac,
    0xaa5,
    0x9af,
    0x8a6,
    0xfaa,
    0xea3,
    0xda9,
    0xca0,
    0x460,
    0x569,
    0x663,
    0x76a,
    0x66,
    0x16f,
    0x265,
    0x36c,
    0xc6c,
    0xd65,
    0xe6f,
    0xf66,
    0x86a,
    0x963,
    0xa69,
    0xb60,
    0x5f0,
    0x4f9,
    0x7f3,
    0x6fa,
    0x1f6,
    0xff,
    0x3f5,
    0x2fc,
    0xdfc,
    0xcf5,
    0xfff,
    0xef6,
    0x9fa,
    0x8f3,
    0xbf9,
    0xaf0,
    0x650,
    0x759,
    0x453,
    0x55a,
    0x256,
    0x35f,
    0x55,
    0x15c,
    0xe5c,
    0xf55,
    0xc5f,
    0xd56,
    0xa5a,
    0xb53,
    0x859,
    0x950,
    0x7c0,
    0x6c9,
    0x5c3,
    0x4ca,
    0x3c6,
    0x2cf,
    0x1c5,
    0xcc,
    0xfcc,
    0xec5,
    0xdcf,
    0xcc6,
    0xbca,
    0xac3,
    0x9c9,
    0x8c0,
    0x8c0,
    0x9c9,
    0xac3,
    0xbca,
    0xcc6,
    0xdcf,
    0xec5,
    0xfcc,
    0xcc,
    0x1c5,
    0x2cf,
    0x3c6,
    0x4ca,
    0x5c3,
    0x6c9,
    0x7c0,
    0x950,
    0x859,
    0xb53,
    0xa5a,
    0xd56,
    0xc5f,
    0xf55,
    0xe5c,
    0x15c,
    0x55,
    0x35f,
    0x256,
    0x55a,
    0x453,
    0x759,
    0x650,
    0xaf0,
    0xbf9,
    0x8f3,
    0x9fa,
    0xef6,
    0xfff,
    0xcf5,
    0xdfc,
    0x2fc,
    0x3f5,
    0xff,
    0x1f6,
    0x6fa,
    0x7f3,
    0x4f9,
    0x5f0,
    0xb60,
    0xa69,
    0x963,
    0x86a,
    0xf66,
    0xe6f,
    0xd65,
    0xc6c,
    0x36c,
    0x265,
    0x16f,
    0x66,
    0x76a,
    0x663,
    0x569,
    0x460,
    0xca0,
    0xda9,
    0xea3,
    0xfaa,
    0x8a6,
    0x9af,
    0xaa5,
    0xbac,
    0x4ac,
    0x5a5,
    0x6af,
    0x7a6,
    0xaa,
    0x1a3,
    0x2a9,
    0x3a0,
    0xd30,
    0xc39,
    0xf33,
    0xe3a,
    0x936,
    0x83f,
    0xb35,
    0xa3c,
    0x53c,
    0x435,
    0x73f,
    0x636,
    0x13a,
    0x33,
    0x339,
    0x230,
    0xe90,
    0xf99,
    0xc93,
    0xd9a,
    0xa96,
    0xb9f,
    0x895,
    0x99c,
    0x69c,
    0x795,
    0x49f,
    0x596,
    0x29a,
    0x393,
    0x99,
    0x190,
    0xf00,
    0xe09,
    0xd03,
    0xc0a,
    0xb06,
    0xa0f,
    0x905,
    0x80c,
    0x70c,
    0x605,
    0x50f,
    0x406,
    0x30a,
    0x203,
    0x109,
    0x0,
];

const TRI_TABLE: [[Option<(usize, usize, usize)>; 5]; 256] = [
    [None, None, None, None, None, ],
    [Some((0, 8, 3)), None, None, None, None, ],
    [Some((0, 1, 9)), None, None, None, None, ],
    [Some((1, 8, 3)), Some((9, 8, 1)), None, None, None, ],
    [Some((1, 2, 10)), None, None, None, None, ],
    [Some((0, 8, 3)), Some((1, 2, 10)), None, None, None, ],
    [Some((9, 2, 10)), Some((0, 2, 9)), None, None, None, ],
    [Some((2, 8, 3)), Some((2, 10, 8)), Some((10, 9, 8)), None, None, ],
    [Some((3, 11, 2)), None, None, None, None, ],
    [Some((0, 11, 2)), Some((8, 11, 0)), None, None, None, ],
    [Some((1, 9, 0)), Some((2, 3, 11)), None, None, None, ],
    [Some((1, 11, 2)), Some((1, 9, 11)), Some((9, 8, 11)), None, None, ],
    [Some((3, 10, 1)), Some((11, 10, 3)), None, None, None, ],
    [Some((0, 10, 1)), Some((0, 8, 10)), Some((8, 11, 10)), None, None, ],
    [Some((3, 9, 0)), Some((3, 11, 9)), Some((11, 10, 9)), None, None, ],
    [Some((9, 8, 10)), Some((10, 8, 11)), None, None, None, ],
    [Some((4, 7, 8)), None, None, None, None, ],
    [Some((4, 3, 0)), Some((7, 3, 4)), None, None, None, ],
    [Some((0, 1, 9)), Some((8, 4, 7)), None, None, None, ],
    [Some((4, 1, 9)), Some((4, 7, 1)), Some((7, 3, 1)), None, None, ],
    [Some((1, 2, 10)), Some((8, 4, 7)), None, None, None, ],
    [Some((3, 4, 7)), Some((3, 0, 4)), Some((1, 2, 10)), None, None, ],
    [Some((9, 2, 10)), Some((9, 0, 2)), Some((8, 4, 7)), None, None, ],
    [Some((2, 10, 9)), Some((2, 9, 7)), Some((2, 7, 3)), Some((7, 9, 4)), None, ],
    [Some((8, 4, 7)), Some((3, 11, 2)), None, None, None, ],
    [Some((11, 4, 7)), Some((11, 2, 4)), Some((2, 0, 4)), None, None, ],
    [Some((9, 0, 1)), Some((8, 4, 7)), Some((2, 3, 11)), None, None, ],
    [Some((4, 7, 11)), Some((9, 4, 11)), Some((9, 11, 2)), Some((9, 2, 1)), None, ],
    [Some((3, 10, 1)), Some((3, 11, 10)), Some((7, 8, 4)), None, None, ],
    [Some((1, 11, 10)), Some((1, 4, 11)), Some((1, 0, 4)), Some((7, 11, 4)), None, ],
    [Some((4, 7, 8)), Some((9, 0, 11)), Some((9, 11, 10)), Some((11, 0, 3)), None, ],
    [Some((4, 7, 11)), Some((4, 11, 9)), Some((9, 11, 10)), None, None, ],
    [Some((9, 5, 4)), None, None, None, None, ],
    [Some((9, 5, 4)), Some((0, 8, 3)), None, None, None, ],
    [Some((0, 5, 4)), Some((1, 5, 0)), None, None, None, ],
    [Some((8, 5, 4)), Some((8, 3, 5)), Some((3, 1, 5)), None, None, ],
    [Some((1, 2, 10)), Some((9, 5, 4)), None, None, None, ],
    [Some((3, 0, 8)), Some((1, 2, 10)), Some((4, 9, 5)), None, None, ],
    [Some((5, 2, 10)), Some((5, 4, 2)), Some((4, 0, 2)), None, None, ],
    [Some((2, 10, 5)), Some((3, 2, 5)), Some((3, 5, 4)), Some((3, 4, 8)), None, ],
    [Some((9, 5, 4)), Some((2, 3, 11)), None, None, None, ],
    [Some((0, 11, 2)), Some((0, 8, 11)), Some((4, 9, 5)), None, None, ],
    [Some((0, 5, 4)), Some((0, 1, 5)), Some((2, 3, 11)), None, None, ],
    [Some((2, 1, 5)), Some((2, 5, 8)), Some((2, 8, 11)), Some((4, 8, 5)), None, ],
    [Some((10, 3, 11)), Some((10, 1, 3)), Some((9, 5, 4)), None, None, ],
    [Some((4, 9, 5)), Some((0, 8, 1)), Some((8, 10, 1)), Some((8, 11, 10)), None, ],
    [Some((5, 4, 0)), Some((5, 0, 11)), Some((5, 11, 10)), Some((11, 0, 3)), None, ],
    [Some((5, 4, 8)), Some((5, 8, 10)), Some((10, 8, 11)), None, None, ],
    [Some((9, 7, 8)), Some((5, 7, 9)), None, None, None, ],
    [Some((9, 3, 0)), Some((9, 5, 3)), Some((5, 7, 3)), None, None, ],
    [Some((0, 7, 8)), Some((0, 1, 7)), Some((1, 5, 7)), None, None, ],
    [Some((1, 5, 3)), Some((3, 5, 7)), None, None, None, ],
    [Some((9, 7, 8)), Some((9, 5, 7)), Some((10, 1, 2)), None, None, ],
    [Some((10, 1, 2)), Some((9, 5, 0)), Some((5, 3, 0)), Some((5, 7, 3)), None, ],
    [Some((8, 0, 2)), Some((8, 2, 5)), Some((8, 5, 7)), Some((10, 5, 2)), None, ],
    [Some((2, 10, 5)), Some((2, 5, 3)), Some((3, 5, 7)), None, None, ],
    [Some((7, 9, 5)), Some((7, 8, 9)), Some((3, 11, 2)), None, None, ],
    [Some((9, 5, 7)), Some((9, 7, 2)), Some((9, 2, 0)), Some((2, 7, 11)), None, ],
    [Some((2, 3, 11)), Some((0, 1, 8)), Some((1, 7, 8)), Some((1, 5, 7)), None, ],
    [Some((11, 2, 1)), Some((11, 1, 7)), Some((7, 1, 5)), None, None, ],
    [Some((9, 5, 8)), Some((8, 5, 7)), Some((10, 1, 3)), Some((10, 3, 11)), None, ],
    [Some((5, 7, 0)), Some((5, 0, 9)), Some((7, 11, 0)), Some((1, 0, 10)), Some((11, 10, 0)), ],
    [Some((11, 10, 0)), Some((11, 0, 3)), Some((10, 5, 0)), Some((8, 0, 7)), Some((5, 7, 0)), ],
    [Some((11, 10, 5)), Some((7, 11, 5)), None, None, None, ],
    [Some((10, 6, 5)), None, None, None, None, ],
    [Some((0, 8, 3)), Some((5, 10, 6)), None, None, None, ],
    [Some((9, 0, 1)), Some((5, 10, 6)), None, None, None, ],
    [Some((1, 8, 3)), Some((1, 9, 8)), Some((5, 10, 6)), None, None, ],
    [Some((1, 6, 5)), Some((2, 6, 1)), None, None, None, ],
    [Some((1, 6, 5)), Some((1, 2, 6)), Some((3, 0, 8)), None, None, ],
    [Some((9, 6, 5)), Some((9, 0, 6)), Some((0, 2, 6)), None, None, ],
    [Some((5, 9, 8)), Some((5, 8, 2)), Some((5, 2, 6)), Some((3, 2, 8)), None, ],
    [Some((2, 3, 11)), Some((10, 6, 5)), None, None, None, ],
    [Some((11, 0, 8)), Some((11, 2, 0)), Some((10, 6, 5)), None, None, ],
    [Some((0, 1, 9)), Some((2, 3, 11)), Some((5, 10, 6)), None, None, ],
    [Some((5, 10, 6)), Some((1, 9, 2)), Some((9, 11, 2)), Some((9, 8, 11)), None, ],
    [Some((6, 3, 11)), Some((6, 5, 3)), Some((5, 1, 3)), None, None, ],
    [Some((0, 8, 11)), Some((0, 11, 5)), Some((0, 5, 1)), Some((5, 11, 6)), None, ],
    [Some((3, 11, 6)), Some((0, 3, 6)), Some((0, 6, 5)), Some((0, 5, 9)), None, ],
    [Some((6, 5, 9)), Some((6, 9, 11)), Some((11, 9, 8)), None, None, ],
    [Some((5, 10, 6)), Some((4, 7, 8)), None, None, None, ],
    [Some((4, 3, 0)), Some((4, 7, 3)), Some((6, 5, 10)), None, None, ],
    [Some((1, 9, 0)), Some((5, 10, 6)), Some((8, 4, 7)), None, None, ],
    [Some((10, 6, 5)), Some((1, 9, 7)), Some((1, 7, 3)), Some((7, 9, 4)), None, ],
    [Some((6, 1, 2)), Some((6, 5, 1)), Some((4, 7, 8)), None, None, ],
    [Some((1, 2, 5)), Some((5, 2, 6)), Some((3, 0, 4)), Some((3, 4, 7)), None, ],
    [Some((8, 4, 7)), Some((9, 0, 5)), Some((0, 6, 5)), Some((0, 2, 6)), None, ],
    [Some((7, 3, 9)), Some((7, 9, 4)), Some((3, 2, 9)), Some((5, 9, 6)), Some((2, 6, 9)), ],
    [Some((3, 11, 2)), Some((7, 8, 4)), Some((10, 6, 5)), None, None, ],
    [Some((5, 10, 6)), Some((4, 7, 2)), Some((4, 2, 0)), Some((2, 7, 11)), None, ],
    [Some((0, 1, 9)), Some((4, 7, 8)), Some((2, 3, 11)), Some((5, 10, 6)), None, ],
    [Some((9, 2, 1)), Some((9, 11, 2)), Some((9, 4, 11)), Some((7, 11, 4)), Some((5, 10, 6)), ],
    [Some((8, 4, 7)), Some((3, 11, 5)), Some((3, 5, 1)), Some((5, 11, 6)), None, ],
    [Some((5, 1, 11)), Some((5, 11, 6)), Some((1, 0, 11)), Some((7, 11, 4)), Some((0, 4, 11)), ],
    [Some((0, 5, 9)), Some((0, 6, 5)), Some((0, 3, 6)), Some((11, 6, 3)), Some((8, 4, 7)), ],
    [Some((6, 5, 9)), Some((6, 9, 11)), Some((4, 7, 9)), Some((7, 11, 9)), None, ],
    [Some((10, 4, 9)), Some((6, 4, 10)), None, None, None, ],
    [Some((4, 10, 6)), Some((4, 9, 10)), Some((0, 8, 3)), None, None, ],
    [Some((10, 0, 1)), Some((10, 6, 0)), Some((6, 4, 0)), None, None, ],
    [Some((8, 3, 1)), Some((8, 1, 6)), Some((8, 6, 4)), Some((6, 1, 10)), None, ],
    [Some((1, 4, 9)), Some((1, 2, 4)), Some((2, 6, 4)), None, None, ],
    [Some((3, 0, 8)), Some((1, 2, 9)), Some((2, 4, 9)), Some((2, 6, 4)), None, ],
    [Some((0, 2, 4)), Some((4, 2, 6)), None, None, None, ],
    [Some((8, 3, 2)), Some((8, 2, 4)), Some((4, 2, 6)), None, None, ],
    [Some((10, 4, 9)), Some((10, 6, 4)), Some((11, 2, 3)), None, None, ],
    [Some((0, 8, 2)), Some((2, 8, 11)), Some((4, 9, 10)), Some((4, 10, 6)), None, ],
    [Some((3, 11, 2)), Some((0, 1, 6)), Some((0, 6, 4)), Some((6, 1, 10)), None, ],
    [Some((6, 4, 1)), Some((6, 1, 10)), Some((4, 8, 1)), Some((2, 1, 11)), Some((8, 11, 1)), ],
    [Some((9, 6, 4)), Some((9, 3, 6)), Some((9, 1, 3)), Some((11, 6, 3)), None, ],
    [Some((8, 11, 1)), Some((8, 1, 0)), Some((11, 6, 1)), Some((9, 1, 4)), Some((6, 4, 1)), ],
    [Some((3, 11, 6)), Some((3, 6, 0)), Some((0, 6, 4)), None, None, ],
    [Some((6, 4, 8)), Some((11, 6, 8)), None, None, None, ],
    [Some((7, 10, 6)), Some((7, 8, 10)), Some((8, 9, 10)), None, None, ],
    [Some((0, 7, 3)), Some((0, 10, 7)), Some((0, 9, 10)), Some((6, 7, 10)), None, ],
    [Some((10, 6, 7)), Some((1, 10, 7)), Some((1, 7, 8)), Some((1, 8, 0)), None, ],
    [Some((10, 6, 7)), Some((10, 7, 1)), Some((1, 7, 3)), None, None, ],
    [Some((1, 2, 6)), Some((1, 6, 8)), Some((1, 8, 9)), Some((8, 6, 7)), None, ],
    [Some((2, 6, 9)), Some((2, 9, 1)), Some((6, 7, 9)), Some((0, 9, 3)), Some((7, 3, 9)), ],
    [Some((7, 8, 0)), Some((7, 0, 6)), Some((6, 0, 2)), None, None, ],
    [Some((7, 3, 2)), Some((6, 7, 2)), None, None, None, ],
    [Some((2, 3, 11)), Some((10, 6, 8)), Some((10, 8, 9)), Some((8, 6, 7)), None, ],
    [Some((2, 0, 7)), Some((2, 7, 11)), Some((0, 9, 7)), Some((6, 7, 10)), Some((9, 10, 7)), ],
    [Some((1, 8, 0)), Some((1, 7, 8)), Some((1, 10, 7)), Some((6, 7, 10)), Some((2, 3, 11)), ],
    [Some((11, 2, 1)), Some((11, 1, 7)), Some((10, 6, 1)), Some((6, 7, 1)), None, ],
    [Some((8, 9, 6)), Some((8, 6, 7)), Some((9, 1, 6)), Some((11, 6, 3)), Some((1, 3, 6)), ],
    [Some((0, 9, 1)), Some((11, 6, 7)), None, None, None, ],
    [Some((7, 8, 0)), Some((7, 0, 6)), Some((3, 11, 0)), Some((11, 6, 0)), None, ],
    [Some((7, 11, 6)), None, None, None, None, ],
    [Some((7, 6, 11)), None, None, None, None, ],
    [Some((3, 0, 8)), Some((11, 7, 6)), None, None, None, ],
    [Some((0, 1, 9)), Some((11, 7, 6)), None, None, None, ],
    [Some((8, 1, 9)), Some((8, 3, 1)), Some((11, 7, 6)), None, None, ],
    [Some((10, 1, 2)), Some((6, 11, 7)), None, None, None, ],
    [Some((1, 2, 10)), Some((3, 0, 8)), Some((6, 11, 7)), None, None, ],
    [Some((2, 9, 0)), Some((2, 10, 9)), Some((6, 11, 7)), None, None, ],
    [Some((6, 11, 7)), Some((2, 10, 3)), Some((10, 8, 3)), Some((10, 9, 8)), None, ],
    [Some((7, 2, 3)), Some((6, 2, 7)), None, None, None, ],
    [Some((7, 0, 8)), Some((7, 6, 0)), Some((6, 2, 0)), None, None, ],
    [Some((2, 7, 6)), Some((2, 3, 7)), Some((0, 1, 9)), None, None, ],
    [Some((1, 6, 2)), Some((1, 8, 6)), Some((1, 9, 8)), Some((8, 7, 6)), None, ],
    [Some((10, 7, 6)), Some((10, 1, 7)), Some((1, 3, 7)), None, None, ],
    [Some((10, 7, 6)), Some((1, 7, 10)), Some((1, 8, 7)), Some((1, 0, 8)), None, ],
    [Some((0, 3, 7)), Some((0, 7, 10)), Some((0, 10, 9)), Some((6, 10, 7)), None, ],
    [Some((7, 6, 10)), Some((7, 10, 8)), Some((8, 10, 9)), None, None, ],
    [Some((6, 8, 4)), Some((11, 8, 6)), None, None, None, ],
    [Some((3, 6, 11)), Some((3, 0, 6)), Some((0, 4, 6)), None, None, ],
    [Some((8, 6, 11)), Some((8, 4, 6)), Some((9, 0, 1)), None, None, ],
    [Some((9, 4, 6)), Some((9, 6, 3)), Some((9, 3, 1)), Some((11, 3, 6)), None, ],
    [Some((6, 8, 4)), Some((6, 11, 8)), Some((2, 10, 1)), None, None, ],
    [Some((1, 2, 10)), Some((3, 0, 11)), Some((0, 6, 11)), Some((0, 4, 6)), None, ],
    [Some((4, 11, 8)), Some((4, 6, 11)), Some((0, 2, 9)), Some((2, 10, 9)), None, ],
    [Some((10, 9, 3)), Some((10, 3, 2)), Some((9, 4, 3)), Some((11, 3, 6)), Some((4, 6, 3)), ],
    [Some((8, 2, 3)), Some((8, 4, 2)), Some((4, 6, 2)), None, None, ],
    [Some((0, 4, 2)), Some((4, 6, 2)), None, None, None, ],
    [Some((1, 9, 0)), Some((2, 3, 4)), Some((2, 4, 6)), Some((4, 3, 8)), None, ],
    [Some((1, 9, 4)), Some((1, 4, 2)), Some((2, 4, 6)), None, None, ],
    [Some((8, 1, 3)), Some((8, 6, 1)), Some((8, 4, 6)), Some((6, 10, 1)), None, ],
    [Some((10, 1, 0)), Some((10, 0, 6)), Some((6, 0, 4)), None, None, ],
    [Some((4, 6, 3)), Some((4, 3, 8)), Some((6, 10, 3)), Some((0, 3, 9)), Some((10, 9, 3)), ],
    [Some((10, 9, 4)), Some((6, 10, 4)), None, None, None, ],
    [Some((4, 9, 5)), Some((7, 6, 11)), None, None, None, ],
    [Some((0, 8, 3)), Some((4, 9, 5)), Some((11, 7, 6)), None, None, ],
    [Some((5, 0, 1)), Some((5, 4, 0)), Some((7, 6, 11)), None, None, ],
    [Some((11, 7, 6)), Some((8, 3, 4)), Some((3, 5, 4)), Some((3, 1, 5)), None, ],
    [Some((9, 5, 4)), Some((10, 1, 2)), Some((7, 6, 11)), None, None, ],
    [Some((6, 11, 7)), Some((1, 2, 10)), Some((0, 8, 3)), Some((4, 9, 5)), None, ],
    [Some((7, 6, 11)), Some((5, 4, 10)), Some((4, 2, 10)), Some((4, 0, 2)), None, ],
    [Some((3, 4, 8)), Some((3, 5, 4)), Some((3, 2, 5)), Some((10, 5, 2)), Some((11, 7, 6)), ],
    [Some((7, 2, 3)), Some((7, 6, 2)), Some((5, 4, 9)), None, None, ],
    [Some((9, 5, 4)), Some((0, 8, 6)), Some((0, 6, 2)), Some((6, 8, 7)), None, ],
    [Some((3, 6, 2)), Some((3, 7, 6)), Some((1, 5, 0)), Some((5, 4, 0)), None, ],
    [Some((6, 2, 8)), Some((6, 8, 7)), Some((2, 1, 8)), Some((4, 8, 5)), Some((1, 5, 8)), ],
    [Some((9, 5, 4)), Some((10, 1, 6)), Some((1, 7, 6)), Some((1, 3, 7)), None, ],
    [Some((1, 6, 10)), Some((1, 7, 6)), Some((1, 0, 7)), Some((8, 7, 0)), Some((9, 5, 4)), ],
    [Some((4, 0, 10)), Some((4, 10, 5)), Some((0, 3, 10)), Some((6, 10, 7)), Some((3, 7, 10)), ],
    [Some((7, 6, 10)), Some((7, 10, 8)), Some((5, 4, 10)), Some((4, 8, 10)), None, ],
    [Some((6, 9, 5)), Some((6, 11, 9)), Some((11, 8, 9)), None, None, ],
    [Some((3, 6, 11)), Some((0, 6, 3)), Some((0, 5, 6)), Some((0, 9, 5)), None, ],
    [Some((0, 11, 8)), Some((0, 5, 11)), Some((0, 1, 5)), Some((5, 6, 11)), None, ],
    [Some((6, 11, 3)), Some((6, 3, 5)), Some((5, 3, 1)), None, None, ],
    [Some((1, 2, 10)), Some((9, 5, 11)), Some((9, 11, 8)), Some((11, 5, 6)), None, ],
    [Some((0, 11, 3)), Some((0, 6, 11)), Some((0, 9, 6)), Some((5, 6, 9)), Some((1, 2, 10)), ],
    [Some((11, 8, 5)), Some((11, 5, 6)), Some((8, 0, 5)), Some((10, 5, 2)), Some((0, 2, 5)), ],
    [Some((6, 11, 3)), Some((6, 3, 5)), Some((2, 10, 3)), Some((10, 5, 3)), None, ],
    [Some((5, 8, 9)), Some((5, 2, 8)), Some((5, 6, 2)), Some((3, 8, 2)), None, ],
    [Some((9, 5, 6)), Some((9, 6, 0)), Some((0, 6, 2)), None, None, ],
    [Some((1, 5, 8)), Some((1, 8, 0)), Some((5, 6, 8)), Some((3, 8, 2)), Some((6, 2, 8)), ],
    [Some((1, 5, 6)), Some((2, 1, 6)), None, None, None, ],
    [Some((1, 3, 6)), Some((1, 6, 10)), Some((3, 8, 6)), Some((5, 6, 9)), Some((8, 9, 6)), ],
    [Some((10, 1, 0)), Some((10, 0, 6)), Some((9, 5, 0)), Some((5, 6, 0)), None, ],
    [Some((0, 3, 8)), Some((5, 6, 10)), None, None, None, ],
    [Some((10, 5, 6)), None, None, None, None, ],
    [Some((11, 5, 10)), Some((7, 5, 11)), None, None, None, ],
    [Some((11, 5, 10)), Some((11, 7, 5)), Some((8, 3, 0)), None, None, ],
    [Some((5, 11, 7)), Some((5, 10, 11)), Some((1, 9, 0)), None, None, ],
    [Some((10, 7, 5)), Some((10, 11, 7)), Some((9, 8, 1)), Some((8, 3, 1)), None, ],
    [Some((11, 1, 2)), Some((11, 7, 1)), Some((7, 5, 1)), None, None, ],
    [Some((0, 8, 3)), Some((1, 2, 7)), Some((1, 7, 5)), Some((7, 2, 11)), None, ],
    [Some((9, 7, 5)), Some((9, 2, 7)), Some((9, 0, 2)), Some((2, 11, 7)), None, ],
    [Some((7, 5, 2)), Some((7, 2, 11)), Some((5, 9, 2)), Some((3, 2, 8)), Some((9, 8, 2)), ],
    [Some((2, 5, 10)), Some((2, 3, 5)), Some((3, 7, 5)), None, None, ],
    [Some((8, 2, 0)), Some((8, 5, 2)), Some((8, 7, 5)), Some((10, 2, 5)), None, ],
    [Some((9, 0, 1)), Some((5, 10, 3)), Some((5, 3, 7)), Some((3, 10, 2)), None, ],
    [Some((9, 8, 2)), Some((9, 2, 1)), Some((8, 7, 2)), Some((10, 2, 5)), Some((7, 5, 2)), ],
    [Some((1, 3, 5)), Some((3, 7, 5)), None, None, None, ],
    [Some((0, 8, 7)), Some((0, 7, 1)), Some((1, 7, 5)), None, None, ],
    [Some((9, 0, 3)), Some((9, 3, 5)), Some((5, 3, 7)), None, None, ],
    [Some((9, 8, 7)), Some((5, 9, 7)), None, None, None, ],
    [Some((5, 8, 4)), Some((5, 10, 8)), Some((10, 11, 8)), None, None, ],
    [Some((5, 0, 4)), Some((5, 11, 0)), Some((5, 10, 11)), Some((11, 3, 0)), None, ],
    [Some((0, 1, 9)), Some((8, 4, 10)), Some((8, 10, 11)), Some((10, 4, 5)), None, ],
    [Some((10, 11, 4)), Some((10, 4, 5)), Some((11, 3, 4)), Some((9, 4, 1)), Some((3, 1, 4)), ],
    [Some((2, 5, 1)), Some((2, 8, 5)), Some((2, 11, 8)), Some((4, 5, 8)), None, ],
    [Some((0, 4, 11)), Some((0, 11, 3)), Some((4, 5, 11)), Some((2, 11, 1)), Some((5, 1, 11)), ],
    [Some((0, 2, 5)), Some((0, 5, 9)), Some((2, 11, 5)), Some((4, 5, 8)), Some((11, 8, 5)), ],
    [Some((9, 4, 5)), Some((2, 11, 3)), None, None, None, ],
    [Some((2, 5, 10)), Some((3, 5, 2)), Some((3, 4, 5)), Some((3, 8, 4)), None, ],
    [Some((5, 10, 2)), Some((5, 2, 4)), Some((4, 2, 0)), None, None, ],
    [Some((3, 10, 2)), Some((3, 5, 10)), Some((3, 8, 5)), Some((4, 5, 8)), Some((0, 1, 9)), ],
    [Some((5, 10, 2)), Some((5, 2, 4)), Some((1, 9, 2)), Some((9, 4, 2)), None, ],
    [Some((8, 4, 5)), Some((8, 5, 3)), Some((3, 5, 1)), None, None, ],
    [Some((0, 4, 5)), Some((1, 0, 5)), None, None, None, ],
    [Some((8, 4, 5)), Some((8, 5, 3)), Some((9, 0, 5)), Some((0, 3, 5)), None, ],
    [Some((9, 4, 5)), None, None, None, None, ],
    [Some((4, 11, 7)), Some((4, 9, 11)), Some((9, 10, 11)), None, None, ],
    [Some((0, 8, 3)), Some((4, 9, 7)), Some((9, 11, 7)), Some((9, 10, 11)), None, ],
    [Some((1, 10, 11)), Some((1, 11, 4)), Some((1, 4, 0)), Some((7, 4, 11)), None, ],
    [Some((3, 1, 4)), Some((3, 4, 8)), Some((1, 10, 4)), Some((7, 4, 11)), Some((10, 11, 4)), ],
    [Some((4, 11, 7)), Some((9, 11, 4)), Some((9, 2, 11)), Some((9, 1, 2)), None, ],
    [Some((9, 7, 4)), Some((9, 11, 7)), Some((9, 1, 11)), Some((2, 11, 1)), Some((0, 8, 3)), ],
    [Some((11, 7, 4)), Some((11, 4, 2)), Some((2, 4, 0)), None, None, ],
    [Some((11, 7, 4)), Some((11, 4, 2)), Some((8, 3, 4)), Some((3, 2, 4)), None, ],
    [Some((2, 9, 10)), Some((2, 7, 9)), Some((2, 3, 7)), Some((7, 4, 9)), None, ],
    [Some((9, 10, 7)), Some((9, 7, 4)), Some((10, 2, 7)), Some((8, 7, 0)), Some((2, 0, 7)), ],
    [Some((3, 7, 10)), Some((3, 10, 2)), Some((7, 4, 10)), Some((1, 10, 0)), Some((4, 0, 10)), ],
    [Some((1, 10, 2)), Some((8, 7, 4)), None, None, None, ],
    [Some((4, 9, 1)), Some((4, 1, 7)), Some((7, 1, 3)), None, None, ],
    [Some((4, 9, 1)), Some((4, 1, 7)), Some((0, 8, 1)), Some((8, 7, 1)), None, ],
    [Some((4, 0, 3)), Some((7, 4, 3)), None, None, None, ],
    [Some((4, 8, 7)), None, None, None, None, ],
    [Some((9, 10, 8)), Some((10, 11, 8)), None, None, None, ],
    [Some((3, 0, 9)), Some((3, 9, 11)), Some((11, 9, 10)), None, None, ],
    [Some((0, 1, 10)), Some((0, 10, 8)), Some((8, 10, 11)), None, None, ],
    [Some((3, 1, 10)), Some((11, 3, 10)), None, None, None, ],
    [Some((1, 2, 11)), Some((1, 11, 9)), Some((9, 11, 8)), None, None, ],
    [Some((3, 0, 9)), Some((3, 9, 11)), Some((1, 2, 9)), Some((2, 11, 9)), None, ],
    [Some((0, 2, 11)), Some((8, 0, 11)), None, None, None, ],
    [Some((3, 2, 11)), None, None, None, None, ],
    [Some((2, 3, 8)), Some((2, 8, 10)), Some((10, 8, 9)), None, None, ],
    [Some((9, 10, 2)), Some((0, 9, 2)), None, None, None, ],
    [Some((2, 3, 8)), Some((2, 8, 10)), Some((0, 1, 8)), Some((1, 10, 8)), None, ],
    [Some((1, 10, 2)), None, None, None, None, ],
    [Some((1, 3, 8)), Some((9, 1, 8)), None, None, None, ],
    [Some((0, 9, 1)), None, None, None, None, ],
    [Some((0, 3, 8)), None, None, None, None, ],
    [None, None, None, None, None, ],
];
