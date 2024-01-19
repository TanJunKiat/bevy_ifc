use bevy::prelude::*;

#[derive(Default, Component, Clone)]
pub struct IfcDirection{
    pub idx: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Default, Component, Clone)]
pub struct IfcCartesianPoint{
    pub idx: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Default, Component, Clone)]
pub struct IfcAxis2Placement3D{
    pub idx: String,
    pub ifc_cartesian_point: String,
    pub ifc_direction_1: String,
    pub ifc_direction_2: String,
}

#[derive(Default, Component, Clone)]
pub struct IfcAxis2Placement2D{
    pub idx: String,
    pub ifc_cartesian_point: String,
    pub ifc_direction: String,
}
// #55=IFCAXIS2PLACEMENT2D(#54,#11);

#[derive(Default, Component, Clone)]
pub struct IfcLocalPlacement{
    pub idx: String,
    pub ifc_local_placement: String,
    pub ifc_axis_2_placement_3d: String,
}
// #1917=IFCLOCALPLACEMENT(#38,#1916);

#[derive(Default, Component, Clone)]
pub struct IfcPolyLine{
    pub idx: String,
    pub ifc_cartesian_points: Vec<String>,
}
// #2498=IFCPOLYLINE((#2493,#2494,#2495,#2496,#2497,#2493));

#[derive(Default, Component, Clone)]
pub struct IfcPolyLoop{
    pub idx: String,
    pub ifc_cartesian_points: Vec<String>,
}
// #2405=IFCPOLYLOOP((#2389,#2390,#2329,#2330));

#[derive(Default, Component, Clone)]
pub struct IfcFace{
    pub idx: String,
    pub ifc_face_outer_bound: Vec<String>,
}
// #5371=IFCFACE((#5370));

#[derive(Default, Component, Clone)]
pub struct IfcFaceOuterBound{
    pub idx: String,
    pub ifc_loop: String,
    pub orientation: String,
}
// #813=IFCFACEOUTERBOUND(#812,.T.);

#[derive(Default, Component, Clone)]
pub struct IfcShapeRepresentation{
    pub idx: String,
    pub ifc_geometric_representation_context: String,
    pub ifc_label_1: String,
    pub ifc_label_2: String,
    pub ifc_items: Vec<String>,
}
// #6362=IFCSHAPEREPRESENTATION(#27,'Body','SweptSolid',(#6299,#6320,#6341,#6359));

#[derive(Default, Component, Clone)]
pub struct IfcRectangleProfileDef{
    pub idx: String,
    pub ifc_profile_type_enum: String,
    pub ifc_label: String,
    pub ifc_axis_2_placement_2d: String,
    pub x_dim: f32,
    pub y_dim: f32,
}
// #1774=IFCRECTANGLEPROFILEDEF(.AREA.,$,#1773,5.808999999999987,2.229999999999981);

#[derive(Default, Component, Clone)]
pub struct IfcPropertySingleValue{
    pub idx: String,
    pub ifc_identifier: String,
    pub ifc_text: String,
    pub ifc_axis_2_placement_2d: String,
    pub ifc_value: String,
}
// #1586=IFCPROPERTYSINGLEVALUE('Area',$,IFCAREAMEASURE(26.11931424999996),$);

#[derive(Default, Component, Clone)]
pub struct IfcPropertySet{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_name: String,
    pub ifc_description: String,
    pub ifc_properties: Vec<String>,
}
// #6134=IFCPROPERTYSET('0iU8CzkZD7IeHfebiIsch6',#33,'Pset_SlabCommon',$,(#6133,#4133,#4645));

#[derive(Default, Component, Clone)]
pub struct IfcRelDefinesByProperties{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_text: String,
    pub ifc_objects: Vec<String>,
    pub ifc_property_set: String,
}
// #38662=IFCRELDEFINESBYPROPERTIES('20J_fddU9DWwKQcjOo9U02',#33,$,$,(#24596,#24680,#24768,#24813,#31431,#31476),#24660);

