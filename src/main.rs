use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*};
use std::f32;
use rand::Rng;

mod lib;
use lib::{Boid, BoidGroup};

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const CAMERA_SPEED: f32 = 50.0;

fn main() {
    let boids = BoidGroup::new();
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(boids)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: SCREEN_WIDTH,
                height: SCREEN_WIDTH,
                title: "BOIDS!!!".to_string(),
                resizable: true,
                ..Default::default()
            },
            ..default()
        }))
        .add_plugin(ShapePlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(setup_system)
        .add_system(update_boids)
        .add_system(camera_movement)
        .run();
}



fn camera_movement (
    keyboard_input: Res<Input<KeyCode>>,
    mut camera: Query<&mut Transform, With<Camera>>,
    mut boids: Query<(&mut Boid, &mut Transform, Without<Camera>)>,
    mut boid_group: ResMut<BoidGroup>
) {
    
    for mut transform in camera.iter_mut() {
        
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= CAMERA_SPEED;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += CAMERA_SPEED;
        }
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += CAMERA_SPEED;
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= CAMERA_SPEED;
        }

        if keyboard_input.pressed(KeyCode::X) {
            transform.scale.x += 0.1;
            transform.scale.y += 0.1;
        }

        if keyboard_input.pressed(KeyCode::Z) {
            transform.scale.x -= 0.1;
            transform.scale.y -= 0.1;
        }

        if keyboard_input.pressed(KeyCode::R) {
            for (mut boid, mut transform, _) in boids.iter_mut() {
                // please just fuking make the angles work properly
                boid.direction += 5.0;
                if boid.direction > 360.0 || boid.direction < -360.0 {
                    boid.direction = 0.0;
                }
                transform.rotation = Quat::from_rotation_z(boid.direction.to_radians());
             }
        }

        if keyboard_input.pressed(KeyCode::T) {
            for (mut boid, mut transform, _) in boids.iter_mut() {
                // please just fuking make the angles work properly
                boid.direction -= 5.0;
                if boid.direction > 360.0 || boid.direction < -360.0 {
                    boid.direction = 0.0;
                }
                transform.rotation = Quat::from_rotation_z(boid.direction.to_radians());
             }
        }

        if keyboard_input.just_pressed(KeyCode::F) {
            for (mut boid, mut transform, _) in boids.iter_mut() {
                boid.rotate_towards(0.0, 0.0);
                transform.rotation = Quat::from_rotation_z(boid.direction.to_radians());
             }
        }
    }

}
    

fn spawn_camera(mut commands: Commands) {
    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        }
    );
}

fn setup_system(mut commands: Commands, mut boid_group: ResMut<BoidGroup>) {
    
    for i in 0..3000 {
        let boid = shapes::RegularPolygon {
            sides: 3,
            feature: shapes::RegularPolygonFeature::Radius(10.0),
            ..shapes::RegularPolygon::default()
        };

        let random_coors = Vec3::new(rand::random::<f32>() * SCREEN_WIDTH * 4.0, rand::random::<f32>() * SCREEN_HEIGHT * 4.0, 0.0);
        let rotation = 0.0;
        let mut rng = rand::thread_rng();
        let random_velocity = Vec2::new(rng.gen_range(-5..=5) as f32, rng.gen_range(-5..=5) as f32);
        let boid_entity  = Boid::new(random_coors, rotation, random_velocity);

        boid_group.add_boid(boid_entity);

        commands.spawn(GeometryBuilder::build_as(
            &boid,
            DrawMode::Fill(
                FillMode { options: FillOptions::DEFAULT, color: Color::WHITE}
            ),
            Transform {
                translation: random_coors,
                rotation: Quat::from_rotation_z(rotation),
                ..Default::default()
            },
        ))
        // .with_children(
        //     |parent| {
        //         let rect_rotation: f32 = 150.0;
        //         parent.spawn(
        //             GeometryBuilder::build_as(
        //                 &shapes::Rectangle {
        //                     extents: Vec2::new(20.0, 1.0),
        //                     ..shapes::Rectangle::default()
        //                 },
        //                 DrawMode::Fill(
        //                     FillMode { options: FillOptions::DEFAULT, color: Color::RED}
        //                 ),
        //                 Transform {
        //                     translation: Vec3::new(6.0, -4.5, -1.0),
        //                     rotation: Quat::from_rotation_z(rect_rotation.to_radians()),
        //                     ..Default::default()
        //                 },
        //             )
        //         );
        //     }
        // )
        .insert(boid_entity);
    
    }
}

fn update_boids(
    time: Res<Time>, 
    mut boids: Query<(&mut Boid, &mut Transform)>,
    mut boid_group: ResMut<BoidGroup>
) {

    let mut rng = rand::thread_rng();
    let group_rotation_probability = rng.gen_range(0..=100);
    let random_angle = rand::thread_rng().gen_range(-25..25);

    for (mut boid, mut transform) in boids.iter_mut() {
        boid.update(boid_group.clone());
        transform.translation = boid.position;

        boid.check_bounds(SCREEN_WIDTH * 5.0, SCREEN_HEIGHT * 5.0);

        if group_rotation_probability > 95 {
            boid.direction += random_angle as f32;
            if boid.direction > 360.0 {
                boid.direction = 0.0;
            }
        }
        
        transform.rotation = Quat::from_rotation_z(boid.direction.to_radians());
    }

}