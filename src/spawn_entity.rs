use bevy::prelude::*;

use crate::IfcDirection;
use crate::IfcCartesianPoint;
use crate::IfcAxis2Placement3D;
use crate::IfcAxis2Placement2D;
use crate::IfcLocalPlacement;
use crate::IfcPolyLine;
use crate::IfcPolyLoop;
use crate::IfcFace;
use crate::IfcShapeRepresentation;
use crate::IfcRectangleProfileDef;
use crate::IfcPropertySingleValue;
use crate::IfcFaceOuterBound;
use crate::IfcPropertySet;
use crate::IfcRelDefinesByProperties;
use crate::IfcExtrudedAreaSolid;
use crate::IfcPlane;
use crate::IfcConnectionSurfaceGeometry;
use crate::IfcCompositeCurveSegment;
use crate::IfcArbitraryOpenProfileDef;
use crate::IfcConnectedFaceSet;
use crate::IfcMappedItem;
use crate::IfcWallStandardCase;
use crate::IfcCartesianTransformationOperator3D;
use crate::IfcStyledItem;
use crate::IfcProductDefinitionShape;
use crate::IfcSurfaceOfLinearExtrusion;
use crate::IfcPresentationStyleAssignment;
use crate::IfcMaterialLayerSetUsage;
use crate::IfcRelSpaceBoundary;
use crate::IfcRelAssociatesMaterial;
use crate::IfcRepresentationMap;
use crate::IfcCircle;
use crate::IfcTrimmedCurve;
use crate::IfcArbitraryClosedProfileDef;
use crate::IfcCurveStyleFontPattern;
use crate::IfcCurveBoundedPlane;
use crate::IfcFaceBound;
use crate::IfcColourRgb;
use crate::IfcCurveStyle;
use crate::IfcMaterialLayer;
use crate::IfcSlab;
use crate::IfcRelConnectsPathElements;
use crate::IfcFurnishingElement;
use crate::IfcOpeningElement;
use crate::IfcRelVoidsElement;
use crate::IfcCompositeCurve;
use crate::IfcDoor;
use crate::IfcGeometricSet;
use crate::IfcRelFillsElement;
use crate::IfcMaterial;
use crate::IfcArbitraryProfileDefWithVoids;
use crate::IfcWindow;
use crate::IfcFurnitureType;
use crate::IfcSpace;
use crate::IfcFaceBasedSurfaceModel;
use crate::IfcSurfaceStyleRendering;
use crate::IfcSurfaceStyle;
use crate::IfcWindowStyle;
use crate::IfcCurveStyleFont;
use crate::IfcRelDefinesByType;
use crate::IfcFillAreaStyleHatching;
use crate::IfcQuantityArea;
use crate::IfcElementQuantity;
use crate::IfcMaterialDefinitionRepresentation;
use crate::IfcStyledRepresentation;
use crate::IfcRelContainedInSpatialStructure;
use crate::IfcPresentationLayerAssignment;
use crate::IfcCovering;
use crate::IfcFillAreaStyle;
use crate::IfcMaterialLayerSet;
use crate::IfcPolygonalBoundedHalfSpace;
use crate::IfcWindowLiningProperties;
use crate::IfcBeam;
use crate::IfcCircleProfileDef;
use crate::IfcFooting;
use crate::IfcDoorStyle;
use crate::IfcRelAggregates;
use crate::IfcBooleanClippingResult;
use crate::IfcBuildingStorey;
use crate::IfcRailing;
use crate::IfcDoorLiningProperties;
use crate::IfcStair;
use crate::IfcStairFlight;


pub fn spawn_ifc_direction_entity(
    commands: &mut Commands,
    content: &str,
){
    // get direction vector
    let left_trimmed = &content[content.find("((").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("));").unwrap_or(left_trimmed.len())];
    let mut coordinates: Vec<f32> = vec![0.0,0.0,0.0];
    let mut i = 0;
    for value in right_trimmed.split(","){
        coordinates[i] = value.parse::<f32>().unwrap();
        i = i + 1;
    }

    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let chars = right_trimmed_2.chars();

    // spawn direction entity
    commands.spawn((
        IfcDirection{
            idx: chars.as_str().parse::<String>().unwrap(),
            x: coordinates[0],
            y: coordinates[1],
            z: coordinates[2],
        },
    ));
}

pub fn spawn_ifc_cartesian_point_entity(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    content: &str,
){
    // get coordinates
    let left_trimmed = &content[content.find("((").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("));").unwrap_or(left_trimmed.len())];
    let mut coordinates: Vec<f32> = vec![0.0,0.0,0.0];
    let mut i = 0;
    for value in right_trimmed.split(","){
        coordinates[i] = value.parse::<f32>().unwrap();
        i = i + 1;
    }

    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // spawn direction entity
    commands.spawn((
        IfcCartesianPoint{
            idx: chars.as_str().parse::<String>().unwrap(),
            x: coordinates[0],
            y: coordinates[1],
            z: coordinates[2],
        },
        // PbrBundle {
        //     mesh: meshes.add(shape::UVSphere{
        //         radius: 0.1,
        //         ..default()
        //     }.into()),
        //     material: materials.add(Color::rgba(1.0,0.1,0.1,1.0).into()),
        //     transform: Transform {
        //         translation: Vec3::new(coordinates[0],coordinates[1],coordinates[2]),
        //         ..default()
        //     },
        //     ..default()
        // },
    ));
}

pub fn spawn_ifc_axis_2_placement_3d_entity(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    content: &str,
){
    // get coordinates
    let left_trimmed = &content[content.find("IFCAXIS2PLACEMENT3D(").unwrap_or(content.len())+20..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    let mut i = 0;
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
        i = i + 1;
    }

    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // eprintln!("{}",string_vec[0]);
    commands.spawn((
        IfcAxis2Placement3D{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_cartesian_point: string_vec[0].clone(),
            ifc_direction_1: string_vec[1].clone(),
            ifc_direction_2: string_vec[2].clone(),
        },
    ));
}