#[derive(Default, Component, Clone)]
pub struct IfcExtrudedAreaSolid{
    pub idx: String,
    pub ifc_profile_def: String,
    pub ifc_axis_2_placement_3d: String,
    pub ifc_direction: String,
    pub length: f32,
}
// #9394=IFCEXTRUDEDAREASOLID(#9391,#9393,#9,1.060000000000001);

#[derive(Default, Component, Clone)]
pub struct IfcPlane{
    pub idx: String,
    pub ifc_axis_2_placement_3d: String,
}
// #226=IFCPLANE(#225);

#[derive(Default, Component, Clone)]
pub struct IfcConnectionSurfaceGeometry{
    pub idx: String,
    pub ifc_surface_on_relating_element: String,
    pub ifc_surface_on_related_element: String,
}
// #657=IFCCONNECTIONSURFACEGEOMETRY(#656,$);

#[derive(Default, Component, Clone)]
pub struct IfcCompositeCurveSegment{
    pub idx: String,
    pub ifc_transition_code: String,
    pub ifc_same_sense: String,
    pub ifc_parent_curve: String,
}
// #31923=IFCCOMPOSITECURVESEGMENT(.CONTINUOUS.,.T.,#31922);

#[derive(Default, Component, Clone)]
pub struct IfcArbitraryOpenProfileDef{
    pub idx: String,
    pub ifc_profile_type_enum: String,
    pub ifc_label: String,
    pub ifc_bounded_curve: String,
}
// #2504=IFCARBITRARYOPENPROFILEDEF(.CURVE.,$,#2503);

#[derive(Default, Component, Clone)]
pub struct IfcConnectedFaceSet{
    pub idx: String,
    pub ifc_faces: Vec<String>,
}
// #10025=IFCCONNECTEDFACESET((#10005,#10010,#10015,#10018,#10021,#10024)); 

#[derive(Default, Component, Clone)]
pub struct IfcMappedItem{
    pub idx: String,
    pub ifc_representation_map: String,
    pub ifc_cartesian_transformation_operator_3d: String,
}
// #16632=IFCMAPPEDITEM(#15451,#16631);

#[derive(Default, Component, Clone)]
pub struct IfcWallStandardCase{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_product_definition: String,
    pub ifc_object_placement: String,
    pub ifc_product_definition_shape: String,
    pub ifc_identifier: String,
}
// #38060=IFCWALLSTANDARDCASE('3Y4YRln2r91vflHcHE5IVJ',#33,'Basic Wall:Interior - Furring (38 mm Stud):217417',$,'Basic Wall:Interior - Furring (38 mm Stud):128404',#38047,#38059,'217417');

#[derive(Default, Component, Clone)]
pub struct IfcCartesianTransformationOperator3D{
    pub idx: String,
    pub ifc_direction_1: String,
    pub ifc_direction_2: String,
    pub ifc_cartesian_point: String,
    pub ifc_scale: String,
    pub ifc_direction_3: String,
}
// #31390=IFCCARTESIANTRANSFORMATIONOPERATOR3D($,$,#3,1.,$);

#[derive(Default, Component, Clone)]
pub struct IfcStyledItem{
    pub idx: String,
    pub ifc_representation_item: String,
    pub ifc_styles: Vec<String>,
    pub ifc_label: String,
}
// #35354=IFCSTYLEDITEM(#35352,(#35353),$);

#[derive(Default, Component, Clone)]
pub struct IfcProductDefinitionShape{
    pub idx: String,
    pub ifc_label: String,
    pub ifc_text: String,
    pub ifc_representation: Vec<String>,
}
// #22662=IFCPRODUCTDEFINITIONSHAPE($,$,(#22653,#22661));

#[derive(Default, Component, Clone)]
pub struct IfcSurfaceOfLinearExtrusion{
    pub idx: String,
    pub ifc_arbitrary_open_profile_def: String,
    pub ifc_axis_2_placement_3d: String,
    pub ifc_direction: String,
    pub depth: f32,
}
// #96=IFCSURFACEOFLINEAREXTRUSION(#94,#95,#9,2.6);

