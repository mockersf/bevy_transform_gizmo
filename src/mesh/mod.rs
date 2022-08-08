use crate::{
    gizmo_material::GizmoMaterial, PickableGizmo, TransformGizmoBundle, TransformGizmoInteraction,
};
use bevy::{pbr::NotShadowCaster, prelude::*};
use bevy_mod_raycast::NoBackfaceCulling;

mod cone;
mod truncated_torus;

#[derive(Component)]
pub struct RotationGizmo;

#[derive(Component)]
pub struct ViewTranslateGizmo;

/// Startup system that builds the procedural mesh and materials of the gizmo.
pub fn build_gizmo(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GizmoMaterial>>,
) {
    let axis_length = 1.0;
    let arc_radius = 0.8;
    let plane_size = axis_length * 0.3;
    let plane_offset = plane_size / 2. + axis_length * 0.2;
    // Define gizmo meshes
    let arrow_tail_mesh = meshes.add(Mesh::from(shape::Capsule {
        radius: 0.02,
        depth: axis_length,
        ..Default::default()
    }));
    let cone_mesh = meshes.add(Mesh::from(cone::Cone {
        height: 0.2,
        radius: 0.09,
        ..Default::default()
    }));
    let plane_mesh = meshes.add(Mesh::from(shape::Plane { size: plane_size }));
    let sphere_mesh = meshes.add(Mesh::from(shape::Icosphere {
        radius: 0.1,
        subdivisions: 3,
    }));
    let rotation_mesh = meshes.add(Mesh::from(truncated_torus::TruncatedTorus {
        radius: arc_radius,
        ring_radius: 0.02,
        ..Default::default()
    }));
    //let cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 0.15 }));
    // Define gizmo materials
    let (s, l, a) = (0.8, 0.6, 0.8);
    let gizmo_matl_x = materials.add(GizmoMaterial::from(Color::hsla(0.0, s, l, a)));
    let gizmo_matl_y = materials.add(GizmoMaterial::from(Color::hsla(120.0, s, l, a)));
    let gizmo_matl_z = materials.add(GizmoMaterial::from(Color::hsla(240.0, s, l, a)));
    let gizmo_matl_x_sel = materials.add(GizmoMaterial::from(Color::hsl(0.0, s, l)));
    let gizmo_matl_y_sel = materials.add(GizmoMaterial::from(Color::hsl(120.0, s, l)));
    let gizmo_matl_z_sel = materials.add(GizmoMaterial::from(Color::hsl(240.0, s, l)));
    let gizmo_matl_v_sel = materials.add(GizmoMaterial::from(Color::hsl(0., 0.0, l)));
    /*let gizmo_matl_origin = materials.add(StandardMaterial {
        unlit: true,
        base_color: Color::rgb(0.7, 0.7, 0.7),
        ..Default::default()
    });*/
    // Build the gizmo using the variables above.
    commands
        .spawn_bundle(TransformGizmoBundle::default())
        .with_children(|parent| {
            // Translation Axes
            parent
                .spawn_bundle(MaterialMeshBundle {
                    mesh: arrow_tail_mesh.clone(),
                    material: gizmo_matl_x.clone(),
                    transform: Transform::from_matrix(Mat4::from_rotation_translation(
                        Quat::from_rotation_z(std::f32::consts::PI / 2.0),
                        Vec3::new(axis_length / 2.0, 0.0, 0.0),
                    )),
                    ..Default::default()
                })
                .insert(NotShadowCaster);
            parent
                .spawn_bundle(MaterialMeshBundle {
                    mesh: arrow_tail_mesh.clone(),
                    material: gizmo_matl_y.clone(),
                    transform: Transform::from_matrix(Mat4::from_rotation_translation(
                        Quat::from_rotation_y(std::f32::consts::PI / 2.0),
                        Vec3::new(0.0, axis_length / 2.0, 0.0),
                    )),
                    ..Default::default()
                })
                .insert(NotShadowCaster);
            parent
                .spawn_bundle(MaterialMeshBundle {
                    mesh: arrow_tail_mesh,
                    material: gizmo_matl_z.clone(),
                    transform: Transform::from_matrix(Mat4::from_rotation_translation(
                        Quat::from_rotation_x(std::f32::consts::PI / 2.0),
                        Vec3::new(0.0, 0.0, axis_length / 2.0),
                    )),
                    ..Default::default()
                })
                .insert(NotShadowCaster);

            // Translation Handles
            parent
                .spawn_bundle(MaterialMeshBundle {
                    mesh: cone_mesh.clone(),
                    material: gizmo_matl_x_sel.clone(),
                    transform: Transform::from_matrix(Mat4::from_rotation_translation(
                        Quat::from_rotation_z(std::f32::consts::PI / -2.0),
                        Vec3::new(axis_length, 0.0, 0.0),
                    )),
                    ..Default::default()
                })
                .insert(PickableGizmo::default())
                .insert(TransformGizmoInteraction::TranslateAxis {
                    original: Vec3::X,
                    axis: Vec3::X,
                })
                .insert(NotShadowCaster);
            parent
                .spawn_bundle(MaterialMeshBundle {
                    mesh: plane_mesh.clone(),
                    material: gizmo_matl_x_sel.clone(),
                    transform: Transform::from_matrix(Mat4::from_rotation_translation(
                        Quat::from_rotation_z(std::f32::consts::PI / -2.0),
                        Vec3::new(0., plane_offset, plane_offset),
                    )),
                    ..Default::default()
                })
                .insert(PickableGizmo::default())
                .insert(TransformGizmoInteraction::TranslatePlane {
                    original: Vec3::X,
                    normal: Vec3::X,
                })
                .insert(NoBackfaceCulling)
                .insert(NotShadowCaster);
            parent
                .spawn_bundle(MaterialMeshBundle {
                    mesh: cone_mesh.clone(),
                    material: gizmo_matl_y_sel.clone(),
                    transform: Transform::from_translation(Vec3::new(0.0, axis_length, 0.0)),
                    ..Default::default()
                })
                .insert(PickableGizmo::default())
                .insert(TransformGizmoInteraction::TranslateAxis {
                    original: Vec3::Y,
                    axis: Vec3::Y,
                })
                .insert(NotShadowCaster);
            parent
                .spawn_bundle(MaterialMeshBundle {
                    mesh: plane_mesh.clone(),
                    material: gizmo_matl_y_sel.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        plane_offset,
                        0.0,
                        plane_offset,
                    )),
                    ..Default::default()
                })
                .insert(PickableGizmo::default())
                .insert(TransformGizmoInteraction::TranslatePlane {
                    original: Vec3::Y,
                    normal: Vec3::Y,
                })
                .insert(NoBackfaceCulling)
                .insert(NotShadowCaster);
            parent
                .spawn_bundle(MaterialMeshBundle {
                    mesh: cone_mesh.clone(),
                    material: gizmo_matl_z_sel.clone(),
                    transform: Transform::from_matrix(Mat4::from_rotation_translation(
                        Quat::from_rotation_x(std::f32::consts::PI / 2.0),
                        Vec3::new(0.0, 0.0, axis_length),
                    )),
                    ..Default::default()
                })
                .insert(PickableGizmo::default())
                .insert(TransformGizmoInteraction::TranslateAxis {
                    original: Vec3::Z,
                    axis: Vec3::Z,
                })
                .insert(NotShadowCaster);
            parent
                .spawn_bundle(MaterialMeshBundle {
                    mesh: plane_mesh.clone(),
                    material: gizmo_matl_z_sel.clone(),
                    transform: Transform::from_matrix(Mat4::from_rotation_translation(
                        Quat::from_rotation_x(std::f32::consts::PI / 2.0),
                        Vec3::new(plane_offset, plane_offset, 0.0),
                    )),
                    ..Default::default()
                })
                .insert(PickableGizmo::default())
                .insert(TransformGizmoInteraction::TranslatePlane {
                    original: Vec3::Z,
                    normal: Vec3::Z,
                })
                .insert(NoBackfaceCulling)
                .insert(NotShadowCaster);

            parent
                .spawn_bundle(MaterialMeshBundle {
                    mesh: plane_mesh.clone(),
                    material: gizmo_matl_v_sel.clone(),
                    ..Default::default()
                })
                .insert(PickableGizmo::default())
                .insert(TransformGizmoInteraction::TranslatePlane {
                    original: Vec3::ZERO,
                    normal: Vec3::Z,
                })
                .insert(ViewTranslateGizmo)
                .insert(NotShadowCaster);

            // Rotation Arcs
            parent
                .spawn_bundle(MaterialMeshBundle {
                    mesh: rotation_mesh.clone(),
                    material: gizmo_matl_x.clone(),
                    transform: Transform::from_rotation(Quat::from_axis_angle(
                        Vec3::Z,
                        f32::to_radians(90.0),
                    )),
                    ..Default::default()
                })
                .insert(RotationGizmo)
                .insert(NotShadowCaster);
            parent
                .spawn_bundle(MaterialMeshBundle {
                    mesh: rotation_mesh.clone(),
                    material: gizmo_matl_y.clone(),
                    ..Default::default()
                })
                .insert(RotationGizmo)
                .insert(NotShadowCaster);
            parent
                .spawn_bundle(MaterialMeshBundle {
                    mesh: rotation_mesh.clone(),
                    material: gizmo_matl_z.clone(),
                    transform: Transform::from_rotation(
                        Quat::from_axis_angle(Vec3::Z, f32::to_radians(90.0))
                            * Quat::from_axis_angle(Vec3::X, f32::to_radians(90.0)),
                    ),
                    ..Default::default()
                })
                .insert(RotationGizmo)
                .insert(NotShadowCaster);

            // Rotation Handles
            parent
                .spawn_bundle(MaterialMeshBundle {
                    mesh: sphere_mesh.clone(),
                    material: gizmo_matl_x_sel.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        0.0,
                        f32::to_radians(45.0).sin() * arc_radius,
                        f32::to_radians(45.0).sin() * arc_radius,
                    )),
                    ..Default::default()
                })
                .insert(PickableGizmo::default())
                .insert(TransformGizmoInteraction::RotateAxis {
                    original: Vec3::X,
                    axis: Vec3::X,
                })
                .insert(RotationGizmo)
                .insert(NotShadowCaster);
            parent
                .spawn_bundle(MaterialMeshBundle {
                    mesh: sphere_mesh.clone(),
                    material: gizmo_matl_y_sel.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        f32::to_radians(45.0).sin() * arc_radius,
                        0.0,
                        f32::to_radians(45.0).sin() * arc_radius,
                    )),
                    ..Default::default()
                })
                .insert(PickableGizmo::default())
                .insert(TransformGizmoInteraction::RotateAxis {
                    original: Vec3::Y,
                    axis: Vec3::Y,
                })
                .insert(RotationGizmo)
                .insert(NotShadowCaster);
            parent
                .spawn_bundle(MaterialMeshBundle {
                    mesh: sphere_mesh.clone(),
                    material: gizmo_matl_z_sel.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        f32::to_radians(45.0).sin() * arc_radius,
                        f32::to_radians(45.0).sin() * arc_radius,
                        0.0,
                    )),
                    ..Default::default()
                })
                .insert(PickableGizmo::default())
                .insert(TransformGizmoInteraction::RotateAxis {
                    original: Vec3::Z,
                    axis: Vec3::Z,
                })
                .insert(RotationGizmo)
                .insert(NotShadowCaster);
        });
}