pub fn spawn_ifc_axis_2_placement_2d_entity(
    commands: &mut Commands,
    content: &str,
){
    // get coordinates
    let left_trimmed = &content[content.find("IFCAXIS2PLACEMENT2D(").unwrap_or(content.len())+20..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    let mut i = 0;
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
        i = i + 1;
    }

    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // eprintln!("{}",string_vec[0]);
    commands.spawn((
        IfcAxis2Placement2D{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_cartesian_point: string_vec[0].clone(),
            ifc_direction: string_vec[1].clone(),
        },
    ));
}

pub fn spawn_ifc_local_placement_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get coordinates
    let left_trimmed = &content[content.find("IFCLOCALPLACEMENT(").unwrap_or(content.len())+18..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    // eprintln!("{}",string_vec[0]);
    commands.spawn((
        IfcLocalPlacement{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_local_placement: string_vec[0].clone(),
            ifc_axis_2_placement_3d: string_vec[1].clone(),
        },
    ));
}

pub fn spawn_ifc_poly_line_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCPOLYLINE((").unwrap_or(content.len())+18..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("));").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcPolyLine{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_cartesian_points: string_vec.clone(),
        },
    ));
}

pub fn spawn_ifc_poly_loop_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCPOLYLOOP((").unwrap_or(content.len())+18..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("));").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcPolyLoop{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_cartesian_points: string_vec.clone(),
        },
    ));
}

pub fn spawn_ifc_face_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCFACE((").unwrap_or(content.len())+9..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("));").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    if right_trimmed.contains(","){
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }
    }
    else{
        string_vec.push(right_trimmed.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcFace{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_face_outer_bound: string_vec.clone(),
        },
    ));
}

pub fn spawn_ifc_shape_representation_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCSHAPEREPRESENTATION(").unwrap_or(content.len())+23..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("));").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    // get item vector
    let left_trimmed = &content[content.find(",(#").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("));").unwrap_or(left_trimmed.len())];
    let mut item_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        item_vec.push(value.parse::<String>().unwrap());
    }
    // eprintln!("{:?}",item_vec);

    commands.spawn((
        IfcShapeRepresentation{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_geometric_representation_context: string_vec[0].clone(),
            ifc_label_1: string_vec[1].clone(),
            ifc_label_2: string_vec[2].clone(),
            ifc_items: item_vec.clone(),
        },
    ));
}

pub fn spawn_ifc_rectangle_profile_def_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCRECTANGLEPROFILEDEF(").unwrap_or(content.len())+18..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }
    // eprintln!("{:?}",chars);
    commands.spawn((
        IfcRectangleProfileDef{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_profile_type_enum: string_vec[0].clone(),
            ifc_label: string_vec[1].clone(),
            ifc_axis_2_placement_2d: string_vec[2].clone(),
            x_dim: string_vec[3].clone().parse::<f32>().unwrap(),
            y_dim: string_vec[4].clone().parse::<f32>().unwrap(),
        },
    ));
}

pub fn spawn_ifc_property_single_value_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCPROPERTYSINGLEVALUE(").unwrap_or(content.len())+23..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }
    // eprintln!("{:?}",chars);
    commands.spawn((
        IfcPropertySingleValue{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_identifier: string_vec[0].clone(),
            ifc_text: string_vec[1].clone(),
            ifc_axis_2_placement_2d: string_vec[2].clone(),
            ifc_value: string_vec[3].clone(),
        },
    ));
}

pub fn spawn_ifc_face_outer_bound_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCFACEOUTERBOUND(").unwrap_or(content.len())+18..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }
    // eprintln!("{:?}",chars);
    commands.spawn((
        IfcFaceOuterBound{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_loop: string_vec[0].clone(),
            orientation: string_vec[1].clone(),
        },
    ));
}

pub fn spawn_ifc_property_set_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCPROPERTYSET(").unwrap_or(content.len())+15..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("));").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    // get item vector
    let left_trimmed = &content[content.find(",(#").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("));").unwrap_or(left_trimmed.len())];
    let mut item_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        item_vec.push(value.parse::<String>().unwrap());
    }
    // eprintln!("{:?}",item_vec);

    commands.spawn((
        IfcPropertySet{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_name: string_vec[2].clone(),
            ifc_description: string_vec[3].clone(),
            ifc_properties: item_vec.clone(),
        },
    ));
}

pub fn spawn_ifc_rel_defines_by_properties_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCRELDEFINESBYPROPERTIES(").unwrap_or(content.len())+26..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    // get item vector
    let left_trimmed = &content[content.find(",(#").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("),#").unwrap_or(left_trimmed.len())];
    let mut item_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        item_vec.push(value.parse::<String>().unwrap());
    }
    // eprintln!("{:?}",item_vec);

    commands.spawn((
        IfcRelDefinesByProperties{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_text: string_vec[3].clone(),
            ifc_objects: item_vec.clone(),
            ifc_property_set: string_vec[string_vec.len()-1].clone(),
        },
    ));
}

pub fn spawn_ifc_extruded_area_solid_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCEXTRUDEDAREASOLID(").unwrap_or(content.len())+21..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcExtrudedAreaSolid{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_profile_def: string_vec[0].clone(),
            ifc_axis_2_placement_3d: string_vec[1].clone(),
            ifc_direction: string_vec[2].clone(),
            length: string_vec[3].parse::<f32>().unwrap().clone(),
        },
    ));
}

pub fn spawn_ifc_plane_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get element
    let left_trimmed = &content[content.find("IFCPLANE(").unwrap_or(content.len())+9..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];

    commands.spawn((
        IfcPlane{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_axis_2_placement_3d: right_trimmed.parse::<String>().unwrap().clone(),
        },
    ));
}

