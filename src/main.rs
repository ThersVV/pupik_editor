#![windows_subsystem = "windows"]

use bevy::prelude::*;
use bevy::window::*;
use bevy::winit::WinitWindows;
use bevy_egui::*;
use winit::window::Icon;

pub const CLEAR: Color = Color::rgb(0.75, 0.70, 1.);
pub const RESOLUTION: f32 = 1920. / 1080.;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Default, States)]
pub enum GameState {
    #[default]
    Building,
    Erasing,
}

#[derive(Resource)]
pub struct ExitSheet(pub Handle<TextureAtlas>);

#[derive(Resource)]
pub struct ExportSheet(pub Handle<TextureAtlas>);

#[derive(Resource)]
pub struct WhiteSheet(pub Handle<TextureAtlas>);

#[derive(Resource)]
pub struct EraserSheet(pub Handle<TextureAtlas>);

#[derive(Resource)]
pub struct CombinedSheet(pub Handle<TextureAtlas>);

#[derive(Resource)]
pub struct HolesSheet(pub Handle<TextureAtlas>);
#[derive(Resource)]
pub struct PlanesSheet(pub Handle<TextureAtlas>);
#[derive(Resource)]
pub struct PlanetSheet(pub Handle<TextureAtlas>);
#[derive(Resource)]
pub struct EnergySheet(pub Handle<TextureAtlas>);
#[derive(Resource)]
pub struct RainbowSheet(pub Handle<TextureAtlas>);
#[derive(Resource)]
pub struct LoveSheet(pub Handle<TextureAtlas>);

#[derive(Default, Resource)]
struct UiState {
    name: String,
    weight_s: String,
    ready_to_export: bool,
}

mod export;
mod mouse;
mod structure_ui;

use export::ExportPlugin;
use mouse::MousePlugin;
use structure_ui::StructureUIPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .add_state::<GameState>()
        .init_resource::<UiState>()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_linear())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "pupik".to_string(),
                        present_mode: PresentMode::Fifo,
                        position: WindowPosition::At(IVec2::new(100, 50)),
                        decorations: false,
                        resizable: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_systems(Startup, (set_window_icon, spawn_camera))
        .add_systems(PreStartup, load_all)
        .add_plugins(EguiPlugin)
        .add_plugins((StructureUIPlugin, MousePlugin, ExportPlugin))
        .run();
}

fn load_all(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    struct SheetInfo {
        name: &'static str,
        x: f32,
        y: f32,
        columns: usize,
        rows: usize,
        padding: Option<Vec2>,
        offset: Option<Vec2>,
    }
    impl SheetInfo {
        pub fn new(
            name: &'static str,
            x: f32,
            y: f32,
            columns: usize,
            rows: usize,
            padding: Option<Vec2>,
            offset: Option<Vec2>,
        ) -> SheetInfo {
            SheetInfo {
                name,
                x,
                y,
                columns,
                rows,
                padding,
                offset,
            }
        }
    }
    let init_arr = [
        SheetInfo::new("duha.png", 21., 1., 1, 1, None, None),
        SheetInfo::new("planet_sheet.png", 100., 100., 1, 1, None, None),
        SheetInfo::new("plane_sheet1.png", 322., 108., 1, 1, None, None),
        SheetInfo::new("energy_sheet.png", 243., 117., 1, 1, None, None),
        SheetInfo::new("blackhole_sheet.png", 223., 223., 1, 1, None, None),
        SheetInfo::new("lovesheet.png", 100., 100., 1, 1, None, None),
        SheetInfo::new("combined_sheet.png", 2254. / 7., 223., 7, 1, None, None),
        SheetInfo::new("eraser.png", 256., 256., 1, 1, None, None),
        SheetInfo::new("white_transparent.png", 1., 1., 1, 1, None, None),
        SheetInfo::new("export.png", 218., 218., 1, 1, None, None),
        SheetInfo::new("exit.png", 225., 225., 1, 1, None, None),
    ];
    for sheet in init_arr {
        let image = assets.load(sheet.name);
        let atlas = TextureAtlas::from_grid(
            image,
            Vec2::new(sheet.x, sheet.y),
            sheet.columns,
            sheet.rows,
            sheet.padding,
            sheet.offset,
        );

        let atlas_handle = texture_atlases.add(atlas);
        match sheet.name {
            "duha.png" => commands.insert_resource(RainbowSheet(atlas_handle)),
            "plane_sheet1.png" => commands.insert_resource(PlanesSheet(atlas_handle)),
            "energy_sheet.png" => commands.insert_resource(EnergySheet(atlas_handle)),
            "blackhole_sheet.png" => commands.insert_resource(HolesSheet(atlas_handle)),
            "planet_sheet.png" => commands.insert_resource(PlanetSheet(atlas_handle)),
            "lovesheet.png" => commands.insert_resource(LoveSheet(atlas_handle)),
            "combined_sheet.png" => commands.insert_resource(CombinedSheet(atlas_handle)),
            "eraser.png" => commands.insert_resource(EraserSheet(atlas_handle)),
            "white_transparent.png" => commands.insert_resource(WhiteSheet(atlas_handle)),
            "export.png" => commands.insert_resource(ExportSheet(atlas_handle)),
            "exit.png" => commands.insert_resource(ExitSheet(atlas_handle)),
            _ => {
                panic!("=============FILE NAME MISSING IN MAIN.RS MATCH EXPRESSION!=============");
            }
        };
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(0., 0., 1000.),
            ..default()
        },
        ..default()
    };

    camera.projection = OrthographicProjection {
        area: Rect::new(-1.0 * RESOLUTION, -1.0, 1.0 * RESOLUTION, 1.0), /*
                                                                         scaling_mode: ScalingMode::Fixed {
                                                                             width: 1. * RESOLUTION,
                                                                             height: 0.,
                                                                         }, */
        scale: 1.,
        ..Default::default()
    };

    commands.spawn(camera);
}

fn set_window_icon(
    main_window: Query<Entity, With<PrimaryWindow>>,
    windows: NonSend<WinitWindows>,
) {
    let Some(primary) = windows.get_window(main_window.single()) else {return};

    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("icon.ico")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    primary.set_window_icon(Some(icon));
}
