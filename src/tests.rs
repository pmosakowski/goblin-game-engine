#[test]
fn test_identity_matrix_creation() {
    use matrix::Matrix2d;
    let identity = Matrix2d::identity();

    assert_eq!(identity.data[0], [1.0, 0.0, 0.0]);
    assert_eq!(identity.data[1], [0.0, 1.0, 0.0]);
    assert_eq!(identity.data[2], [0.0, 0.0, 1.0]);
}

#[test]
fn test_matrix_multiplication() {
    use matrix::Matrix2d;
    let identity = Matrix2d::identity();
    let mut a = Matrix2d::new();
    a.data[0][1] = 2.0;
    a.data[1][2] = 4.0;
    a.data[2][1] = -5.0;

    let b = identity.multiply_m(&a);

    assert_eq!(a.data[0], b.data[0]);
    assert_eq!(a.data[1], b.data[1]);
    assert_eq!(a.data[2], b.data[2]);

    let c = identity * a;

    assert_eq!(a.data[0], c.data[0]);
    assert_eq!(a.data[1], c.data[1]);
    assert_eq!(a.data[2], c.data[2]);
}

#[test]
fn test_vector_creation () {
    use vector::Vector2d;
    let v1 = Vector2d::new(5.0, 10.0);

    assert_eq!(v1.get_pos(), (5.0, 10.0));
}

#[test]
fn test_matrix_vector_multiplication () {
    use matrix::Matrix2d;
    use vector::Vector2d;

    let identity = Matrix2d::identity();
    let v1 = Vector2d::new(-5.0, 3.0);

    let v2 = identity.multiply_v(&v1);

    assert_eq!(v1.get_pos(), v2.get_pos());
}

#[test]
fn test_matrix_rotation () {
    use matrix::{Matrix2d, PI};
    use vector::Vector2d;

    let rotation = Matrix2d::identity().rotate(PI/2.0);
    let v1 = Vector2d::new(10.0, 0.0);
    let v2 = rotation * v1;

    let (v2x, v2y) = v2.get_pos();

    assert!( (v2x - 0.0).abs() <= 0.000001 );
    assert!( (v2y - 10.0).abs() <= 0.000001 );
}

#[test]
fn test_matrix_translation () {
    use matrix::Matrix2d;
    use vector::Vector2d;

    let translation = Matrix2d::identity().translate(-2.0, 3.0);
    let v1 = Vector2d::new(5.0, 6.0);
    let v2 = translation * v1;

    assert_eq!(v2.get_pos(), (3.0,9.0));
}

#[test]
fn test_matrix_scaling () {
    use matrix::Matrix2d;
    use vector::Vector2d;
    let scale = Matrix2d::identity().scale(4.0, 2.0);
    let v1 = Vector2d::new(0.5, 0.5);
    let v2 = scale * v1;

    assert_eq!(v2.get_pos(), (2.0, 1.0));
}

#[test]
fn test_vector3d_creation () {
    use vector::Vector3d;
    let v1 = Vector3d::new(1.0, 1.0, 1.0);

    assert_eq!(v1.get_pos(), (1.0, 1.0, 1.0));
}

#[test]
fn test_matrix3d_identity_creation() {
    use matrix::Matrix3d;
    let identity = Matrix3d::identity();

    assert_eq!(identity.data[0], [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(identity.data[1], [0.0, 1.0, 0.0, 0.0]);
    assert_eq!(identity.data[2], [0.0, 0.0, 1.0, 0.0]);
    assert_eq!(identity.data[3], [0.0, 0.0, 0.0, 1.0]);
}

#[test]
fn test_matrix3d_multiplication() {
    use matrix::Matrix3d;
    let identity = Matrix3d::identity();

    let mut a = Matrix3d::new();
    a.data[0][1] = 2.0;
    a.data[1][2] = 4.0;
    a.data[2][1] = -5.0;

    let b = identity.multiply_m(&a);

    assert_eq!(a.data[0], b.data[0]);
    assert_eq!(a.data[1], b.data[1]);
    assert_eq!(a.data[2], b.data[2]);
    assert_eq!(a.data[3], b.data[3]);

    let c = identity * a;

    assert_eq!(a.data[0], c.data[0]);
    assert_eq!(a.data[1], c.data[1]);
    assert_eq!(a.data[2], c.data[2]);
    assert_eq!(a.data[3], c.data[3]);
}

#[test]
fn test_matrix3d_vector3d_multiplication () {
    use matrix::Matrix3d;
    use vector::Vector3d;

    let identity = Matrix3d::identity();
    let v1 = Vector3d::new(-5.0, 3.0, 1.0);

    let v2 = identity.multiply_v(&v1);
    assert_eq!(v1.get_pos(), v2.get_pos());

    let v3 = identity * v1;
    assert_eq!(v1.get_pos(), v3.get_pos());
}

#[test]
fn test_matrix3d_rotation () {
    use matrix::{Matrix3d, PI};
    use vector::Vector3d;

    let rotation = Matrix3d::identity().rotate(PI/2.0, PI/2.0, 0.0);
    let v1 = Vector3d::new(0.0, 0.0, 10.0);
    let v2 = rotation * v1;

    let (v2x, v2y, v2z) = v2.get_pos();
    let (x, y, z) = (10.0, 0.0, 0.0);

    assert!( (v2x - x).abs() <= 0.000001 );
    assert!( (v2y - y).abs() <= 0.000001 );
    assert!( (v2z - z).abs() <= 0.000001 );
}

#[test]
fn test_matrix3d_rotation_x () {
    use matrix::{Matrix3d, PI};
    use vector::Vector3d;

    let rotation = Matrix3d::identity().rotate_x(PI/2.0);
    let v1 = Vector3d::new(0.0, 10.0, 0.0);
    let v2 = rotation * v1;

    let (v2x, v2y, v2z) = v2.get_pos();
    let (x, y, z) = (0.0, 0.0, 10.0);

    assert!( (v2x - x).abs() <= 0.000001 );
    assert!( (v2y - y).abs() <= 0.000001 );
    assert!( (v2z - z).abs() <= 0.000001 );
}

#[test]
fn test_matrix3d_rotation_y () {
    use matrix::{Matrix3d, PI};
    use vector::Vector3d;

    let rotation = Matrix3d::identity().rotate_y(PI/2.0);
    let v1 = Vector3d::new(10.0, 0.0, 0.0);
    let v2 = rotation * v1;

    let (v2x, v2y, v2z) = v2.get_pos();
    let (x, y, z) = (0.0, 0.0, -10.0);

    assert!( (v2x - x).abs() <= 0.000001 );
    assert!( (v2y - y).abs() <= 0.000001 );
    assert!( (v2z - z).abs() <= 0.000001 );
}

#[test]
fn test_matrix3d_rotation_z () {
    use matrix::{Matrix3d, PI};
    use vector::Vector3d;

    let rotation = Matrix3d::identity().rotate_z(PI/2.0);
    let v1 = Vector3d::new(10.0, 10.0, 0.0);
    let v2 = rotation * v1;

    let (v2x, v2y, v2z) = v2.get_pos();
    let (x, y, z) = (-10.0, 10.0, 0.0);

    assert!( (v2x - x).abs() <= 0.000001 );
    assert!( (v2y - y).abs() <= 0.000001 );
    assert!( (v2z - z).abs() <= 0.000001 );
}