pub fn spawn_ifc_connecting_surface_geometry_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCCONNECTIONSURFACEGEOMETRY(").unwrap_or(content.len())+29..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }
    commands.spawn((
        IfcConnectionSurfaceGeometry{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_surface_on_relating_element: string_vec[0].clone(),
            ifc_surface_on_related_element: string_vec[1].clone(),
        },
    ));
}

pub fn spawn_ifc_composite_curve_segment_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCCOMPOSITECURVESEGMENT(").unwrap_or(content.len())+25..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }
    commands.spawn((
        IfcCompositeCurveSegment{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_transition_code: string_vec[0].clone(),
            ifc_same_sense: string_vec[1].clone(),
            ifc_parent_curve: string_vec[2].clone(),
        },
    ));
}

pub fn spawn_ifc_arbitrary_open_profile_def_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCARBITRARYOPENPROFILEDEF(").unwrap_or(content.len())+27..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }
    commands.spawn((
        IfcArbitraryOpenProfileDef{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_profile_type_enum: string_vec[0].clone(),
            ifc_label: string_vec[1].clone(),
            ifc_bounded_curve: string_vec[2].clone(),
        },
    ));
}

pub fn spawn_ifc_connected_face_set_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCCONNECTEDFACESET((").unwrap_or(content.len())+21..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("));").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }
    commands.spawn((
        IfcConnectedFaceSet{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_faces: string_vec.clone(),
        },
    ));
}

pub fn spawn_ifc_mapped_item_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCMAPPEDITEM(").unwrap_or(content.len())+14..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }
    commands.spawn((
        IfcMappedItem{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_representation_map: string_vec[0].clone(),
            ifc_cartesian_transformation_operator_3d: string_vec[1].clone(),
        },
    ));
}

pub fn spawn_ifc_wall_standard_case_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCWALLSTANDARDCASE(").unwrap_or(content.len())+20..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }
    commands.spawn((
        IfcWallStandardCase{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_name: string_vec[3].clone(),
            ifc_product_definition: string_vec[4].clone(),
            ifc_object_placement: string_vec[5].clone(),
            ifc_product_definition_shape: string_vec[6].clone(),
            ifc_identifier: string_vec[7].clone(),
        },
    ));
}

pub fn spawn_ifc_cartesian_transformation_operator_3d_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCCARTESIANTRANSFORMATIONOPERATOR3D(").unwrap_or(content.len())+37..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }
    commands.spawn((
        IfcCartesianTransformationOperator3D{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_direction_1: string_vec[0].clone(),
            ifc_direction_2: string_vec[1].clone(),
            ifc_cartesian_point: string_vec[2].clone(),
            ifc_scale: string_vec[3].clone(),
            ifc_direction_3: string_vec[4].clone(),
        },
    ));
}

pub fn spawn_ifc_styled_item_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCSTYLEDITEM(").unwrap_or(content.len())+14..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    // get item vector
    let left_trimmed = &content[content.find(",(#").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("),").unwrap_or(left_trimmed.len())];
    let mut item_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        item_vec.push(value.parse::<String>().unwrap());
    }
    // eprintln!("{:?}",item_vec);

    commands.spawn((
        IfcStyledItem{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_representation_item: string_vec[0].clone(),
            ifc_styles: item_vec.clone(),
            ifc_label: string_vec[string_vec.len()-1].clone(),
        },
    ));
}

pub fn spawn_ifc_product_definition_shape_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCPRODUCTDEFINITIONSHAPE(").unwrap_or(content.len())+26..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    // get item vector
    let left_trimmed = &content[content.find(",(#").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("))").unwrap_or(left_trimmed.len())];
    let mut item_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        item_vec.push(value.parse::<String>().unwrap());
    }
    // eprintln!("{:?}",item_vec);

    commands.spawn((
        IfcProductDefinitionShape{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_label: string_vec[0].clone(),
            ifc_text: string_vec[1].clone(),
            ifc_representation: item_vec.clone(),
        },
    ));
}

pub fn spawn_ifc_surface_of_linear_extrusion_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCSURFACEOFLINEAREXTRUSION(").unwrap_or(content.len())+28..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcSurfaceOfLinearExtrusion{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_arbitrary_open_profile_def: string_vec[0].clone(),
            ifc_axis_2_placement_3d: string_vec[1].clone(),
            ifc_direction: string_vec[2].clone(),
            depth: string_vec[3].parse::<f32>().unwrap().clone(),
        },
    ));
}

pub fn spawn_ifc_presentation_style_assigment_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCPRESENTATIONSTYLEASSIGNMENT((").unwrap_or(content.len())+32..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("));").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcPresentationStyleAssignment{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_presentation_style_select: string_vec.clone(),
        },
    ));
}

pub fn spawn_ifc_material_layer_set_usage_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCMATERIALLAYERSETUSAGE(").unwrap_or(content.len())+25..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcMaterialLayerSetUsage{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_material_layer_set: string_vec[0].clone(),
            ifc_layer_set_direction_enum: string_vec[0].clone(),
            ifc_direction_sense_enum: string_vec[0].clone(),
            ifc_length_measure: string_vec[0].clone(),
        },
    ));
}

pub fn spawn_ifc_rel_space_boundary_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCRELSPACEBOUNDARY(").unwrap_or(content.len())+20..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcRelSpaceBoundary{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_text: string_vec[3].clone(),
            ifc_space: string_vec[4].clone(),
            ifc_element: string_vec[5].clone(),
            ifc_connection_geometry: string_vec[6].clone(),
            ifc_physical_or_virtual_enum: string_vec[7].clone(),
            ifc_internal_or_external_enum: string_vec[8].clone(),
        },
    ));
}

