use bevy::{
    prelude::*,
    render::{
        shader::{ShaderSource, ShaderStage},
        RenderStage,
    },
};

#[cfg(not(feature = "shadows"))]
const TEMPLATE: &str = include_str!("pbr_template.glsl");
#[cfg(feature = "shadows")]
const TEMPLATE: &str = include_str!("pbr_template_shadows.glsl");

pub struct PbrIncludePlugin;

impl Plugin for PbrIncludePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(RenderStage::RenderResource, pbr_include_system.system());
    }
}

fn pbr_include_system(
    mut shaders: ResMut<Assets<Shader>>,
    mut events: EventReader<AssetEvent<Shader>>,
) {
    for event in events.iter() {
        if let AssetEvent::Created { handle } = event {
            if let Some(shader) = shaders.get_mut(handle) {
                if let ShaderStage::Fragment = shader.stage {
                    if let ShaderSource::Glsl(glsl) = &mut shader.source {
                        *glsl = glsl.replace("#pbr", TEMPLATE);
                    }
                }
            }
        }
    }
}
