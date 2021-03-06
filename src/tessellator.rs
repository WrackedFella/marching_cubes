
use mesh::Mesh;
use mesh::Triangle;

use std::collections::HashMap;
use cube::tessellate_corners;
use field::Field;

pub fn create_mesh(
    field: &Field,
    min_bound: &(f32, f32, f32),
    max_bound: &(f32, f32, f32),
) -> Mesh {
    let cube_size = (
        (max_bound.0 - min_bound.0) / (field.cube_count().0 as f32),
        (max_bound.1 - min_bound.1) / (field.cube_count().1 as f32),
        (max_bound.2 - min_bound.2) / (field.cube_count().2 as f32),
    );
    let mut verts = Vec::new();
    let mut tris = Vec::new();

    let mut edge_to_vert_map = HashMap::new();

    for z in 0..field.cube_count().2 {
        for y in 0..field.cube_count().1 {
            for x in 0..field.cube_count().0 {
                let (fx, fy, fz) = (x as f32, y as f32, z as f32);
                let c0 = (
                    min_bound.0 + fx * cube_size.0,
                    min_bound.1 + fy * cube_size.1,
                    min_bound.2 + fz * cube_size.2,
                );
                let c1 = (c0.0 + cube_size.0, c0.1 + cube_size.1, c0.2 + cube_size.2);
                let p = [
                    (c0.0, c0.1, c0.2),
                    (c1.0, c0.1, c0.2),
                    (c1.0, c0.1, c1.2),
                    (c0.0, c0.1, c1.2),
                    (c0.0, c1.1, c0.2),
                    (c1.0, c1.1, c0.2),
                    (c1.0, c1.1, c1.2),
                    (c0.0, c1.1, c1.2),
                ];
                let f = [
                    field.f(x, y, z),
                    field.f(x + 1, y, z),
                    field.f(x + 1, y, z + 1),
                    field.f(x, y, z + 1),
                    field.f(x, y + 1, z),
                    field.f(x + 1, y + 1, z),
                    field.f(x + 1, y + 1, z + 1),
                    field.f(x, y + 1, z + 1),
                ];
                let Mesh(cube_verts, cube_tris) = tessellate_corners(&p, &f);
                for Triangle(i0, i1, i2) in cube_tris {
                    let (e0, e1, e2) = (
                        grid_shared_edge_index(x, y, z, i0),
                        grid_shared_edge_index(x, y, z, i1),
                        grid_shared_edge_index(x, y, z, i2),
                    );
                    let v0 = if let Some(v) = edge_to_vert_map.get(&e0) {
                        *v
                    } else {
                        let v = verts.len();
                        edge_to_vert_map.insert(e0, v);
                        verts.push(cube_verts[i0].clone());
                        v
                    };
                    let v1 = if let Some(v) = edge_to_vert_map.get(&e1) {
                        *v
                    } else {
                        let v = verts.len();
                        edge_to_vert_map.insert(e1, v);
                        verts.push(cube_verts[i1].clone());
                        v
                    };
                    let v2 = if let Some(v) = edge_to_vert_map.get(&e2) {
                        *v
                    } else {
                        let v = verts.len();
                        edge_to_vert_map.insert(e2, v);
                        verts.push(cube_verts[i2].clone());
                        v
                    };

                    tris.push(Triangle(v0, v1, v2));
                }
            }
        }
    }
    Mesh(verts, tris)
}

fn grid_shared_edge_index(
    cube_x: usize,
    cube_y: usize,
    cube_z: usize,
    cube_edge: usize,
) -> (usize, usize, usize, usize) {
    match cube_edge {
        0 => (cube_x, cube_y, cube_z, 0),
        1 => (cube_x + 1, cube_y, cube_z, 2),
        2 => (cube_x, cube_y, cube_z + 1, 0),
        3 => (cube_x, cube_y, cube_z, 2),
        4 => (cube_x, cube_y + 1, cube_z, 0),
        5 => (cube_x + 1, cube_y + 1, cube_z, 2),
        6 => (cube_x, cube_y + 1, cube_z + 1, 0),
        7 => (cube_x, cube_y + 1, cube_z, 2),
        8 => (cube_x, cube_y, cube_z, 1),
        9 => (cube_x + 1, cube_y, cube_z, 1),
        10 => (cube_x + 1, cube_y, cube_z + 1, 1),
        11 => (cube_x, cube_y, cube_z + 1, 1),
        _ => panic!("Invalid cube edge: {}", cube_edge),
    }
    //(cube_x, cube_y, cube_z, cube_edge)
}