#[derive(Default, Component, Clone)]
pub struct IfcPresentationStyleAssignment{
    pub idx: String,
    pub ifc_presentation_style_select: Vec<String>,
}
// #17964=IFCPRESENTATIONSTYLEASSIGNMENT((#17963));

#[derive(Default, Component, Clone)]
pub struct IfcMaterialLayerSetUsage{
    pub idx: String,
    pub ifc_material_layer_set: String,
    pub ifc_layer_set_direction_enum: String,
    pub ifc_direction_sense_enum: String,
    pub ifc_length_measure: String,
}
// #6108=IFCMATERIALLAYERSETUSAGE(#5842,.AXIS2.,.NEGATIVE.,0.092);

#[derive(Default, Component, Clone)]
pub struct IfcRelSpaceBoundary{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_text: String,
    pub ifc_space: String,
    pub ifc_element: String,
    pub ifc_connection_geometry: String,
    pub ifc_physical_or_virtual_enum: String,
    pub ifc_internal_or_external_enum: String,
}
// #38921=IFCRELSPACEBOUNDARY('3B6dOANnLEI9SPSTnuJl5m',#33,'1stLevel',$,#3707,#22708,#3738,.PHYSICAL.,.EXTERNAL.);

#[derive(Default, Component, Clone)]
pub struct IfcRelAssociatesMaterial{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_text: String,
    pub ifc_root: Vec<String>,
    pub ifc_material_select: String,
}
// #38412=IFCRELASSOCIATESMATERIAL('2cMQz3Ya57sxpYhx2ANHU1',#33,$,$,(#22798),#22827);

#[derive(Default, Component, Clone)]
pub struct IfcRepresentationMap{
    pub idx: String,
    pub ifc_axis2_placement: String,
    pub ifc_representation: String,
}
// #20642=IFCREPRESENTATIONMAP(#20640,#20639);

#[derive(Default, Component, Clone)]
pub struct IfcCircle{
    pub idx: String,
    pub ifc_axis2_placement: String,
    pub radius: f32,
}
// #37309=IFCCIRCLE(#37308,0.0199);

#[derive(Default, Component, Clone)]
pub struct IfcTrimmedCurve{
    pub idx: String,
    pub ifc_curve: String,
    pub ifc_trim_1: String,
    pub ifc_trim_2: String,
    pub ifc_sense: String,
    pub ifc_trimming_preference: String,
}
// #31823=IFCTRIMMEDCURVE(#31822,(IFCPARAMETERVALUE(90.00000000000009)),(IFCPARAMETERVALUE(269.9999999999999)),.T.,.PARAMETER.);

#[derive(Default, Component, Clone)]
pub struct IfcArbitraryClosedProfileDef{
    pub idx: String,
    pub ifc_profile_type_enum: String,
    pub ifc_label: String,
    pub ifc_bounded_curve: String,
}
// #21733=IFCARBITRARYCLOSEDPROFILEDEF(.AREA.,$,#21732);

#[derive(Default, Component, Clone)]
pub struct IfcCurveStyleFontPattern{
    pub idx: String,
    pub ifc_visible_segment_length: f32,
    pub ifc_invisible_segment_length: f32,
}
// #21308=IFCCURVESTYLEFONTPATTERN(0.033528,0.372872);

#[derive(Default, Component, Clone)]
pub struct IfcCurveBoundedPlane{
    pub idx: String,
    pub ifc_plane: String,
    pub ifc_outer_boundary: String,
    pub ifc_inner_boundaries: Vec<String>,
}
// #2499=IFCCURVEBOUNDEDPLANE(#2492,#2490,(#2498));

#[derive(Default, Component, Clone)]
pub struct IfcFaceBound{
    pub idx: String,
    pub ifc_loop: String,
    pub ifc_orientation: String,
}
// #14288=IFCFACEBOUND(#14287,.T.);

#[derive(Default, Component, Clone)]
pub struct IfcColourRgb{
    pub idx: String,
    pub ifc_label: String,
    pub ifc_red: f32,
    pub ifc_green: f32,
    pub ifc_blue: f32,
}
// #60=IFCCOLOURRGB($,0.,0.5686274509803921,0.788235294117647);