pub fn spawn_ifc_rel_associates_material_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCRELASSOCIATESMATERIAL(").unwrap_or(content.len())+25..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    // get item vector
    let left_trimmed = &content[content.find(",(#").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("),").unwrap_or(left_trimmed.len())];
    let mut item_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        item_vec.push(value.parse::<String>().unwrap());
    }
    // eprintln!("{:?}",item_vec);

    commands.spawn((
        IfcRelAssociatesMaterial{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_text: string_vec[3].clone(),
            ifc_root: item_vec.clone(),
            ifc_material_select: string_vec[string_vec.len()-1].clone(),
        },
    ));
}

pub fn spawn_ifc_representation_map_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCREPRESENTATIONMAP(").unwrap_or(content.len())+21..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcRepresentationMap{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_axis2_placement: string_vec[0].clone(),
            ifc_representation: string_vec[1].clone(),
        },
    ));
}

pub fn spawn_ifc_circle_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCCIRCLE(").unwrap_or(content.len())+10..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcCircle{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_axis2_placement: string_vec[0].clone(),
            radius: string_vec[1].parse::<f32>().unwrap().clone(),
        },
    ));
}

pub fn spawn_ifc_trimmed_curve_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCTRIMMEDCURVE(").unwrap_or(content.len())+16..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcTrimmedCurve{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_curve: string_vec[0].clone(),
            ifc_trim_1: string_vec[1].clone(),
            ifc_trim_2: string_vec[2].clone(),
            ifc_sense: string_vec[3].clone(),
            ifc_trimming_preference: string_vec[4].clone(),
        },
    ));
}

pub fn spawn_ifc_arbitrary_closed_profile_def_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCARBITRARYCLOSEDPROFILEDEF(").unwrap_or(content.len())+29..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }
    commands.spawn((
        IfcArbitraryClosedProfileDef{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_profile_type_enum: string_vec[0].clone(),
            ifc_label: string_vec[1].clone(),
            ifc_bounded_curve: string_vec[2].clone(),
        },
    ));
}

pub fn spawn_ifc_curve_style_font_pattern_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCCURVESTYLEFONTPATTERN(").unwrap_or(content.len())+25..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<f32> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<f32>().unwrap());
    }
    commands.spawn((
        IfcCurveStyleFontPattern{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_visible_segment_length: string_vec[0].clone(),
            ifc_invisible_segment_length: string_vec[1].clone(),
        },
    ));
}

pub fn spawn_ifc_curve_bounded_plane_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCCURVEBOUNDEDPLANE(").unwrap_or(content.len())+21..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    // get item vector
    let left_trimmed = &content[content.find(",(").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("));").unwrap_or(left_trimmed.len())];
    let mut item_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        item_vec.push(value.parse::<String>().unwrap());
    }
    // eprintln!("{:?}",item_vec);

    commands.spawn((
        IfcCurveBoundedPlane{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_plane: string_vec[0].clone(),
            ifc_outer_boundary: string_vec[1].clone(),
            ifc_inner_boundaries: item_vec.clone(),
        },
    ));
}

pub fn spawn_ifc_face_bound_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCFACEBOUND(").unwrap_or(content.len())+13..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcFaceBound{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_loop: string_vec[0].clone(),
            ifc_orientation: string_vec[1].clone(),
        },
    ));
}

pub fn spawn_ifc_color_rgb_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCCOLOURRGB(").unwrap_or(content.len())+13..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcColourRgb{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_label: string_vec[0].clone(),
            ifc_red: string_vec[1].parse::<f32>().unwrap().clone(),
            ifc_green: string_vec[2].parse::<f32>().unwrap().clone(),
            ifc_blue: string_vec[3].parse::<f32>().unwrap().clone(),
        },
    ));
}

pub fn spawn_ifc_curve_style_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCCURVESTYLE(").unwrap_or(content.len())+14..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcCurveStyle{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_label: string_vec[0].clone(),
            ifc_curve_font: string_vec[1].clone(),
            ifc_curve_width: string_vec[2].clone(),
            ifc_color: string_vec[3].clone(),
        },
    ));
}

pub fn spawn_ifc_material_layer_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCMATERIALLAYER(").unwrap_or(content.len())+17..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcMaterialLayer{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_material: string_vec[0].clone(),
            ifc_layer_thickness: string_vec[1].clone(),
            ifc_is_ventilated: string_vec[2].clone(),
        },
    ));
}

pub fn spawn_ifc_slab_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCSLAB(").unwrap_or(content.len())+8..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }
    commands.spawn((
        IfcSlab{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_name: string_vec[3].clone(),
            ifc_product_definition: string_vec[4].clone(),
            ifc_object_placement: string_vec[5].clone(),
            ifc_product_definition_shape: string_vec[6].clone(),
            ifc_identifier: string_vec[7].clone(),
        },
    ));
}

pub fn spawn_ifc_rel_connects_path_element_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCRELCONNECTSPATHELEMENTS(").unwrap_or(content.len())+27..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcRelConnectsPathElements{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_name: string_vec[3].clone(),
            ifc_element_1: string_vec[4].clone(),
            ifc_element_2: string_vec[5].clone(),
            ifc_relating_priorities: string_vec[6].clone(),
            ifc_related_priorities: string_vec[7].clone(),
            ifc_related_connection_type: string_vec[7].clone(),
            ifc_relating_connection_type: string_vec[7].clone(),
        },
    ));
}

pub fn spawn_ifc_furnishing_element_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCFURNISHINGELEMENT(").unwrap_or(content.len())+21..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }
    commands.spawn((
        IfcFurnishingElement{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_name: string_vec[3].clone(),
            ifc_product_label: string_vec[4].clone(),
            ifc_object_placement: string_vec[5].clone(),
            ifc_product_definition_shape: string_vec[6].clone(),
            ifc_identifier: string_vec[7].clone(),
        },
    ));
}

