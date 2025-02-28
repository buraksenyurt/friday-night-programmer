use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .run();
}
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    // Zemin
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 0.7, 0.0),
            custom_size: Some(Vec2::new(800.0, 20.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -200.0, 0.0),
        RigidBody::Fixed, // Sabit olduğu belirtilir
        Collider::cuboid(400.0, 10.0), // Collider eklenir
        Friction::coefficient(0.1),     // Burada bir sürtüne değeri veriyoruz
    ));

    // Kırmızı renkli dairesel top
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(40.0, 40.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 400.0, 0.0),
        RigidBody::Dynamic, // Bu hareket eden bir nesne olduğundan Dynamic
        Collider::ball(20.0), // Collider boyutu yarıçap uzunluğu kadar
        Restitution::coefficient(1.5), // Yerden sekme için kullanılan değer
        Friction::coefficient(0.2), // Özellikle top yuvarlanırken işe yarayacak bir sürtünme değeri
        Damping {
            linear_damping: 0.1,  // Hava direnci olarak kullanılıyor
            angular_damping: 0.2, // Dönüşler için engelleme değeri
        },
        GravityScale(9.0), // Yerçekimi değeri
    ));

    // Mavi Renkli dairesel top
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(30.0, 30.0)),
            ..default()
        },
        Transform::from_xyz(250.0, 200.0, 0.0),
        RigidBody::Dynamic, // Bu da fizik kurallarına göre hareket edeceğinden Dynamic
        Collider::ball(15.0), // Yine yarıçap değeri kadar collider
        Restitution::coefficient(1.5), // Yerden sekme için bir değer
        Friction::coefficient(0.1), // Sürtünme değeri
        Damping {
            linear_damping: 0.1, // Hava direnci
            angular_damping: 0.2, // Dönmeye karşı direnç değeri
        },
        GravityScale(9.0),
        Velocity {
            linvel: Vec2::new(-50.0, -10.0), // Belli miktarda bir hız ile başlatıyoruz
            angvel: 5.0, // ve başlangıçta biraz kendi ekseninde dönmesini sağlıyoruz
        },
    ));
}