#[derive(Default, Component, Clone)]
pub struct IfcCurveStyle{
    pub idx: String,
    pub ifc_label: String,
    pub ifc_curve_font: String,
    pub ifc_curve_width: String,
    pub ifc_color: String,
}
// #3882=IFCCURVESTYLE($,#3881,$,#3880);

#[derive(Default, Component, Clone)]
pub struct IfcMaterialLayer{
    pub idx: String,
    pub ifc_material: String,
    pub ifc_layer_thickness: String,
    pub ifc_is_ventilated: String,
}
// #4359=IFCMATERIALLAYER(#3941,0.016,$);

#[derive(Default, Component, Clone)]
pub struct IfcSlab{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_product_definition: String,
    pub ifc_object_placement: String,
    pub ifc_product_definition_shape: String,
    pub ifc_identifier: String,
}
// #21336=IFCSLAB('2OBrcmyk58NupXoVOHUt0F',#33,'Floor:Finish Floor - Ceramic Tile:169866',$,'Floor:Finish Floor - Ceramic Tile',#21326,#21335,'169866',.FLOOR.);

#[derive(Default, Component, Clone)]
pub struct IfcRelConnectsPathElements{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_element_1: String,
    pub ifc_element_2: String,
    pub ifc_relating_priorities: String,
    pub ifc_related_priorities: String,
    pub ifc_related_connection_type: String,
    pub ifc_relating_connection_type: String,
}
// #38988=IFCRELCONNECTSPATHELEMENTS('0McGwuNxb88faofyDaYfbx',#33,$,$,$,#4287,#4508,(),(),.ATEND.,.ATPATH.);

#[derive(Default, Component, Clone)]
pub struct IfcFurnishingElement{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_product_label: String,
    pub ifc_object_placement: String,
    pub ifc_product_definition_shape: String,
    pub ifc_identifier: String,
}
// #16997=IFCFURNISHINGELEMENT('2OBrcmyk58NupXoVOHUvr5',#33,'M_Base Cabinet-Double Door & 2 Drawer:1000mm:1000mm:162496',$,
//'1000mm',#16996,#16990,'162496');

#[derive(Default, Component, Clone)]
pub struct IfcOpeningElement{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_product_label: String,
    pub ifc_object_placement: String,
    pub ifc_product_definition_shape: String,
    pub ifc_identifier: String,
}
// #7280=IFCOPENINGELEMENT('1xS3BCk291UvhgP2dvNoO$',#33,'M_Fixed:819mm x 759mm:819mm x 759mm:147994:1',$,'Opening',#7279,#7276,$);

#[derive(Default, Component, Clone)]
pub struct IfcRelVoidsElement{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_element: String,
    pub ifc_feature_element_subtraction: String,
}
// #22547=IFCRELVOIDSELEMENT('0t64iZK_nByQImgyrW5X44',#33,$,$,#22492,#22546);


#[derive(Default, Component, Clone)]
pub struct IfcCompositeCurve{
    pub idx: String,
    pub ifc_segments: Vec<String>,
    pub ifc_is_closed: String,
}
// #17992=IFCCOMPOSITECURVE((#17969,#17973,#17978,#17982,#17987,#17991),.F.);

#[derive(Default, Component, Clone)]
pub struct IfcDoor{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_product_definition: String,
    pub ifc_object_placement: String,
    pub ifc_product_definition_shape: String,
    pub ifc_identifier: String,
}
// #8066=IFCDOOR('1hOSvn6df7F8_7GcBWlS8Z',#33,'M_Single-Flush:0762 x 2032mm:0762 x 2032mm:150173',$,'0762 x 2032mm',#8065,#8060,'150173',2.032,0.7619999999999989);

#[derive(Default, Component, Clone)]
pub struct IfcGeometricSet{
    pub idx: String,
    pub ifc_elements: Vec<String>,
}
// #20638=IFCGEOMETRICSET((#20613,#20616,#20619,#20622,#20625,#20628,#20631,#20634,#20637));

