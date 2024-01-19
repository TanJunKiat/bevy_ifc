use std::env;
use std::fs;
use bevy::prelude::*;
use std::collections::HashMap;
use bevy_ifc::*;
use bevy_panorbit_camera::*;


#[derive(Default, Resource)]
pub struct IfcResource{
    pub item: HashMap<String, Vec<String>>,
}

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.8, 0.8, 0.8)))
    .add_plugins(DefaultPlugins)
    .add_plugins(PanOrbitCameraPlugin)
    .init_resource::<IfcResource>()
    .add_systems(Startup, spawn_camera)
    .add_systems(Startup, read_ifc)
    .add_systems(PostStartup, spawn_wall.after(read_ifc))
    .run();
}


fn spawn_camera(
    mut commands: Commands,
){
    eprintln!("Camera initialized.");

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-40.5, 40.5, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}

fn read_ifc(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){

    let current_path = get_current_working_dir();

    let contents = fs::read_to_string(current_path+"/samples/duplex.ifc")
        .expect("Should have been able to read the file");

    for content in contents.split("\n"){
        if content.contains("IFCDIRECTION("){
            spawn_ifc_direction_entity(&mut commands, content);
        }
        else if content.contains("IFCCARTESIANPOINT("){
            spawn_ifc_cartesian_point_entity(&mut commands, &mut meshes, &mut materials, content);
        }
        else if content.contains("IFCAXIS2PLACEMENT3D("){
            spawn_ifc_axis_2_placement_3d_entity(&mut commands, &mut meshes, &mut materials, content);
        }
        else if content.contains("IFCAXIS2PLACEMENT2D("){
            spawn_ifc_axis_2_placement_2d_entity(&mut commands, content);
        }
        else if content.contains("IFCLOCALPLACEMENT("){
            spawn_ifc_local_placement_entity(&mut commands, content);
        }
        else if content.contains("IFCPOLYLINE("){
            spawn_ifc_poly_line_entity(&mut commands, content);
        }
        else if content.contains("IFCPOLYLOOP("){
            spawn_ifc_poly_loop_entity(&mut commands, content);
        }
        else if content.contains("IFCFACE("){
            spawn_ifc_face_entity(&mut commands, content);
        }
        else if content.contains("IFCFACEOUTERBOUND("){
            spawn_ifc_face_outer_bound_entity(&mut commands, content);
        }
        else if content.contains("IFCSHAPEREPRESENTATION("){
            spawn_ifc_shape_representation_entity(&mut commands, content);
        }
        else if content.contains("IFCRECTANGLEPROFILEDEF("){
            spawn_ifc_rectangle_profile_def_entity(&mut commands, content);
        }
        else if content.contains("IFCPROPERTYSINGLEVALUE("){
            spawn_ifc_property_single_value_entity(&mut commands, content);
        }
        else if content.contains("IFCPROPERTYSET("){
            spawn_ifc_property_set_entity(&mut commands, content);
        }
        else if content.contains("IFCRELDEFINESBYPROPERTIES("){
            spawn_ifc_rel_defines_by_properties_entity(&mut commands, content);
        }
        else if content.contains("IFCEXTRUDEDAREASOLID("){
            spawn_ifc_extruded_area_solid_entity(&mut commands, content);
        }
        else if content.contains("IFCPLANE("){
            spawn_ifc_plane_entity(&mut commands, content);
        }
        else if content.contains("IFCCONNECTIONSURFACEGEOMETRY("){
            spawn_ifc_connecting_surface_geometry_entity(&mut commands, content);
        }
        else if content.contains("IFCCOMPOSITECURVESEGMENT("){
            spawn_ifc_composite_curve_segment_entity(&mut commands, content);
        }
        else if content.contains("IFCARBITRARYOPENPROFILEDEF("){
            spawn_ifc_arbitrary_open_profile_def_entity(&mut commands, content);
        }
        else if content.contains("IFCCONNECTEDFACESET("){
            spawn_ifc_connected_face_set_entity(&mut commands, content);
        }
        else if content.contains("IFCMAPPEDITEM("){
            spawn_ifc_mapped_item_entity(&mut commands, content);
        }
        else if content.contains("IFCWALLSTANDARDCASE("){
            spawn_ifc_wall_standard_case_entity(&mut commands, content);
        }
        else if content.contains("IFCCARTESIANTRANSFORMATIONOPERATOR3D("){
            spawn_ifc_cartesian_transformation_operator_3d_entity(&mut commands, content);
        }
        else if content.contains("IFCSTYLEDITEM("){
            spawn_ifc_styled_item_entity(&mut commands, content);
        }
        else if content.contains("IFCPRODUCTDEFINITIONSHAPE("){
            spawn_ifc_product_definition_shape_entity(&mut commands, content);
        }
        else if content.contains("IFCSURFACEOFLINEAREXTRUSION("){
            spawn_ifc_surface_of_linear_extrusion_entity(&mut commands, content);
        }
        else if content.contains("IFCPRESENTATIONSTYLEASSIGNMENT("){
            spawn_ifc_presentation_style_assigment_entity(&mut commands, content);
        }
        else if content.contains("IFCMATERIALLAYERSETUSAGE("){
            spawn_ifc_material_layer_set_usage_entity(&mut commands, content);
        }
        else if content.contains("IFCRELSPACEBOUNDARY("){
            spawn_ifc_rel_space_boundary_entity(&mut commands, content);
        }
        else if content.contains("IFCRELASSOCIATESMATERIAL("){
            spawn_ifc_rel_associates_material_entity(&mut commands, content);
        }
        else if content.contains("IFCREPRESENTATIONMAP("){
            spawn_ifc_representation_map_entity(&mut commands, content);
        }
        else if content.contains("IFCCIRCLE("){
            spawn_ifc_circle_entity(&mut commands, content);
        }
        else if content.contains("IFCTRIMMEDCURVE("){
            spawn_ifc_trimmed_curve_entity(&mut commands, content);
        }
        else if content.contains("IFCARBITRARYCLOSEDPROFILEDEF("){
            spawn_ifc_arbitrary_closed_profile_def_entity(&mut commands, content);
        }
        else if content.contains("IFCCURVESTYLEFONTPATTERN("){
            spawn_ifc_curve_style_font_pattern_entity(&mut commands, content);
        }
        else if content.contains("IFCCURVEBOUNDEDPLANE("){
            spawn_ifc_curve_bounded_plane_entity(&mut commands, content);
        }
        else if content.contains("IFCFACEBOUND("){
            spawn_ifc_face_bound_entity(&mut commands, content);
        }
        else if content.contains("IFCCOLOURRGB("){
            spawn_ifc_color_rgb_entity(&mut commands, content);
        }
        else if content.contains("IFCCOLOURRGB("){
            spawn_ifc_curve_style_entity(&mut commands, content);
        }
        else if content.contains("IFCMATERIALLAYER("){
            spawn_ifc_material_layer_entity(&mut commands, content);
        }
        else if content.contains("IFCSLAB("){
            spawn_ifc_slab_entity(&mut commands, content);
        }
        else if content.contains("IFCRELCONNECTSPATHELEMENTS("){
            spawn_ifc_rel_connects_path_element_entity(&mut commands, content);
        }
        else if content.contains("IFCFURNISHINGELEMENT("){
            spawn_ifc_furnishing_element_entity(&mut commands, content);
        }
        else if content.contains("IFCOPENINGELEMENT("){
            spawn_ifc_opening_element_entity(&mut commands, content);
        }
        else if content.contains("IFCRELVOIDSELEMENT("){
            spawn_ifc_rel_voids_element_entity(&mut commands, content);
        }
        else if content.contains("IFCCOMPOSITECURVE("){
            spawn_ifc_composite_curve_entity(&mut commands, content);
        }
        else if content.contains("IFCDOOR("){
            spawn_ifc_door_entity(&mut commands, content);
        }
        else if content.contains("IFCGEOMETRICSET("){
            spawn_ifc_geometric_set_entity(&mut commands, content);
        }
        else if content.contains("IFCRELFILLSELEMENT("){
            spawn_ifc_rel_fills_element_entity(&mut commands, content);
        }
        else if content.contains("IFCMATERIAL("){
            spawn_ifc_material_entity(&mut commands, content);
        }
        else if content.contains("IFCARBITRARYPROFILEDEFWITHVOIDS("){
            spawn_ifc_arbitrary_profile_def_with_voids_entity(&mut commands, content);
        }
        else if content.contains("IFCWINDOW("){
            spawn_ifc_window_entity(&mut commands, content);
        }
        else if content.contains("IFCFURNITURETYPE("){
            spawn_ifc_furniture_type_entity(&mut commands, content);
        }
        else if content.contains("IFCSPACE("){
            spawn_ifc_space_entity(&mut commands, content);
        }
        else if content.contains("IFCFACEBASEDSURFACEMODEL("){
            spawn_ifc_face_based_surface_model_entity(&mut commands, content);
        }
        else if content.contains("IFCSURFACESTYLERENDERING("){
            spawn_ifc_surface_style_rendering_entity(&mut commands, content);
        }
        else if content.contains("IFCSURFACESTYLE("){
            spawn_ifc_surface_style_entity(&mut commands, content);
        }
        else if content.contains("IFCWINDOWSTYLE("){
            spawn_ifc_window_style_entity(&mut commands, content);
        }
        else if content.contains("IFCCURVESTYLEFONT("){
            spawn_ifc_curve_style_font_entity(&mut commands, content);
        }
        else if content.contains("IFCRELDEFINESBYTYPE("){
            spawn_ifc_rel_defines_by_type_entity(&mut commands, content);
        }
        else if content.contains("IFCFILLAREASTYLEHATCHING("){
            spawn_ifc_fill_area_style_hatching_entity(&mut commands, content);
        }
        else if content.contains("IFCQUANTITYAREA("){
            spawn_ifc_quantity_area_entity(&mut commands, content);
        }
        else if content.contains("IFCELEMENTQUANTITY("){
            spawn_ifc_element_quantity_entity(&mut commands, content);
        }
        else if content.contains("IFCMATERIALDEFINITIONREPRESENTATION("){
            spawn_ifc_material_definition_representation_entity(&mut commands, content);
        }
        else if content.contains("IFCSTYLEDREPRESENTATION("){
            spawn_ifc_styled_representation_entity(&mut commands, content);
        }
        else if content.contains("IFCRELCONTAINEDINSPATIALSTRUCTURE("){
            spawn_ifc_rel_contained_in_spatial_structure_entity(&mut commands, content);
        }
        else if content.contains("IFCPRESENTATIONLAYERASSIGNMENT("){
            spawn_ifc_presentation_layer_assignment_entity(&mut commands, content);
        }
        else if content.contains("IFCCOVERING("){
            spawn_ifc_covering_entity(&mut commands, content);
        }
        else if content.contains("IFCFILLAREASTYLE("){
            spawn_ifc_fill_area_style_entity(&mut commands, content);
        }
        else if content.contains("IFCMATERIALLAYERSET("){
            spawn_ifc_material_layer_set_entity(&mut commands, content);
        }
        else if content.contains("IFCPOLYGONALBOUNDEDHALFSPACE("){
            spawn_ifc_polygonal_bounded_half_space_entity(&mut commands, content);
        }
        else if content.contains("IFCWINDOWLININGPROPERTIES("){
            spawn_ifc_window_lining_properties_entity(&mut commands, content);
        }
        else if content.contains("IFCBEAM("){
            spawn_ifc_beam_entity(&mut commands, content);
        }
        else if content.contains("IFCCIRCLEPROFILEDEF("){
            spawn_ifc_circle_profile_def_entity(&mut commands, content);
        }
        else if content.contains("IFCFOOTING("){
            spawn_ifc_footing_entity(&mut commands, content);
        }
        else if content.contains("IFCDOORSTYLE("){
            spawn_ifc_door_style_entity(&mut commands, content);
        }
        else if content.contains("IFCRELAGGREGATES("){
            spawn_ifc_rel_aggregates_entity(&mut commands, content);
        }
        else if content.contains("IFCBOOLEANCLIPPINGRESULT("){
            spawn_ifc_boolean_clipping_result_entity(&mut commands, content);
        }
        else if content.contains("IFCBUILDINGSTOREY("){
            spawn_ifc_building_storey_entity(&mut commands, content);
        }
        else if content.contains("IFCRAILING("){
            spawn_ifc_railing_entity(&mut commands, content);
        }
        else if content.contains("IFCDOORLININGPROPERTIES("){
            spawn_ifc_door_lining_properties_entity(&mut commands, content);
        }
        else if content.contains("IFCSTAIR("){
            spawn_ifc_stair_entity(&mut commands, content);
        }
        else if content.contains("IFCSTAIRFLIGHT("){
            // eprintln!("{}",content);
            spawn_ifc_stair_flight_entity(&mut commands, content);
        }
    }
}

fn get_current_working_dir() -> String 
{
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}

fn spawn_wall(
    ifc_wall_query: Query<&mut IfcWallStandardCase,With<IfcWallStandardCase>>,
    ifc_product_definition_shape_query: Query<&mut IfcProductDefinitionShape,With<IfcProductDefinitionShape>>,
    ifc_shape_representation_shape_query: Query<&mut IfcShapeRepresentation,With<IfcShapeRepresentation>>,
){
    for ifc_wall in ifc_wall_query.iter(){
        let ifc_product_definition_shape: IfcProductDefinitionShape = get_ifc_product_definition_shape(&ifc_product_definition_shape_query,&ifc_wall.ifc_product_definition_shape);
        for representation in ifc_product_definition_shape.ifc_representation{
            let ifc_representation = get_ifc_shape_representation_shape(&ifc_shape_representation_shape_query,&representation);

            if ifc_representation.ifc_label_1 == "'Body'".to_string(){
                eprintln!("{}",ifc_representation.idx);
            }
        }

    }
}
