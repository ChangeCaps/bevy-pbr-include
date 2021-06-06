use bevy::{prelude::*, render::{pipeline::{PipelineDescriptor, RenderPipeline}, shader::{ShaderStage, ShaderStages}}};
use bevy_pbr_include::PbrIncludePlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PbrIncludePlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut shaders: ResMut<Assets<Shader>>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let vert = shaders.add(Shader::from_glsl(ShaderStage::Vertex, include_str!("simple.vert"))); 
    let frag = shaders.add(Shader::from_glsl(ShaderStage::Fragment, include_str!("simple.frag")));

    let pipeline = PipelineDescriptor::default_config(ShaderStages {
        vertex: vert,
        fragment: Some(frag),
    });

    let pipeline = pipelines.add(pipeline);

    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(shape::Icosphere::default().into()),
        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(pipeline)]),
        ..Default::default()
    });

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
        ..Default::default()
    });
}