#[derive(Default, Component, Clone)]
pub struct IfcRelFillsElement{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_relating_opening_element: String,
    pub ifc_related_building_element: String,
}
// #21901=IFCRELFILLSELEMENT('3BCyGluCb5gxw9EueiSUy5',#33,$,$,#21705,#21821);

#[derive(Default, Component, Clone)]
pub struct IfcMaterial{
    pub idx: String,
    pub ifc_label: String,
}
// #3902=IFCMATERIAL('Insulation / Thermal Barriers - Rigid insulation');

#[derive(Default, Component, Clone)]
pub struct IfcArbitraryProfileDefWithVoids{
    pub idx: String,
    pub ifc_profile_type_enum: String,
    pub ifc_label: String,
    pub ifc_outer_curve: String,
    pub ifc_inner_curve: Vec<String>,
}
//#7337=IFCARBITRARYPROFILEDEFWITHVOIDS(.AREA.,$,#7336,(#7330)); 


#[derive(Default, Component, Clone)]
pub struct IfcWindow{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_product_definition: String,
    pub ifc_object_placement: String,
    pub ifc_product_definition_shape: String,
    pub ifc_identifier: String,
    pub ifc_overall_height: f32,
    pub ifc_overall_width: f32,
}
// #7951=IFCWINDOW('1hOSvn6df7F8_7GcBWlS4Q',#33,'M_Fixed:819mm x 759mm:819mm x 759mm:149924',$,'819mm x 759mm',#7950,#7945,'149924',0.758999999999998,0.8190000000000001);

#[derive(Default, Component, Clone)]
pub struct IfcFurnitureType{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_applicable_occurrence: String,
    pub ifc_property_sets: String,
    pub ifc_representation_maps: Vec<String>,
    pub ifc_tag: String,
    pub ifc_identifier: String,
    pub ifc_assembly_place: String,
}
// #17518=IFCFURNITURETYPE('2OBrcmyk58NupXoVOHUsk4',#33,'0610 x 0610 x 0610mm',$,$,$,(#17516,#17517),'167584','0610 x 0610 x 0610mm',.NOTDEFINED.);

#[derive(Default, Component, Clone)]
pub struct IfcSpace{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_product_definition: String,
    pub ifc_object_placement: String,
    pub ifc_product_definition_shape: String,
    pub ifc_identifier: String,
    pub ifc_composite_type: String,
    pub ifc_interior_or_exterior_space: String,
    pub ifc_elevation_with_flooring: String,
}
// #819=IFCSPACE('0BTBFw6f90Nfh9rP1dlXri',#33,'A201','',$,#733,#818,'Hallway',.ELEMENT.,.INTERNAL.,$);

#[derive(Default, Component, Clone)]
pub struct IfcFaceBasedSurfaceModel{
    pub idx: String,
    pub ifc_connected_face_sets: Vec<String>,
}
// #15088=IFCFACEBASEDSURFACEMODEL((#14853,#14906,#14959,#15060,#15087));

#[derive(Default, Component, Clone)]
pub struct IfcSurfaceStyleRendering{
    pub idx: String,
    pub ifc_surface_colour: String,
    pub ifc_transparency: String,
    pub ifc_diffuse_colour: String,
    pub ifc_transmission_colour: String,
    pub ifc_diffuse_transmission_colour: String,
    pub ifc_reflection_colour: String,
    pub ifc_specular_colour: String,
    pub ifc_specular_highlight: String,
    pub ifc_reflectance_method: String,
}
// #6322=IFCSURFACESTYLERENDERING(#6321,0.,$,$,$,$,IFCNORMALISEDRATIOMEASURE(0.00390625),IFCSPECULAREXPONENT(128.),.NOTDEFINED.);

#[derive(Default, Component, Clone)]
pub struct IfcSurfaceStyle{
    pub idx: String,
    pub ifc_name: String,
    pub ifc_side: String,
    pub ifc_styles: Vec<String>,
}
// #8460=IFCSURFACESTYLE('Wood - Flooring',.BOTH.,(#8459));