#[cfg(test)]
mod tests {
    use super::*;

    use mesh::Vertex;

    #[test]
    fn test_edge_index() {
        assert_eq!(
            grid_shared_edge_index(0, 0, 1, 0),
            grid_shared_edge_index(0, 0, 0, 2)
        );
        assert_eq!(
            grid_shared_edge_index(0, 1, 1, 0),
            grid_shared_edge_index(0, 0, 0, 6)
        );
        assert_eq!(
            grid_shared_edge_index(0, 1, 0, 0),
            grid_shared_edge_index(0, 0, 0, 4)
        );

        assert_eq!(
            grid_shared_edge_index(0, 0, 0, 1),
            grid_shared_edge_index(1, 0, 0, 3)
        );
        assert_eq!(
            grid_shared_edge_index(0, 1, 0, 1),
            grid_shared_edge_index(1, 0, 0, 7)
        );
        assert_eq!(
            grid_shared_edge_index(0, 1, 0, 1),
            grid_shared_edge_index(0, 0, 0, 5)
        );

        assert_eq!(
            grid_shared_edge_index(0, 0, 0, 2),
            grid_shared_edge_index(0, 0, 1, 0)
        );
        assert_eq!(
            grid_shared_edge_index(0, 1, 0, 2),
            grid_shared_edge_index(0, 0, 1, 4)
        );
        assert_eq!(
            grid_shared_edge_index(0, 1, 0, 2),
            grid_shared_edge_index(0, 0, 0, 6)
        );

        assert_eq!(
            grid_shared_edge_index(1, 0, 0, 3),
            grid_shared_edge_index(0, 0, 0, 1)
        );
        assert_eq!(
            grid_shared_edge_index(1, 1, 0, 3),
            grid_shared_edge_index(0, 0, 0, 5)
        );
        assert_eq!(
            grid_shared_edge_index(0, 1, 0, 3),
            grid_shared_edge_index(0, 0, 0, 7)
        );

        assert_eq!(
            grid_shared_edge_index(1, 0, 0, 8),
            grid_shared_edge_index(0, 0, 0, 9)
        );
        assert_eq!(
            grid_shared_edge_index(0, 0, 1, 8),
            grid_shared_edge_index(0, 0, 0, 11)
        );
        assert_eq!(
            grid_shared_edge_index(1, 0, 1, 8),
            grid_shared_edge_index(0, 0, 0, 10)
        );

        assert_eq!(
            grid_shared_edge_index(0, 0, 1, 9),
            grid_shared_edge_index(0, 0, 0, 10)
        );
        assert_eq!(
            grid_shared_edge_index(0, 0, 0, 9),
            grid_shared_edge_index(1, 0, 0, 8)
        );
        assert_eq!(
            grid_shared_edge_index(0, 0, 1, 9),
            grid_shared_edge_index(1, 0, 0, 11)
        );

        assert_eq!(
            grid_shared_edge_index(0, 0, 0, 10),
            grid_shared_edge_index(1, 0, 0, 11)
        );
        assert_eq!(
            grid_shared_edge_index(0, 0, 0, 10),
            grid_shared_edge_index(0, 0, 1, 9)
        );
        assert_eq!(
            grid_shared_edge_index(0, 0, 0, 10),
            grid_shared_edge_index(1, 0, 1, 8)
        );

        assert_eq!(
            grid_shared_edge_index(0, 0, 0, 11),
            grid_shared_edge_index(0, 0, 1, 8)
        );
        assert_eq!(
            grid_shared_edge_index(1, 0, 0, 11),
            grid_shared_edge_index(0, 0, 0, 10)
        );
        assert_eq!(
            grid_shared_edge_index(1, 0, 0, 11),
            grid_shared_edge_index(0, 0, 1, 9)
        );

    }