pub fn spawn_ifc_opening_element_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCOPENINGELEMENT(").unwrap_or(content.len())+18..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }
    commands.spawn((
        IfcOpeningElement{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_name: string_vec[3].clone(),
            ifc_product_label: string_vec[4].clone(),
            ifc_object_placement: string_vec[5].clone(),
            ifc_product_definition_shape: string_vec[6].clone(),
            ifc_identifier: string_vec[7].clone(),
        },
    ));
}

pub fn spawn_ifc_rel_voids_element_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCRELVOIDSELEMENT(").unwrap_or(content.len())+19..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }
    commands.spawn((
        IfcRelVoidsElement{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_name: string_vec[3].clone(),
            ifc_element: string_vec[4].clone(),
            ifc_feature_element_subtraction: string_vec[5].clone(),
        },
    ));
}

pub fn spawn_ifc_composite_curve_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCCOMPOSITECURVE(").unwrap_or(content.len())+18..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    // get item vector
    let left_trimmed = &content[content.find("((#").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("),").unwrap_or(left_trimmed.len())];
    let mut item_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        item_vec.push(value.parse::<String>().unwrap());
    }
    // eprintln!("{:?}",item_vec);

    commands.spawn((
        IfcCompositeCurve{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_segments: item_vec.clone(),
            ifc_is_closed: string_vec[string_vec.len()-1].clone(),
        },
    ));
}

pub fn spawn_ifc_door_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCDOOR(").unwrap_or(content.len())+8..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }
    commands.spawn((
        IfcDoor{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_name: string_vec[3].clone(),
            ifc_product_definition: string_vec[4].clone(),
            ifc_object_placement: string_vec[5].clone(),
            ifc_product_definition_shape: string_vec[6].clone(),
            ifc_identifier: string_vec[7].clone(),
        },
    ));
}

pub fn spawn_ifc_geometric_set_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCGEOMETRICSET((").unwrap_or(content.len())+17..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("));").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }
    commands.spawn((
        IfcGeometricSet{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_elements: string_vec.clone(),
        },
    ));
}

pub fn spawn_ifc_rel_fills_element_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCRELFILLSELEMENT(").unwrap_or(content.len())+19..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }
    commands.spawn((
        IfcRelFillsElement{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_name: string_vec[3].clone(),
            ifc_relating_opening_element: string_vec[4].clone(),
            ifc_related_building_element: string_vec[5].clone(),
        },
    ));
}

pub fn spawn_ifc_material_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCMATERIAL(").unwrap_or(content.len())+12..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];

    commands.spawn((
        IfcMaterial{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_label: right_trimmed.parse::<String>().unwrap().clone(),
        },
    ));
}

pub fn spawn_ifc_arbitrary_profile_def_with_voids_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCARBITRARYPROFILEDEFWITHVOIDS(").unwrap_or(content.len())+32..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    // get item vector
    let left_trimmed = &content[content.find(",(").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("));").unwrap_or(left_trimmed.len())];
    let mut item_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        item_vec.push(value.parse::<String>().unwrap());
    }
    // eprintln!("{:?}",item_vec);

    commands.spawn((
        IfcArbitraryProfileDefWithVoids{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_profile_type_enum: string_vec[0].clone(),
            ifc_label: string_vec[1].clone(),
            ifc_outer_curve: string_vec[2].clone(),
            ifc_inner_curve: item_vec.clone(),
        },
    ));
}

pub fn spawn_ifc_window_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCWINDOW(").unwrap_or(content.len())+10..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }
    commands.spawn((
        IfcWindow{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_name: string_vec[3].clone(),
            ifc_product_definition: string_vec[4].clone(),
            ifc_object_placement: string_vec[5].clone(),
            ifc_product_definition_shape: string_vec[6].clone(),
            ifc_identifier: string_vec[7].clone(),
            ifc_overall_height: string_vec[8].parse::<f32>().unwrap().clone(),
            ifc_overall_width: string_vec[9].parse::<f32>().unwrap().clone(),
        },
    ));
}

pub fn spawn_ifc_furniture_type_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCFURNITURETYPE(").unwrap_or(content.len())+17..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    // get item vector
    let left_trimmed = &content[content.find(",(#").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("),").unwrap_or(left_trimmed.len())];
    let mut item_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        item_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcFurnitureType{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_name: string_vec[3].clone(),
            ifc_applicable_occurrence: string_vec[4].clone(),
            ifc_property_sets: string_vec[5].clone(),
            ifc_representation_maps: item_vec.clone(),
            ifc_tag: string_vec[string_vec.len()-3].clone(),
            ifc_identifier: string_vec[string_vec.len()-2].clone(),
            ifc_assembly_place: string_vec[string_vec.len()-1].clone(),
        },
    ));
}

pub fn spawn_ifc_space_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCSPACE(").unwrap_or(content.len())+9..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcSpace{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_name: string_vec[3].clone(),
            ifc_product_definition: string_vec[4].clone(),
            ifc_object_placement: string_vec[5].clone(),
            ifc_product_definition_shape: string_vec[6].clone(),
            ifc_identifier: string_vec[7].clone(),
            ifc_composite_type: string_vec[8].clone(),
            ifc_interior_or_exterior_space: string_vec[9].clone(),
            ifc_elevation_with_flooring: string_vec[10].clone(),
        },
    ));
}

pub fn spawn_ifc_face_based_surface_model_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCFACEBASEDSURFACEMODEL((").unwrap_or(content.len())+26..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("));").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcFaceBasedSurfaceModel{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_connected_face_sets: string_vec.clone(),
        },
    ));
}