#[derive(Default, Component, Clone)]
pub struct IfcWindowStyle{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_applicable_occurrence: String,
    pub ifc_property_sets: Vec<String>,
    pub ifc_representation_maps: Vec<String>,
    pub ifc_identifier: String,
    pub ifc_construction_type: String,
    pub ifc_operation_type: String,
    pub ifc_parameter_takes_precedence: String,
    pub ifc_sizeable: String,
}
// #6413=IFCWINDOWSTYLE('1Jx6Wzbs17ugoFEefHo1BC',#33,'4835mm x 2420mm',$,$,(#6412),(#6410,#6411),'145788',.NOTDEFINED.,.NOTDEFINED.,.F.,.F.);

#[derive(Default, Component, Clone)]
pub struct IfcCurveStyleFont{
    pub idx: String,
    pub ifc_label: String,
    pub ifc_pattern_list: Vec<String>,
}
// #21290=IFCCURVESTYLEFONT('Sand:1',(#21284,#21285,#21286,#21287,#21288,#21289));


#[derive(Default, Component, Clone)]
pub struct IfcRelDefinesByType{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_related_objects: Vec<String>,
    pub ifc_relating_type: String,
}
// #38482=IFCRELDEFINESBYTYPE('35bXALi9j0Nhnogn8tX0BU',#33,$,$,(#35783,#35854),#35772);

#[derive(Default, Component, Clone)]
pub struct IfcFillAreaStyleHatching{
    pub idx: String,
    pub ifc_curve_style: String,
    pub ifc_hatch_line_distance_select: String,
    pub ifc_point_of_reference_hatch_line: String,
    pub ifc_pattern_start: String,
    pub ifc_hatch_line_angle: String,
}
// #20864=IFCFILLAREASTYLEHATCHING(#20863,IFCPOSITIVELENGTHMEASURE(0.299999984114212),$,#4,90.);


#[derive(Default, Component, Clone)]
pub struct IfcQuantityArea{
    pub idx: String,
    pub ifc_name: String,
    pub ifc_description: String,
    pub ifc_unit: String,
    pub ifc_area_value: f32,
}
// #2109=IFCQUANTITYAREA('GSA BIM Area',$,$,17.93623674999994);



#[derive(Default, Component, Clone)]
pub struct IfcElementQuantity{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_method_of_measurement: String,
    pub ifc_quantities: Vec<String>,
}
// #1930=IFCELEMENTQUANTITY('2EpJAn0FzEoe9ftyEgraot',#33,'GSA Space Areas',$,'GSA BIM Area',(#1929));

#[derive(Default, Component, Clone)]
pub struct IfcMaterialDefinitionRepresentation{
    pub idx: String,
    pub ifc_name: String,
    pub ifc_description: String,
    pub ifc_representations: Vec<String>,
    pub ifc_represented_material: String,
}
// #3980=IFCMATERIALDEFINITIONREPRESENTATION($,$,(#3979),#3941);

#[derive(Default, Component, Clone)]
pub struct IfcStyledRepresentation{
    pub idx: String,
    pub ifc_context_of_items: String,
    pub ifc_representation_identifier: String,
    pub ifc_representation_type: String,
    pub ifc_items: Vec<String>,
}
// #3979=IFCSTYLEDREPRESENTATION(#29,'Style','Material and Cut Pattern',(#3978));

#[derive(Default, Component, Clone)]
pub struct IfcRelContainedInSpatialStructure{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_related_elements: Vec<String>,
    pub ifc_relating_structure: String,
}
// #38348=IFCRELCONTAINEDINSPATIALSTRUCTURE('2V7GTP4vX5SxzTYePgxo_z',#33,$,$,(#36125,#36151),#1059);

#[derive(Default, Component, Clone)]
pub struct IfcPresentationLayerAssignment{
    pub idx: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_related_elements: Vec<String>,
    pub ifc_identifier: String,
}
// #39110=IFCPRESENTATIONLAYERASSIGNMENT('A-ROOF',$,(#22489,#22528,#22541),$);