    #[test]
    fn test_sphere() {
        //let sfield = SphereField::new(0.98);
        let r = 0.98;

        let field_table = Field::from_closure(
            |x,y,z| (x*x+y*y+z*z).sqrt() - r,
            &(-1.0, -1.0, -1.0),
            &(1.0, 1.0, 1.0),
            &(50, 50, 50)
        );
        let mesh = create_mesh(&field_table, &(-1.0, -1.0, -1.0), &(1.0, 1.0, 1.0));

        assert_is_sphere(&mesh, r);
    }

    #[test]
    fn test_field() {
        let r = 1.0;
        let field_table = Field::from_closure(
            |x,y,z| (x*x+y*y+z*z).sqrt() - r,
            &(-1.0, -1.0, -1.0), &(1.0, 1.0, 1.0), &(2, 2, 2)
        );
        let mesh = create_mesh(&field_table, &(-1.0, -1.0, -1.0), &(1.0, 1.0, 1.0));

        assert_is_octahedron(&mesh, r);
    }

    #[test]
    fn test_precomputed() {
        let field = field_precomputed();
        let mesh = create_mesh(&field, &(-1.0, -1.0, -1.0), &(1.0, 1.0, 1.0));
        assert_is_octahedron(&mesh, 0.5);
    }

    fn assert_is_sphere(mesh: &Mesh, r: f32) {
        // All vertices are within radius r
        for vert in &mesh.0 {
            assert!((vert.0 * vert.0 + vert.1 * vert.1 + vert.2 * vert.2 - r * r).abs() < 0.001);
        }

        // All triangle normals point towards radius vec
        let mut fail = false;
        for tri in &mesh.1 {
            let v0 = &mesh.0[tri.0];
            let v1 = &mesh.0[tri.1];
            let v2 = &mesh.0[tri.2];
            let e0 = (v1.0 - v0.0, v1.1 - v0.1, v1.2 - v0.2);
            let e1 = (v2.0 - v0.0, v2.1 - v0.1, v2.2 - v0.2);
            let n = (
                e0.1 * e1.2 - e0.2 * e1.1,
                -(e0.0 * e1.2 - e1.0 * e0.2),
                e0.0 * e1.1 - e0.1 * e1.0,
            );
            let rv = (
                (v0.0 + v1.0 + v2.0) / 3.0,
                (v0.1 + v1.1 + v2.1) / 3.0,
                (v0.2 + v1.2 + v2.2) / 3.0,
            );
            let nd = (n.0 * n.0 + n.1 * n.1 + n.2 * n.2).sqrt();
            //let n = (n.0 / nd, n.1 / nd, n.2 / nd);
            let d = n.0 * rv.0 + n.1 * rv.1 + n.2 * rv.2;
            if nd / 2.0 > (2.0 / 50.0) * (2.0 / 50.0) {
                println!("AREA: {}", nd);
            }
            if d < 0.0 {
                println!("{}", d);
                fail = true;
            }
        }
        if fail {
            panic!("There was some triangles facing the wrong way.");
        }
    }

