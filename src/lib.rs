
use bevy::ecs::system::Command;
use bevy::input::mouse::MouseWheel;
use bevy::window::PrimaryWindow;
use bevy::{
    prelude::*,
    render::render_resource::AsBindGroup,
    sprite::{Material2d, MaterialMesh2dBundle},
};

#[derive(Component)]
pub struct Draggable;
#[derive(Component)]
pub struct Dragged;

#[derive(Component)]
pub struct Dropped;

pub const THIN_LINE: Color = Color::rgba(0.5, 0.5, 0.5, 0.2);
pub const THICK_LINE: Color = Color::rgba(0.0, 0.0, 0.0, 1.0);

#[derive(Clone, Resource, Default)]
pub struct MaterialId(AssetId<InfinityGridMaterial2D>);

#[derive(Clone)]
struct MaterialIdUpdate(AssetId<InfinityGridMaterial2D>);

impl Command for MaterialIdUpdate {
    fn apply(self, world: &mut World) {
        let mut id = world.get_resource_or_insert_with(|| MaterialId::default());
        id.0 = self.0;
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct InfinityGridMaterial2D {
    #[uniform(0)]
    pub(crate) thin_color: Vec4,
    #[uniform(1)]
    pub(crate) thick_color: Vec4,
    #[uniform(2)]
    pub(crate) bg_color: Vec4,
    #[uniform(3)]
    pub(crate) size: Vec4,
    #[uniform(4)]
    pub(crate) pan: Vec4
}

impl Material2d for InfinityGridMaterial2D {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/infinity.wgsl".into()
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<InfinityGridMaterial2D>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single();
    let material = materials.add(InfinityGridMaterial2D {
        thick_color: THICK_LINE.rgba_to_vec4(),
        thin_color: THIN_LINE.rgba_to_vec4(),
        bg_color: Color::rgba(0., 0., 0., 0.6).rgba_to_vec4(),
        size: Vec4::new(window.width(), window.height(), 0., 0.),
        pan: Vec4::new(0., 0., 0., 0.)
    });

    commands.add(MaterialIdUpdate(material.id()));
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: bevy::sprite::Mesh2dHandle(
                meshes.add(Mesh::from(Rectangle::from_size(Vec2::new(500., 500.)))),
            ),
            material,
            transform: Transform::from_scale(Vec3::splat(128.)),
            ..default()
        },
        Draggable,
    ));
}

pub fn update_material(
    mut mouse_wheel: EventReader<MouseWheel>,
    // buttons: Res<ButtonInput<MouseButton>>,
    // mut mouse_cursor: EventReader<CursorMoved>,
    material_id: Res<MaterialId>,
    mut materials: ResMut<Assets<InfinityGridMaterial2D>>,
) {
    for ev in mouse_wheel.read() {
        match ev.unit {
            bevy::input::mouse::MouseScrollUnit::Pixel => {
                if let Some(prev) = materials.get(material_id.0) {
                    let mut mat = prev.clone();
                    mat.pan.x -= ev.x;
                    mat.pan.y -= ev.y;
                    materials.insert(material_id.0, mat);
                }
            }
            _ => {}
        }
    }
}

pub fn on_draggable(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    pressed: Query<Entity, With<Draggable>>,
    released: Query<Entity, With<Dragged>>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        if let Some(entity) = pressed.iter().next() {
            commands.entity(entity).insert(Dragged);
        }
    } else if mouse_button.just_released(MouseButton::Left) {
        for entity in released.iter() {
            commands.entity(entity).remove::<Dragged>();

            commands.entity(entity).insert(Dropped);
        }
    }
}

pub fn on_drag(
    mut mouse_cursor: EventReader<CursorMoved>,
    mut dragged: Query<(Entity, &mut Transform, &GlobalTransform), Added<Dragged>>,
) {
    for _ev in mouse_cursor.read() {
        if let Some((_entity, mut transform, _global_transform)) = dragged.iter_mut().next() {
            if let Some(delta) = _ev.delta {
                transform.translation.x -= delta.x;
                transform.translation.y -= delta.y;
            }
        }
    }
}