#[derive(Default, Component, Clone)]
pub struct IfcCovering{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_object_type: String,
    pub ifc_object_placement: String,
    pub ifc_representation: String,
    pub ifc_tag: String,
    pub ifc_type: String,
}
// #24544=IFCCOVERING('3bXiCStxP6Fgxdej$yc55m',#33,'Compound Ceiling:Gypsum Board:187701',$,'Compound Ceiling:Gypsum Board',#24503,#24543,'187701',.CEILING.);

#[derive(Default, Component, Clone)]
pub struct IfcFillAreaStyle{
    pub idx: String,
    pub ifc_name: String,
    pub ifc_fill_styles: Vec<String>,
}
// #3976=IFCFILLAREASTYLE('Sand',(#3955,#3965,#3975));

#[derive(Default, Component, Clone)]
pub struct IfcMaterialLayerSet{
    pub idx: String,
    pub ifc_material_layers: Vec<String>,
    pub ifc_layer_set_name: String,
}
// #4202=IFCMATERIALLAYERSET((#4199,#4200,#4201),'Basic Wall:Interior - Partition (92mm Stud)');

#[derive(Default, Component, Clone)]
pub struct IfcPolygonalBoundedHalfSpace{
    pub idx: String,
    pub ifc_base_surface: String,
    pub ifc_agreement_flag: String,
    pub ifc_position: String,
    pub ifc_polygonal_boundary: String,
}
// #4272=IFCPOLYGONALBOUNDEDHALFSPACE(#4269,.T.,#4271,#4266);

#[derive(Default, Component, Clone)]
pub struct IfcWindowLiningProperties{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_lining_depth: String,
    pub ifc_lining_thickness: String,
    pub ifc_transom_thickness: String,
    pub ifc_mullion_thickness: String,
    pub ifc_first_transom_offset: String,
    pub ifc_second_transom_offset: String,
    pub ifc_first_mullion_offset: String,
    pub ifc_second_mullion_offset: String,
    pub ifc_shape_aspect_style: String,
}
// #7625=IFCWINDOWLININGPROPERTIES('0rYYHuosj16R8igFUVugC5',#33,'M_Casement:819mm x 759mm:819mm x 759mm:148607',$,$,$,$,$,$,$,$,$,$);

#[derive(Default, Component, Clone)]
pub struct IfcBeam{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_object_type: String,
    pub ifc_object_placement: String,
    pub ifc_representation: String,
    pub ifc_tag: String
}
// #37716=IFCBEAM('2OrWItJ6zAwBNp0OUxK$Dw',#33,'M_W-Wide Flange:W410X60:W410X60:209263',$,'M_W-Wide Flange:W410X60:208814',#37638,#37715,'209263');

#[derive(Default, Component, Clone)]
pub struct IfcCircleProfileDef{
    pub idx: String,
    pub ifc_profile_type: String,
    pub ifc_profile_name: String,
    pub ifc_position: String,
    pub ifc_radius: f32,
}
// #9754=IFCCIRCLEPROFILEDEF(.AREA.,$,#9753,0.009999999999999999);

#[derive(Default, Component, Clone)]
pub struct IfcFooting{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_object_type: String,
    pub ifc_object_placement: String,
    pub ifc_representation: String,
    pub ifc_tag: String,
    pub ifc_predefined_type: String,
}
// #23562=IFCFOOTING('0kF45Qs8L9PAM9kmb1lT2Z',#33,'Wall Foundation:Bearing Footing - 900 x 300:186656',$,'Wall Foundation:Bearing Footing - 900 x 300',#23552,#23561,'186656',.STRIP_FOOTING.);

#[derive(Default, Component, Clone)]
pub struct IfcDoorStyle{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_applicable_occurrence: String,
    pub ifc_property_sets: Vec<String>,
    pub ifc_representation_maps: Vec<String>,
    pub ifc_identifier: String,
    pub ifc_construction_type: String,
    pub ifc_operation_type: String,
    pub ifc_parameter_takes_precedence: String,
    pub ifc_sizeable: String,
}
// #8270=IFCDOORSTYLE('3HoP2o48vFlPEmd6lsQCdI',#33,'0864 x 2032mm',$,$,(#8269),(#8267,#8268),'150378',.SINGLE_SWING_RIGHT.,.NOTDEFINED.,.F.,.F.);





