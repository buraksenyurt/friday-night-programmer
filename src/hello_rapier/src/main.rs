use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, apply_ball_animation)
        .run();
}
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d::default());

    let texture_handle: Handle<Image> = asset_server.load("Ball.png");
    let frame_size = UVec2::new(32, 32);
    let atlas_layout = TextureAtlasLayout::from_grid(frame_size, 4, 2, None, None);
    let atlas_layout_handle = texture_atlases.add(atlas_layout);

    // Zemin
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 0.7, 0.0),
            custom_size: Some(Vec2::new(800.0, 20.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -200.0, 0.0),
        RigidBody::Fixed,              // Sabit olduğu belirtilir
        Collider::cuboid(400.0, 10.0), // Collider eklenir
        Friction::coefficient(0.1),    // Burada bir sürtüne değeri veriyoruz
    ));

    // Ball.png' nin kullanıldığı animasyon efektli top
    commands.spawn((
        Sprite {
            image: texture_handle.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: atlas_layout_handle,
                index: 0,
            }),
            custom_size: Vec2::splat(64.0).into(),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 300.0, 0.0),
        RigidBody::Dynamic,
        Collider::ball(32.0),
        Restitution::coefficient(1.5),
        Friction::coefficient(0.2),
        Damping {
            linear_damping: 0.1,
            angular_damping: 0.2,
        },
        GravityScale(9.0),
        BallAnimation::default(),
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
            linear_damping: 0.1,  // Hava direnci
            angular_damping: 0.2, // Dönmeye karşı direnç değeri
        },
        GravityScale(9.0),
        Velocity {
            linvel: Vec2::new(-50.0, -10.0), // Belli miktarda bir hız ile başlatıyoruz
            angvel: 5.0, // ve başlangıçta biraz kendi ekseninde dönmesini sağlıyoruz
        },
    ));
}

#[derive(Component)]
struct BallAnimation {
    timer: Timer,
}

impl Default for BallAnimation {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.25, TimerMode::Repeating),
        }
    }
}

fn apply_ball_animation(time: Res<Time>, mut query: Query<(&mut BallAnimation, &mut Sprite)>) {
    for (mut animation, mut sprite) in &mut query.iter_mut() {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = (atlas.index + 1) % 8;
            }
        }
    }
}