    fn assert_is_octahedron(mesh: &Mesh, r: f32) {
        assert_eq!(6, mesh.0.len());
        assert_eq!(8, mesh.1.len());
        assert_eq!(
            Some(&Vertex(-r, 0.0, 0.0)),
            mesh.0.iter().min_by(
                |&&Vertex(x0, _y0, _z0), &&Vertex(x1, _y1, _z1)| {
                    x0.partial_cmp(&x1).unwrap()
                },
            )
        );
        assert_eq!(
            Some(&Vertex(r, 0.0, 0.0)),
            mesh.0.iter().max_by(
                |&&Vertex(x0, _y0, _z0), &&Vertex(x1, _y1, _z1)| {
                    x0.partial_cmp(&x1).unwrap()
                },
            )
        );
        assert_eq!(
            Some(&Vertex(0.0, -r, 0.0)),
            mesh.0.iter().min_by(
                |&&Vertex(_x0, y0, _z0), &&Vertex(_x1, y1, _z1)| {
                    y0.partial_cmp(&y1).unwrap()
                },
            )
        );
        assert_eq!(
            Some(&Vertex(0.0, r, 0.0)),
            mesh.0.iter().max_by(
                |&&Vertex(_x0, y0, _z0), &&Vertex(_x1, y1, _z1)| {
                    y0.partial_cmp(&y1).unwrap()
                },
            )
        );
        assert_eq!(
            Some(&Vertex(0.0, 0.0, -r)),
            mesh.0.iter().min_by(
                |&&Vertex(_x0, _y0, z0), &&Vertex(_x1, _y1, z1)| {
                    z0.partial_cmp(&z1).unwrap()
                },
            )
        );
        assert_eq!(
            Some(&Vertex(0.0, 0.0, r)),
            mesh.0.iter().max_by(
                |&&Vertex(_x0, _y0, z0), &&Vertex(_x1, _y1, z1)| {
                    z0.partial_cmp(&z1).unwrap()
                },
            )
        );

    }


    fn field_precomputed() -> Field {
        let f = Field::from_vecs(vec![
            vec![
                vec![-1.0, -1.0, -1.0],
                vec![-1.0, -1.0, -1.0],
                vec![-1.0, -1.0, -1.0],
            ],
            vec![
                vec![-1.0, -1.0, -1.0],
                vec![-1.0, 1.0, -1.0],
                vec![-1.0, -1.0, -1.0],
            ],
            vec![
                vec![-1.0, -1.0, -1.0],
                vec![-1.0, -1.0, -1.0],
                vec![-1.0, -1.0, -1.0],
            ],
        ]);
        assert_eq!(f.corner_count(), (3, 3, 3));
        assert_eq!(f.cube_count(), (2, 2, 2));
        assert_eq!(f.f(0, 0, 0), -1.0);
        assert_eq!(f.f(1, 0, 0), -1.0);
        assert_eq!(f.f(2, 0, 0), -1.0);
        assert_eq!(f.f(0, 1, 0), -1.0);
        assert_eq!(f.f(1, 1, 0), -1.0);
        assert_eq!(f.f(2, 1, 0), -1.0);
        assert_eq!(f.f(0, 2, 0), -1.0);
        assert_eq!(f.f(1, 2, 0), -1.0);
        assert_eq!(f.f(2, 2, 0), -1.0);

        assert_eq!(f.f(0, 0, 1), -1.0);
        assert_eq!(f.f(1, 0, 1), -1.0);
        assert_eq!(f.f(2, 0, 1), -1.0);
        assert_eq!(f.f(0, 1, 1), -1.0);
        assert_eq!(f.f(1, 1, 1), 1.0);
        assert_eq!(f.f(2, 1, 1), -1.0);
        assert_eq!(f.f(0, 2, 1), -1.0);
        assert_eq!(f.f(1, 2, 1), -1.0);
        assert_eq!(f.f(2, 2, 1), -1.0);

        assert_eq!(f.f(0, 0, 2), -1.0);
        assert_eq!(f.f(1, 0, 2), -1.0);
        assert_eq!(f.f(2, 0, 2), -1.0);
        assert_eq!(f.f(0, 1, 2), -1.0);
        assert_eq!(f.f(1, 1, 2), -1.0);
        assert_eq!(f.f(2, 1, 2), -1.0);
        assert_eq!(f.f(0, 2, 2), -1.0);
        assert_eq!(f.f(1, 2, 2), -1.0);
        assert_eq!(f.f(2, 2, 2), -1.0);

        f
    }


}