#[derive(Default, Component, Clone)]
pub struct IfcRelAggregates{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_relating_object: String,
    pub ifc_related_objects: Vec<String>,
}
// #38331=IFCRELAGGREGATES('3ym82ISZP8DxxqI6AGAA9k',#33,$,$,#36,(#39,#43,#47,#51));

#[derive(Default, Component, Clone)]
pub struct IfcBooleanClippingResult{
    pub idx: String,
    pub ifc_operator: String,
    pub ifc_representation_item: String,
    pub ifc_geometric_representation_item: String,
}
// #4273=IFCBOOLEANCLIPPINGRESULT(.DIFFERENCE.,#4259,#4272);

#[derive(Default, Component, Clone)]
pub struct IfcBuildingStorey{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_object_type: String,
    pub ifc_object_placement: String,
    pub ifc_representation: String,
    pub ifc_long_name: String,
    pub ifc_composition_type: String,
    pub ifc_elevation: String,
}
// #51=IFCBUILDINGSTOREY('1xS3BCk291UvhgP2dvNtSE',#33,'Roof',$,$,#50,$,$,.ELEMENT.,6.00000000000039);








#[derive(Default, Component, Clone)]
pub struct IfcRailing{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_object_type: String,
    pub ifc_object_placement: String,
    pub ifc_representation: String,
    pub ifc_tag: String,
    pub ifc_predefined_type: String,
}
// #35163=IFCRAILING('21ldoMpbP4VfsJ0XGY_335',#33,'Railing:1100mm Guard Rail:198972',$,'Railing:1100mm Guard Rail',#32761,#35162,'198972',.NOTDEFINED.);

#[derive(Default, Component, Clone)]
pub struct IfcDoorLiningProperties{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_lining_depth: String,
    pub ifc_lining_thickness: String,
    pub ifc_threshold_depth: String,
    pub ifc_threshold_thickness: String,
    pub ifc_transom_thickness: String,
    pub ifc_transom_offset: String,
    pub ifc_lining_offset: String,
    pub ifc_threshold_offset: String,
    pub ifc_casing_thickness: String,
    pub ifc_casing_depth: String,
    pub ifc_shape_aspect_style: String,
}
// #21807=IFCDOORLININGPROPERTIES('0gGQg5l1T9CAZUDzePJ9wW',#33,'M_Single-Glass 1:0813 x 2420mm:0813 x 2420mm:171853',$,$,$,$,$,$,$,$,$,$,$,$);

#[derive(Default, Component, Clone)]
pub struct IfcStair{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_product_definition: String,
    pub ifc_object_placement: String,
    pub ifc_product_definition_shape: String,
    pub ifc_identifier: String,
    pub ifc_shape_type: String,
}
// #32116=IFCSTAIR('21ldoMpbP4VfsJ0XGY_34d',#33,'Stair:Residential - 200mm Max Riser 250mm Tread:198878',$,'Stair:Residential - 200mm Max Riser 250mm Tread',#31507,$,'198878',.STRAIGHT_RUN_STAIR.);

#[derive(Default, Component, Clone)]
pub struct IfcStairFlight{
    pub idx: String,
    pub ifc_global_id: String,
    pub ifc_owner_history: String,
    pub ifc_label: String,
    pub ifc_name: String,
    pub ifc_product_definition: String,
    pub ifc_object_placement: String,
    pub ifc_product_definition_shape: String,
    pub ifc_identifier: String,
    pub ifc_number_of_riser: String,
    pub ifc_number_of_treads: String,
    pub ifc_riser_height: String,
    pub ifc_tread_length: String,
}
// #32063=IFCSTAIRFLIGHT('3KMJUyUe9DfQ2FOCd5ZoiN',#33,'Stair:Residential - 200mm Max Riser 250mm Tread:198878:1',$,'Stair:Residential - 200mm Max Riser 250mm Tread',#32062,#32061,'198878',16,15,0.6356627296588328,0.8202099737532809);