pub fn spawn_ifc_surface_style_rendering_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCSURFACESTYLERENDERING(").unwrap_or(content.len())+25..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcSurfaceStyleRendering{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_surface_colour: string_vec[0].clone(),
            ifc_transparency: string_vec[1].clone(),
            ifc_diffuse_colour: string_vec[2].clone(),
            ifc_transmission_colour: string_vec[3].clone(),
            ifc_diffuse_transmission_colour: string_vec[4].clone(),
            ifc_reflection_colour: string_vec[5].clone(),
            ifc_specular_colour: string_vec[6].clone(),
            ifc_specular_highlight: string_vec[7].clone(),
            ifc_reflectance_method: string_vec[8].clone(),
        },
    ));
}

pub fn spawn_ifc_surface_style_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCSURFACESTYLE(").unwrap_or(content.len())+16..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("));").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    // get item vector
    let left_trimmed = &content[content.find(",(").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("));").unwrap_or(left_trimmed.len())];
    let mut item_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        item_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcSurfaceStyle{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_name: string_vec[0].clone(),
            ifc_side: string_vec[1].clone(),
            ifc_styles: item_vec.clone(),
        },
    ));
}

pub fn spawn_ifc_window_style_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCWINDOWSTYLE(").unwrap_or(content.len())+15..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    // get item vector 1
    let left_trimmed = &content[content.find(",(").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("),(").unwrap_or(left_trimmed.len())];
    let mut item_vec_1: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        item_vec_1.push(value.parse::<String>().unwrap());
    }

    // get item vector 1
    let left_trimmed = &content[content.find("),(").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("),").unwrap_or(left_trimmed.len())];
    let mut item_vec_2: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        item_vec_2.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcWindowStyle{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[0].clone(),
            ifc_label: string_vec[0].clone(),
            ifc_name: string_vec[0].clone(),
        
            ifc_applicable_occurrence: string_vec[0].clone(),
            ifc_property_sets: item_vec_1.clone(),
        
            ifc_representation_maps: item_vec_2.clone(),
            ifc_identifier: string_vec[0].clone(),
        
            ifc_construction_type: string_vec[string_vec.len()-4].clone(),
            ifc_operation_type: string_vec[string_vec.len()-3].clone(),
            ifc_parameter_takes_precedence: string_vec[string_vec.len()-2].clone(),
            ifc_sizeable: string_vec[string_vec.len()-1].clone(),
        },
    ));
}

pub fn spawn_ifc_curve_style_font_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCCURVESTYLEFONT(").unwrap_or(content.len())+18..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    // get item vector 1
    let left_trimmed = &content[content.find(",(").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("));").unwrap_or(left_trimmed.len())];
    let mut item_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        item_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcCurveStyleFont{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_label: string_vec[0].clone(),
            ifc_pattern_list: item_vec.clone(),
        },
    ));
}

pub fn spawn_ifc_rel_defines_by_type_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCRELDEFINESBYTYPE(").unwrap_or(content.len())+20..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    // get item vector
    let left_trimmed = &content[content.find(",(").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("),").unwrap_or(left_trimmed.len())];
    let mut item_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        item_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcRelDefinesByType{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_name: string_vec[3].clone(),
            ifc_related_objects: item_vec.clone(),
            ifc_relating_type: string_vec[string_vec.len()-1].clone(),
        },
    ));
}

pub fn spawn_ifc_fill_area_style_hatching_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCFILLAREASTYLEHATCHING(").unwrap_or(content.len())+25..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcFillAreaStyleHatching{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_curve_style: string_vec[0].clone(),
            ifc_hatch_line_distance_select: string_vec[1].clone(),
            ifc_point_of_reference_hatch_line: string_vec[2].clone(),
            ifc_pattern_start: string_vec[3].clone(),
            ifc_hatch_line_angle: string_vec[4].clone(),
        },
    ));
}

pub fn spawn_ifc_quantity_area_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCQUANTITYAREA(").unwrap_or(content.len())+16..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcQuantityArea{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_name: string_vec[0].clone(),
            ifc_description: string_vec[1].clone(),
            ifc_unit: string_vec[2].clone(),
            ifc_area_value: string_vec[3].parse::<f32>().unwrap().clone(),
        },
    ));
}

pub fn spawn_ifc_element_quantity_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCELEMENTQUANTITY(").unwrap_or(content.len())+19..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("));").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    // get item vector
    let left_trimmed = &content[content.find(",(").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("));").unwrap_or(left_trimmed.len())];
    let mut item_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        item_vec.push(value.parse::<String>().unwrap());
    }
    // eprintln!("{:?}",item_vec);

    commands.spawn((
        IfcElementQuantity{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_name: string_vec[3].clone(),
            ifc_method_of_measurement: string_vec[4].clone(),
            ifc_quantities: item_vec.clone(),
        },
    ));
}

pub fn spawn_ifc_material_definition_representation_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCMATERIALDEFINITIONREPRESENTATION(").unwrap_or(content.len())+36..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    // get item vector
    let left_trimmed = &content[content.find(",(").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("));").unwrap_or(left_trimmed.len())];
    let mut item_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        item_vec.push(value.parse::<String>().unwrap());
    }
    // eprintln!("{:?}",item_vec);

    commands.spawn((
        IfcMaterialDefinitionRepresentation{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_name: string_vec[0].clone(),
            ifc_description: string_vec[1].clone(),
            ifc_representations: item_vec.clone(),
            ifc_represented_material: string_vec[string_vec.len()-1].clone(),
        },
    ));
}

pub fn spawn_ifc_styled_representation_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCSTYLEDREPRESENTATION(").unwrap_or(content.len())+24..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    // get item vector
    let left_trimmed = &content[content.find(",(").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("));").unwrap_or(left_trimmed.len())];
    let mut item_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        item_vec.push(value.parse::<String>().unwrap());
    }
    // eprintln!("{:?}",item_vec);

    commands.spawn((
        IfcStyledRepresentation{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_context_of_items: string_vec[0].clone(),
            ifc_representation_identifier: string_vec[1].clone(),
            ifc_representation_type: string_vec[2].clone(),
            ifc_items: item_vec.clone(),
        },
    ));
}

