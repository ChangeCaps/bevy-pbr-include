use bevy::{prelude::*, render::{pipeline::{PipelineDescriptor, RenderPipeline}, shader::{ShaderStage, ShaderStages}}};
use bevy_pbr_include::PbrIncludePlugin;
use bevy_shadows::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PbrIncludePlugin)
        .add_plugin(ShadowPlugin::default())
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut shaders: ResMut<Assets<Shader>>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let vert = shaders.add(Shader::from_glsl(ShaderStage::Vertex, include_str!("simple.vert"))); 
    let frag = shaders.add(Shader::from_glsl(ShaderStage::Fragment, include_str!("simple.frag")));

    let pipeline = PipelineDescriptor::default_config(ShaderStages {
        vertex: vert,
        fragment: Some(frag),
    });

    let pipeline = pipelines.add(pipeline);

    commands.spawn()
        .insert(Transform::identity())
        .insert(GlobalTransform::identity())
        .insert(DirectionalLight::new(Color::WHITE, 32000.0, Vec3::new(1.0, -1.0, 1.0)));

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(shape::Icosphere::default().into()),
        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(pipeline.clone())]),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        material: materials.add(Color::DARK_GRAY.into()),
        transform: Transform::from_xyz(0.0, -1.5, 0.0),
        mesh: meshes.add(shape::Plane { size: 10.0 }.into()),
        ..Default::default()
    });

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(-5.0, 2.0, 2.0)).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
