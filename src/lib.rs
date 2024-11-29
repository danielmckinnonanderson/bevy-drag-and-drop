use bevy::{
    input::common_conditions::{input_just_pressed, input_just_released},
    prelude::*,
    window::PrimaryWindow,
};

/// Marker component which designates a camera to use when updating the
/// cursor position resource.
#[derive(Component)]
pub struct DragAndDropMainCamera;

/// Marker component that indicates its entity can be dragged via the mouse.
/// The rectangle is its bounding box.
#[derive(Component)]
pub struct Draggable(pub Rectangle);

/// Marker component that indicates that its entity is being dragged by the mouse.
#[derive(Component)]
pub struct Dragging;

/// Resource which tracks the current position of the relevant drag & drop cursor.
#[derive(Resource, Default)]
pub struct DragAndDropCursorPosition(pub Vec2);

pub struct CursorDragAndDropPlugin;

impl Plugin for CursorDragAndDropPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DragAndDropCursorPosition>()
            .add_systems(Update, update_cursor_ui_position_sys)
            .add_systems(
                Update,
                begin_dragging_draggable_sys
                    .run_if(input_just_pressed(MouseButton::Left))
                    .after(update_cursor_ui_position_sys),
            )
            .add_systems(
                Update,
                translate_dragging_draggable_sys.after(begin_dragging_draggable_sys),
            )
            .add_systems(
                Update,
                end_dragging_motion_sys
                    .run_if(input_just_released(MouseButton::Left))
                    .after(translate_dragging_draggable_sys),
            );
    }
}

/// Every frame, update the cursor position resource with the current mouse position.
fn update_cursor_ui_position_sys(
    mut mouse_pos: ResMut<DragAndDropCursorPosition>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<DragAndDropMainCamera>>,
) {
    let (camera, xform) = camera_q.single();
    let window = window_q.single();

    if let Some(world_pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(xform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mouse_pos.0 = world_pos;
        debug!("Mouse world coords: {},{}", world_pos.x, world_pos.y);
    }
}

/// Check if the mouse position is over a Draggable entity.
/// If so, configure the dragging state to target this entity.
fn begin_dragging_draggable_sys(
    mut commands: Commands,
    mut draggables_q: Query<(&Transform, &Draggable, Entity)>,
    mouse_pos: Res<DragAndDropCursorPosition>,
) {
    // Iterate through query, checking each position against
    // the mouse position during the input event.
    // If the mouse is over ("within" in 2D space) an element,
    // add a `Dragging` component to that entity.
    draggables_q.iter_mut().for_each(|draggable| {
        let xlation = draggable.0.translation.xy();
        let draggable_rect: Rectangle = draggable.1 .0;
        if is_point_in_rectangle(mouse_pos.0, xlation, &draggable_rect) {
            commands.entity(draggable.2).insert(Dragging);
            info!("Added `Dragging` to {}", draggable.2);
        }
    });
}

/// Translate the `Dragging` entity according to the mouse position
fn translate_dragging_draggable_sys(
    mut dragging_q: Query<(Entity, &mut Transform), (With<Dragging>, With<Draggable>)>,
    mouse_pos: Res<DragAndDropCursorPosition>,
) {
    if let Ok(mut dragging) = dragging_q.get_single_mut() {
        dragging.1.translation.x = mouse_pos.0.x;
        dragging.1.translation.y = mouse_pos.0.y;
    }
}

/// Triggered when the player releases left-mouse button
/// while dragging a draggable entity, the `Dragging` component
/// is removed, causing the drag to end.
fn end_dragging_motion_sys(
    mut commands: Commands,
    dragging_q: Query<Entity, (With<Dragging>, With<Draggable>)>,
) {
    if let Ok(dragging_entity) = dragging_q.get_single() {
        commands.entity(dragging_entity).remove::<Dragging>();
        info!("Removed `Dragging` from {}", dragging_entity);
    }
}

// Utility fn to determine if a given point is within the bounds of a rect
fn is_point_in_rectangle(point: Vec2, rect_origin: Vec2, rectangle: &Rectangle) -> bool {
    // TODO - Consider using bit manipulation to get abs for better performance
    // Calculate the absolute distance between point and rectangle origin
    let abs_dist = Vec2 {
        x: (point.x - rect_origin.x).abs(),
        y: (point.y - rect_origin.y).abs(),
    };

    abs_dist.x <= rectangle.half_size.x && abs_dist.y <= rectangle.half_size.y
}