pub fn spawn_ifc_rel_contained_in_spatial_structure_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCRELCONTAINEDINSPATIALSTRUCTURE(").unwrap_or(content.len())+34..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    // get item vector
    let left_trimmed = &content[content.find(",(").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("),").unwrap_or(left_trimmed.len())];
    let mut item_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        item_vec.push(value.parse::<String>().unwrap());
    }
    // eprintln!("{:?}",item_vec);

    commands.spawn((
        IfcRelContainedInSpatialStructure{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_name: string_vec[3].clone(),
            ifc_related_elements: item_vec.clone(),
            ifc_relating_structure: string_vec[string_vec.len()-1].clone(),
        },
    ));
}

pub fn spawn_ifc_presentation_layer_assignment_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCPRESENTATIONLAYERASSIGNMENT(").unwrap_or(content.len())+31..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    // get item vector
    let left_trimmed = &content[content.find(",(").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("),").unwrap_or(left_trimmed.len())];
    let mut item_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        item_vec.push(value.parse::<String>().unwrap());
    }
    // eprintln!("{:?}",item_vec);

    commands.spawn((
        IfcPresentationLayerAssignment{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_label: string_vec[0].clone(),
            ifc_name: string_vec[1].clone(),
            ifc_related_elements: item_vec.clone(),
            ifc_identifier: string_vec[string_vec.len()-1].clone(),
        },
    ));
}

pub fn spawn_ifc_covering_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCCOVERING(").unwrap_or(content.len())+12..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcCovering{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_name: string_vec[3].clone(),
            ifc_object_type: string_vec[4].clone(),
            ifc_object_placement: string_vec[5].clone(),
            ifc_representation: string_vec[6].clone(),
            ifc_tag: string_vec[7].clone(),
            ifc_type: string_vec[8].clone(),
        },
    ));
}

pub fn spawn_ifc_fill_area_style_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCFILLAREASTYLE(").unwrap_or(content.len())+17..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    // get item vector
    let left_trimmed = &content[content.find(",(").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("));").unwrap_or(left_trimmed.len())];
    let mut item_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        item_vec.push(value.parse::<String>().unwrap());
    }
    // eprintln!("{:?}",item_vec);

    commands.spawn((
        IfcFillAreaStyle{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_name: string_vec[0].clone(),
            ifc_fill_styles: item_vec.clone(),
        },
    ));
}

pub fn spawn_ifc_material_layer_set_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCMATERIALLAYERSET(").unwrap_or(content.len())+20..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    // get item vector
    let left_trimmed = &content[content.find("((").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("),").unwrap_or(left_trimmed.len())];
    let mut item_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        item_vec.push(value.parse::<String>().unwrap());
    }
    // eprintln!("{:?}",item_vec);

    commands.spawn((
        IfcMaterialLayerSet{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_material_layers: item_vec.clone(),
            ifc_layer_set_name: string_vec[0].clone(),
        },
    ));
}

pub fn spawn_ifc_polygonal_bounded_half_space_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCPOLYGONALBOUNDEDHALFSPACE(").unwrap_or(content.len())+29..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcPolygonalBoundedHalfSpace{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_base_surface: string_vec[0].clone(),
            ifc_agreement_flag: string_vec[1].clone(),
            ifc_position: string_vec[2].clone(),
            ifc_polygonal_boundary: string_vec[3].clone(),
        },
    ));
}

pub fn spawn_ifc_window_lining_properties_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCWINDOWLININGPROPERTIES(").unwrap_or(content.len())+26..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcWindowLiningProperties{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_name: string_vec[3].clone(),
            ifc_lining_depth: string_vec[4].clone(),
            ifc_lining_thickness: string_vec[5].clone(),
            ifc_transom_thickness: string_vec[6].clone(),
            ifc_mullion_thickness: string_vec[7].clone(),
            ifc_first_transom_offset: string_vec[8].clone(),
            ifc_second_transom_offset: string_vec[9].clone(),
            ifc_first_mullion_offset: string_vec[10].clone(),
            ifc_second_mullion_offset: string_vec[11].clone(),
            ifc_shape_aspect_style: string_vec[12].clone(),
        },
    ));
}

pub fn spawn_ifc_beam_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCBEAM(").unwrap_or(content.len())+8..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcBeam{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_name: string_vec[3].clone(),
            ifc_object_type: string_vec[4].clone(),
            ifc_object_placement: string_vec[5].clone(),
            ifc_representation: string_vec[6].clone(),
            ifc_tag: string_vec[7].clone(),
        },
    ));
}

pub fn spawn_ifc_circle_profile_def_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCCIRCLEPROFILEDEF(").unwrap_or(content.len())+20..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcCircleProfileDef{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_profile_type: string_vec[0].clone(),
            ifc_profile_name: string_vec[1].clone(),
            ifc_position: string_vec[2].clone(),
            ifc_radius: string_vec[3].parse::<f32>().unwrap().clone(),
        },
    ));
}

pub fn spawn_ifc_footing_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCFOOTING(").unwrap_or(content.len())+11..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcFooting{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_name: string_vec[3].clone(),
            ifc_object_type: string_vec[4].clone(),
            ifc_object_placement: string_vec[5].clone(),
            ifc_representation: string_vec[6].clone(),
            ifc_tag: string_vec[7].clone(),
            ifc_predefined_type: string_vec[8].clone(),
        },
    ));
}

pub fn spawn_ifc_door_style_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCDOORSTYLE(").unwrap_or(content.len())+13..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    // get item vector 1
    let left_trimmed = &content[content.find(",(").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("),(").unwrap_or(left_trimmed.len())];
    let mut item_vec_1: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        item_vec_1.push(value.parse::<String>().unwrap());
    }

    // get item vector 1
    let left_trimmed = &content[content.find("),(").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("),").unwrap_or(left_trimmed.len())];
    let mut item_vec_2: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        item_vec_2.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcDoorStyle{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[0].clone(),
            ifc_label: string_vec[0].clone(),
            ifc_name: string_vec[0].clone(),
        
            ifc_applicable_occurrence: string_vec[0].clone(),
            ifc_property_sets: item_vec_1.clone(),
        
            ifc_representation_maps: item_vec_2.clone(),
            ifc_identifier: string_vec[0].clone(),
        
            ifc_construction_type: string_vec[string_vec.len()-4].clone(),
            ifc_operation_type: string_vec[string_vec.len()-3].clone(),
            ifc_parameter_takes_precedence: string_vec[string_vec.len()-2].clone(),
            ifc_sizeable: string_vec[string_vec.len()-1].clone(),
        },
    ));
}

pub fn spawn_ifc_rel_aggregates_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCRELAGGREGATES(").unwrap_or(content.len())+17..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("));").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    // get item vector
    let left_trimmed = &content[content.find(",(").unwrap_or(content.len())+2..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find("));").unwrap_or(left_trimmed.len())];
    let mut item_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        item_vec.push(value.parse::<String>().unwrap());
    }
    // eprintln!("{:?}",item_vec);

    commands.spawn((
        IfcRelAggregates{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_name: string_vec[3].clone(),
            ifc_relating_object: string_vec[4].clone(),
            ifc_related_objects: item_vec.clone(),
        },
    ));
}

pub fn spawn_ifc_boolean_clipping_result_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCBOOLEANCLIPPINGRESULT(").unwrap_or(content.len())+25..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcBooleanClippingResult{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_operator: string_vec[0].clone(),
            ifc_representation_item: string_vec[1].clone(),
            ifc_geometric_representation_item: string_vec[2].clone(),
        },
    ));
}

pub fn spawn_ifc_building_storey_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCBUILDINGSTOREY(").unwrap_or(content.len())+18..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }
    commands.spawn((
        IfcBuildingStorey{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_name: string_vec[3].clone(),
            ifc_object_type: string_vec[4].clone(),
            ifc_object_placement: string_vec[5].clone(),
            ifc_representation: string_vec[6].clone(),
            ifc_long_name: string_vec[7].clone(),
            ifc_composition_type: string_vec[8].clone(),
            ifc_elevation: string_vec[9].clone(),
        },
    ));
}






pub fn spawn_ifc_railing_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCRAILING(").unwrap_or(content.len())+11..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }
    commands.spawn((
        IfcRailing{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_name: string_vec[3].clone(),
            ifc_object_type: string_vec[4].clone(),
            ifc_object_placement: string_vec[5].clone(),
            ifc_representation: string_vec[6].clone(),
            ifc_tag: string_vec[7].clone(),
            ifc_predefined_type: string_vec[8].clone(),
        },
    ));
}

pub fn spawn_ifc_door_lining_properties_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCDOORLININGPROPERTIES(").unwrap_or(content.len())+24..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }

    commands.spawn((
        IfcDoorLiningProperties{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_name: string_vec[3].clone(),
            ifc_lining_depth: string_vec[4].clone(),
            ifc_lining_thickness: string_vec[5].clone(),
            ifc_threshold_depth: string_vec[6].clone(),
            ifc_threshold_thickness: string_vec[7].clone(),
            ifc_transom_thickness: string_vec[8].clone(),
            ifc_transom_offset: string_vec[9].clone(),
            ifc_lining_offset: string_vec[10].clone(),
            ifc_threshold_offset: string_vec[11].clone(),
            ifc_casing_thickness: string_vec[12].clone(),
            ifc_casing_depth: string_vec[13].clone(),
            ifc_shape_aspect_style: string_vec[14].clone(),
        },
    ));
}

pub fn spawn_ifc_stair_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCSTAIR(").unwrap_or(content.len())+9..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }
    commands.spawn((
        IfcStair{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_name: string_vec[3].clone(),
            ifc_product_definition: string_vec[4].clone(),
            ifc_object_placement: string_vec[5].clone(),
            ifc_product_definition_shape: string_vec[6].clone(),
            ifc_identifier: string_vec[7].clone(),
            ifc_shape_type: string_vec[8].clone(),
        },
    ));
}

pub fn spawn_ifc_stair_flight_entity(
    commands: &mut Commands,
    content: &str,
){
    // get index
    let right_trimmed_2 = &content[0..content.find("=").unwrap_or(content.len())];
    let mut chars = right_trimmed_2.chars();

    // get vector
    let left_trimmed = &content[content.find("IFCSTAIRFLIGHT(").unwrap_or(content.len())+15..content.len()];
    let right_trimmed = &left_trimmed[0..left_trimmed.find(");").unwrap_or(left_trimmed.len())];
    let mut string_vec: Vec<String> = vec![];
    for value in right_trimmed.split(","){
        string_vec.push(value.parse::<String>().unwrap());
    }
    commands.spawn((
        IfcStairFlight{
            idx: chars.as_str().parse::<String>().unwrap(),
            ifc_global_id: string_vec[0].clone(),
            ifc_owner_history: string_vec[1].clone(),
            ifc_label: string_vec[2].clone(),
            ifc_name: string_vec[3].clone(),
            ifc_product_definition: string_vec[4].clone(),
            ifc_object_placement: string_vec[5].clone(),
            ifc_product_definition_shape: string_vec[6].clone(),
            ifc_identifier: string_vec[7].clone(),
            ifc_number_of_riser: string_vec[8].clone(),
            ifc_number_of_treads: string_vec[9].clone(),
            ifc_riser_height: string_vec[10].clone(),
            ifc_tread_length: string_vec[11].clone(),
        },
    ));
}


